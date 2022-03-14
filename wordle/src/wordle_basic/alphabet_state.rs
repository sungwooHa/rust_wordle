use std::fmt;
use colored::Colorize;

#[derive(PartialEq, Clone)]
enum State{
    COLLECT,
    HAVE,
    WRONG,
}

pub struct Alphabet{
    state : State,
    alphabet : String,
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.state{
            State::COLLECT => &self.alphabet.truecolor(106,170,100),
            State::HAVE => &self.alphabet.truecolor(201,180,88),
            State::WRONG => &self.alphabet.truecolor(120,124,126),
        })
    }
}