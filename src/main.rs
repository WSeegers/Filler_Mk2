mod models;
use models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

mod player_com;
use player_com::PlayerCom;

mod manager;
use manager::{Manager, Winner};

fn main() {
	let player1_start = Point { x: 2, y: 2 };
	let player2_start = Point { x: 15, y: 15 };

	let plat = match Plateau::new(30, 30, &player1_start, &player2_start) {
		Ok(plat) => plat,
		Err(msg) => panic!(msg),
	};

	let p_bag = PieceBag::new([3, 4], [3, 4]);

    let p_com = PlayerCom::new(
            String::from("./resources/players/carli.filler"),
            String::from("./resources/players/carli.filler"),
            2);

    let mut steve = Manager::new(plat, p_bag, p_com);

    while steve.get_winner() == &Winner::None {
        steve.p1_move();
        // steve.p2_move();
    }

    match steve.get_winner() {
        Winner::Player1 => println!("Player1 has won"),
        Winner::Player2 => println!("Player2 has won"),
        _ => (),
    }
}
