use serde::Serialize;

#[derive(Serialize)]
pub struct ChallengeTimeout {
    message: String
}

impl ChallengeTimeout {

    pub fn clone(&self) -> ChallengeTimeout {
        ChallengeTimeout {
            message: self.message.clone()
        }
    }
}