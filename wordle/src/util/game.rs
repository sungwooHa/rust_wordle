

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

impl Wordle{
    fn get_rest_count(&self) -> u32 {
        MAX_COUNT - self.history.size()
    }

    fn is_answer(&self, guess_word : &String) -> bool {
        self.word.eq_ignore_ascii_case(&guess_word)
    }

    fn get_alphabet_collect(&self, guess_word : &String) -> Option<Vec<State>>{
        if self.word.is_empty() || guess_word.is_empty(){
            return None;
        }
        
        Some(self.word.chars().zip(guess_word.chars()).map(|(self_char, guess_char)| {
            if self_char.eq_ignore_ascii_case(&guess_char) {
                State::COLLECT
            } else {
                State::WRONG
            }
        }).collect())
    }

    fn get_alphabet_have(&self, guess_word : &String, collect_only_checked_states : Vec<State>) -> Option<Vec<State>>{
        let mut word_map : HashMap<char, u32> = HashMap::new();
        
        self.word.to_uppercase().chars().iter().enumerate().for_each(|index, letter| {
            if collect_only_checked_states[index] != State::COLLECT {
                if let Some(count) = word_map.get_mut(&letter.get_value()) {
                    *count += 1;
                }else{
                    word_map.insert(letter.get_value(), 1);
                }
            }
        });

        let mut wordle = collect_only_checked_states.to_vec().iter().enumerate().for_each(|index, &state|{
            //입력한 정답을 collect가 아닌 word의 char과 비교.
            if let Some(ch) = word_map.get_mut(&guess_word.to_uppercase().chars().collect().get(index)){
                //count 가 0일 경우 넘어감.
                //있을 경우 count --,
                if *count != 0 && state != State::COLLECT{
                    *count -= 1;
                    state = State::HAVE;
                }
            }
        });

        let mut idx = 0;
        for ch in guess_word.to_uppercase().chars(){
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
}
