use serde::Deserialize;

#[derive(Deserialize)]
pub struct StartGame {
    pub key: String
}