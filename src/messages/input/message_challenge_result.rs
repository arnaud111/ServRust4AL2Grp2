use serde::Deserialize;
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