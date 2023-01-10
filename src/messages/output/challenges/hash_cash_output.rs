use serde::Serialize;

#[derive(Serialize)]
pub struct Md5HashCashOutput {
    pub complexity : i32,
    pub message : String
}

impl Md5HashCashOutput {

    pub fn clone(&self) -> Md5HashCashOutput {
        Md5HashCashOutput {
            complexity: self.complexity,
            message: self.message.clone()
        }
    }
}
