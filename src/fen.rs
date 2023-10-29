use crate::board::Board;
use crate::color::Color;
use crate::error::chess_error;
use crate::piece::{letter_to_piece, piece_to_letter, Piece};
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

    let _en_passant_target_square = split
        .next()
        .ok_or(chess_error("En passant target square field is missing"))?;
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
    })
}

fn import_piece_placement(placement: &str) -> Result<Box<[[Option<Piece>; 8]; 8]>> {
    let mut res = Box::new([[None; 8]; 8]);

    let ranks = placement.split('/');

    for (rank_idx, rank) in zip((0..8).rev(), ranks) {
        import_rank(rank_idx, rank, &mut res)?;
    }

    Ok(res)
}

fn import_rank(
    rank_idx: usize,
    rank: &str,
    pieces: &mut Box<[[Option<Piece>; 8]; 8]>,
) -> Result<()> {
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

                pieces[next_piece_file][rank_idx] = Some(import_piece(ch)?);
                next_piece_file += 1;
            }
        }
    }

    Ok(())
}

fn import_piece(letter: char) -> Result<Piece> {
    let p_type = letter_to_piece(letter)?;

    let color = if letter.is_uppercase() {
        Color::WHITE
    } else {
        Color::BLACK
    };

    Ok(Piece {
        p_type: p_type,
        color: color,
        has_moved: false, // XXX: Fix later
    })
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
        'w' => Ok(Color::WHITE),
        'b' => Ok(Color::BLACK),
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
            match board.get_piece(&Square(file, rank)) {
                Some(piece) => {
                    if steps_to_next_piece > 0 {
                        res.push_str(steps_to_next_piece.to_string().as_str());
                    }
                    steps_to_next_piece = 0;

                    res.push(export_piece(&piece));
                }
                None => {
                    steps_to_next_piece += 1;
                    if file == 7 {
                        // No more piece will come. Fill out with a number.
                        res.push_str(steps_to_next_piece.to_string().as_str());
                    }
                }
            }
        }

        if rank > 0 {
            res.push('/');
        }
    }

    match board.side_to_move() {
        Color::WHITE => res.push_str(" w"),
        Color::BLACK => res.push_str(" b"),
    }

    // TODO: Castling
    res.push_str(" KQkq");

    // TODO: En passant
    res.push_str(" -");

    // TODO: Halfmove clock
    res.push_str(" 0");

    // TODO: Fullmove counter
    res.push_str(" 0");

    res
}

fn export_piece(piece: &Piece) -> char {
    piece_to_letter(piece.p_type, Some(piece.color))
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
