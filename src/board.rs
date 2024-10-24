use itertools::Itertools;

use crate::error::chess_error;
use crate::fen;
use crate::internal::utils::clamp_board_idx;
use crate::piece::{
    is_piece, piece_color, piece_type, Color, Piece, BITS_BISHOP, BITS_BLACK, BITS_KING,
    BITS_KNIGHT, BITS_NO_PIECE, BITS_PAWN, BITS_QUEEN, BITS_ROOK, BITS_WHITE,
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
                    BITS_KNIGHT => res.append(&mut self.gen_knight_moves(&from)),
                    BITS_BISHOP => res.append(&mut self.gen_bishop_moves(&from)),
                    BITS_QUEEN => res.append(&mut self.gen_queen_moves(&from)),
                    p => panic!("Piece type {p} Not implemented yet"),
                }
            }
        }

        res.into_iter()
            .filter(|mv| !self.move_cause_self_check(*mv))
            .collect_vec()
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
        let facing_dir: i32 = if self.side_to_move() == BITS_WHITE {
            1
        } else {
            -1
        };

        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_PAWN);
        assert_eq!(
            piece_color(self.pieces[from.0][from.1]),
            self.side_to_move()
        );
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
                && (rank == 6 && piece_color(piece) == BITS_BLACK))
                && self.pieces[file][rank_dest] == BITS_NO_PIECE
            {
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

    fn gen_bishop_moves(&self, &from: &Square) -> Vec<Move> {
        assert_eq!(
            piece_color(self.pieces[from.0][from.1]),
            self.side_to_move()
        );

        let mut res = Vec::new();

        // Walk along the diagonal directions
        res.append(&mut self.straight_path(&from, 1, 1));
        res.append(&mut self.straight_path(&from, 1, -1));
        res.append(&mut self.straight_path(&from, -1, -1));
        res.append(&mut self.straight_path(&from, -1, 1));

        res.iter().map(|dest| (from, *dest)).collect_vec()
    }

    fn gen_rook_moves(&self, &from: &Square) -> Vec<Move> {
        assert_eq!(
            piece_color(self.pieces[from.0][from.1]),
            self.side_to_move()
        );

        let mut res = Vec::new();

        // Walk along the orthogonal directions
        res.append(&mut self.straight_path(&from, 1, 0));
        res.append(&mut self.straight_path(&from, -1, 0));
        res.append(&mut self.straight_path(&from, 0, 1));
        res.append(&mut self.straight_path(&from, 0, -1));

        res.iter().map(|dest| (from, *dest)).collect_vec()
    }

    fn gen_knight_moves(&self, &from: &Square) -> Vec<Move> {
        let file = from.0;
        let rank = from.1;
        let piece = self.pieces[file][rank];
        let knight_color = piece_color(piece);

        assert_eq!(piece_type(self.pieces[from.0][from.1]), BITS_KNIGHT);
        assert_eq!(
            piece_color(self.pieces[from.0][from.1]),
            self.side_to_move()
        );

        let mut res = Vec::new();

        let step_offsets = vec![
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, 1),
            (2, 2),
        ];
        for (file_step, rank_step) in step_offsets {
            let dest_file = file as i32 + file_step;
            let dest_rank = rank as i32 + rank_step;
            if dest_file >= 0 && dest_file < 8 && dest_rank >= 0 && dest_rank < 8 {
                let p = self.pieces[dest_file as usize][dest_rank as usize];
                if !(is_piece(p) && piece_color(p) == knight_color) {
                    res.push(Square(dest_file as usize, dest_rank as usize));
                }
            }
        }

        res.iter().map(|dest| (from, *dest)).collect_vec()
    }

    fn gen_queen_moves(&self, &from: &Square) -> Vec<Move> {
        self.gen_bishop_moves(&from)
            .into_iter()
            .chain(self.gen_rook_moves(&from).into_iter())
            .collect_vec()
    }

    // TODO: Refactor this. It shouldn't require a mut reference.
    fn move_cause_self_check(&mut self, move_: Move) -> bool {
        let from = move_.0;
        let to = move_.1;

        assert!(piece_color(self.pieces[from.0][from.1]) == self.side_to_move());

        // Do the move temporarily
        let target_sq_state = self.pieces[to.0][to.1];
        self.pieces[to.0][to.1] = self.pieces[from.0][from.1];
        self.pieces[from.0][from.1] = BITS_NO_PIECE;

        // Check for self check
        let in_check = self.check_for_check(self.side_to_move());

        // Revert the move
        self.pieces[from.0][from.1] = self.pieces[to.0][to.1];
        self.pieces[to.0][to.1] = target_sq_state;

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

        // Check for knight
        let knight_offsets = vec![
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ];
        for offset in &knight_offsets {
            let file = kf + offset.0;
            let rank = kr + offset.1;
            let p = self.get_piece_unbounded(file, rank);
            if piece_type(p) == BITS_KNIGHT && piece_color(p) != color {
                return true;
            }
        }

        // Check for bishop or queen (diagonally)
        let bishop_dirs = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
        for dir in &bishop_dirs {
            let (p, _) = self.walk_to_piece_or_border(&king_sq, dir.0, dir.1);
            if (piece_type(p) == BITS_BISHOP || piece_type(p) == BITS_QUEEN)
                && piece_color(p) != color
            {
                return true;
            }
        }

        // Check for rook or queen (orthogonally)
        let rook_dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for dir in &rook_dirs {
            let (p, _) = self.walk_to_piece_or_border(&king_sq, dir.0, dir.1);
            if (piece_type(p) == BITS_ROOK || piece_type(p) == BITS_QUEEN)
                && piece_color(p) != color
            {
                return true;
            }
        }

        false
    }

    /// Walk in a specified direction from a starting square until a piece or border is found.
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
    /// piece, or the number of steps to the border in case no piece was found.
    fn walk_to_piece_or_border(
        &self,
        start: &Square,
        file_step_sz: i32,
        rank_step_sz: i32,
    ) -> (Piece, usize) {
        let mut sq = Square(
            (start.0 as i32 + file_step_sz) as usize,
            (start.1 as i32 + rank_step_sz) as usize,
        );
        let mut steps_taken = 0;

        while (0..8).contains(&sq.0) && (0..8).contains(&sq.1) {
            steps_taken += 1;

            let p = self.pieces[sq.0][sq.1];
            if p != BITS_NO_PIECE {
                return (p, steps_taken);
            }

            sq.0 = (sq.0 as i32 + file_step_sz) as usize;
            sq.1 = (sq.1 as i32 + rank_step_sz) as usize;
        }

        (BITS_NO_PIECE, steps_taken)
    }

    fn straight_path(&self, start: &Square, file_step_sz: i32, rank_step_sz: i32) -> Vec<Square> {
        let piece = self.get_piece(start);
        assert!(is_piece(piece));
        let p_color = piece_color(piece);

        let (p, steps) = self.walk_to_piece_or_border(&start, file_step_sz, rank_step_sz);
        let mut moves = (1..steps)
            .map(|x| {
                Square(
                    (start.0 as i32 + (file_step_sz * x as i32)) as usize,
                    (start.1 as i32 + rank_step_sz * x as i32) as usize,
                )
            })
            .collect_vec();

        if !is_piece(p) || is_piece(p) && piece_color(p) != p_color {
            moves.push(Square(
                (start.0 as i32 + file_step_sz * steps as i32) as usize,
                (start.1 as i32 + rank_step_sz * steps as i32) as usize,
            ));
        }

        moves
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
    fn pawns() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/pawns.json")
    }

    #[test]
    fn knights() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/knights.json")
    }

    #[test]
    fn bishops() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/bishops.json")
    }

    #[test]
    fn rooks() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/rooks.json")
    }

    #[test]
    fn queen() -> crate::Result<()> {
        crate::internal::test_utils::json::run_check_num_moves_test("test_cases/queens.json")
    }
}
