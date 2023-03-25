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

pub fn get_color(piece: &Piece) -> Color {
    match piece {
        Piece::BISHOP(c) => *c,
        Piece::KING(c, _) => *c,
        Piece::KNIGHT(c) => *c,
        Piece::PAWN(c, _) => *c,
        Piece::QUEEN(c) => *c,
        Piece::ROOK(c, _) => *c,
    }
}

pub fn tag_as_moved(piece: &Piece) -> Piece {
    match piece {
        Piece::BISHOP(color) => Piece::BISHOP(*color),
        Piece::KING(color, _) => Piece::KING(*color, true),
        Piece::KNIGHT(color) => Piece::KNIGHT(*color),
        Piece::PAWN(color, _) => Piece::PAWN(*color, true),
        Piece::QUEEN(color) => Piece::QUEEN(*color),
        Piece::ROOK(color, _) => Piece::ROOK(*color, true),
    }
}
