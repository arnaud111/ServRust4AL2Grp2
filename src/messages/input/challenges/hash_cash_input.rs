use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MD5HashCashInput {
    pub seed : u64,
    pub hashcode : String
}
