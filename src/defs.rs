// Contains definitions needed across the crate.

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

/// Provides the index of the single pieces in the array of BitBoards
pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

/// Provides Labels for the `Piece` struct
pub struct BitPieces;
impl BitPieces {
    pub const HAS_MOVED: u8 = 0b00000001;
    pub const KING: u8 = 0b00000010;
    pub const QUEEN: u8 = 0b00000100;
    pub const ROOK: u8 = 0b00001000;
    pub const KNIGHT: u8 = 0b00010000;
    pub const BISHOP: u8 = 0b00100000;
    pub const PAWN: u8 = 0b01000000;
    pub const BLACK: u8 = 0b10000000;
}

/// Provides labels for the `CastlingRights struct`
pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_00: u8 = 0b00000001;
    pub const WHITE_000: u8 = 0b00000010;
    pub const BLACK_00: u8 = 0b00000100;
    pub const BLACK_000: u8 = 0b00001000;

    pub const KING_SIDE: u8 = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8 = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8 = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8 = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8 = Self::BLACK_CASTLING | Self::WHITE_CASTLING;
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}
impl From<bool> for Color {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Black,
            false => Self::White,
        }
    }
}
