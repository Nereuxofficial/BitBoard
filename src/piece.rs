use crate::defs::Color;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_color() {
        let mut piece = Piece(0b11111111);
        assert_eq!(Color::Black, piece.get_color());
        let mut piece = Piece(0b10010000);
        assert_eq!(Color::Black, piece.get_color());
        let mut piece = Piece(0b10010011);
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
