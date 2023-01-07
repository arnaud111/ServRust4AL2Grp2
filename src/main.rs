mod messages;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::env;
use crate::messages::input::messages_input_types::MessageInputType;
use crate::messages::output::message_welcome::Welcome;
use crate::messages::output::messages_output_types::MessageOutputType;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let mut client_streams = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let client_ip = stream.peer_addr().unwrap();

        client_streams.insert(client_ip, stream.try_clone().expect(""));

        let input_message = read(stream.try_clone().expect(""));

        match input_message {
            MessageInputType::Hello => {
                let welcome = MessageOutputType::Welcome(Welcome {version: 1});
                send(stream.try_clone().expect(""), welcome);
            }
            MessageInputType::Subscribe(_) => {}
            MessageInputType::ChallengeResult(_) => {}
        }
    }
}

fn read (mut stream: TcpStream) -> MessageInputType {
    loop {
        let mut nb = [0; 4];
        stream.read(&mut nb).expect("Error Reading");
        let nb = i32::from_be_bytes(nb);

        if nb > 0 {
            let mut str_bytes = vec![0; nb as usize];
            stream.read_exact(&mut str_bytes).expect("Error Reading");
            let str = str::from_utf8(&str_bytes).unwrap();
            println!("Read : {}", str);

            let message: MessageInputType = match serde_json::from_str(str) {
                Ok(message) => message,
                Err(_) => continue
            };
            return message;
        }
    }
}

fn send(mut stream: TcpStream, message: MessageOutputType){

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
