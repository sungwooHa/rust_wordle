use std::{collections::HashMap};

const COLLECT_SIMBOL : &str = "■";
const HAVE_SIMBOL : &str = "▨";
const WRONG_SIMBOL : &str = "□";

static MSG_GOOD_GAME : &'static str =  "nice, good job";
static MSG_WRONG_LENGTH : &'static str =  "wrong word, different length";
static MSG_WRONG : &'static str =  "wrong word";
static MSG_NO_CHANCE : &'static str = "FAIL. you don't have any chance";
static MSG_STRIKE_IS : &'static str =  "strike : ";
static MSG_BALL_IS : &'static str =  "ball : ";

#[derive(PartialEq, Clone)]
enum State{
    COLLECT,
    HAVE,
    WRONG,
}

impl State{
    pub fn print_state(&self) -> &str{
        match self{
            State::COLLECT => COLLECT_SIMBOL,
            State::HAVE => HAVE_SIMBOL,
            State::WRONG => WRONG_SIMBOL,
        }
    }
}

struct Log{
    guess_word : String,
    state : Vec<State>,
}

impl Log{
    pub fn new(guess_word : &str, state : Vec<State> ) -> Log{
        Log{
            guess_word : String::from(guess_word),
            state : state,
        }
    }

    pub fn print_log(&self){
        for ch in &self.state {
            print!("{} ", ch.print_state());
        }
        println!("");
    }
}

pub struct Game{
    word : String,
    game_log : Vec<Log>,
    max_round : u32,
}

impl Game{
    pub fn new(word : String, round : u32) -> Game{
        Game{
            word,
            max_round : round,
            game_log : Vec::new()
        }
    }

    pub fn play(&mut self, guess_word : &String) -> Result<bool, &str> {
        if self.verify(guess_word) == false {
            return Err(MSG_WRONG_LENGTH);
        }
        
        //"COLLECT" check.
        let wordle = match self.check_collect(guess_word){
            Some(wordle) => wordle,
            None => {
                panic!("{}", MSG_WRONG);
            },
        };

        //"HAVE" check
        let wordle = match self.check_have(guess_word, wordle){
            Some(v) => v,
            None => {
                panic!("{}", MSG_WRONG);
            },
        };

        let log = Log::new(guess_word, wordle);
        log.print_log();
        self.game_log.push(log);

        //ANSWER.
        if self.is_answer(guess_word) {
            println!("{}", MSG_GOOD_GAME);
            return Ok(true);
        }        

        if self.game_log.len() as u32 >= self.max_round {
            //fail
            return Err(MSG_NO_CHANCE);
        } 
        else {
            //Keep
            Ok(false)
        }
    }

    fn verify(&self, guess_word : &String ) -> bool {
        self.word.len() == guess_word.len()
    }

    pub fn get_rest_round(&self) -> u32 {
        self.max_round - self.game_log.len() as u32
    }

    fn is_answer(&self, guess_word : &String) -> bool {
        self.word.eq_ignore_ascii_case(&guess_word)
    }
    
    fn check_collect(&self, guess_word : &String) -> Option<Vec<State>>{
        if self.word.is_empty() || guess_word.is_empty(){
            return None;
        }

        let char_word : Vec<char> = self.word.chars().collect();
        let guess_word : Vec<char> = guess_word.chars().collect();
        
        let mut wordle : Vec<State> = Vec::new();

        for idx in 0..char_word.len() {
            let state= if char_word[idx].eq_ignore_ascii_case(&guess_word[idx]) {
                State::COLLECT
            } else
            {
                State::WRONG
            };
            
            wordle.push(state);
        }
        
        Some(wordle)
    }

    fn check_have(&self, guess_word : &String, wordle_collect_only : Vec<State>) -> Option<Vec<State>>{
        
        let mut word_map : HashMap<char, u32> = HashMap::new();
        let mut idx = 0;
        for ch in self.word.to_uppercase().chars() {
            //비교군 생성, collect 가 아닌 것만 map에 넣음
            if wordle_collect_only[idx] != State::COLLECT {
                if let Some(x) = word_map.get_mut(&ch) {
                    *x += 1;
                }else{
                    word_map.insert(ch, 1);
                }
            }
            idx += 1;
        }

        let mut wordle = wordle_collect_only.to_vec();
        let mut idx = 0;
        for ch in guess_word.to_uppercase().chars(){
            //입력한 정답을 collect가 아닌 word의 char과 비교.
            if let Some(count) = word_map.get_mut(&ch){
                //count 가 0일 경우 넘어감.
                //있을 경우 count --,
                if *count != 0 && wordle[idx] != State::COLLECT {
                    *count -= 1;
                    wordle[idx] = State::HAVE;
                }
            }
            idx += 1;
        }

        Some(wordle)
    }

    pub fn get_answer(&self) -> &str{
        return &self.word;
    }
}