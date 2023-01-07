use serde::Deserialize;

#[derive(Deserialize)]
pub struct MD5HashCashInput {
    pub seed : u64,
    pub hashcode : String
}