use serde::Serialize;
use crate::messages::input::message_challenge_result::ChallengeAnswer;
use crate::messages::output::challenges::hash_cash_output::Md5HashCashOutput;
use crate::messages::output::challenges::nonogram_output::NonogramSolverOutput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;

#[derive(Serialize)]
pub enum ChallengeMessage {
    MD5HashCash(Md5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
    NonogramSolver(NonogramSolverOutput)
}

impl ChallengeMessage {
    pub fn clone(&self) -> ChallengeMessage {
        return match self {
            ChallengeMessage::MD5HashCash(val) => {
                ChallengeMessage::MD5HashCash(val.clone())
            }
            ChallengeMessage::RecoverSecret(val) => {
                ChallengeMessage::RecoverSecret(val.clone());
            }
            ChallengeMessage::NonogramSolver(val) => {
                ChallengeMessage::NonogramSolver(val.clone())
            }
        }
    }
}
