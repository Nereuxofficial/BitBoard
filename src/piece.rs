use crate::defs::{Color, Pieces};
use bit::BitIndex;

/// A Piece consists of 8 bits
///
/// ```text
/// 0      0      0         0        0      0       0       0
/// ^      ^      ^         ^        ^      ^       ^       ^
/// Team   Pawn   Bishop    Knight   Rook   Queen   King    has moved
/// ```
/// The first bit is 0 if the team is white
#[derive(Clone, Debug, PartialEq)]
pub struct Piece(pub u8);

impl Piece {
    pub fn get_color(&self) -> Color {
        self.0.bit(7).into()
    }
    pub fn get_has_moved(&self) -> bool {
        self.0.bit(0)
    }
    /// Returns a usize representing a piece according to the Pieces struct.
    pub fn get_type(&self) -> usize {
        if self.0.bit(6) {
            Pieces::PAWN
        } else if self.0.bit(5) {
            Pieces::BISHOP
        } else if self.0.bit(4) {
            Pieces::KNIGHT
        } else if self.0.bit(3) {
            Pieces::ROOK
        } else if self.0.bit(2) {
            Pieces::QUEEN
        } else {
            Pieces::KING
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_piece() {
        let mut piece = Piece(0b01000000);
        assert_eq!(0, piece.get_type());
        piece = Piece(0b10100000);
        assert_eq!(1, piece.get_type());
        piece = Piece(0b00010000);
        assert_eq!(2, piece.get_type());
        piece = Piece(0b10001001);
        assert_eq!(3, piece.get_type());
        piece = Piece(0b00000100);
        assert_eq!(4, piece.get_type());
        piece = Piece(0b10000011);
        assert_eq!(5, piece.get_type());
    }
    #[test]
    fn test_get_color() {
        let mut piece = Piece(0b11111111);
        assert_eq!(Color::Black, piece.get_color());
        piece = Piece(0b10010000);
        assert_eq!(Color::Black, piece.get_color());
        piece = Piece(0b10010011);
        assert_eq!(Color::Black, piece.get_color());
        piece = Piece(0b0101111);
        assert_eq!(Color::White, piece.get_color());
        piece = Piece(0b0100000);
        assert_eq!(Color::White, piece.get_color());
    }
    #[test]
    fn test_get_has_moved() {
        let mut piece = Piece(0b11111111);
        assert!(piece.get_has_moved());
        piece = Piece(0b10010000);
        assert!(!piece.get_has_moved());
        piece = Piece(0b10010011);
        assert!(piece.get_has_moved());
        piece = Piece(0b0101111);
        assert!(piece.get_has_moved());
        piece = Piece(0b0100000);
        assert!(!piece.get_has_moved());
    }
}
