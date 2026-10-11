use crate::position::Position;
use crate::ship::{ShipPlacement};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    UNKNOWN,
    WATER,
    SHIP,
}

#[derive(Debug)]
enum PlacementError {
    OutOfBounds,
    Overlap,
    Touching,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkingError {
    OutOfBounds,
    Contradiction,
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

    fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), MarkingError> {
        if position.x >= 10 || position.y >= 10 {
            return Err(MarkingError::OutOfBounds);
        }

        let idx = usize::from(position.x) * 10 + usize::from(position.y);
        self.board[idx] = cell;
        Ok(())
    }

    pub fn is_valid_placement(&self, ship_placement: ShipPlacement) -> bool {
        let positions = ship_placement.determine_ship_locations();

        return Self::check_ship_placement(self, &positions).is_ok();
    }

    fn check_ship_placement(&self, positions: &[Position]) -> Result<(), PlacementError> {
        for position in positions {
            if position.x >= 10 || position.y >= 10 {
                return Err(PlacementError::OutOfBounds);
            }
            if let Some(cell) = self.get_cell(*position) {
                if cell.state == CellState::SHIP {
                    return Err(PlacementError::Overlap);
                }
            } // technically not needed 
            for x in position.x.saturating_sub(1)..=(position.x + 1).min(9) {
                for y in position.y.saturating_sub(1)..=(position.y + 1).min(9) {
                    if let Some(cell) = self.get_cell(Position { x, y }) {
                        if cell.state == CellState::SHIP {
                            return Err(PlacementError::Touching);
                        }
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn mark_water(&mut self, position: Position) -> Result<(), MarkingError> {
        if let Some(cell) = self.get_cell(position) {
            if cell.state == CellState::SHIP {
                return Err(MarkingError::Contradiction);
            } else {
                return self.set_cell(
                    position,
                    Cell {
                        state: (CellState::WATER),
                    },
                );
            }
        } else {
            return Err(MarkingError::OutOfBounds);
        }
    }

    pub fn mark_ship(&mut self, position: Position) -> Result<(), MarkingError> {
        if let Some(cell) = self.get_cell(position) {
            if cell.state == CellState::WATER {
                return Err(MarkingError::Contradiction);
            } else {
                return self.set_cell(
                    position,
                    Cell {
                        state: (CellState::SHIP),
                    },
                );
            }
        } else {
            return Err(MarkingError::OutOfBounds);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship::{Orientation, Ship};

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

    #[test]
    fn mark_unknown_as_ship() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        assert!(board.mark_ship(position).is_ok());
        assert_eq!(board.get_cell(position).unwrap().state, CellState::SHIP);
    }

    #[test]
    fn mark_ship_as_ship() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        board.mark_ship(position).unwrap();

        assert!(board.mark_ship(position).is_ok());
        assert_eq!(board.get_cell(position).unwrap().state, CellState::SHIP);
    }

    #[test]
    fn mark_water_as_ship_is_contradiction() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        board.mark_water(position).unwrap();

        assert_eq!(board.mark_ship(position), Err(MarkingError::Contradiction));

        assert_eq!(board.get_cell(position).unwrap().state, CellState::WATER);
    }

    #[test]
    fn mark_unknown_as_water() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        assert!(board.mark_water(position).is_ok());
        assert_eq!(board.get_cell(position).unwrap().state, CellState::WATER);
    }

    #[test]
    fn mark_water_as_water() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        board.mark_water(position).unwrap();

        assert!(board.mark_water(position).is_ok());
        assert_eq!(board.get_cell(position).unwrap().state, CellState::WATER);
    }

    #[test]
    fn mark_ship_as_water_is_contradiction() {
        let mut board = Board::new();
        let position = Position { x: 3, y: 4 };

        board.mark_ship(position).unwrap();

        assert_eq!(board.mark_water(position), Err(MarkingError::Contradiction));

        assert_eq!(board.get_cell(position).unwrap().state, CellState::SHIP);
    }

    #[test]
    fn marking_out_of_bounds_fails() {
        let mut board = Board::new();

        assert_eq!(
            board.mark_ship(Position { x: 10, y: 5 }),
            Err(MarkingError::OutOfBounds)
        );

        assert_eq!(
            board.mark_water(Position { x: 5, y: 10 }),
            Err(MarkingError::OutOfBounds)
        );
    }
}
