use serde::Deserialize;
use crate::messages::input::message_challenge_result::ChallengeResult;
use crate::messages::input::message_subscribe::Subscribe;

#[derive(Deserialize)]
pub enum MessageInputType {
    Hello,
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult)
}
