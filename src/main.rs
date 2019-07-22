mod models;
use models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

fn main() {
	let player1_start = Point { x: 2, y: 2 };
	let player2_start = Point { x: 15, y: 15 };

	let mut p = match Plateau::new(30, 30, &player1_start, &player2_start) {
		Ok(plat) => plat,
		Err(msg) => panic!(msg),
	};

	let pb = PieceBag::new([10, 11], [10, 11]);

	let piece_1 = pb.next();
	let piece_2 = pb.next();

	match p.place_piece(&piece_1, &Point { x: 0, y: 0 }, Player::Player1) {
		Err(msg) => println!("Player1: {}", msg),
		Ok(_) => (),
	}

	match p.place_piece(&piece_2, &Point { x: 10, y: 10 }, Player::Player2) {
		Err(msg) => println!("Player2: {}", msg),
		Ok(_) => (),
	}

	print!("{}", p);
	print!("{}", piece_1);
	print!("{}", piece_2);
}
