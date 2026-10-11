use crate::board::{Board, Cell, CellState};
use crate::position::Position;
use crate::ship::ShipPlacement;

#[derive(Debug)]
pub enum PlacementError {
    OutOfBounds,
    Overlap,
    Touching,
}

struct Generator {
    board: Board,
}

impl Generator {
    pub fn is_valid_placement(&self, ship_placement: ShipPlacement) -> bool {
        let positions = ship_placement.determine_ship_locations();

        return Self::check_ship_placement(self, &positions).is_ok();
    }

    fn check_ship_placement(&self, positions: &[Position]) -> Result<(), PlacementError> {
        for position in positions {
            if position.x >= 10 || position.y >= 10 {
                return Err(PlacementError::OutOfBounds);
            }
            if let Some(cell) = self.board.get_cell(*position) {
                if cell.state == CellState::SHIP || cell.state == CellState::WATER {
                    return Err(PlacementError::Overlap);
                }
            } // technically not needed 
            for x in position.x.saturating_sub(1)..=(position.x + 1).min(9) {
                for y in position.y.saturating_sub(1)..=(position.y + 1).min(9) {
                    if let Some(cell) = self.board.get_cell(Position { x, y }) {
                        if cell.state == CellState::SHIP {
                            return Err(PlacementError::Touching);
                        }
                    }
                }
            }
        }
        return Ok(());
    }

    fn place_ship(&mut self, ship_placement: ShipPlacement) -> Result<(), PlacementError> {
        let positions = ship_placement.determine_ship_locations();
        self.check_ship_placement(&positions)?;

        for position in &positions {
            self.board
                .set_cell(
                    *position,
                    Cell {
                        state: CellState::SHIP,
                    },
                )
                .unwrap();
        }

        for position in &positions {
            for x in position.x.saturating_sub(1)..=(position.x + 1).min(9) {
                for y in position.y.saturating_sub(1)..=(position.y + 1).min(9) {
                    if let Some(neighbour) = self.board.get_cell(Position { x, y }) {
                        if neighbour.state != CellState::SHIP {
                            self.board
                                .set_cell(
                                    Position { x, y },
                                    Cell {
                                        state: CellState::WATER,
                                    },
                                )
                                .unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::Cell,
        ship::{Orientation, Ship, ShipPlacement},
    };

    #[test]
    fn valid_ship_on_empty_board() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 2, y: 3 },
        };
        let generator = Generator { board };
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_ending_on_boundary() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 6 },
        };

        let generator = Generator { board };
        // Occupies (5,6), (5,7), (5,8), (5,9)
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn invalid_ship_outside_boundary() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 7 },
        };

        let generator = Generator { board };
        // Would occupy (5,7), (5,8), (5,9), (5,10)
        assert!(!generator.is_valid_placement(placement));
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

        let generator = Generator { board };
        assert!(!generator.is_valid_placement(placement));
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

        let generator = Generator { board };
        // (4,4)-(4,5) touches (5,5)
        assert!(!generator.is_valid_placement(placement));
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

        let generator = Generator { board };
        assert!(!generator.is_valid_placement(placement));
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

        let generator = Generator { board };
        // Entire row 4 separates the two ships.
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_at_top_left_corner() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 0, y: 0 },
        };

        let generator = Generator { board };
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn valid_ship_at_bottom_right_corner() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 9, y: 9 },
        };

        let generator = Generator { board };
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn valid_vertical_ship() {
        let board = Board::new();

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::VERTICAL,
            position: Position { x: 3, y: 6 },
        };

        let generator = Generator { board };
        // (3,6), (4,6), (5,6), (6,6)
        assert!(generator.is_valid_placement(placement));
    }

    #[test]
    fn place_ship_marks_ship_and_surrounding_water() {
        let board = Board::new();
        let mut generator = Generator { board };

        let placement = ShipPlacement {
            ship: Ship { length: 3 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 4, y: 3 },
        };

        assert!(generator.place_ship(placement).is_ok());

        // Ship cells
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 4, y: 3 })
                .unwrap()
                .state,
            CellState::SHIP
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 4, y: 4 })
                .unwrap()
                .state,
            CellState::SHIP
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 4, y: 5 })
                .unwrap()
                .state,
            CellState::SHIP
        );

        // Surrounding water
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 3, y: 3 })
                .unwrap()
                .state,
            CellState::WATER
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 5, y: 4 })
                .unwrap()
                .state,
            CellState::WATER
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 3, y: 2 })
                .unwrap()
                .state,
            CellState::WATER
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 4, y: 2 })
                .unwrap()
                .state,
            CellState::WATER
        );
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 4, y: 6 })
                .unwrap()
                .state,
            CellState::WATER
        );

        // Unrelated cell should remain untouched
        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 0, y: 0 })
                .unwrap()
                .state,
            CellState::UNKNOWN
        );
    }

    #[test]
    fn place_ship_at_corner_marks_only_valid_neighbours() {
        let board = Board::new();
        let mut generator = Generator { board };

        let placement = ShipPlacement {
            ship: Ship { length: 1 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 0, y: 0 },
        };

        assert!(generator.place_ship(placement).is_ok());

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 0, y: 0 })
                .unwrap()
                .state,
            CellState::SHIP
        );

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 0, y: 1 })
                .unwrap()
                .state,
            CellState::WATER
        );

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 1, y: 0 })
                .unwrap()
                .state,
            CellState::WATER
        );

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 1, y: 1 })
                .unwrap()
                .state,
            CellState::WATER
        );
    }

    #[test]
    fn invalid_place_ship_does_not_modify_board() {
        let board = Board::new();
        let mut generator = Generator { board };

        let placement = ShipPlacement {
            ship: Ship { length: 4 },
            orientation: Orientation::HORIZONTAL,
            position: Position { x: 5, y: 8 },
        };

        assert!(generator.place_ship(placement).is_err());

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 5, y: 8 })
                .unwrap()
                .state,
            CellState::UNKNOWN
        );

        assert_eq!(
            generator
                .board
                .get_cell(Position { x: 5, y: 9 })
                .unwrap()
                .state,
            CellState::UNKNOWN
        );
    }
}
