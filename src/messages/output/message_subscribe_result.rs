use serde::Serialize;
use crate::read;

#[derive(Serialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

impl SubscribeResult {

    pub fn clone(&self) -> SubscribeResult {
        return match self {
            SubscribeResult::Ok => self.clone(),
            SubscribeResult::Err(err) => err.clone()
        }
    }
}

impl SubscribeError {

    pub fn clone(&self) -> SubscribeError {
        return self.clone()
    }
}
