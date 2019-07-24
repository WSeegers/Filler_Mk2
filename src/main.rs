extern crate clap;

use std::path;

fn validate_player_path(path: String) -> Result<(), String> {
    let path = path::Path::new(&path);

    println!("ex: {}", path.extension().unwrap().to_str().unwrap());

    match path.exists() {
        false => Err(String::from("Path invaild: File not found")),
        true if path.is_dir() => Err(String::from("Path invaild: Directory not found")),
        true => match path.extension() {
            Some(ex) if ex == "filler" => Ok(()),
            _ => Err(String::from("File invaild: '.filler' file required")),
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
        // This can customize Error messages
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
        1 | 2 => (),
        _ => panic!("Maximum of 2 players"),
    };

    let players: Vec<&str> = args
        .values_of("player")
        .expect("Error getting players")
        .collect();

    println!("Players");
    for (i, player) in players.iter().enumerate() {
        println!("{}: {}", i + 1, player);
    }
}
