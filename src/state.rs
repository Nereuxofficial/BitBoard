use crate::defs::{Castling, Sides};
use crate::Square;

/// Contains castling_rights, move_clocks, en_passant_square if possible and the side to move
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    stm: Sides,
}

impl State {
    pub fn empty() -> Self {
        Self {
            castling_rights: CastlingRights::empty(),
            en_passant_square: None,
            half_move_counter: 0,
            stm: Sides::White,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            half_move_counter: 0,
            stm: Sides::White,
        }
    }
}

/// Castling rights are stored in a [`u8`], which is divided into the following parts:
/// ```text
/// 0 1 0 1   1                1               0                0
/// ^^^^^^^   ^                ^               ^                ^
/// unused    Black queen side Black king side White queen side White king side
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    fn empty() -> Self {
        Self(Castling::NO_CASTLING)
    }
    fn all() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}
