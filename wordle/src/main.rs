use std::io::BufRead;

use wordle::Game;
use std::io;

fn main() {
    println!("Hello, world!");

    let maxRound = 10;       
    let word= String::from("Hello");
    let mut game = Game::new(word, maxRound);
    
    loop{
        let mut guess_word = String::new();
        let guess_word  = match io::stdin().read_line(&mut guess_word){
            Ok(_) => String::from(guess_word.trim()),
            Err(_) => {
                return panic!("Failed to read line");
            },
        };
        


        match game.play(&guess_word) {
            Ok(isSuccess) => {
                if isSuccess{
                    println!("good success");
                    break;

                }
                else {
                    println!("round : {} / {}" , game.get_rest_round(), maxRound);
                }
            },
            Err(errMsg) => {
                println!("{}", errMsg); 
            }
        }
    }
}
