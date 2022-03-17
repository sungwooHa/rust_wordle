

trait Luncher {
    type GameObject;
    fn start_game(&mut self) -> Self::GameObject;
}

struct WordleFactory{
    today_word : String
}

impl WordleFactory{
    fn new() -> WordleFactory {
        WordleFactory {
            today_word : get_today_word().unwrap(),
        }
    }

    fn get_today_word() -> Result<String, &str> {
        let today_word = String::from("Hello");
        Ok(today_word);
    }

}

impl Luncher for WordleFactory{
    type GameObject = Wordle;
    fn start_game(&mut self) -> Self::GameOBject{
        Wordle::new(today_word);

        while Wordle.play() {};
    }
}