use super::{Cell, Plateau, EMPTY, PLAYER1, PLAYER2};
use std::convert::TryFrom;

impl TryFrom<String> for Plateau {
    type Error = String;

    fn try_from(map: String) -> Result<Self, String> {
        if map.is_empty() {
            return Err(String::from("Map is empty"));
        }
        let width = match map.find(|c| c == '\n') {
            Some(w) => w,
            None => map.len(),
        };

        let mut height = 0;
        let mut cells = Vec::new();
        for (y, row) in map.lines().enumerate() {
            if row.len() != width {
                return Err(String::from("Rows sizes are inconsistent"));
            }

            for (x, c) in row.chars().enumerate() {
                use Cell::*;
                let cell = match c {
                    PLAYER1 => Player1(false),
                    PLAYER2 => Player2(false),
                    EMPTY => Empty,
                    _ => return Err(format!("Unknown cell '{}' found at [{}, {}]", c, x, y)),
                };
                cells.push(cell);
            }
            height += 1;
        }

        let p = Plateau {
            width: width as u32,
            height,
            cells,
            last_piece: None,
        };

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plateau_from_string_1() {
        use Cell::*;

        let cell_map = vec![Player1(false), Empty, Empty, Player2(false)];
        let string_map = String::from("O.\n.X\n");

        let plat = Plateau::try_from(string_map).unwrap();
        assert_eq!(plat.width, 2, "Width incorrect");
        assert_eq!(plat.height, 2, "Height incorrect");
        assert_eq!(
            plat.cells.len(),
            cell_map.len(),
            "Contains incorrect number of cells"
        );

        assert_eq!(plat.cells, cell_map, "Cell map incorrect");
    }

    #[test]
    fn plateau_from_string_2() {
        use Cell::*;

        let cell_map = vec![
            Player1(false),
            Player2(false),
            Empty,
            Player2(false),
            Player1(false),
            Empty,
        ];
        let string_map = String::from("OX.\nXO.\n");

        let plat = Plateau::try_from(string_map).unwrap();
        assert_eq!(plat.width, 3, "Width incorrect");
        assert_eq!(plat.height, 2, "Height incorrect");
        assert_eq!(
            plat.cells.len(),
            cell_map.len(),
            "Contains incorrect number of cells"
        );

        assert_eq!(plat.cells, cell_map, "Cell map incorrect");
    }
}
