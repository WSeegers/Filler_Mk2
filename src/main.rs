extern crate clap;

use std::path;

mod models;
use models::{PieceBag, Plateau, Point};

mod engine;

use engine::{
    player_com::PlayerCom,
    player_manager::{Manager, Winner},
};

fn validate_player_path(path: String) -> Result<(), String> {
    let path = path::Path::new(&path);

    match path.exists() {
        false => Err(String::from("Path invalid: Path not found")),
        true if path.is_dir() => Err(String::from("Path invalid: Path is directory")),
        true => match path.extension() {
            Some(ex) if ex == "filler" => Ok(()),
            _ => Err(String::from("File invalid: '.filler' file required")),
        },
    }
}

fn main() {
    let player_arg = clap::Arg::with_name("player")
        .short("p")
        .long("player")
        .takes_value(true)
        .multiple(true)
        .value_name("PLAYER_PATH")
        .validator(validate_player_path);

    let clap_args = [player_arg];

    let args = clap::App::new("Filler_mk2")
        .version("0.1.0")
        .author("Random Guys")
        .about("About info")
        .args(&clap_args)
        .get_matches();

    match args.occurrences_of("player") {
        0 => panic!("No Players"),
        1 => panic!("Require 2 players (Will be changed in future)"),
        2 => (), // 2 players are required for now
        // 1 | 2 => (),
        _ => panic!("Maximum of 2 players"),
    };

    let players: Vec<&str> = args
        .values_of("player")
        .expect("Error getting players") // This should never be reached
        .collect();

    println!("Players");
    for (i, &player) in players.iter().enumerate() {
        println!("{}: {}", i + 1, player);
    }

    let player1_start = Point { x: 4, y: 4 };
    let player2_start = Point { x: 44, y: 44 };

    let plat = match Plateau::new(50, 50, &player1_start, &player2_start) {
        Ok(plat) => plat,
        Err(msg) => panic!(msg),
    };

    let p_bag = PieceBag::new([5, 7], [5, 7]);

    let p_com = match PlayerCom::new(String::from(players[0]), String::from(players[1]), 2) {
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
