mod wordle;
mod util;

use wordle::Wordle;
use util::read_lines;


fn main() {
    let dictionary: Vec<String> = read_lines("./src/words.txt");
    let mut wordle: Wordle = Wordle::new(dictionary);
    wordle.play();
}
