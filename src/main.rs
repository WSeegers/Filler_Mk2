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

    let p_com = match PlayerCom::new(
        String::from("./resources/players/gsteyn.filler"),
        String::from("./resources/players/gsteyn.filler"),
        2,
    ) {
        Ok(manager) => manager,
        Err(e) => panic!("{}", e),
    };

    let mut steve = Manager::new(plat, p_bag, p_com);

    loop {
        match steve.p1_move() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        print!("{}", steve.get_plateau());
        print!("{}", steve.get_current_piece().as_ref().unwrap());
        print!("<got (O): {}", steve.get_p1_last_move().as_ref().unwrap());
        match steve.p2_move() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        print!("{}", steve.get_plateau());
        print!("{}", steve.get_current_piece().as_ref().unwrap());
        println!("<got (X): {}", steve.get_p2_last_move().as_ref().unwrap());
    }

    let (p1_mc, p2_mc) = steve.get_move_counts();

    if p1_mc > p2_mc {
        println!("Player1 has won");
    } else if p2_mc > p1_mc {
        println!("Player2 has won");
    } else {
        println!("It was a draw!");
    }
}
