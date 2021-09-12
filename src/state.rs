use crate::Square;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    move_clocks: MoveClocks,
    en_passant_square: Option<Square>,
}

impl State {
    pub fn empty() -> Self {
        Self {
            castling_rights: CastlingRights::empty(),
            move_clocks: MoveClocks::empty(),
            en_passant_square: None,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MoveClocks {
    castling_rights: CastlingRights,
    move_counter: u16,
    half_move_counter: u8,
}

impl MoveClocks {
    pub fn empty() -> Self {
        Self {
            move_counter: 0,
            half_move_counter: 0,
            castling_rights: CastlingRights::empty(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct CastlingRights {}

impl CastlingRights {
    fn empty() -> Self {
        Self {}
    }
}
