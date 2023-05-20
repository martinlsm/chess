// Public modules
pub mod board;
pub mod color;
pub mod error;
pub mod fen;
pub mod piece;
pub mod square;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
