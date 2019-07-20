mod plateau;

use plateau::{Cell, Plateau};

fn main() {
    let mut p = Plateau::new(10, 10);

    p.set(0, 0, Cell::Player1);
    p.set(0, 1, Cell::Player2);

    println!("{:?}", p);
}
