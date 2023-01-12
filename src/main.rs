mod messages;
mod challenge_compute;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str;
use std::env;
use std::time::Duration;
use crate::challenge_compute::challenge_recover_secret::RecoverSecret;
use crate::challenge_compute::hash_cash::HashCash;
use crate::messages::input::messages_input_types::{MessageInputResult, MessageInputType};
use crate::messages::output::message_challenge::ChallengeMessage;
use crate::messages::output::message_subscribe_result::{SubscribeError, SubscribeResult};
use crate::messages::output::message_welcome::Welcome;
use crate::messages::output::messages_output_types::MessageOutputType;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let mut client_names = HashMap::new();
    let complexity = 16;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        if receive_new_stream(&stream, &mut client_names) {
            break;
        }
    }
    if client_names.len() == 0 {
        return;
    }
    receive_start_game(&listener);
    play(&mut client_names, complexity);

    for stream in client_names.values() {
        stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
    }
}

fn play(client_names: &mut HashMap<String, TcpStream>, complexity: i32) {
    let mut actual_player = String::new();
    for key in client_names.keys() {
        actual_player = key.clone();
    }

    for i in 0..100 {
        println!("round : {}", i);
        println!("{}", actual_player);
        let challenge_output = HashCash::new(complexity);
        let message_out = MessageOutputType::Challenge(ChallengeMessage::MD5HashCash(challenge_output.clone()));
        send(&client_names[&actual_player], message_out.clone());
        let challenge_result = read(&client_names[&actual_player], Duration::from_secs(2));
        match challenge_result {
            None => {
                client_names.remove(&actual_player);
            },
            Some(result) => {
                let is_valid = result.match_challenge_result(ChallengeMessage::MD5HashCash(challenge_output));
                match is_valid {
                    None => {
                        client_names.remove(&actual_player);
                    },
                    Some(valid) => {
                        println!("{}", valid);
                    }
                }
            }
        }
    }
}

fn receive_start_game(listener: &TcpListener) {
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let optional_message_in = read(&stream, Duration::from_secs(2));
        match optional_message_in {
            None => {}
            Some(message_in) => {
                match message_in {
                    MessageInputType::StartGame(_) => return,
                    _ => {}
                }
            }
        }
    }
}

fn receive_new_stream(stream: &TcpStream, client_names: &mut HashMap<String, TcpStream>) -> bool {
    loop {
        let optional_message_in = read(stream, Duration::from_secs(2));
        match optional_message_in {
            None => break,
            Some(message_in) => {
                let optional_message_out = message_in.match_message(client_names, stream);
                match optional_message_out {
                    MessageInputResult::MessageOutputType(message_out) => {
                        send(stream, message_out);
                    }
                    MessageInputResult::LaunchGame => return true,
                    MessageInputResult::None => return false
                }
            }
        }
    }
    false
}

fn read (mut stream: &TcpStream, timeout: Duration) -> Option<MessageInputType> {

    stream.set_read_timeout(Option::from(timeout));

    let mut nb = [0; 4];
    match stream.read(&mut nb) {
        Ok(_) => {}
        Err(_) => return None
    };
    let nb = i32::from_be_bytes(nb);

    let mut str_bytes = vec![0; nb as usize];
    match stream.read_exact(&mut str_bytes) {
        Ok(_) => {}
        Err(_) => return None
    };

    let str = str::from_utf8(&str_bytes).unwrap();
    println!("Read : {}", str);

    let message: MessageInputType = match serde_json::from_str(str) {
        Ok(message) => message,
        Err(_) => return None
    };
    return Option::from(message);
}

fn send(mut stream: &TcpStream, message: MessageOutputType){

    let str = &*serde_json::to_string(&message).unwrap();
    println!("Send : {}", str);
    let str_bytes = str.as_bytes();

    let nb: u32 = str_bytes.len() as u32;

    let mut buf= vec![0; 4];
    buf = Vec::from(nb.to_be_bytes());

    for x in str_bytes {
        buf.push(*x);
    }

    stream.write(&buf).expect("Error Sending Message");
}
