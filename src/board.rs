use crate::color::Color;
use crate::error::chess_error;
use crate::piece::{get_color, tag_as_moved, Piece};
use crate::square::Square;
use crate::fen;
use crate::Result;

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

    pub fn move_piece(&mut self, from: &Square, to: &Square) -> Result<()> {
        let piece = &self.pieces[from.0][from.1];
        if *piece == None {
            return Err(chess_error("Square is empty"));
        }
        let piece = piece.unwrap();

        if get_color(&piece) != self.side_to_move {
            return Err(chess_error("Not this player's turn to move"));
        }

        if !self.valid_move(&piece, from, to) {
            return Err(chess_error("Invalid move"));
        }

        self.pieces[to.0][to.1] = self.pieces[from.0][from.1].map(|p| tag_as_moved(&p));
        self.pieces[from.0][from.1] = None;
        tag_as_moved(&mut self.pieces[to.0][to.1].unwrap());
        self.side_to_move = if self.side_to_move == Color::WHITE {
            Color::BLACK
        } else {
            Color::WHITE
        };

        Ok(())
    }

    fn valid_move(&self, piece: &Piece, from: &Square, to: &Square) -> bool {
        if from == to {
            return false;
        }

        if to.0 >= 8 || to.1 >= 8 {
            return false;
        }

        if Some(get_color(&piece)) == self.pieces[to.0][to.1].map(|p| get_color(&p)) {
            return false;
        }

        match piece {
            Piece::BISHOP(_) => todo!(),
            Piece::KING(_, _) => todo!(),
            Piece::KNIGHT(_) => self.valid_knight_move(from, to),
            Piece::PAWN(color, has_moved) => self.valid_pawn_move(*color, *has_moved, from, to),
            Piece::QUEEN(_) => todo!(),
            Piece::ROOK(_, _) => todo!(),
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
        if let Some(target) = self.pieces[to.0][to.1] {
            assert!(get_color(&target) != color); // Should already be checked

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

    fn valid_knight_move(&self, from: &Square, to: &Square) -> bool {
        let file_diff = (from.0 as i32 - to.0 as i32).abs();
        let rank_diff = (from.1 as i32 - to.1 as i32).abs();

        file_diff == 2 && rank_diff == 1 || file_diff == 1 && rank_diff == 2
    }
}

// TODO: Return an iterator instead of Vec
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
    use super::*;

    use crate::square::square;

    use itertools::iproduct;

    #[test]
    fn white_is_starting_player() {
        let board = Board::new();
        assert_eq!(board.side_to_move(), Color::WHITE);
    }

    #[test]
    fn blacks_turn_after_white_has_taken_a_turn() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("A2"), square!("A4"))?;

        assert_eq!(board.side_to_move(), Color::BLACK);

        Ok(())
    }

    #[test]
    fn whites_turn_after_an_invalid_move_by_white() -> Result<()> {
        let mut board = Board::new();

        let _ = board.move_piece(square!("C3"), square!("C7"));

        assert_eq!(board.side_to_move(), Color::WHITE);

        Ok(())
    }

    #[test]
    fn white_is_active_player_after_both_players_have_moved() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("A2"), square!("A4"))?;
        board.move_piece(square!("D7"), square!("D6"))?;

        assert_eq!(board.side_to_move(), Color::WHITE);

        Ok(())
    }

    #[test]
    fn all_pieces_are_setup_correctly() {
        let board = Board::new();

        // White's pawns
        for col in 0..8 {
            assert_eq!(
                board.get_piece(&Square(col, 1)),
                Some(Piece::PAWN(Color::WHITE, false))
            );
        }

        // Rest of white's pieces
        assert_eq!(
            board.get_piece(square!("A1")),
            Some(Piece::ROOK(Color::WHITE, false))
        );
        assert_eq!(
            board.get_piece(square!("B1")),
            Some(Piece::KNIGHT(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(square!("C1")),
            Some(Piece::BISHOP(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(square!("D1")),
            Some(Piece::QUEEN(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(square!("E1")),
            Some(Piece::KING(Color::WHITE, false))
        );
        assert_eq!(
            board.get_piece(square!("F1")),
            Some(Piece::BISHOP(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(square!("G1")),
            Some(Piece::KNIGHT(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(square!("H1")),
            Some(Piece::ROOK(Color::WHITE, false))
        );

        // Black's pawns
        for col in 0..8 {
            assert_eq!(
                board.get_piece(&Square(col, 6)),
                Some(Piece::PAWN(Color::BLACK, false))
            );
        }

        // Rest of black's pieces
        assert_eq!(
            board.get_piece(square!("A8")),
            Some(Piece::ROOK(Color::BLACK, false))
        );
        assert_eq!(
            board.get_piece(square!("B8")),
            Some(Piece::KNIGHT(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(square!("C8")),
            Some(Piece::BISHOP(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(square!("D8")),
            Some(Piece::QUEEN(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(square!("E8")),
            Some(Piece::KING(Color::BLACK, false))
        );
        assert_eq!(
            board.get_piece(square!("F8")),
            Some(Piece::BISHOP(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(square!("G8")),
            Some(Piece::KNIGHT(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(square!("H8")),
            Some(Piece::ROOK(Color::BLACK, false))
        );

        // Rest of the squares should be empty
        assert!(iproduct!(0..8, 2..6).all(|(col, row)| board.get_piece(&Square(col, row)) == None));
    }

    #[test]
    fn piece_can_be_getted_after_moved() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("A2"), square!("A3"))?;

        assert_eq!(board.get_piece(square!("A2")), None);
        assert_eq!(
            board.get_piece(square!("A3")),
            Some(Piece::PAWN(Color::WHITE, true))
        );

        Ok(())
    }

    #[test]
    fn pawn_cannot_capture_piece_of_its_own_color() -> Result<()> {
        let mut board = Board::new();

        // Position two white pawns diagonally adjacent to eachother
        board.move_piece(square!("D2"), square!("D4"))?;
        board.move_piece(square!("H7"), square!("H6"))?;
        board.move_piece(square!("C2"), square!("C3"))?;
        board.move_piece(square!("H6"), square!("H5"))?;

        assert!(board.move_piece(square!("C3"), square!("D4")).is_err());

        Ok(())
    }

    #[test]
    fn white_pawn_can_move_one_step_after_new_game() {
        let mut board = Board::new();

        assert!(board.move_piece(square!("A2"), square!("A3")).is_ok());
    }

    #[test]
    fn white_pawn_can_move_two_steps_after_new_game() {
        let mut board = Board::new();

        assert!(board.move_piece(square!("D2"), square!("D4")).is_ok());
    }

    #[test]
    fn pawn_cannot_move_two_steps_when_already_moved() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("E2"), square!("E3"))?;
        // Move a black piece in between
        board.move_piece(square!("A7"), square!("A6"))?;

        assert!(board.move_piece(square!("E3"), square!("E5")).is_err());

        Ok(())
    }

    #[test]
    fn reject_white_to_move_twice() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("D2"), square!("D4"))?;

        assert!(board.move_piece(square!("A2"), square!("A3")).is_err());

        Ok(())
    }

    #[test]
    fn reject_white_pawn_to_move_diagonally_forwards() {
        let mut board = Board::new();

        assert!(board.move_piece(square!("E2"), square!("D3")).is_err());
    }

    #[test]
    fn get_pawn_at_new_square_after_it_has_moved() -> Result<()> {
        let mut board = Board::new();
        let from_sq = square!("C2");
        let target_sq = square!("C4");

        // Move a pawn forward two steps
        board.move_piece(&from_sq, &target_sq)?;

        assert_eq!(board.get_piece(&from_sq), None);
        assert_eq!(
            board.get_piece(&target_sq),
            Some(Piece::PAWN(Color::WHITE, true))
        );

        Ok(())
    }

    #[test]
    fn black_is_not_allowed_to_move_first() {
        let mut board = Board::new();

        // Check every pawn
        for i in 0..8 {
            assert!(board.move_piece(&Square(i, 6), &Square(i, 5)).is_err());
        }

        // Check knights
        assert!(board.move_piece(square!("B8"), square!("A6")).is_err());
        assert!(board.move_piece(square!("B8"), square!("C6")).is_err());
        assert!(board.move_piece(square!("G8"), square!("F6")).is_err());
        assert!(board.move_piece(square!("G8"), square!("H6")).is_err());
    }

    #[test]
    fn black_is_allowed_to_move_after_white() -> Result<()> {
        let mut board = Board::new();

        // Move a white pawn
        board.move_piece(square!("B2"), square!("B3"))?;

        // Move a black pawn
        assert!(board.move_piece(square!("E7"), square!("E5")).is_ok());

        Ok(())
    }

    #[test]
    fn black_is_not_able_to_move_after_white_failed_a_move() -> Result<()> {
        let mut board = Board::new();

        // Try to move a white pawn diagonally (invalid)
        let _ = board.move_piece(square!("D2"), square!("E3"));

        assert!(board.move_piece(square!("A7"), square!("A6")).is_err());

        Ok(())
    }

    #[test]
    fn pawns_cannot_move_into_eachother() -> Result<()> {
        let mut board = Board::new();

        // Move white and black pawn adjacent to eachother
        board.move_piece(square!("D2"), square!("D4"))?;
        board.move_piece(square!("D7"), square!("D5"))?;

        assert!(board.move_piece(square!("D4"), square!("D5")).is_err());

        Ok(())
    }

    #[test]
    fn white_pawn_cannot_move_backward() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("C2"), square!("C4"))?;
        board.move_piece(square!("H7"), square!("H6"))?;

        assert!(board.move_piece(square!("C4"), square!("C3")).is_err());

        Ok(())
    }

    #[test]
    fn black_pawn_cannot_move_backward() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("H2"), square!("H3"))?;
        board.move_piece(square!("H7"), square!("H5"))?;
        board.move_piece(square!("B2"), square!("B4"))?;

        assert!(board.move_piece(square!("H5"), square!("H6")).is_err());

        Ok(())
    }

    #[test]
    fn piece_disappears_when_captured() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("B2"), square!("B4"))?;
        board.move_piece(square!("A7"), square!("A5"))?;
        board.move_piece(square!("B4"), square!("A5"))?;

        assert_eq!(board.get_piece(square!("B4")), None);
        assert_eq!(
            board.get_piece(square!("A5")),
            Some(Piece::PAWN(Color::WHITE, true))
        );

        Ok(())
    }

    #[test]
    fn white_pawn_can_capture_piece_to_its_right() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("D2"), square!("D4"))?;
        board.move_piece(square!("E7"), square!("E5"))?;

        assert!(board.move_piece(square!("D4"), square!("E5")).is_ok());

        Ok(())
    }

    #[test]
    fn white_pawn_can_capture_piece_to_its_left() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("H2"), square!("H4"))?;
        board.move_piece(square!("G7"), square!("G5"))?;

        assert!(board.move_piece(square!("H4"), square!("G5")).is_ok());

        Ok(())
    }

    #[test]
    fn black_pawn_can_capture_to_its_left() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("H2"), square!("H3"))?;
        board.move_piece(square!("G7"), square!("G5"))?;
        board.move_piece(square!("H3"), square!("H4"))?;

        assert!(board.move_piece(square!("G5"), square!("H4")).is_ok());

        Ok(())
    }

    #[test]
    fn black_pawn_can_capture_to_its_right() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("D2"), square!("D3"))?;
        board.move_piece(square!("E7"), square!("E5"))?;
        board.move_piece(square!("D3"), square!("D4"))?;

        assert!(board.move_piece(square!("E5"), square!("D4")).is_ok());

        Ok(())
    }

    #[test]
    fn knights_can_move_all_8_directions() {
        let mut board = Board::new();

        // Move the 4 knights around without capturing any pieces
        assert!(board.move_piece(square!("B1"), square!("C3")).is_ok()); // 2 up   & 1 right
        assert!(board.move_piece(square!("B8"), square!("C6")).is_ok()); // 2 down & 1 right
        assert!(board.move_piece(square!("G1"), square!("F3")).is_ok()); // 2 up   & 1 left
        assert!(board.move_piece(square!("G8"), square!("F6")).is_ok()); // 2 down & 1 left
        assert!(board.move_piece(square!("C3"), square!("E4")).is_ok()); // 1 up   & 2 left
        assert!(board.move_piece(square!("C6"), square!("E5")).is_ok()); // 1 down & 2 right
        assert!(board.move_piece(square!("F3"), square!("D4")).is_ok()); // 1 up   & 2 left
        assert!(board.move_piece(square!("F6"), square!("D5")).is_ok()); // 1 down & 2 left
    }

    #[test]
    fn knight_cannot_capture_piece_of_its_own_color() {
        let mut board = Board::new();

        assert!(board.move_piece(square!("B1"), square!("D2")).is_err());
        assert!(board.move_piece(square!("G1"), square!("E2")).is_err());
    }

    #[test]
    fn piece_cannot_move_outside_board() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("D2"), square!("D3"))?;

        assert!(board.move_piece(square!("B8"), &Square(0, 9)).is_err());

        Ok(())
    }

    #[test]
    fn white_knight_can_capture_a_piece() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("B1"), square!("C3"))?;
        board.move_piece(square!("D7"), square!("D5"))?;

        assert!(board.move_piece(square!("C3"), square!("D5")).is_ok());

        Ok(())
    }

    #[test]
    fn knights_can_capture_pieces() -> Result<()> {
        let mut board = Board::new();

        board.move_piece(square!("D2"), square!("D4"))?;
        board.move_piece(square!("B8"), square!("C6"))?;
        board.move_piece(square!("G1"), square!("F3"))?;

        assert!(board.move_piece(square!("C6"), square!("D4")).is_ok());
        assert!(board.move_piece(square!("F3"), square!("D4")).is_ok());

        Ok(())
    }

    #[test]
    fn invalid_step_sizes_for_knight_are_rejected() {
        let mut board = Board::new();

        assert!(board.move_piece(square!("B1"), square!("B3")).is_err());
        assert!(board.move_piece(square!("B1"), square!("B4")).is_err());
        assert!(board.move_piece(square!("B1"), square!("B1")).is_err());
        assert!(board.move_piece(square!("B1"), square!("D3")).is_err());
    }

    // TODO: Add test case that checks that pawns can't move through pieces when moving two steps
}
