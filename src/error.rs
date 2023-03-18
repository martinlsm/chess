use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct ChessError(String);

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0)
    }
}

impl Error for ChessError {}

pub fn chess_error(msg: &str) -> Box<dyn Error> {
    Box::new(ChessError(String::from(msg)))
}