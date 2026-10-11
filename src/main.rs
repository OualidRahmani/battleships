use crate::CellState::UNKNOWN;

#[derive(Clone, Copy)]
enum CellState {
    UNKNOWN,
    WATER,
    SHIP,
}

enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

struct Position {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy)]
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

        let idx = usize::from(position.x) + usize::from(position.y) * 10;

        self.board.get(idx)
    }

    fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), (Position, &'static str)> {
        if position.x >= 10 || position.y >= 10 {
            return Err((position, "wrong index"))
        }

        let idx = usize::from(position.x) + usize::from(position.y) * 10;
        self.board[idx] = cell;
        Ok(())
    }
}

fn main() {}
