use crate::messages::input::challenges::recover_secret_input::RecoverSecretInput;
use crate::messages::output::challenges::recover_secret_output::RecoverSecretOutput;
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

pub struct RecoverSecret {
    input: RecoverSecretInput,
    output: RecoverSecretOutput
}

impl RecoverSecret {
    
    pub fn new() -> RecoverSecretOutput {
        let mut rng = thread_rng();
        let sentence = RecoverSecret::create_sentence(&mut rng);
        let mut is_used: [bool; 20] = [false; 20];
        let mut letters=  String::new();
        let mut tuple_sizes= Vec::new();

        while !RecoverSecret::all_used(&is_used) {
            let size: usize = rng.gen_range(4, 10);
            let mut vec = RecoverSecret::create_vec_index_letters(size, &mut rng);
            while RecoverSecret::all_already_used(&vec, &is_used) {
                vec = RecoverSecret::create_vec_index_letters(size, &mut rng);
            }
            for i in 0..vec.len() {
                let char = sentence.chars().nth(vec[i]);
                match char {
                    None => {}
                    Some(c) => {
                        letters.push(c);
                    }
                }
                is_used[vec[i]] = true;
            }
            tuple_sizes.push(size);
        }
        RecoverSecretOutput {
            word_count: 1,
            letters,
            tuple_sizes
        }
    }

    fn create_vec_index_letters(size: usize, rng: &mut ThreadRng) -> Vec<usize> {
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
            if !is_used[vec[i]] {
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

    fn create_sentence(rng: &mut ThreadRng) -> String {
        let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPKRSTUVWXYZ".chars().collect();

        chars.shuffle(rng);
        chars[..20].iter().collect()
    }
}
