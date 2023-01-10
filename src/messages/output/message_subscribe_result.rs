use serde::Serialize;
use crate::read;

#[derive(Serialize, Clone)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize, Clone)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}
