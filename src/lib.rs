pub mod bitboard;
mod board;
pub mod defs;
pub mod piece;
mod position;
mod state;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Square(u64);
