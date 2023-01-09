use std::collections::HashMap;
use std::net::TcpStream;
use serde::Deserialize;
use crate::{MessageOutputType, SubscribeError, SubscribeResult, Welcome};
use crate::messages::input::message_challenge_result::ChallengeResult;
use crate::messages::input::message_start_game::StartGame;
use crate::messages::input::message_subscribe::Subscribe;

#[derive(Deserialize)]
pub enum MessageInputType {
    Hello,
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult),
    StartGame(StartGame)
}

pub enum MessageInputResult {
    MessageOutputType(MessageOutputType),
    LaunchGame,
    None
}

impl MessageInputType {

    pub fn match_message(&self, client_names: &mut HashMap<String, TcpStream>, stream: &TcpStream) -> MessageInputResult {

        match self {
            MessageInputType::Hello => {
                return MessageInputResult::MessageOutputType(MessageOutputType::Welcome(Welcome { version: 1 }));
            }
            MessageInputType::Subscribe(subscribe_message) => {
                return if client_names.contains_key(&*subscribe_message.name) {
                    MessageInputResult::MessageOutputType(MessageOutputType::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered)))
                } else {
                    client_names.insert(subscribe_message.name.clone(), stream.try_clone().expect(""));
                    MessageInputResult::MessageOutputType(MessageOutputType::SubscribeResult(SubscribeResult::Ok))
                }
            }
            MessageInputType::ChallengeResult(_) => {}
            MessageInputType::StartGame(start_game_message) => {
                if start_game_message.key == "1234" {
                    return MessageInputResult::LaunchGame;
                }
            }
        }

        MessageInputResult::None
    }
}
