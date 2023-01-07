use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecoverSecretInput {
    pub secret_sentence: String,
}