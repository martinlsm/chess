use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    BISHOP(Color),
    KING(Color, bool),
    KNIGHT(Color),
    PAWN(Color, bool),
    QUEEN(Color),
    ROOK(Color, bool),
}
