use std::io::BufRead;

use wordle::Game;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, this is wordle game!");

    print!("round : ");
    io::stdout().flush().unwrap(); //flush를 통해서 print가 즉시 실행되도록, https://doc.rust-lang.org/std/macro.print.html
    let mut game_round = String::new();
    io::stdin().read_line(&mut game_round).expect("failed to read line");
    let game_round : u32 = game_round.trim().parse().expect("please type a number");


    let word= String::from("Hello");

    let mut game = Game::new(word, game_round);
    
    println!("--- Game Start ---");
    loop{
        print!("guess word : ");
        io::stdout().flush().unwrap();
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
                    break;

                }
                else {
                    println!("round : {} / {}" , game.get_rest_round(), game_round);
                }
            },
            Err(errMsg) => {
                println!("{}", errMsg); 
            }
        }
        println!("--- next round ---");
    }
}
