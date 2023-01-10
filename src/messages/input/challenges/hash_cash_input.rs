use serde::Deserialize;

#[derive(Deserialize)]
pub struct MD5HashCashInput {
    pub seed : u64,
    pub hashcode : String
}

impl MD5HashCashInput {

    pub fn clone(&self) -> MD5HashCashInput {
        MD5HashCashInput {
            seed: self.seed,
            hashcode: self.hashcode.clone()
        }
    }
}