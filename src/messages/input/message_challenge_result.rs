use serde::Deserialize;
use crate::{ChallengeMessage, HashCash};
use crate::messages::input::challenges::hash_cash_input::MD5HashCashInput;
use crate::messages::input::challenges::nonogram_input::NonogramSolverInput;
use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;

#[derive(Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashInput),
    NonogramSolver(NonogramSolverInput),
    RecoverSecret(RecoverSecretInput)
}

impl ChallengeAnswer {

    pub fn is_true(&self, challenge_sent: ChallengeMessage) -> Option<bool> {
        return match self {
            ChallengeAnswer::MD5HashCash(md5) => {
                match challenge_sent {
                    ChallengeMessage::MD5HashCash(md5_sent) => {
                        let hash_cash = HashCash {
                            input: md5.clone(),
                            output: md5_sent
                        };
                        Option::from(hash_cash.is_valid())
                    }
                    _ => None
                }
            }
            ChallengeAnswer::NonogramSolver(nonogram) => {
                None
            }
            ChallengeAnswer::RecoverSecret(secret) => {
                None
            }
        }
    }
}