use std::fmt::Display;

use array2d::Array2D;

macro_rules! p {
    ($col:ident,$row:ident) => {
        Point {
            row: $row,
            col: $col,
        }  
    };
    ($col:literal,$row:literal) => {
        Point {
            row: $row,
            col: $col,
        }
    };
}

const BOARD_SIZE: usize = 50;

const DIRECTION_DELTAS: &'static [(isize, isize)] = &[
    ( 1, 0),
    ( 1, 1),
    ( 0, 1),
    (-1, 1),
    (-1, 0),
    (-1,-1),
    ( 0,-1),
    ( 1,-1),
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        *self == Cell::Alive
    }
}

#[derive(Debug)]
pub struct GameState {
    width: usize,
    height: usize,
    cells: Array2D<Cell>,
}


impl GameState {
    pub fn new(width: usize, height: usize) -> GameState {
        GameState {
            width,
            height,
            cells: Array2D::filled_with(Cell::Dead, height, width),
        }
    }

    pub fn update(&mut self) {
        let _cells = self.cells
                        .elements_row_major_iter();
    }

    pub fn get_cell(&self, p: Point) -> Option<&Cell> {
        self.cells.get(p.row, p.col)
    }

    pub fn set_cell(&mut self, p: Point, value: Cell) {
        self.cells.set(p.row, p.col, value).unwrap();
    }

    pub fn alive_neighbor_count(&self, p: Point) -> usize {
        let mut result = 0;

        for delta in DIRECTION_DELTAS {
            let r = p.row.checked_add_signed(delta.0);
            let c = p.col.checked_add_signed(delta.1);

            let (Some(c), Some(r)) = (c, r) else {
                continue;
            };
            
            if let Some(Cell::Alive) = self.get_cell(p!(c, r)) {
                result += 1;
            };
        }

        result
    }

    pub fn alive_neighbor_count_iter(&self, p: Point) -> usize {
        DIRECTION_DELTAS
            .into_iter()
            .filter_map(|delta| {
                let (Some(r),Some(c)) = (
                    p.row.checked_add_signed(delta.0),
                    p.col.checked_add_signed(delta.1)
                ) else {
                    return None;
                };

                Some(p!(c, r))
            })
            .filter_map(|point| self.get_cell(point))
            .filter(|x| x.is_alive())
            .count()
            //.for_each(|v| println!("{v:?}"));

    }

    pub fn alive_cell_count(&self) -> usize {
        self.cells
            .elements_row_major_iter()
            .filter(|c| **c == Cell::Alive)
            .count()
    }


    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}


fn main() {
    let mut game = GameState::new(BOARD_SIZE, BOARD_SIZE);

    game.set_cell(p!(0,1), Cell::Alive);
    game.set_cell(p!(1,1), Cell::Alive);
    game.set_cell(p!(1,0), Cell::Alive);

    let nc_1  = game.alive_neighbor_count(p!(0, 0));
    let nc_1a = game.alive_neighbor_count(p!(1, 1));
    let nc_2 = game.alive_neighbor_count_iter(p!(0,0));
    let nc_2a = game.alive_neighbor_count_iter(p!(1, 1));

    dbg!(&nc_1);
    dbg!(&nc_2);
    dbg!(&nc_1a);
    dbg!(&nc_2a);
}
