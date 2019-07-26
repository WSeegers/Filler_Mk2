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
use std::time::Duration;

mod rend;
use rend::App;

fn main() {
    let player1_start = Point { x: 4, y: 4 };
    let player2_start = Point { x: 195, y: 195 };

    let plat = match Plateau::new(200, 200, &player1_start, &player2_start) {
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

    let mut renderer = App::new(1000, 1000, 200, 200);

    loop {
        match steve.p1_move() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        // print!("{}", steve.get_plateau());
        // print!("{}", steve.get_current_piece().as_ref().unwrap());
        // print!("<got (O): {}", steve.get_p1_last_move().as_ref().unwrap());
        match steve.p2_move() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        // print!("{}", steve.get_plateau());
        // print!("{}", steve.get_current_piece().as_ref().unwrap());
        // println!("<got (X): {}", steve.get_p2_last_move().as_ref().unwrap());
        renderer.main_loop(steve.get_plateau());
        // thread::sleep(Duration::from_millis(10));
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
