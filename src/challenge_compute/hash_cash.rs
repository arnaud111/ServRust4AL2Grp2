use rand::prelude::{SliceRandom, ThreadRng};
use rand::thread_rng;
use crate::messages::input::challenges::hash_cash_input::MD5HashCashInput;
use crate::messages::output::challenges::hash_cash_output::Md5HashCashOutput;

pub struct HashCash {
    pub input: MD5HashCashInput,
    pub output: Md5HashCashOutput
}

impl HashCash {

    pub fn is_valid(&self) -> bool {

        let mut complete_seed = "0000000000000000".to_string();
        let hexa = format!("{:X}", self.input.seed);
        complete_seed = complete_seed[0..16 - hexa.len()].to_string();
        complete_seed.push_str(&*hexa.to_string());

        let mut md5_hasher = Md5::new();
        md5_hasher.update(complete_seed.clone() + &*self.output.message);
        let val = md5_hasher.finalize();

        if val != self.input.hashcode {
            return false;
        }

        let mut binary_value = convert_to_binary_from_hex( &*format!("{:X}", val) ).to_string();
        binary_value = binary_value[0..momo as usize].to_string();

        if isize::from_str_radix(&*binary_value, 2).unwrap() == 0 {
            return true;
        }

        return false;
    }

    pub fn new(complexity: i32) -> Md5HashCashOutput {
        let mut rng = thread_rng();
        let sentence = HashCash::create_sentence(&mut rng);
        
        Md5HashCashOutput {
            complexity,
            message: sentence
        }
    }

    fn create_sentence(rng: &mut ThreadRng) -> String {
        let mut chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPKRSTUVWXYZ".chars().collect();

        chars.shuffle(rng);
        chars[..20].iter().collect()
    }
}