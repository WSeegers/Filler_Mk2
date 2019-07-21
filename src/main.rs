mod piece;
mod plateau;

use plateau::{Cell, Plateau};

fn main() {
    let mut p = Plateau::new(30, 14);

    p.set(0, 0, Cell::Player1);
    p.set(5, 5, Cell::Player2);

    print!("{}", p);

    let pb = piece::PieceBag::new([10, 30], [10, 30]);

    print!("{}", pb.next());
}
