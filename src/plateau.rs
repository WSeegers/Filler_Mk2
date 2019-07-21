use std::fmt;

const PLAYER1: char = 'X';
const PLAYER2: char = 'O';
const EMPTY: char = '.';

#[derive(Debug, Clone)]
pub enum Cell {
    Player1,
    Player2,
    Empty,
}

#[derive(Debug)]
pub struct Plateau {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Plateau {
    pub fn new(width: u32, height: u32) -> Plateau {
        Plateau {
            width,
            height,
            cells: vec![Cell::Empty; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> &Cell {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        } else {
            match self.cells.get((self.width * y + x) as usize) {
                Some(c) => c,
                None => panic!("Cells incorrectly initialized"),
            }
        }
    }

    pub fn set(&mut self, x: u32, y: u32, cell: Cell) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        } else {
            self.cells[(self.width * y + x) as usize] = cell;
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cell = match self {
            Cell::Player1 => PLAYER1,
            Cell::Player2 => PLAYER2,
            Cell::Empty => EMPTY,
        };
        write!(f, "{}", cell)
    }
}

impl fmt::Display for Plateau {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Plateau {} {}:", self.height, self.width)?;
        for y in 0..(self.width) {
            write!(f, "{}", y % 10)?;
        }
        writeln!(f, "")?;

        for y in 0..(self.height) {
            write!(f, "{:03} ", y)?;
            for x in 0..(self.width) {
                let cell = self.get(x, y);
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
