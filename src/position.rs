use crate::bitboard::BitBoard;
use crate::piece::Piece;
use crate::state::State;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    /// Board for each side
    bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    bb_pieces: [[BitBoard; 6]; 2],
    // State contains info about who castles, if en passant is present and so on
    state: State,
}

impl Position {
    pub fn empty() -> Self {
        let bb_sides = [BitBoard::empty(); 2];
        let bb_pieces = [[BitBoard::empty(); 6]; 2];
        let state = State::empty();
        Position {
            bb_sides,
            bb_pieces,
            state,
        }
    }
    pub fn at(&self) -> Piece {
        //FIXME
        Piece(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::{Pieces, Sides};

    #[test]
    fn test_empty_position() {
        let pos = Position::empty();
        pos.bb_pieces
            .iter()
            .for_each(|team| team.iter().for_each(|bb| assert!(bb.is_empty())));
        pos.bb_sides.iter().for_each(|bb| assert!(bb.is_empty()));
    }
}
