use std::collections::HashSet;

struct Log{
    Strike : u32,
    Ball : u32,
    Guess : String,
}

impl Log{
    pub fn new(Strike : u32, Ball : u32, Guess : &String) -> Log {
        Log {
                Strike,
                Ball,
                Guess : Guess.clone()
        }
    }
    pub fn print_log(&self){
        println!("Guess : {}, Strike : {}, Ball : {}", self.Guess, self.Strike, self.Ball);
    }
}    

pub struct Game{
    word : String,
    game_log : Vec<Log>,
    max_round : u32,
}

static MSG_GOOD_GAME : &'static str =  "nice, good job";
static MSG_WRONG_LENGTH : &'static str =  "wrong word, different length";
static MSG_NO_CHANCE : &'static str = "FAIL. you don't have any chance";
static MSG_STRIKE_IS : &'static str =  "strike : ";
static MSG_BALL_IS : &'static str =  "ball : ";

impl Game{
    pub fn new(word : String, round : u32) -> Game{
        Game{
            word,
            max_round : round,
            game_log : Vec::new()
        }
    }

    pub fn Try(&mut self, guess_word : &String) -> Result<bool, &str> {
        //strike, ball
        //log ++
        if self.Verify(guess_word) == false {
            return Err(MSG_WRONG_LENGTH);
        }
            
        let numb_strike = match self.CountStrike(guess_word){
            Some(strike) =>{
                if strike == self.word.len() as u32 {
                    return Ok(true);
                }
                
                strike
            }
            None => 0,
        };

        let numb_ball = match self.CountBall(guess_word){
            Some(v) => v,
            None => 0,
        };

        let log = Log::new(numb_strike, numb_ball, guess_word);
        log.print_log();
        self.game_log.push(log);

        if self.game_log.len() as u32 >= self.max_round {
            //fail
            return Err(MSG_NO_CHANCE);
        } 
        else {
            //Keep
            Ok(false)
        }
    }

    fn Verify(&self, guess_word : &String ) -> bool {
        self.word.len() == guess_word.len()
    }

    pub fn get_rest_round(&self) -> u32 {
        self.max_round - self.game_log.len() as u32
    }

    fn CountStrike(&self, guess_word : &String) -> Option<u32>{
        let charWord : Vec<char> = self.word.to_uppercase().chars().collect();
        let guessWord : Vec<char> = guess_word.to_uppercase().chars().collect();

        let mut strikeCount = 0;
        for idx in 0..charWord.len(){
            if charWord[idx] == guessWord[idx] {
                strikeCount += 1;
            }
        }

        match strikeCount{
            0 => None,
            _ => Some(strikeCount)
        }
    }

    fn CountBall(&self, guess_word : &String) -> Option<u32>{
        let charWordSet : HashSet<_> = self.word.to_uppercase().chars().collect();
        let guessWordSet : HashSet<_> = guess_word.to_uppercase().chars().collect();

        let ballCount = self.word.len() as u32 - charWordSet.difference(&guessWordSet).count() as u32;

        match self.CountStrike(guess_word){
            None => Some(ballCount),
            Some(strikeCount) => Some(ballCount - strikeCount),
        }
    }

    pub fn GetAnswer(&self) -> &str{
        return &self.word;
    }
}