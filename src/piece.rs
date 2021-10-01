use crate::defs::{Color, Pieces};
use bit::BitIndex;

/// A Piece consists of 8 bits
/// # Assignment of bits
/// ```text
/// 0      0      0         0        0      0       0       0
/// ^      ^      ^         ^        ^      ^       ^       ^
/// Team   Pawn   Bishop    Knight   Rook   Queen   King    has moved
/// ```
/// The first bit is 0 if the team is white
/// # Edge Cases
/// If a piece is <1 then it's empty
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
        debug_assert!(!self.is_invalid());
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
        } else if self.0.bit(1) {
            Pieces::KING
        } else {
            panic!("No piece found")
        }
    }
    /// Returns true if a piece is invalid(Has multiple/no piece type). Useful for debugging.
    pub fn is_invalid(&self) -> bool {
        let mut types: u8 = 0;
        for bit in 1..=6 {
            if self.0.bit(bit) {
                types += 1;
            }
        }
        types != 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::BitPieces;
    #[test]
    fn test_get_piece() {
        let mut piece = Piece(BitPieces::PAWN);
        assert_eq!(Pieces::PAWN, piece.get_type());
        piece = Piece(BitPieces::BISHOP | BitPieces::BLACK);
        assert_eq!(Pieces::BISHOP, piece.get_type());
        piece = Piece(BitPieces::KNIGHT | BitPieces::HAS_MOVED);
        assert_eq!(Pieces::KNIGHT, piece.get_type());
        piece = Piece(BitPieces::BLACK | BitPieces::ROOK | BitPieces::HAS_MOVED);
        assert_eq!(Pieces::ROOK, piece.get_type());
        piece = Piece(BitPieces::QUEEN);
        assert_eq!(Pieces::QUEEN, piece.get_type());
        piece = Piece(BitPieces::KING | BitPieces::BLACK);
        assert_eq!(5, piece.get_type());
    }
    #[test]
    fn test_get_color() {
        let mut piece = Piece(0b11111111);
        assert_eq!(Color::Black, piece.get_color());
        piece = Piece(BitPieces::BLACK | BitPieces::ROOK);
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
    #[test]
    fn test_is_invalid() {
        let mut piece = Piece(0b00000001);
        assert!(piece.is_invalid());
        piece = Piece(BitPieces::HAS_MOVED | BitPieces::ROOK);
        assert!(!piece.is_invalid());
        piece = Piece(BitPieces::BLACK | BitPieces::HAS_MOVED | BitPieces::ROOK);
        assert!(!piece.is_invalid());
        piece.0 |= BitPieces::KING;
        assert!(piece.is_invalid());
        piece.0 |= BitPieces::PAWN;
        assert!(piece.is_invalid());
        piece.0.set_bit(0, false);
        assert!(piece.is_invalid());
        piece.0 = BitPieces::PAWN;
        assert!(!piece.is_invalid());
    }
}
