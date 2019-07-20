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
