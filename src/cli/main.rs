extern crate clap;
extern crate fillercore;

use std::path;

use fillercore::{engine, models};
use models::{PieceBag, Plateau, Point};

use engine::Engine;

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

fn player_arg<'a>() -> clap::Arg<'a, 'a> {
    let player_arg: (&'a str, &'a str) = ("player", "p");
    clap::Arg::with_name("player")
        .long(player_arg.0)
        .short(player_arg.1)
        .takes_value(true)
        .multiple(true)
        .value_name("PLAYER_PATH")
        .required(true)
        .max_values(2)
        .validator(validate_player_path)
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Filler_mk2")
        .version("0.1.0")
        .author("Random Guys")
        .about("About info")
        .arg(player_arg())
        .get_matches()
}

fn main() {
    let args = get_matches();

    let players: Vec<&str> = args
        .values_of("player")
        .expect("Clap failed at handling of players")
        .collect();

    // Section needs to handling input of map -----
    let player1_start = Point { x: 4, y: 4 };
    let player2_start = Point { x: 44, y: 44 };

    let plat = match Plateau::new(50, 50, &player1_start, &player2_start) {
        Ok(plat) => plat,
        Err(msg) => panic!(msg),
    };
    // --------------------------------------------

    let p_bag = PieceBag::new([5, 7], [5, 7]);

    let player1 = String::from(players[0]);
    let player2 = match players.get(1) {
        Some(&player) => Some(String::from(player)),
        None => None,
    };

    let mut steve = match Engine::new(plat, p_bag, player1, player2, 2) {
        Err(e) => panic!(e),
        Ok(engin) => engin,
    };

    steve.run();

    let placements = steve.placement_counts();
    println!("Final Score: ");
    for (player, count) in placements {
        println!("<{}> -> {}", player, count);
    }
}
