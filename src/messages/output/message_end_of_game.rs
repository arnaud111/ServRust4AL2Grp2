use serde::Serialize;
use crate::messages::output::message_public_leader_board::PublicPlayer;

#[derive(Serialize, Clone)]
pub struct EndOfGame {
    pub(crate) leader_board: Vec<PublicPlayer>
}
