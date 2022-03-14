use std::fmt;

pub struct Log{
    #[allow(dead_code)]
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
}

impl fmt::Display for Log {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result{
        for ch in &self.state {
            write!(f, "{}", ch.print_state());
        }
    }
}