use crate::color::Color;
use crate::error::chess_error;
use crate::Result;

// TODO: Rename
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

pub fn piece_type(piece: Piece) -> Piece {
    piece & 0b111
}

pub fn piece_color(piece: Piece) -> Piece {
    piece & (1 << 3)
}

pub fn is_piece(piece: Piece) -> bool {
    piece_type(piece) == BITS_NO_PIECE
}

pub fn piece_to_letter(piece_bits: Piece) -> char {
    let ch = match piece_type(piece_bits) {
        BITS_BISHOP => 'b',
        BITS_KING => 'k',
        BITS_KNIGHT => 'n',
        BITS_PAWN => 'p',
        BITS_QUEEN => 'q',
        BITS_ROOK => 'r',
        _ => panic!("Invalid piece bits")
    };

    match piece_color(piece_bits) {
        BITS_WHITE => ch.to_uppercase().next().unwrap(),
        BITS_BLACK => ch,
        _ => panic!("Invalid piece bits")
    }
}

pub fn letter_to_piece(letter: char) -> Result<Piece> {
    let piece_type = match letter.to_uppercase().next().unwrap() {
        'B' => BITS_BISHOP,
        'K' => BITS_KING,
        'N' => BITS_KNIGHT,
        'P' => BITS_PAWN,
        'Q' => BITS_QUEEN,
        'R' => BITS_ROOK,
        _ => return Err(chess_error(&format!("Invalid piece type '{}'", letter))),
    };

    let color = if letter.is_uppercase() { BITS_WHITE } else { BITS_BLACK };

    Ok(color & piece_type)
}
