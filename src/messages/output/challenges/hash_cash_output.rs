use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Md5HashCashOutput {
    pub complexity : i32,
    pub message : String
}
