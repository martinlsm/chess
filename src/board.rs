use crate::color::Color;
use crate::piece::{Piece, get_color, tag_as_moved};

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct ChessError(String);

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0)
    }
}

impl Error for ChessError {}

pub trait Board {
    fn whose_move(&self) -> Color;
    fn get_piece(&self, square: &Square) -> Option<Piece>;
    fn move_piece(&mut self, from: &Square, to: &Square) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, PartialEq)]
pub struct Square(usize, usize);

pub fn new_board() -> Box<dyn Board> {
    let mut pieces = [[None; 8]; 8];

    for col in 0..8 {
        pieces[col][1] = Some(Piece::PAWN(Color::WHITE, false));
    }
    pieces[0][0] = Some(Piece::ROOK(Color::WHITE, false));
    pieces[1][0] = Some(Piece::KNIGHT(Color::WHITE));
    pieces[2][0] = Some(Piece::BISHOP(Color::WHITE));
    pieces[3][0] = Some(Piece::QUEEN(Color::WHITE));
    pieces[4][0] = Some(Piece::KING(Color::WHITE, false));
    pieces[5][0] = Some(Piece::BISHOP(Color::WHITE));
    pieces[6][0] = Some(Piece::KNIGHT(Color::WHITE));
    pieces[7][0] = Some(Piece::ROOK(Color::WHITE, false));

    for col in 0..8 {
        pieces[col][6] = Some(Piece::PAWN(Color::BLACK, false));
    }
    pieces[0][7] = Some(Piece::ROOK(Color::BLACK, false));
    pieces[1][7] = Some(Piece::KNIGHT(Color::BLACK));
    pieces[2][7] = Some(Piece::BISHOP(Color::BLACK));
    pieces[3][7] = Some(Piece::QUEEN(Color::BLACK));
    pieces[4][7] = Some(Piece::KING(Color::BLACK, false));
    pieces[5][7] = Some(Piece::BISHOP(Color::BLACK));
    pieces[6][7] = Some(Piece::KNIGHT(Color::BLACK));
    pieces[7][7] = Some(Piece::ROOK(Color::BLACK, false));

    Box::new(BoardImpl {
        pieces: pieces,
        player_turn: Color::WHITE,
    })
}

struct BoardImpl {
    pieces: [[Option<Piece>; 8]; 8],
    player_turn: Color,
}

impl Board for BoardImpl {
    fn whose_move(&self) -> Color {
        Color::WHITE
    }

    fn get_piece(&self, sq: &Square) -> Option<Piece> {
        self.pieces[sq.0][sq.1].clone()
    }

    fn move_piece(&mut self, from: &Square, to: &Square) -> Result<(), Box<dyn Error>> {
        let piece = &self.pieces[from.0][from.1];
        if *piece == None {
            return Err(Box::new(ChessError(String::from("Square is empty"))));
        }
        let piece = piece.unwrap();

        if get_color(&piece) != self.player_turn {
            return Err(Box::new(ChessError(String::from(
                "Not this player's turn to move",
            ))));
        }

        if !self.valid_move(&piece, from, to) {
            return Err(Box::new(ChessError(String::from("Invalid move"))));
        }

        self.pieces[to.0][to.1] = self.pieces[from.0][from.1].map(|p| tag_as_moved(&p));
        self.pieces[from.0][from.1] = None;
        tag_as_moved(&mut self.pieces[to.0][to.1].unwrap());
        self.player_turn = if self.player_turn == Color::WHITE {
            Color::BLACK
        } else {
            Color::WHITE
        };

        Ok(())
    }
}

