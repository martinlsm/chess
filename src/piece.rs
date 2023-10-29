use crate::color::Color;
use crate::error::chess_error;
use crate::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub p_type: PieceType,
    pub color: Color,
    pub has_moved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

pub fn piece_to_letter(p_type: PieceType, color: Option<Color>) -> char {
    let ch = match p_type {
        PieceType::Bishop => 'b',
        PieceType::King => 'k',
        PieceType::Knight => 'n',
        PieceType::Pawn => 'p',
        PieceType::Queen => 'q',
        PieceType::Rook => 'r',
    };

    match color {
        Some(Color::WHITE) => ch.to_uppercase().next().unwrap(),
        _ => ch,
    }
}

pub fn letter_to_piece(letter: char) -> Result<PieceType> {
    match letter {
        'b' | 'B' => Ok(PieceType::Bishop),
        'k' | 'K' => Ok(PieceType::King),
        'n' | 'N' => Ok(PieceType::Knight),
        'p' | 'P' => Ok(PieceType::Pawn),
        'q' | 'Q' => Ok(PieceType::Queen),
        'r' | 'R' => Ok(PieceType::Rook),
        _ => return Err(chess_error(&format!("Invalid piece type '{}'", letter))),
    }
}
