use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RecoverSecretInput {
    pub secret_sentence: String,
}