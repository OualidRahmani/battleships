use crate::position::Position;

#[derive(PartialEq)]
pub enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

pub struct Ship {
    pub length: u8,
}

pub struct ShipPlacement {
    pub ship: Ship,
    pub orientation: Orientation,
    pub position: Position,
}
