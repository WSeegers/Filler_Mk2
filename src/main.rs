mod models;
use models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

<<<<<<< HEAD
mod player_com;
use player_com::PlayerCom;

mod manager;
use manager::{Manager, Winner};

fn main() {
    let player1_start = Point { x: 10, y: 10 };
    let player2_start = Point { x: 90, y: 90 };

    let plat = match Plateau::new(200, 100, &player1_start, &player2_start) {
        Ok(plat) => plat,
        Err(msg) => panic!(msg),
    };

    let p_bag = PieceBag::new([10, 15], [10, 15]);

    let p_com = PlayerCom::new(
            String::from("./resources/players/wseegers.filler"),
            String::from("./resources/players/wseegers.filler"),
            2
    );

    let mut steve = Manager::new(plat, p_bag, p_com);

    while steve.get_winner() == &Winner::None {
        steve.p1_move();
        steve.p2_move();
    }

    match steve.get_winner() {
        Winner::Player1 => println!("Player1 has won"),
        Winner::Player2 => println!("Player2 has won"),
        _ => (),
    }
=======
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
>>>>>>> origin
}
