use serde::Serialize;

#[derive(Serialize)]
pub struct Welcome {
    pub version: u8,
}
