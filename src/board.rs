use itertools::Itertools;

use crate::error::chess_error;
use crate::fen;
use crate::internal::utils::clamp_board_idx;
use crate::piece::{
    is_piece, piece_color, piece_type, Color, Piece, BITS_KING, BITS_NO_PIECE, BITS_PAWN, BITS_WHITE, BITS_BLACK, BITS_BISHOP,
};
use crate::square::Square;
use crate::Result;

pub type Move = (Square, Square);

#[derive(Clone)]
pub struct Board {
    pub pieces: Box<[[Piece; 8]; 8]>,
    pub side_to_move: Color,
    /// This is set to the square that a pawn can be captured on in case it can be captured via en passant.
    /// If en passant is not possible, this is set to None. The color is set to the color of the pawn.
    /// This struct member is reset/cleared after each move.
    pub en_passant: Option<Square>,
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

    pub fn gen_moves(&mut self) -> Vec<Move> {
        let mut res = Vec::new();

        for (rank, file) in (0..8).cartesian_product(0..8) {
            let from = Square(file, rank);
            let piece = self.pieces[file][rank];
            if is_piece(piece) && piece_color(piece) == self.side_to_move() {
                match piece_type(piece) {
                    BITS_KING => res.append(&mut self.gen_king_moves(&from)),
                    BITS_PAWN => res.append(&mut self.gen_pawn_moves(&from)),
                    BITS_ROOK => res.append(&mut self.gen_rook_moves(&from)),
                    p => panic!("Piece type {p} Not implemented yet"),
                }
            }
        }

        res.into_iter().filter(|mv| !self.move_cause_self_check(*mv)).collect_vec()
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

    fn gen_king_moves(&self, from: &Square) -> Vec<Move> {
        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_KING);

        let mut res = Vec::new();

        for file in clamp_board_idx(from.0 as i32 - 1)..(clamp_board_idx(from.0 as i32 + 1) + 1) {
            for rank in clamp_board_idx(from.1 as i32 - 1)..(clamp_board_idx(from.1 as i32 + 1) + 1)
            {
                if from.0 == file && from.1 == rank {
                    // The king's own position
                    continue;
                }

                let king_col: Color = piece_color(self.pieces[from.0][from.1]);

                let p = self.get_piece_unbounded(file as i32, rank as i32);
                if is_piece(p) && piece_color(p) == king_col {
                    continue;
                }

                res.push((*from, Square(file, rank)));
            }
        }

        res
    }

    fn gen_pawn_moves(&self, from: &Square) -> Vec<Move> {
        let file = from.0;
        let rank = from.1;
        let piece = self.pieces[file][rank];
        let facing_dir: i32 = if self.side_to_move() == BITS_WHITE { 1 } else { -1 };

        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_PAWN);
        assert_eq!(piece_color(self.pieces[from.0][from.1]), self.side_to_move());
        assert!(rank > 0);
        assert!(rank < 7);

        let mut res = Vec::new();

        // Move forward one step
        let rank_dest = (rank as i32 + facing_dir) as usize;
        if self.pieces[file][rank_dest] == BITS_NO_PIECE {
            res.push((*from, Square(file, rank_dest)));

            // Move forward two steps
            let rank_dest = (rank as i32 + 2 * facing_dir) as usize;
            if ((rank == 1 && piece_color(piece) == BITS_WHITE)
            && (rank == 6 && piece_color(piece) == BITS_BLACK)) && self.pieces[file][rank_dest] == BITS_NO_PIECE {
                res.push((*from, Square(file, rank_dest)));
            }
        }

        // Capture right
        if file < 7 {
            let dest = self.pieces[file + 1][rank_dest];
            if is_piece(dest) && piece_color(piece) != piece_color(dest) {
                res.push((*from, Square(file + 1, rank_dest)));
            } else if self
                .en_passant
                .map_or(false, |sq| Square(file + 1, rank_dest) == sq)
            {
                res.push((*from, Square(file + 1, rank_dest)));
            }
        }

        // Capture left
        if file > 0 {
            let dest = self.pieces[file - 1][rank_dest];
            if is_piece(dest) && piece_color(piece) != piece_color(dest) {
                res.push((*from, Square(file - 1, rank_dest)));
            } else if self
                .en_passant
                .map_or(false, |sq| Square(file - 1, rank_dest) == sq)
            {
                res.push((*from, Square(file - 1, rank_dest)));
            }
        }

