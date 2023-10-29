use crate::{error, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Square(pub usize, pub usize);

#[macro_export]
macro_rules! square {
    (  $str:tt ) => {{
        assert_eq!($str.len(), 2);

        let u_str = $str.to_uppercase();

        let fst: u8 = u_str.as_bytes()[0];
        assert!(fst >= 'A' as u8);
        assert!(fst <= 'H' as u8);
        let fst = (fst - 'A' as u8) as usize;

        let snd: u8 = u_str.as_bytes()[1];
        assert!(snd >= '1' as u8);
        assert!(snd <= '8' as u8);
        let snd = (snd - '1' as u8) as usize;

        &Square(fst, snd)
    }};
}

pub use square;

impl Square {
    pub fn from(s: &str) -> Result<Self> {
        if s.len() != 2 {
            return Err(error::chess_error("Invalid length of square notation"));
        }

        let u_str = s.to_uppercase();
        let fst = u_str.as_bytes()[0] as char;
        if fst < 'A' || fst > 'H' {
            return Err(error::chess_error(&format!("Invalid rank: '{}'", fst)));
        }
        let fst = fst as usize - 'A' as usize;

        let snd = u_str.as_bytes()[1] as char;
        if snd < '1' || snd > '8' {
            return Err(error::chess_error(&format!("Invalid file: '{}'", snd)));
        }
        let snd = snd as usize - '1' as usize;

        Ok(Square(fst, snd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_macro_is_case_insensitive() {
        assert_eq!(square!("A1"), square!("a1"));
    }
}
