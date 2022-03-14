

static MAX_ROUND : u6 = 6;

pub trait luncher{
    pub fn run(&game : Game) {}
}

pub trait Game{
    pub fn new() -> Game {} 
    pub fn play(&self) -> Result<bool, &str> {}
}

pub struct Wordle{
    word : String,
    history : Vec<Log>,
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
}