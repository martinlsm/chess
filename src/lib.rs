// Public modules
pub mod board;
pub mod error;
pub mod fen;
pub mod piece;
pub mod square;

// Private modules
mod internal;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
