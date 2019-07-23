use crate::models::{piece, point};

use piece::Piece;
use point::Point;
use std::fmt;

const PLAYER1: char = 'O';
const PLAYER2: char = 'X';
const EMPTY: char = '.';

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Player1,
    Player2,
    Empty,
}

pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug)]
pub struct Plateau {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
}

impl Plateau {
    pub fn new(
        width: u32,
        height: u32,
        player1: &Point,
        player2: &Point,
    ) -> Result<Plateau, String> {
        let mut plateau = Plateau {
            width,
            height,
            cells: vec![Cell::Empty; (width * height) as usize],
        };

        match plateau.is_in_bounds(player1) {
            true => plateau.set(player1, Cell::Player1),
            false => return Err(String::from("Player1 out of bounds")),
        };

        match plateau.is_in_bounds(player2) {
            true => plateau.set(player2, Cell::Player2),
            false => return Err(String::from("Player2 out of bounds")),
        };

        Ok(plateau)
    }

    pub fn is_in_bounds(&self, p: &Point) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn get(&self, p: &Point) -> Cell {
        match self.cells.get((self.width * p.y + p.x) as usize) {
            Some(c) => c.clone(),
            None => panic!("Cells incorrectly initialized"),
        }
    }

    fn set(&mut self, p: &Point, cell: Cell) {
        self.cells[(self.width * p.y + p.x) as usize] = cell;
    }

    fn is_valid_placement(
        &self,
        piece: &Piece,
        placement: &Point,
        owner: &Cell,
    ) -> Result<(), String> {
        let mut overlap = 0;

        for y in 0..(piece.height) {
            for x in 0..(piece.width) {
                use Cell::{Empty, Player1, Player2};
                if !piece.get(Point { x, y }) {
                    continue;
                }

                let offset = &Point { x, y } + &placement;
                let plat_cell = self.get(&offset);
                match plat_cell {
                    Empty => continue,
                    Player1 | Player2 if plat_cell == *owner => {
                        overlap += 1;
                        if overlap > 1 {
                            return Err(String::from("Overlap greater than one"));
                        }
                    }
                    Player1 | Player2 => return Err(String::from("Overlap on other player")),
                }
            }
        }

        if overlap != 1 {
            return Err(String::from("No Overlap"));
        }

        Ok(())
    }

    pub fn place_piece(
        &mut self,
        piece: &Piece,
        placement: &Point,
        player: Player,
    ) -> Result<(), String> {
        if placement.x + piece.width > self.width || placement.y + piece.height > self.height {
            return Err(String::from("Out of bounds"));
        }

        let owner = match player {
            Player::Player1 => Cell::Player1,
            Player::Player2 => Cell::Player2,
        };

        self.is_valid_placement(piece, placement, &owner)?;

        for y in 0..(piece.height) {
            for x in 0..(piece.width) {
                if !piece.get(Point { x, y }) {
                    continue;
                }

                let offset = &Point { x, y } + &placement;
                self.set(&offset, owner.clone());
            }
        }

        Ok(())
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
                let cell = self.get(&Point { x, y });
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
