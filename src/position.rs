use crate::bitboard::BitBoard;
use crate::defs::BB;
use crate::piece::Piece;
use crate::state::State;
use crate::Square;
use bit::BitIndex;

/// A Position contains everything necessary to calculate moves and evaluate a position.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    /// The board for each side
    bb_sides: [BitBoard; 2],
    /// BitBoards for all pieces and each side
    bb_pieces: [[BitBoard; 6]; 2],
    /// State contains all relevant information for evaluating a position outside the pieces.
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
    /// Returns a piece at a given square
    fn at(&self, square: Square) -> Piece {
        let val = square.0;
        if !(self.bb_sides[0].0.bit(val) || self.bb_sides[1].0.bit(val)) {
            Piece(0)
        } else {
            todo!()
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_position() {
        let pos = Position::empty();
        pos.bb_pieces
            .iter()
            .for_each(|team| team.iter().for_each(|bb| assert!(bb.is_empty())));
        pos.bb_sides.iter().for_each(|bb| assert!(bb.is_empty()));
    }
}
