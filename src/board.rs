use itertools::Itertools;

use crate::color::Color;
use crate::error::chess_error;
use crate::fen;
use crate::internal::utils::clamp_board_idx;
use crate::piece::{Piece, PieceType};
use crate::square::Square;
use crate::Result;

#[derive(Clone)]
pub struct Board {
    pub pieces: Box<[[Option<Piece>; 8]; 8]>,
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        fen::import("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn get_piece(&self, sq: &Square) -> Option<Piece> {
        self.pieces[sq.0][sq.1]
    }

    pub fn gen_moves(&self) -> Vec<(Square, Square)> {
        let mut res = Vec::new();

        for (rank, file) in (0..8).cartesian_product(0..8) {
            let from = Square(file, rank);
            if let Some(piece) = self.pieces[rank][file] {
                if piece.color == self.side_to_move {
                    match piece.p_type {
                        PieceType::King => res.append(&mut self.gen_king_moves(&from)),
                        _ => panic!("Not implemented yet"),
                    }
                }
            }
        }

        res
    }

    pub fn move_piece(&mut self, from: &Square, to: &Square) -> Result<()> {
        let piece = &self.pieces[from.0][from.1];
        if *piece == None {
            return Err(chess_error("Square is empty"));
        }
        let piece = piece.unwrap();

        if piece.color != self.side_to_move {
            return Err(chess_error("Not this player's turn to move"));
        }

        if !self.valid_move(&piece, from, to) {
            return Err(chess_error("Invalid move"));
        }

        let moving_piece = &mut self.pieces[from.0][from.1].unwrap();
        moving_piece.has_moved = true;

        self.pieces[to.0][to.1] = Some(moving_piece.clone());
        self.pieces[from.0][from.1] = None;

        self.side_to_move = if self.side_to_move == Color::WHITE {
            Color::BLACK
        } else {
            Color::WHITE
        };

        Ok(())
    }

    fn gen_king_moves(&self, from: &Square) -> Vec<(Square, Square)> {
        assert_eq!(self.pieces[from.0][from.1].unwrap().p_type, PieceType::King);

        let mut res = Vec::new();

        for file in clamp_board_idx(from.0 as i32 - 1)..(clamp_board_idx(from.0 as i32 + 1) + 1) {
            for rank in clamp_board_idx(from.1 as i32 - 1)..(clamp_board_idx(from.1 as i32 + 1) + 1)
            {
                let target_sq_piece: Option<Piece> = self.pieces[file][rank];

                let valid_move = target_sq_piece
                    .map(|piece| piece.color != self.side_to_move)
                    .unwrap_or(true);

                if valid_move {
                    res.push((*from, Square(file, rank)));
                }
            }
        }

        res
    }

    fn valid_move(&self, piece: &Piece, from: &Square, to: &Square) -> bool {
        if from == to {
            return false;
        }

        if to.0 >= 8 || to.1 >= 8 {
            return false;
        }

        if Some(piece.color) == self.pieces[to.0][to.1].map(|p| p.color) {
            return false;
        }

        match piece.p_type {
            PieceType::Bishop => todo!(),
            PieceType::King => self.valid_king_move(piece.color, piece.has_moved, from, to),
            PieceType::Knight => self.valid_knight_move(from, to),
            PieceType::Pawn => self.valid_pawn_move(piece.color, piece.has_moved, from, to),
            PieceType::Queen => todo!(),
            PieceType::Rook => todo!(),
        }
    }

    fn valid_pawn_move(&self, color: Color, has_moved: bool, from: &Square, to: &Square) -> bool {
        if let Some(path) = perpendicular_path(from, to) {
            // Verify that path has valid length
            if path.len() > 2 || path.len() > 1 && has_moved {
                return false;
            }

            // Verify that the perpendicular path is forward
            if color == Color::WHITE && to.1 <= from.1 || color == Color::BLACK && to.1 >= from.1 {
                return false;
            }

            // Verify that squares in the path are empty
            return path.iter().all(|sq| self.pieces[sq.0][sq.1] == None);
        }

        // Check pawn captures
        if let Some(target) = &self.pieces[to.0][to.1] {
            assert!(target.color != color); // Should already be checked

            if color == Color::WHITE
                && to.1 == from.1 + 1
                && (to.0 as i32 - from.0 as i32).abs() == 1
            {
                return true;
            }
            if color == Color::BLACK
                && to.1 == from.1 - 1
                && (to.0 as i32 - from.0 as i32).abs() == 1
            {
                return true;
            }
        }

        false
    }

    fn valid_king_move(&self, _color: Color, _has_moved: bool, from: &Square, to: &Square) -> bool {
        (to.0.abs_diff(from.0) == 1) || (to.1.abs_diff(from.1) == 1)
    }

    fn valid_knight_move(&self, from: &Square, to: &Square) -> bool {
        let file_diff = (from.0 as i32 - to.0 as i32).abs();
        let rank_diff = (from.1 as i32 - to.1 as i32).abs();

        file_diff == 2 && rank_diff == 1 || file_diff == 1 && rank_diff == 2
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn king_movement() -> crate::Result<()> {
        crate::internal::test_utils::json::run_test("assets/move_tests/single_unit/king.json")
    }
}
