

trait Game {
    pub fn play(&self) -> bool;
}

pub struct Wordle{
    word : String,
    history : Vec<Log>,
}

impl Wordle{
    fn new(&self, today_word : String) -> Wordle {
        Wordle{
            word,
            history : Vec::new()
        }
    }

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

        let char_word : Vec<char> = self.word.chars().collect();
        let guess_word : Vec<char> = guess_word.chars().collect();

        Some(Vec::with_capacity(self.word.size()).set_len(self,word.size())
                .iter().enumerate().for_each(|index, state|
                    {
                        state = if char_word[idx].eq_ignore_ascii_case(&guess_word[idx]) {
                            State::COLLECT
                        } else{
                            State::WRONG
                        }
                    }
                )
            )
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

    fn get_guess_word() -> Option<String> {
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

impl Game for Wordle{
    pub fn play(&self) -> bool{
        //남은 round 계산, 
        //남은 round가 MAX Round와 같아 질 때 그만,

        //입력 받기
        //비교하기
        //결과 보여주기
    }
}