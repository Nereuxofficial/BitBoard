use crate::position::Position;

#[derive(Debug)]
pub struct Board {
    pos: Position,
    move_counter: u16,
}