

static MAX_ROUND : u6 = 6;

pub trait luncher{
    pub fn run(&game : Game) {}
}

pub trait Game{
    pub fn new() -> Game {} 
    pub fn play(&self) -> Result<bool, &str> {}
    pub fn is_over(&self) -> Result<bool, &str> {}
}

pub struct Wordle{
    word : String,
    history : Vec<Log>,
}

impl Wordle{
    fn get_rest_count(&self) -> u32 {
        MAX_COUNT - self.history.size()
    }

    fn is_answer(&self, guess_word : &String) -> bool {
        self.word.eq_ignore_ascii_case(&guess_word)
    }

    fn check_alphabet(&self, guess_word : &String) -> Option<Vec<State>>{

    }
}

impl Game for Wordle{
    fn new() -> Game {
        Game{
            word,
            history : Vec::new()
        }
    }

    fn play(&self) -> Result<bool, &str> {

    }

    fn is_over(&self) -> Result<bool, &str> {

    }
}