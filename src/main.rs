mod models;
use models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

mod player_com;
use player_com::PlayerCom;

mod manager;
use manager::{Manager, Winner};

use std::{thread, time};

fn main() {
    let player1_start = Point { x: 4, y: 4 };
    let player2_start = Point { x: 45, y: 45 };

    let plat = match Plateau::new(50, 50, &player1_start, &player2_start) {
        Ok(plat) => plat,
        Err(msg) => panic!(msg),
    };

    let p_bag = PieceBag::new([5, 7], [5, 7]);

    let p_com = PlayerCom::new(
        String::from("./resources/players/carli.filler"),
        String::from("./resources/players/carli.filler"),
        2,
    );

    let mut steve = Manager::new(plat, p_bag, p_com);

    loop {
        steve.p1_move(); // Need to check for a winner after each player's move
        if steve.get_winner() != &Winner::None {
            break;
        }
        steve.p2_move();
        if steve.get_winner() != &Winner::None {
            break;
        }
    }

    match steve.get_winner() {
        Winner::Player1 => println!("Player1 has won"),
        Winner::Player2 => println!("Player2 has won"),
        _ => (),
    }
}
