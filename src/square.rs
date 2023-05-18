#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_macro_is_case_insensitive() {
        assert_eq!(square!("A1"), square!("a1"));
    }
}
