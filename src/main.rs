use crate::{
    CellState::{SHIP, UNKNOWN},
    Orientation::HORIZONTAL,
};

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    UNKNOWN,
    WATER,
    SHIP,
}

#[derive(PartialEq)]
enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
struct Position {
    x: u8, // row
    y: u8, // column
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    state: CellState,
}

struct Ship {
    length: u8,
}

struct ShipPlacement {
    ship: Ship,
    orientation: Orientation,
    position: Position,
}

struct Board {
    //0..9 instead of 1..10
    board: [Cell; 100],
}

impl Default for Cell {
    fn default() -> Self {
        Self { state: UNKNOWN }
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
}

fn is_valid_placement(board: &Board, ship_placement: ShipPlacement) -> bool {
    let length = usize::from(ship_placement.ship.length);
    let mut positions: Vec<Position> = vec![ship_placement.position];
    for i in 2..=length {
        if ship_placement.orientation == HORIZONTAL {
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

    let is_valid = check_ship_placement(&board, &positions);
    return is_valid;
}

fn check_ship_placement(board: &Board, positions: &[Position]) -> bool {
    for position in positions {
        if position.x >= 10 || position.y >= 10 {
            return false
        }
        if let Some(cell) = board.get_cell(*position) {
            if cell.state == CellState::SHIP {
                return false
            }
        } // technically not needed 
        for x in position.x.saturating_sub(1) ..=(position.x + 1).min(9) {
            for y in position.y.saturating_sub(1) ..=(position.y + 1).min(9) {
                if let Some(cell) = board.get_cell(Position { x, y }) {
                    if cell.state == CellState::SHIP {
                        return false
                    }
                }
            }
        }
    }
    return true
}

fn main() {}