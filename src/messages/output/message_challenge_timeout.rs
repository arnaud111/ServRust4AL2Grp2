use serde::Serialize;

#[derive(Serialize)]
pub struct ChallengeTimeout {
    message: String
}