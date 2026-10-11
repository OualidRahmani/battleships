use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CellState {
    UNKNOWN,
    WATER,
    SHIP,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkingError {
    OutOfBounds,
    Contradiction,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Cell {
    pub(crate) state: CellState,
}

#[derive(Clone, Copy)]
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

    pub(crate) fn get_cell(&self, position: Position) -> Option<&Cell> {
        if position.x >= 10 || position.y >= 10 {
            return None;
        }

        let idx = usize::from(position.x) * 10 + usize::from(position.y);

        self.board.get(idx)
    }

    pub(crate) fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), MarkingError> {
        if position.x >= 10 || position.y >= 10 {
            return Err(MarkingError::OutOfBounds);
        }

        let idx = usize::from(position.x) * 10 + usize::from(position.y);
        self.board[idx] = cell;
        Ok(())
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
