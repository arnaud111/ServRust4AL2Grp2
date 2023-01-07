use serde::Serialize;

#[derive(Serialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Serialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeResult
}

#[derive(Serialize)]
enum ChallengeResult {
    Ok(ChallengeOk),
    Unreachable
}

#[derive(Serialize)]
struct ChallengeOk {
    used_time: f32,
    next_target: String
}
