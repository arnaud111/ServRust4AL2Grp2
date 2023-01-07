use serde::Serialize;

#[derive(Serialize)]
pub struct RecoverSecretOutput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}
