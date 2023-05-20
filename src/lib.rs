// Public modules
pub mod board;
pub mod color;
pub mod error;
pub mod piece;
pub mod square;
pub mod fen;

// Private modules
mod internal;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
