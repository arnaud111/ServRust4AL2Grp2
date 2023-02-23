use std::collections::HashMap;
use std::net::TcpStream;
use serde::Deserialize;
use crate::{ChallengeMessage, MessageOutputType, SubscribeError, SubscribeResult, Welcome};
use crate::messages::input::message_challenge_result::ChallengeResult;
use crate::messages::input::message_start_game::StartGame;
use crate::messages::input::message_subscribe::Subscribe;
use crate::messages::output::message_public_leader_board::PublicPlayer;

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

    pub fn get_next_target(&self) -> String {
        return match self {
            MessageInputType::ChallengeResult(result) => {
                result.next_target.clone()
            }
            _ => String::new()
        }
    }

    pub fn match_challenge_result(&self, challenge_sent: ChallengeMessage) -> bool {
        return match self {
            MessageInputType::ChallengeResult(result) => {
                result.answer.is_true(challenge_sent)
            }
            _ => false
        }
    }

    pub fn match_message(&self, client_names: &mut HashMap<String, (TcpStream, PublicPlayer)>, stream: &TcpStream) -> MessageInputResult {

        match self {
            MessageInputType::Hello => {
                return MessageInputResult::MessageOutputType(MessageOutputType::Welcome(Welcome { version: 1 }));
            }
            MessageInputType::Subscribe(subscribe_message) => {
                return if client_names.contains_key(&*subscribe_message.name) {
                    MessageInputResult::MessageOutputType(MessageOutputType::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered)))
                } else {
                    let player = PublicPlayer {
                        name: subscribe_message.name.clone(),
                        stream_id: stream.peer_addr().expect("").to_string(),
                        score: 0,
                        steps: 0,
                        is_active: true,
                        total_used_time: 0.0
                    };
                    client_names.insert(subscribe_message.name.clone(), (stream.try_clone().expect(""), player));
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
