use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i8,
    pub steps: u64,
    pub is_active: bool,
    pub total_used_time: f32
}
