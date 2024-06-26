mod wordle;

use std::fs::read_to_string;
use rand::seq::{IndexedRandom};
use wordle::Wordle;


fn main() {
    let dictionary: Vec<String> = read_lines("./src/words.txt");
    let word: String = dictionary.choose(&mut rand::thread_rng()).unwrap().clone();
    println!("The secret word is: {}", word);

    let mut wordle: Wordle = Wordle::new(word.clone(), dictionary);
    let mut guess_counter: i32 = 0;
    let mut result = false;

    while guess_counter < 6 {
        result = wordle.guess();
        guess_counter += 1;
        if result == true {
            break;
        }
    }

    if result {
        println!("Word: {} guessed in {} tries.", word, guess_counter);
    } else {
        println!("Failed to guess word: {} in 6 tries.", word);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