        res
    }

    fn gen_rook_moves(&self, &from: &Square) -> Vec<Move> {
        let file = from.0;
        let rank = from.1;
        let piece = self.pieces[file][rank];
        let rook_color = piece_color(piece);

        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_PAWN);
        assert_eq!(piece_color(self.pieces[from.0][from.1]), self.side_to_move());
        assert!(rank > 0);
        assert!(rank < 7);

        let mut res = Vec::new();

        // Walk right until a piece is found.
        let (p, steps) = self.walk_to_find_piece(&from, 1, 0);
        if is_piece(p) {
            let mut moves_right = (0..steps).map(|x| Square(file + x, rank)).collect_vec();
            if piece_color(p) != rook_color {
                moves_right.push(Square(file + steps, rank));
            }
            res.append(&mut moves_right);
        }

        // XXX: Other directions

        panic!()
    }

    // TODO: Refactor this. It shouldn't require a mut reference.
    fn move_cause_self_check(&mut self, move_: Move) -> bool {
        let from = move_.0;
        let to = move_.1;

        assert!(piece_color(self.pieces[from.0][from.1]) == self.side_to_move());
        assert!(!is_piece(self.pieces[to.0][to.1]));

        // Do the move temporarily
        self.pieces[to.0][to.1] = self.pieces[from.0][from.1];
        self.pieces[from.0][from.1] = BITS_NO_PIECE;

        // Check for self check
        let in_check = self.check_for_check(self.side_to_move());

        // Revert the move
        self.pieces[from.0][from.1] = self.pieces[to.0][to.1];
        self.pieces[to.0][to.1] = BITS_NO_PIECE;

        in_check
    }

    fn check_for_check(&self, color: Color) -> bool {
        // TODO: Optimize this code
        // Find the king
        let mut king_file: usize = 0x0badf00d;
        let mut king_rank: usize = 0xdeadbeef;
        for file in 0..8 {
            for rank in 0..8 {
                let p = self.pieces[file][rank];
                if piece_type(p) == BITS_KING && piece_color(p) == color {
                    king_file = file;
                    king_rank = rank;
                }
            }
        }
        let kf = king_file as i32;
        let kr = king_rank as i32;
        let king_sq = Square(king_file, king_rank);

        // Flip pawn facing direction since the opponents pawns are interesting
        let pawn_facing_dir: i32 = if color == BITS_WHITE { -1 } else { 1 };

        // Does a pawn threaten the king from the right file?
        let p = self.get_piece_unbounded(kf + 1, kr - pawn_facing_dir);
        if piece_color(p) != color && piece_type(p) == BITS_PAWN {
            return true;
        }

        // Does a pawn threaten the king from the left file?
        let p = self.get_piece_unbounded(kf - 1, kr - pawn_facing_dir);
        if piece_color(p) != color && piece_type(p) == BITS_PAWN {
            return true;
        }

        // Does the other king threaten the king? This can never happen in a real game,
        // but this needs to be checked to validate if the board is valid after a move.
        for file in (kf - 1)..(kf + 1) {
            for rank in (kr - 1)..(kr + 1) {
                let p = self.get_piece_unbounded(file, rank);
                if piece_type(p) == BITS_KING && piece_color(p) != color {
                    return true;
                }
            }
        }

        // Check for bishop from up/right.
        let (p, _) = self.walk_to_find_piece(&king_sq, 1, 1);
        if piece_type(p) == BITS_BISHOP && piece_color(p) != color {
            return true;
        }
        // Check for bishop from up/left.
        let (p, _) = self.walk_to_find_piece(&king_sq, -1, 1);
        if piece_type(p) == BITS_BISHOP && piece_color(p) != color {
            return true;
        }
        // Check for bishop from down/left.
        let (p, _) = self.walk_to_find_piece(&king_sq, -1, -1);
        if piece_type(p) == BITS_BISHOP && piece_color(p) != color {
            return true;
        }
        // Check for bishop from down/right.
        let (p, _) = self.walk_to_find_piece(&king_sq, 1, -1);
        if piece_type(p) == BITS_BISHOP && piece_color(p) != color {
            return true;
        }

        false
    }

    /// Walk in a specified direction from a starting square until a piece is found.
    ///
    /// This function starts from a given square and moves stepwise as defined by 
    /// `file_step_sz` and `rank_step_sz`. It continues to move in this direction, step 
    /// by step, until it either finds a piece or reaches the edge of the board. If a 
    /// piece is found, the function returns the piece and the number of steps taken to 
    /// reach it. If no piece is found and the edge of the board is reached, it returns 
    /// `BITS_NO_PIECE` and -1.
    /// 
    /// # Returns
    ///
    /// A tuple where the first element is the `Piece` found (or `BITS_NO_PIECE` if no 
    /// piece is found) and the second element is the number of steps taken to find the 
    /// piece (or 0 if no piece is found).
    fn walk_to_find_piece(&self, start: &Square, file_step_sz: i32, rank_step_sz: i32) -> (Piece, usize) {
        let mut sq = Square((start.0 as i32 + file_step_sz) as usize, (start.1 as i32 + rank_step_sz) as usize);
        let mut steps_taken = 1;

        while (0..8).contains(&sq.0) && (0..8).contains(&sq.1) {
            let p = self.pieces[sq.0][sq.1];
            if p != BITS_NO_PIECE {
                return (p, steps_taken);
            }

            sq.0 = (sq.0 as i32 + file_step_sz) as usize;
            sq.1 = (sq.1 as i32 + rank_step_sz) as usize;
            steps_taken += 1;
        }

        (BITS_NO_PIECE, 0)
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

    fn get_piece_unbounded(&self, file: i32, rank: i32) -> Piece {
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            self.pieces[file as usize][rank as usize]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn standard() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/standard.json")
    }

    #[test]
    fn pawns() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/pawns.json")
    }
}
