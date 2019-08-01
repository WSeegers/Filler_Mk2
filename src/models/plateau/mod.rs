mod parser;

use super::{constants, Piece, Player, Point};

use constants::*;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Player1,
    Player2,
    Empty,
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
        p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32
    }

    fn get(&self, p: &Point) -> Cell {
        match self.cells.get((self.width as i32 * p.y + p.x) as usize) {
            Some(c) => c.clone(),
            None => panic!("Cells incorrectly initialized"),
        }
    }

    fn set(&mut self, p: &Point, cell: Cell) {
        self.cells[(self.width as i32 * p.y + p.x) as usize] = cell;
    }

    fn is_valid_placement(
        &self,
        piece: &Piece,
        placement: &Point,
        owner: &Cell,
    ) -> Result<(), String> {
        let mut overlap = false;

        for y in 0..(piece.height) as i32 {
            for x in 0..(piece.width) as i32 {
                use Cell::{Empty, Player1, Player2};
                if !piece.get(Point { x, y }) {
                    continue;
                }

                let offset = &Point { x, y } + &placement;
                if !self.is_in_bounds(&offset) {
                    return Err(String::from("Piece out of bounds"));
                }

                let plat_cell = self.get(&offset);
                match plat_cell {
                    Empty => continue,
                    Player1 | Player2 if plat_cell == *owner => match overlap {
                        true => return Err(String::from("Overlap greater than one")),
                        false => overlap = true,
                    },
                    Player1 | Player2 => return Err(String::from("Overlap on other player")),
                }
            }
        }

        if !overlap {
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
        let owner = match player {
            Player::Player1 => Cell::Player1,
            Player::Player2 => Cell::Player2,
        };

        self.is_valid_placement(piece, placement, &owner)?;

        for y in 0..(piece.height) as i32 {
            for x in 0..(piece.width) as i32 {
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
        write!(f, "    ")?;
        for y in 0..(self.width) {
            write!(f, "{}", y % 10)?;
        }
        writeln!(f, "")?;

        for y in 0..(self.height) as i32 {
            write!(f, "{:03} ", y)?;
            for x in 0..(self.width) as i32 {
                let cell = self.get(&Point { x, y });
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece_horizontal() -> Piece {
        let cells = vec![false, true, true, false];
        let width = 4;
        let height = 1;
        Piece::new(width, height, cells)
    }

    fn piece_vertical() -> Piece {
        let cells = vec![false, true, true, false];
        let width = 1;
        let height = 4;
        Piece::new(width, height, cells)
    }

    fn piece_square() -> Piece {
        let mut cells = vec![true; 9];
        cells[4] = false;
        let width = 3;
        let height = 3;
        Piece::new(width, height, cells)
    }

    #[test]
    fn good_placement_horizontal_with_overlap() {
        let player_1_start = Point::new(1, 1);
        let plateau = Plateau::new(3, 3, &player_1_start, &Point::new(2, 2)).unwrap();
        let piece = piece_horizontal();

        print!("{}", plateau);
        println!("{}", piece);

        let placement = Point::new(0, 1);
        println!("placement: {:?}", placement);
        assert_eq!(
            plateau.is_valid_placement(&piece, &placement, &Cell::Player1),
            Ok(())
        );

        let placement = Point::new(-1, 1);
        println!("Placement: {:?}", placement);
        assert_eq!(
            plateau.is_valid_placement(&piece, &placement, &Cell::Player1),
            Ok(())
        );
    }

    #[test]
    fn good_placement_vertical_with_overlap() {
        let player_1_start = Point::new(1, 1);
        let plateau = Plateau::new(3, 3, &player_1_start, &Point::new(2, 2)).unwrap();
        let piece = piece_vertical();

        print!("{}", plateau);
        println!("{}", piece);

        let placement = Point::new(1, 0);
        println!("placement: {:?}", placement);
        assert_eq!(
            plateau.is_valid_placement(&piece, &placement, &Cell::Player1),
            Ok(())
        );

        let placement = Point::new(1, -1);
        println!("Placement: {:?}", placement);
        assert_eq!(
            plateau.is_valid_placement(&piece, &placement, &Cell::Player1),
            Ok(())
        );
    }

    #[test]
    fn good_placement_wrap() {
        let player_1_start = Point::new(1, 1);
        let plateau = Plateau::new(3, 3, &player_1_start, &Point::new(2, 2)).unwrap();
        let piece = piece_square();

        print!("{}", plateau);
        println!("{}", piece);

        let placement = Point::new(0, 0);
        println!("placement: {:?}", placement);
        assert_eq!(
            plateau.is_valid_placement(&piece, &placement, &Cell::Player2),
            Ok(())
        );
    }
}
