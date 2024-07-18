use crate::error::chess_error;
use crate::Result;

/// Bit field type for representing a piece, its color and whether it has moved or not (castling rules).
///
/// Bits 0..2:
///    0 for no piece
///    1 for pawn
///    2 for rook
///    3 for knight
///    4 for bishop
///    5 for queen
///    6 for king
/// Bit 3:
///    0 for white
///    1 for black
/// Bit 4 (only relevant for rooks and kings):
///    0 if piece has not moved
///    1 if piece has moved
pub type Piece = u8;

pub const BITS_NO_PIECE: Piece = 0 << 0;
pub const BITS_PAWN: Piece = 1 << 0;
pub const BITS_ROOK: Piece = 2 << 0;
pub const BITS_KNIGHT: Piece = 3 << 0;
pub const BITS_BISHOP: Piece = 4 << 0;
pub const BITS_QUEEN: Piece = 5 << 0;
pub const BITS_KING: Piece = 6 << 0;

pub const BITS_WHITE: Piece = 0 << 3;
pub const BITS_BLACK: Piece = 1 << 3;

pub const BITS_UNMOVED: Piece = 0 << 4;
pub const BITS_HAS_MOVED: Piece = 1 << 4;

/// Identical to the Piece type, but to be used in places where the color is the only relevant data.
/// This type should only be equal to one of the following:
///     BITS_WHITE, BITS_BLACK
pub type Color = Piece;

pub fn piece_type(piece: Piece) -> Piece {
    piece & 0b111
}

pub fn piece_color(piece: Piece) -> Color {
    piece & (1 << 3)
}

pub fn is_piece(piece: Piece) -> bool {
    piece_type(piece) != BITS_NO_PIECE
}

pub fn has_moved(piece: Piece) -> bool {
    (piece & (1 << 4)) != 0
}
