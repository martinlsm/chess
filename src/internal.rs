use crate::color::Color;
use crate::piece::Piece;

pub struct BoardImpl {
    pub pieces: Box<[[Option<Piece>; 8]; 8]>,
    pub side_to_move: Color,
}
