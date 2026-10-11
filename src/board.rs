use crate::position::Position;
use crate::ship::{Orientation, ShipPlacement};

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    UNKNOWN,
    WATER,
    SHIP,
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    state: CellState,
}

pub struct Board {
    //0..9 instead of 1..10
    board: [Cell; 100],
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::UNKNOWN,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [Cell::default(); 100],
        }
    }

    fn get_cell(&self, position: Position) -> Option<&Cell> {
        if position.x >= 10 || position.y >= 10 {
            return None;
        }

        let idx = usize::from(position.x) * 10 + usize::from(position.y);

        self.board.get(idx)
    }

    fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), (Position, &'static str)> {
        if position.x >= 10 || position.y >= 10 {
            return Err((position, "wrong index"));
        }

        let idx = usize::from(position.x) * 10 + usize::from(position.y);
        self.board[idx] = cell;
        Ok(())
    }
    
    pub fn is_valid_placement(&self, ship_placement: ShipPlacement) -> bool {
        let length = usize::from(ship_placement.ship.length);
        let mut positions: Vec<Position> = vec![ship_placement.position];
        for i in 2..=length {
            if ship_placement.orientation == Orientation::HORIZONTAL {
                positions.push(Position {
                    x: ship_placement.position.x,
                    y: ship_placement.position.y + (i - 1) as u8,
                })
            } else {
                positions.push(Position {
                    x: ship_placement.position.x + (i - 1) as u8,
                    y: ship_placement.position.y,
                })
            }
        }
    
        let is_valid = Self::check_ship_placement(self, &positions);
        return is_valid;
    }
    
    fn check_ship_placement(&self, positions: &[Position]) -> bool {
        for position in positions {
            if position.x >= 10 || position.y >= 10 {
                return false;
            }
            if let Some(cell) = self.get_cell(*position) {
                if cell.state == CellState::SHIP {
                    return false;
                }
            } // technically not needed 
            for x in position.x.saturating_sub(1)..=(position.x + 1).min(9) {
                for y in position.y.saturating_sub(1)..=(position.y + 1).min(9) {
                    if let Some(cell) = self.get_cell(Position { x, y }) {
                        if cell.state == CellState::SHIP {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship::Ship;

    #[test]
    fn valid_ship_on_empty_board() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 2, y: 3 },
        };

        assert!(board.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_ending_on_boundary() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 6 },
        };

        // Occupies (5,6), (5,7), (5,8), (5,9)
        assert!(board.is_valid_placement(placement));
    }

    #[test]
    fn invalid_ship_outside_boundary() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 7 },
        };

        // Would occupy (5,7), (5,8), (5,9), (5,10)
        assert!(!board.is_valid_placement(placement));
    }

    #[test]
    fn invalid_ship_overlapping_existing_ship() {
        let mut board = Board::new();

        board
            .set_cell(
                Position { x: 5, y: 5 },
                Cell {
                    state: CellState::SHIP,
                },
            )
            .unwrap();

        let placement = ShipPlacement {
            ship: Ship { length: 3 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 4 },
        };

        assert!(!board.is_valid_placement(placement));
    }

    #[test]
    fn invalid_ship_touching_existing_ship() {
        let mut board = Board::new();

        board
            .set_cell(
                Position { x: 5, y: 5 },
                Cell {
                    state: CellState::SHIP,
                },
            )
            .unwrap();

        let placement = ShipPlacement {
            ship: Ship { length: 2 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 4, y: 4 },
        };

        // (4,4)-(4,5) touches (5,5)
        assert!(!board.is_valid_placement(placement));
    }

    #[test]
    fn invalid_ship_diagonally_touching_existing_ship() {
        let mut board = Board::new();

        board
            .set_cell(
                Position { x: 5, y: 5 },
                Cell {
                    state: CellState::SHIP,
                },
            )
            .unwrap();

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 4, y: 4 },
        };

        assert!(!board.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_separated_from_existing_ship() {
        let mut board = Board::new();

        board
            .set_cell(
                Position { x: 5, y: 5 },
                Cell {
                    state: CellState::SHIP,
                },
            )
            .unwrap();

        let placement = ShipPlacement {
            ship: Ship { length: 2 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 3, y: 4 },
        };

        // Entire row 4 separates the two ships.
        assert!(board.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_at_top_left_corner() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 0, y: 0 },
        };

        assert!(board.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_at_bottom_right_corner() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 9, y: 9 },
        };

        assert!(board.is_valid_placement(placement));
    }

    #[test]
    fn valid_vertical_ship() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::VERTICAL,
            position: Position { x: 3, y: 6 },
        };

        // (3,6), (4,6), (5,6), (6,6)
        assert!(board.is_valid_placement(placement));
    }
}
