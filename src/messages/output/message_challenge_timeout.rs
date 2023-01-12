use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ChallengeTimeout {
    message: String
}
