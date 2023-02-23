mod messages;
mod challenge_compute;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str;
use std::env;
use std::time::{Duration, Instant};
use crate::challenge_compute::challenge_recover_secret::RecoverSecret;
use crate::challenge_compute::hash_cash::HashCash;
use crate::messages::input::messages_input_types::{MessageInputResult, MessageInputType};
use crate::messages::output::challenges::hash_cash_output::Md5HashCashOutput;
use crate::messages::output::message_challenge::ChallengeMessage;
use crate::messages::output::message_end_of_game::EndOfGame;
use crate::messages::output::message_public_leader_board::PublicPlayer;
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
    send_all_end_of_game(&mut client_names);

    for stream in client_names.values() {
        stream.0.shutdown(Shutdown::Both);
    }
}

fn play(client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>, complexity: i32) {
    let mut actual_player = get_first_player_actif(client_names);

    for _ in 0..100 {
        match round(&mut actual_player, client_names, complexity) {
            None => {
                actual_player = get_first_player_actif(client_names);
            }
            Some(last_player) => {
                actual_player = last_player;
            }
        }
    }
}

fn get_first_player_actif(client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>) -> String {
    let mut random_player = String::new();
    for (key, value) in client_names.iter() {
        if value.1.is_active {
            random_player = key.clone();
            break;
        }
    }
    random_player
}

fn round(actual_player: &mut String, client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>, complexity: i32) -> Option<String> {

    let timer = Instant::now();
    let mut challenge_output;
    let mut challenge_result;
    let mut last_player = actual_player.clone();
    let mut timer_challenge;

    while timer.elapsed().as_secs() < 2 {
        timer_challenge = Instant::now();
        challenge_output = send_challenge(actual_player, client_names, complexity);
        challenge_result = read(&client_names[actual_player].0, Duration::from_secs(2));
        match challenge_result {
            None => {
                if let Some((_, public_player)) = client_names.get_mut(actual_player) {
                    public_player.is_active = false;
                }
                return None;
            },
            Some(result) => {
                let is_valid = result.match_challenge_result(ChallengeMessage::MD5HashCash(challenge_output.clone()));
                if is_valid {
                    if let Some((_, public_player)) = client_names.get_mut(actual_player) {
                        public_player.steps += 1;
                    }
                    last_player = actual_player.clone();
                    *actual_player = result.get_next_target();
                    if !client_names.contains_key(actual_player) || client_names[actual_player].1.is_active == false {
                        *actual_player = last_player.clone();
                    }
                }
            }
        }
        if let Some((_, public_player)) = client_names.get_mut(&last_player) {
            public_player.total_used_time += timer_challenge.elapsed().as_secs_f32();
        }
        send_all_public_leader_board(client_names);
    }

    if let Some((_, public_player)) = client_names.get_mut(&last_player) {
        public_player.score -= 1;
    }

    Option::from(actual_player.clone())
}

fn send_all_public_leader_board(client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>) {
    let mut public_leader_board = Vec::new();
    for (_, value) in client_names.into_iter() {
        public_leader_board.push(value.1.clone());
    }
    let message_out = MessageOutputType::PublicLeaderBoard(public_leader_board);
    for (_, value) in client_names.into_iter() {
        send(&value.0, message_out.clone());
    }
}

fn send_all_end_of_game(client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>) {
    let mut public_leader_board = Vec::new();
    for (_, value) in client_names.into_iter() {
        public_leader_board.push(value.1.clone());
    }
    let message_out = MessageOutputType::EndOfGame(EndOfGame { leader_board: public_leader_board });
    for (_, value) in client_names.into_iter() {
        send(&value.0, message_out.clone());
    }
}

fn send_challenge(actual_player: &String, client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>, complexity: i32) -> Md5HashCashOutput {
    let challenge_output = HashCash::new(complexity);
    let message_out = MessageOutputType::Challenge(ChallengeMessage::MD5HashCash(challenge_output.clone()));
    send(&client_names[actual_player].0, message_out.clone());
    challenge_output
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

fn receive_new_stream(stream: &TcpStream, client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>) -> bool {
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

    let str = match str::from_utf8(&str_bytes) {
        Ok(str) => str,
        Err(_) => ""
    };
    println!("Read : {}", str);

    let message: MessageInputType = match serde_json::from_str(str) {
        Ok(message) => message,
        Err(_) => return None
    };
    return Option::from(message);
}

fn send(mut stream: &TcpStream, message: MessageOutputType){

    let str = match serde_json::to_string(&message) {
        Ok(str) => str,
        Err(_) => "".to_string()
    };
    println!("Send : {}", str);
    let str_bytes = str.as_bytes();

    let nb: u32 = str_bytes.len() as u32;

    let mut buf= vec![0; 4];
    buf = Vec::from(nb.to_be_bytes());

    for x in str_bytes {
        buf.push(*x);
    }

    stream.write(&buf);
}