impl BoardImpl {
    fn valid_move(&self, piece: &Piece, from: &Square, to: &Square) -> bool {
        if from == to {
            return false;
        }

        let brd = &self.pieces;
        match piece {
            Piece::BISHOP(_) => todo!(),
            Piece::KING(_, _) => todo!(),
            Piece::KNIGHT(_) => todo!(),
            Piece::PAWN(color, has_moved) => {
                if let Some(path) = perpendicular_path(from, to) {
                    // Verify that path has valid length
                    if path.len() > 2 || path.len() > 1 && *has_moved {
                        return false;
                    }

                    // Verify that the perpendicular path is forward
                    if *color == Color::WHITE && to.1 <= from.1
                        || *color == Color::BLACK && to.1 >= from.1
                    {
                        return false;
                    }

                    // Verify that squares in the path are empty
                    return path.iter().all(|sq| brd[sq.0][sq.1] == None);
                }

                // TODO: Pawn should be able to capture
                false
            }
            Piece::QUEEN(_) => todo!(),
            Piece::ROOK(_, _) => todo!(),
        }
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

    use itertools::iproduct;

    #[test]
    fn white_is_starting_player() {
        let board = new_board();
        assert_eq!(board.whose_move(), Color::WHITE);
    }

    #[test]
    fn all_pieces_are_setup_correctly() {
        let board = new_board();

        // White's pawns
        for col in 0..8 {
            assert_eq!(
                board.get_piece(&Square(col, 1)),
                Some(Piece::PAWN(Color::WHITE, false))
            );
        }

        // Rest of white's pieces
        assert_eq!(
            board.get_piece(&Square(0, 0)),
            Some(Piece::ROOK(Color::WHITE, false))
        );
        assert_eq!(
            board.get_piece(&Square(1, 0)),
            Some(Piece::KNIGHT(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(&Square(2, 0)),
            Some(Piece::BISHOP(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(&Square(3, 0)),
            Some(Piece::QUEEN(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(&Square(4, 0)),
            Some(Piece::KING(Color::WHITE, false))
        );
        assert_eq!(
            board.get_piece(&Square(5, 0)),
            Some(Piece::BISHOP(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(&Square(6, 0)),
            Some(Piece::KNIGHT(Color::WHITE))
        );
        assert_eq!(
            board.get_piece(&Square(7, 0)),
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
            board.get_piece(&Square(0, 7)),
            Some(Piece::ROOK(Color::BLACK, false))
        );
        assert_eq!(
            board.get_piece(&Square(1, 7)),
            Some(Piece::KNIGHT(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(&Square(2, 7)),
            Some(Piece::BISHOP(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(&Square(3, 7)),
            Some(Piece::QUEEN(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(&Square(4, 7)),
            Some(Piece::KING(Color::BLACK, false))
        );
        assert_eq!(
            board.get_piece(&Square(5, 7)),
            Some(Piece::BISHOP(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(&Square(6, 7)),
            Some(Piece::KNIGHT(Color::BLACK))
        );
        assert_eq!(
            board.get_piece(&Square(7, 7)),
            Some(Piece::ROOK(Color::BLACK, false))
        );

        // Rest of the squares should be empty
        assert!(iproduct!(0..8, 2..6).all(|(col, row)| board.get_piece(&Square(col, row)) == None));
    }

    #[test]
    fn move_pawn_one_step_after_new_game() {
        let mut board = new_board();

        assert!(board.move_piece(&Square(0, 1), &Square(0, 2)).is_ok());
    }

    #[test]
    fn move_pawn_two_steps_after_new_game() {
        let mut board = new_board();

        assert!(board.move_piece(&Square(3, 1), &Square(3, 3)).is_ok());
    }

    #[test]
    fn reject_white_to_move_twice() -> Result<(), Box<dyn Error>> {
        let mut board = new_board();

        // Valid move
        board.move_piece(&Square(3, 1), &Square(3, 3))?;

        // Valid move, but it's black's turn
        assert!(board.move_piece(&Square(0, 1), &Square(0, 2)).is_err());

        Ok(())
    }

    #[test]
    fn reject_white_pawn_to_move_diagonally_forwards() {
        let mut board = new_board();

        assert!(board.move_piece(&Square(4, 1), &Square(3, 2)).is_err());
    }

    #[test]
    fn get_pawn_after_it_has_moved() -> Result<(), Box<dyn Error>> {
        let mut board = new_board();
        let from_sq = Square(2, 1);
        let target_sq = Square(2, 3);

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
    fn black_is_not_able_to_start_the_game() {
        let mut board = new_board();

        // Check every pawn
        for i in 0..8 {
            assert!(board.move_piece(&Square(i, 6), &Square(i, 5)).is_err());
        }
    }

    #[test]
    fn black_is_allowed_to_move_after_white() -> Result<(), Box<dyn Error>> {
        let mut board = new_board();

        // Move a white pawn
        board.move_piece(&Square(1, 1), &Square(1, 2))?;

        // Move a black pawn
        assert!(board.move_piece(&Square(4, 6), &Square(4, 4)).is_ok());

        Ok(())
    }

    #[test]
    fn black_is_not_able_to_move_after_white_failed_a_move() -> Result<(), Box<dyn Error>> {
        let mut board = new_board();

        // Try to move a white pawn diagonally (invalid)
        let _ = board.move_piece(&Square(3, 1), &Square(4, 2));

        assert!(board.move_piece(&Square(0, 6), &Square(0, 5)).is_err());

        Ok(())
    }

    #[test]
    fn pawns_cannot_move_into_eachother() -> Result<(), Box<dyn Error>> {
        let mut board = new_board();

        // Move white and black pawn adjacent to eachother
        board.move_piece(&Square(3, 1), &Square(3, 3))?;
        board.move_piece(&Square(3, 6), &Square(3, 4))?;

        assert!(board.move_piece(&Square(3, 3), &Square(3, 4)).is_err());

        Ok(())
    }

    // TODO: Add out-of-bounds tests
    // TODO: Macros for squares, e.g. sq!("A1") == Square(0, 0)
}
