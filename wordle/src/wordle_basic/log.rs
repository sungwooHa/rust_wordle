use std::fmt;

use super::alphabet_state;

pub struct Log{
    #[allow(dead_code)]
    guess_word : String,
    state : Vec<alphabet_state::Alphabet>,
}

impl Log{
    pub fn new(guess_word : &str, state : Vec<alphabet_state::Alphabet> ) -> Log{
        Log{
            guess_word : String::from(guess_word),
            state : state,
        }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result{
        for alphabet in &self.alphabet {
            write!(f, "{}", &self);
        }
    }
}