use rand::prelude::IndexedRandom;

pub struct Wordle {
    word: String,
    dictionary: Vec<String>
}

enum Color {
    GREEN,
    YELLOW,
    RED,
}

impl Wordle {
    pub fn new(word: String, dictionary: Vec<String>) -> Wordle {
        return Wordle {
            word,
            dictionary,
        }
    }

    pub fn guess(&mut self) -> bool {
        let guess: String = self.dictionary.choose(&mut rand::thread_rng()).unwrap().clone();
        println!("Guessing: {}", guess);

        if guess == self.word {
            return true
        }

        let guess_chars: Vec<_> = guess.chars().collect();
        let word_chars: Vec<_> = self.word.chars().collect();
        let mut colors: Vec<Color> = Vec::new();

        for i in 0..5 {
            if guess_chars[i] == word_chars[i] {
                colors.push(Color::GREEN);
            } else if word_chars.contains(&guess_chars[i]) {
                colors.push(Color::YELLOW);
            } else {
                colors.push(Color::RED);
            }
        }

        self.dictionary.retain(|word| Wordle::compare(guess.clone(), word.clone(), &colors));

        return false
    }

    fn compare(guess: String, word: String, colors: &Vec<Color>) -> bool {
        let guess_chars: Vec<_> = guess.chars().collect();
        let word_chars: Vec<_> = word.chars().collect();
        let mut result = true;

        for i in 0..5 {
            match colors[i] {
                Color::RED => {
                    if word_chars[i] == guess_chars[i] {
                        result = false;
                        break;
                    }
                }
                Color::YELLOW => {
                    if word_chars[i] == guess_chars[i] {
                        result = false;
                        break;
                    } else if !word_chars.contains(&guess_chars[i]) {
                        result = false;
                        break;
                    }
                }
                Color::GREEN => {}
            }
        }

        return result
    }
}