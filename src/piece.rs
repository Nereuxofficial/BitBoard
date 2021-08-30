use crate::defs::Color;

/// A Piece consists of 8 bits
///
/// ```text
/// 0      0      0         0        0      0       0       0
/// ^      ^      ^         ^        ^      ^       ^       ^
/// Team   Pawn   Bishop    Knight   Rook   Queen   King    has moved
/// ```
/// The color is 0 if the team is white
#[derive(Clone, Debug, PartialEq)]
pub struct Piece(pub u8);

impl Piece {
    pub fn get_color(&self) -> Color {
        (0b10000000 & self.0 != 0).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_color() {
        let mut piece = Piece(0b11111111);
        assert_eq!(Color::White, piece.get_color());
        piece = Piece(0b0101111);
        assert_eq!(Color::Black, piece.get_color());
    }
}
