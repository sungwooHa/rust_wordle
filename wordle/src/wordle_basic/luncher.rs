
use super::game;

pub mod runner{
    use std::io::Write;

    pub fn run(){

        println!("New Game");
        
        loop{
            //Round
            let round = match get_round_from_user() {
                Some(round_number) => round_number,
                None=>{
                    continue;
                }
            };

            let word = match get_wordle_answer() {
                Some(word) => word,
                None =>{
                    println!("Fail to load word");
                    return;
                }
            };

            println!("--- Game Start ---");

            let mut game = super::game::Game::new(word, round);

            loop{
                let guess_word = match get_guess_word(){
                    Some(guess_word) => guess_word,
                    None =>{
                        continue
                    }
                };
        
                match game.play(&guess_word) {
                    Ok(is_success) => {
                        if is_success{
                            break;
        
                        }
                        else {
                            println!("round : {} / {}" , game.get_rest_round(), round);
                        }
                    },
                    Err(err_msg) => {
                        println!("{}", err_msg); 
                    }
                }
                println!("--- next round ---");
            }
        }
    }

    fn get_round_from_user() -> Option<u32>{
        print!("round : ");
        std::io::stdout().flush().unwrap(); //flush를 통해서 print가 즉시 실행되도록, https://doc.rust-lang.org/std/macro.print.html
        let mut game_round = String::new();
        std::io::stdin().read_line(&mut game_round).expect("failed to read line");

        match game_round.trim().parse(){
            Ok(game_round) => Some(game_round),
            Err(_) => {
                println!("please type a number");
                None
            }
        }
    }

    fn get_wordle_answer() -> Option<String>{

        //loader를 이용해보고싶은데..?
        let word = String::from("Hello");
        
        Some(word)
    }

    fn get_guess_word() -> Option<String>{
        print!("guess word : ");
        std::io::stdout().flush().unwrap();
        let mut guess_word = String::new();

        match std::io::stdin().read_line(&mut guess_word){
            Ok(_) => Some(String::from(guess_word.trim())),
            Err(_) => {
                println!("Failed to read line");
                return None;
            },
        }
    }

}