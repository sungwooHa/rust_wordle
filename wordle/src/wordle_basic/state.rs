



pub trait color{
    
}

#[derive(PartialEq, Clone)]
struct State{
    COLLECT,
    HAVE,
    WRONG,
}

impl fmt::Display for State {

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