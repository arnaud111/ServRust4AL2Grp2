mod messages;
mod challenge_compute;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str;
use std::env;
use std::time::Duration;
use crate::messages::input::messages_input_types::{MessageInputResult, MessageInputType};
use crate::messages::output::message_subscribe_result::{SubscribeError, SubscribeResult};
use crate::messages::output::message_welcome::Welcome;
use crate::messages::output::messages_output_types::MessageOutputType;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let mut client_names = HashMap::new();

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
    play(&client_names);

    for stream in client_names.values() {
        stream.shutdown(Shutdown::Both).expect("Error shutdown connexion");
    }
}

fn play(client_names: &HashMap<String, TcpStream>) {
    let mut actual_player: String;
    for key in client_names.keys() {
        actual_player = key.clone();
    }

    for _ in 0..100 {

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

fn read (stream: &TcpStream, timeout: Duration) -> Option<MessageInputType> {

    let mut stream = stream.try_clone().expect("");
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

fn send(stream: &TcpStream, message: MessageOutputType){

    let mut stream = stream.try_clone().expect("");

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
