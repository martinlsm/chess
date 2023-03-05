use crate::color::Color;
use crate::piece::Piece;

pub trait Board {
    fn whose_move(&self) -> Color;
    fn get_piece(&self, square: &Square) -> Option<Piece>;
}

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

    Box::new(BoardImpl { pieces: pieces })
}

struct BoardImpl {
    pieces: [[Option<Piece>; 8]; 8],
}

impl Board for BoardImpl {
    fn whose_move(&self) -> Color {
        Color::WHITE
    }

    fn get_piece(&self, sq: &Square) -> Option<Piece> {
        self.pieces[sq.0][sq.1].clone()
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
            println!("{}", col);
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
}
