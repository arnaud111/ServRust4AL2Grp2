use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;
use rand::{thread_rng, Rng};
use crate::messages::input::message_challenge_result::ChallengeAnswer::RecoverSecret;

pub struct RecoverSecret {
    input: RecoverSecretInput,
    output: RecoverSecretOutput
}

impl RecoverSecret {
    
    pub fn new() -> RecoverSecretOutput {
        let mut rng = thread_rng();
        let sentence = RecoverSecret::create_sentence();
        let mut is_used: [bool; 20] = [false; 20];
        let mut letters=  String::new();
        let mut tuple_sizes= Vec::new();

        while !RecoverSecret::all_used(&is_used) {
            let size: usize = rng.gen_range(4, 10);
            let mut vec = RecoverSecret::create_vec_index_letters(size);
            while RecoverSecret::all_already_used(&vec, &is_used) {
                vec = RecoverSecret::create_vec_index_letters(size);
            }
            for i in 0..vec.len() {
                letters.push(sentence.chars()[vec[i]]);
                is_used[i] = true;
            }
            tuple_sizes.push(size);
        }

        RecoverSecretOutput {
            word_count: 1,
            letters,
            tuple_sizes
        }
    }

    fn create_vec_index_letters(size: usize) -> Vec<usize> {
        let mut vec = Vec::new();
        for _ in 0..size {
            let mut letter_index = rng.gen_range(0, 20);
            while RecoverSecret::in_vec(&vec, letter_index) {
                letter_index = rng.gen_range(0, 20);
            }
            vec.push(letter_index);
        }
        return vec;
    }

    fn all_already_used(vec: &Vec<usize>, is_used: &[bool; 20]) -> bool {
        for i in 0..vec.len() {
            if !is_used[i] {
                return false;
            }
        }
        true
    }

    fn in_vec(vec: &Vec<usize>, number: usize) -> bool {
        for i in 0..vec.len() {
            if vec[i] == number {
                return true;
            }
        }
        false
    }

    fn all_used(is_used: &[bool; 20]) -> bool {
        for i in 0..is_used.len() {
            if !is_used[i] {
                return false;
            }
        }
        true
    }

    fn create_sentence() -> String {
        let mut rng = thread_rng();
        let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPKRSTUVWXYZ".chars().collect();

        rng.shuffle(&mut chars);
        chars[..20].iter().collect()
    }
}
