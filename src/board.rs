use itertools::Itertools;

use crate::error::chess_error;
use crate::fen;
use crate::internal::utils::clamp_board_idx;
use crate::piece::{is_piece, piece_color, piece_type, Piece, Color, BITS_KING};
use crate::square::Square;
use crate::Result;

#[derive(Clone)]
pub struct Board {
    pub pieces: Box<[[Piece; 8]; 8]>,
    pub side_to_move: Color, // TODO: Refactor this to avoid conversions
}

impl Board {
    pub fn new() -> Self {
        fen::import("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn get_piece(&self, sq: &Square) -> Piece {
        self.pieces[sq.0][sq.1]
    }

    pub fn gen_moves(&self) -> Vec<(Square, Square)> {
        let mut res = Vec::new();

        for (rank, file) in (0..8).cartesian_product(0..8) {
            let from = Square(file, rank);
            let piece = self.pieces[rank][file];
            if is_piece(piece) {
                if piece_color(piece) == self.side_to_move {
                    match piece_type(piece) {
                        BITS_KING => res.append(&mut self.gen_king_moves(&from)),
                        _ => panic!("Not implemented yet"),
                    }
                }
            }
        }

        res
    }

    pub fn move_piece(&mut self, from: &Square, to: &Square) -> Result<()> {
        let possible_moves = self.gen_moves();
        
        let move_ = (*from, *to);

        if possible_moves.contains(&move_) {
            Ok(())
        } else {
            Err(chess_error("Not a valid move"))
        }
    }

    fn gen_king_moves(&self, from: &Square) -> Vec<(Square, Square)> {
        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_KING);

        let mut res = Vec::new();

        for file in clamp_board_idx(from.0 as i32 - 1)..(clamp_board_idx(from.0 as i32 + 1) + 1) {
            for rank in clamp_board_idx(from.1 as i32 - 1)..(clamp_board_idx(from.1 as i32 + 1) + 1)
            {
                // TODO: Fix this implementation
                let valid_move = true;
                if valid_move {
                    res.push((*from, Square(file, rank)));
                }
            }
        }

        res
    }


fn perpendicular_path(from: &Square, to: &Square) -> Option<Vec<Square>> {
    assert!(from != to);

    if from.0 == to.0 {
        if from.1 < to.1 {
            Some(
                (from.1 + 1..to.1 + 1)
                    .map(move |x| Square(from.0, x))
                    .collect(),
            )
        } else {
            Some((to.1..from.1).rev().map(move |x| Square(to.0, x)).collect())
        }
    } else if from.1 == to.1 {
        if from.0 < to.0 {
            Some(
                (from.1 + 1..to.1 + 1)
                    .map(move |x| Square(from.0, x))
                    .collect(),
            )
        } else {
            Some(
                (to.1..from.1)
                    .rev()
                    .map(move |x| Square(from.0, x))
                    .collect(),
            )
        }
    } else {
        None
    }
}

}

#[cfg(test)]
mod tests {
    #[test]
    fn king_movement() -> crate::Result<()> {
        crate::internal::test_utils::json::run_test("assets/move_tests/single_unit/king.json")
    }
}
