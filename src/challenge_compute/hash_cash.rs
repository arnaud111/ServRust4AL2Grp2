use rand::prelude::{SliceRandom, ThreadRng};
use rand::{Rng, thread_rng};
use md5::{Digest, Md5};
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

        if format!("{:X}", val) != self.input.hashcode {
            return false;
        }

        let mut binary_value = HashCash::convert_to_binary_from_hex( &*format!("{:X}", val) ).to_string();
        binary_value = binary_value[0..self.output.complexity as usize].to_string();
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

    fn convert_to_binary_from_hex(hex: &str) -> String {
        hex.chars().map(HashCash::hex_to_binary).collect()
    }

    fn hex_to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }

    fn create_sentence(rng: &mut ThreadRng) -> String {
        let subjects = ["I", "You", "They", "We", "John", "Mary", "The dog", "The cat", "The car"];
        let predicates = ["am", "like", "have", "want", "need", "prefer", "hate", "love", "despise", "enjoy", "appreciate"];
        let objects = ["pizza", "an apple", "to be happy", "the sky", "the sun", "the moon", "a book", "a movie", "a song"];
        let adverbs = ["quickly", "carefully", "happily", "angrily", "sadly", "anxiously", "eagerly", "nervously"];
        let temporal_complement = ["yesterday", "today", "tomorrow", "next week", "next month", "next year"];

        let subject = subjects[rng.gen_range(0, subjects.len())];
        let predicate = predicates[rng.gen_range(0, predicates.len())];
        let object = objects[rng.gen_range(0, objects.len())];
        let adverb = adverbs[rng.gen_range(0, adverbs.len())];
        let temporal = temporal_complement[rng.gen_range(0, temporal_complement.len())];
        format!("{} {} {} {} {}.", subject, adverb, predicate, object, temporal)
    }
}