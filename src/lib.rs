pub mod bitboard;
mod board;
pub mod defs;
pub mod piece;
mod position;
mod state;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
/// Represents a single square on the board.
/// # Representation
/// 1 is A1
/// 2 is B1
/// 64 is H8
pub struct Square(u8);

#[repr(u8)]
#[rustfmt::skip]
pub enum SquareLabels {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
#[cfg(test)]
mod tests {
    use super::*;
    fn test_square_labels() {
        let mut test_square = Square(SquareLabels::A1 as u8);
        assert_eq!(1, test_square.0);
        test_square = Square(SquareLabels::H8 as u8);
        assert_eq!(64, test_square.0);
    }
}
