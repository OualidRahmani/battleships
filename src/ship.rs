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

impl ShipPlacement {
    pub fn determine_ship_locations(&self) -> Vec<Position> {
        let length = usize::from(self.ship.length);
        let mut positions: Vec<Position> = vec![self.position];
        for i in 2..=length {
            if self.orientation == Orientation::HORIZONTAL {
                positions.push(Position {
                    x: self.position.x,
                    y: self.position.y + (i - 1) as u8,
                })
            } else {
                positions.push(Position {
                    x: self.position.x + (i - 1) as u8,
                    y: self.position.y,
                })
            }
        }
        positions
    }
}
