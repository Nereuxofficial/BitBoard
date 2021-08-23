use crate::bitboard::BitBoard;

pub struct Position{
    /// Board for each side
    bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    bb_pieces: [[BitBoard; 6]; 2],
}

impl Position{
    pub fn empty() -> Self{
        let bb_sides = [BitBoard::empty(); 2];
        let bb_pieces= [[BitBoard::empty(); 6]; 2];
        Position{bb_sides, bb_pieces}
    }
}