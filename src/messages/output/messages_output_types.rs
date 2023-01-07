use std::option::Option;
use serde::Serialize;
use crate::messages::output::message_challenge::ChallengeMessage;
use crate::messages::output::message_challenge_timeout::ChallengeTimeout;
use crate::messages::output::message_end_of_game::EndOfGame;
use crate::messages::output::message_public_leader_board::PublicPlayer;
use crate::messages::output::message_round_summary::RoundSummary;
use crate::messages::output::message_subscribe_result::SubscribeResult;
use crate::messages::output::message_welcome::Welcome;

#[derive(Serialize)]
pub enum MessageOutputType {
    Welcome(Welcome),
    Challenge(ChallengeMessage),
    SubscribeResult(SubscribeResult),
    ChallengeTimeout(ChallengeTimeout),
    PublicLeaderBoard(Vec<PublicPlayer>),
    EndOfGame(EndOfGame),
    RoundSummary(RoundSummary)
}