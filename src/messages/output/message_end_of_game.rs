use serde::Serialize;
use crate::messages::output::message_public_leader_board::PublicPlayer;

#[derive(Serialize, Clone)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}
