use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Serialize, Clone)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeResult
}

#[derive(Serialize, Clone)]
enum ChallengeResult {
    Ok(ChallengeOk),
    Unreachable
}

#[derive(Serialize, Clone)]
struct ChallengeOk {
    used_time: f32,
    next_target: String
}
