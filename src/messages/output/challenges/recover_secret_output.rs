use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RecoverSecretOutput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}
