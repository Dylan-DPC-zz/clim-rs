use console::Term;
use std::io::Error;

pub struct LineInput<'a> {
    pub input: String,
    terminal: &'a Term,
}

impl<'a> LineInput<'a> {
    pub fn new(term: &'a Term) -> LineInput<'a> {
        LineInput {
            input: String::default(),
            terminal: &term,
        }
    }
}

pub trait Input {
    fn get_from_terminal(&mut self) -> Result<(), Error>;
}

impl<'a> Input for LineInput<'a> {
    fn get_from_terminal(&mut self) -> Result<(), Error> {
        self.input = self.terminal.read_line()?;
        Ok(())
    }
}
