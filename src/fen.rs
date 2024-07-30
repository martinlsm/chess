use crate::board::Board;
use crate::error::chess_error;
use crate::piece::{
    piece_color, piece_type, Color, Piece, BITS_BISHOP, BITS_BLACK, BITS_KING, BITS_KNIGHT,
    BITS_NO_PIECE, BITS_PAWN, BITS_QUEEN, BITS_ROOK, BITS_WHITE,
};
use crate::square::Square;
use crate::Result;

use std::iter::zip;

pub fn import(fen_pos: &str) -> Result<Board> {
    let mut split = fen_pos.split(' ');

    let piece_placement = split
        .next()
        .ok_or(chess_error("Piece placement field is missing"))?;
    let piece_placement = import_piece_placement(piece_placement)?;

    let side_to_move = split
        .next()
        .ok_or(chess_error("Side-to-move field is missing"))?;
    let side_to_move = import_side_to_move(side_to_move)?;

    let _castling_ability = split
        .next()
        .ok_or(chess_error("Castling ability field is missing"))?;
    // TODO: Parse

    let en_passant_sq = split
        .next()
        .ok_or(chess_error("En passant target square field is missing"))?;
    let en_passant_sq = if en_passant_sq != "-" {
        Some(Square::from(en_passant_sq)?)
    } else {
        None
    };

    // TODO: Parse

    let _halfmove_clock = split
        .next()
        .ok_or(chess_error("Halfmove clock field is missing"))?;
    // TODO: Parse

    let _fullmove_counter = split
        .next()
        .ok_or(chess_error("Halfmove counter field is missing"))?;
    // TODO: Parse

    Ok(Board {
        pieces: piece_placement,
        side_to_move,
        en_passant: en_passant_sq,
    })
}

fn import_piece_placement(placement: &str) -> Result<Box<[[Piece; 8]; 8]>> {
    let mut res = Box::new([[BITS_NO_PIECE; 8]; 8]);

    let ranks = placement.split('/');

    for (rank_idx, rank) in zip((0..8).rev(), ranks) {
        import_rank(rank_idx, rank, &mut res)?;
    }

    Ok(res)
}

fn import_rank(rank_idx: usize, rank: &str, pieces: &mut Box<[[Piece; 8]; 8]>) -> Result<()> {
    let mut next_piece_file = 0;

    for ch in rank.chars() {
        match ch {
            // Rank is empty
            '8' => break,
            // File offset to next piece
            '1'..='7' => {
                next_piece_file += ch.to_digit(10).unwrap() as usize;
            }
            // Piece specifier
            _ => {
                if next_piece_file >= 8 {
                    return Err(chess_error(&format!("Rank is invalid ({})", rank)));
                }

                pieces[next_piece_file][rank_idx] = import_piece(ch)?;
                next_piece_file += 1;
            }
        }
    }

    Ok(())
}

fn import_piece(letter: char) -> Result<Piece> {
    let piece_type = match letter.to_uppercase().next().unwrap() {
        'B' => BITS_BISHOP,
        'K' => BITS_KING,
        'N' => BITS_KNIGHT,
        'P' => BITS_PAWN,
        'Q' => BITS_QUEEN,
        'R' => BITS_ROOK,
        _ => return Err(chess_error(&format!("Invalid piece type '{}'", letter))),
    };

    let color = if letter.is_uppercase() {
        BITS_WHITE
    } else {
        BITS_BLACK
    };

    Ok(color | piece_type)
}

fn import_side_to_move(side_to_move: &str) -> Result<Color> {
    if side_to_move.len() != 1 {
        return Err(chess_error(&format!(
            "Invalid side-to-move field (\"{}\"",
            side_to_move
        )));
    }

    let color = side_to_move.chars().nth(0).unwrap();
    match color {
        'w' => Ok(BITS_WHITE),
        'b' => Ok(BITS_BLACK),
        _ => Err(chess_error(&format!(
            "Invalid side-to-move field \"{}\"",
            side_to_move
        ))),
    }
}

pub fn export(board: &Board) -> String {
    let mut res = String::new();

    for rank in (0..8).rev() {
        let mut steps_to_next_piece = 0;
        for file in 0..8 {
            match piece_type(board.get_piece(&Square(file, rank))) {
                BITS_NO_PIECE => {
                    steps_to_next_piece += 1;
                    if file == 7 {
                        // No more piece will come. Fill out with a number.
                        res.push_str(steps_to_next_piece.to_string().as_str());
                    }
                }
                p_type => {
                    if steps_to_next_piece > 0 {
                        res.push_str(steps_to_next_piece.to_string().as_str());
                    }
                    steps_to_next_piece = 0;

                    res.push(piece_to_letter(p_type));
                }
            }
        }

        if rank > 0 {
            res.push('/');
        }
    }

    match board.side_to_move() {
        BITS_WHITE => res.push_str(" w"),
        BITS_BLACK => res.push_str(" b"),
        _ => panic!("Invalid color"),
    }

    // TODO: Castling
    res.push_str(" KQkq");

    // TODO: En passant
    let en_passant_sq = board.en_passant.map_or(String::from("-"), |sq| sq.to_str());
    res.push_str(&format!(" {en_passant_sq}"));

    // TODO: Halfmove clock
    res.push_str(" 0");

    // TODO: Fullmove counter
    res.push_str(" 0");

    res
}

pub fn piece_to_letter(piece_bits: Piece) -> char {
    let ch = match piece_type(piece_bits) {
        BITS_BISHOP => 'b',
        BITS_KING => 'k',
        BITS_KNIGHT => 'n',
        BITS_PAWN => 'p',
        BITS_QUEEN => 'q',
        BITS_ROOK => 'r',
        _ => panic!("Invalid piece bits"),
    };

    match piece_color(piece_bits) {
        BITS_WHITE => ch.to_uppercase().next().unwrap(),
        BITS_BLACK => ch,
        _ => panic!("Invalid piece bits"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fen;
    use crate::internal::test_utils::fen::{compare_fen, CMP_POS, CMP_SIDE_TO_MOVE};

    #[test]
    fn export_is_the_inverse_of_import() {
        let arbitrary_fens = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "R4b2/1K4P1/1P5P/1p6/5B2/3pp1r1/pNQ2P2/4k3 w - - 0 1",
            "2N1b1n1/4b3/1R4P1/7n/2PPk3/2q4p/p5P1/3K3R w - - 0 1",
            "8/k7/8/8/8/8/8/2K5 b - - 0 1",
            "8/4k3/8/1b6/7p/1K3P2/P7/3r4 w - - 0 1",
            "k7/7P/2r2P1P/1Q6/rp1R4/3p2N1/NqPP2p1/b5K1 w - - 0 1",
            "2q1R1n1/pP1pN1KB/P2r2nr/p1R1P3/k1bPQ1pP/1p1P1p1p/3PPNp1/2b3B1 w - - 0 1",
        ];

        let res: Vec<String> = arbitrary_fens
            .iter()
            .map(|s| fen::import(s).unwrap())
            .map(|board| fen::export(&board))
            .collect();

        assert!(zip(arbitrary_fens, res).all(|(a, b)| compare_fen(
            &a,
            &b,
            CMP_POS & CMP_SIDE_TO_MOVE
        )
        .unwrap_or(false)));
    }
}
