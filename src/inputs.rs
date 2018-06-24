use console::Term;
use std::io::Error;

pub struct LineInput {
    pub input: String,
    terminal: Term
}

impl LineInput {
    pub fn new(term: Term) -> LineInput {
        LineInput { input: String::from(""), terminal: term}
    }

}

pub trait Input {
    fn get_from_terminal(&mut self) -> Result<(), Error>;
}

impl Input for LineInput {
    fn get_from_terminal(&mut self) -> Result<(), Error> {
        self.input = self.terminal.read_line()?;
        Ok(())
    }
}