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

const DIRECTIONS: &'static [Direction] = &[
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

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
pub enum Cell {
    Dead  = 0,
    Alive = 1,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        *self == Cell::Alive
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    N  = 0,
    NE = 1,
    E  = 2,
    SE = 3,
    S  = 4,
    SW = 5,
    W  = 6,
    NW = 7,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    col: usize,
    row: usize,
}

impl Point {
    pub fn get_neighbor(&self, d: &Direction) -> Option<Point> {
        let (Some(r), Some(c)) = (
            self.row.checked_add_signed(DIRECTION_DELTAS[*d as usize].0),
            self.col.checked_add_signed(DIRECTION_DELTAS[*d as usize].1),
        ) else {
            return None;
        };

        Some(p!(c, r))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
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
        // Alive
        //    * 2 >  live_neighbors : Dies
        //    * 3 >= live_neighbors >= 2 : Survives
        //    * live_neighbors > 3: Dies
        // Dead
        //    * live_neighbors == 3 : Alive

    }

    pub fn alive_neighbor_count(&self, p: Point) -> usize {
        let get_neighbor = |d: &Direction| Point::get_neighbor(&p, d);
        let get_cell = |p: Point| GameState::get_cell(self, p);
        let is_alive = |cell: &&Cell| Cell::is_alive(cell);

        DIRECTIONS
            .into_iter()
            .filter_map(get_neighbor)
            .filter_map(get_cell)
            .filter(is_alive)
            .count()
    }

    pub fn alive_cell_count(&self) -> usize {
        self.cells
            .elements_row_major_iter()
            .filter(|cell| Cell::is_alive(cell))
            .count()
    }

    pub fn get_cell(&self, p: Point) -> Option<&Cell> {
        self.cells.get(p.row, p.col)
    }

    pub fn set_cell(&mut self, p: Point, value: Cell) {
        self.cells.set(p.row, p.col, value).unwrap();
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
    game.set_cell(p!(2,0), Cell::Alive);
    game.set_cell(p!(2,1), Cell::Alive);

    let nc_1a  = game.alive_neighbor_count(p!(0, 0));
    let nc_1b = game.alive_neighbor_count(p!(1, 1));

    dbg!(&nc_1a);
    dbg!(&nc_1b);
}
