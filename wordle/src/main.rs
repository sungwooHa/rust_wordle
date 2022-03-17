mod util;
use util::luncher::WordleFactory;


mod wordle_basic;
use wordle_basic::luncher::runner;

fn main() {
    let mut wordleFactory = WordleFactory::new();
    wordleFactory.start_game();

    
}
