use serde::Serialize;

#[derive(Serialize)]
pub struct RecoverSecretOutput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

impl RecoverSecretOutput {

    pub fn clone(&self) -> RecoverSecretOutput {
        RecoverSecretOutput {
            word_count: self.word_count,
            letters: self.letters.clone(),
            tuple_sizes: self.tuple_sizes.clone()
        }
    }
}
