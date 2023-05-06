use crate::board::Board;
use crate::color::Color;
use crate::error::chess_error;
use crate::piece::Piece;

use crate::internal::BoardImpl;

use std::error::Error;
use std::iter::zip;

pub fn import(fen_pos: &str) -> Result<Box<dyn Board>, Box<dyn Error>> {
    let mut split = fen_pos.split(' ');

    let piece_placement = split
        .next()
        .ok_or(chess_error("Piece placement field is missing"))?;
    let piece_placement = parse_piece_placement(piece_placement)?;

    let side_to_move = split
        .next()
        .ok_or(chess_error("Side-to-move field is missing"))?;
    let side_to_move = parse_side_to_move(side_to_move)?;

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
        .ok_or(chess_error("Halfmove clock field is missing"))?;
    // TODO: Parse

    Ok(Box::new(BoardImpl {
        pieces: piece_placement,
        side_to_move,
    }))
}

fn parse_piece_placement(placement: &str) -> Result<Box<[[Option<Piece>; 8]; 8]>, Box<dyn Error>> {
    let mut res = Box::new([[None; 8]; 8]);

    let ranks = placement.split('/');

    for (rank_idx, rank) in zip((0..8).rev(), ranks) {
        parse_rank(rank_idx, rank, &mut res)?;
    }

    Ok(res)
}

fn parse_rank(
    rank_idx: usize,
    rank: &str,
    pieces: &mut Box<[[Option<Piece>; 8]; 8]>,
) -> Result<(), Box<dyn Error>> {
    let mut next_piece_file = 0;

    for ch in rank.chars() {
        match ch {
            // Rank is empty
            '8' => break,
            // File offset to next piece
            '1'..='7' => {
                next_piece_file += ch.to_digit(10).unwrap() as usize;
                if next_piece_file >= 8 {
                    return Err(chess_error(&format!("Rank is invalid ({})", rank)));
                }
            }
            // Piece specifier
            _ => {
                pieces[next_piece_file][rank_idx] = Some(parse_piece(ch)?);
                next_piece_file += 1;
            },
        }
    }

    Ok(())
}

fn parse_piece(ch: char) -> Result<Piece, Box<dyn Error>> {
    let color = if ch.is_uppercase() {
        Color::WHITE
    } else {
        Color::BLACK
    };

    match ch {
        'p' | 'P' => Ok(Piece::PAWN(color, false)), // XXX: Think about second arg (false) here
        'n' | 'N' => Ok(Piece::KNIGHT(color)),
        'b' | 'B' => Ok(Piece::BISHOP(color)),
        'r' | 'R' => Ok(Piece::ROOK(color, false)), // XXX: Think about second arg (false) here
        'q' | 'Q' => Ok(Piece::QUEEN(color)),
        'k' | 'K' => Ok(Piece::KING(color, false)), // XXX: Think about second arg (false) here
        _ => Err(chess_error(&format!(
            "Could not parse piece specifier '{}'",
            ch
        ))),
    }
}

fn parse_side_to_move(side_to_move: &str) -> Result<Color, Box<dyn Error>> {
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

pub fn _export(_board: &dyn Board) -> String {
    // XXX
    todo!("implement");
}

// TODO: Shredder FEN
