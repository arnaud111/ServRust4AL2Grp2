use serde::Serialize;
use crate::messages::input::message_challenge_result::ChallengeAnswer;
use crate::messages::output::challenges::hash_cash_output::Md5HashCashOutput;
use crate::messages::output::challenges::nonogram_output::NonogramSolverOutput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;

#[derive(Serialize, Clone)]
pub enum ChallengeMessage {
    MD5HashCash(Md5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
    NonogramSolver(NonogramSolverOutput)
}
