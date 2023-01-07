use serde::Serialize;

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
