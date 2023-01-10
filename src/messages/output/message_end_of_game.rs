use serde::Serialize;
use crate::messages::output::message_public_leader_board::PublicPlayer;

#[derive(Serialize)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}

impl EndOfGame {

    pub fn clone(&self) -> EndOfGame {
        EndOfGame {
            leader_board: self.leader_board.clone()
        }
    }
}
