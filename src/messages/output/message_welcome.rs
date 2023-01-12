use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Welcome {
    pub version: u8,
}
