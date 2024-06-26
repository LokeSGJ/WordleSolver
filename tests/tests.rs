use WordleSolver::util::read_lines;
use WordleSolver::wordle::Wordle;

#[test]
fn test_winrate() {
    let mut win_counter = 0;
    for _i in 0..100 {
        let dictionary: Vec<String> = read_lines("./src/words.txt");
        let mut wordle: Wordle = Wordle::new(dictionary);
        let result = wordle.play();
        if result {
            win_counter += 1;
        }
    }

    println!("Won {} games out of 100.", win_counter)
}
