extern crate clap;
extern crate fillercore;

use std::path;

use fillercore::engine;

use engine::Engine;

fn validate_path(path: String) -> Result<(), String> {
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
        .validator(validate_path)
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Filler_mk2")
        .version("0.1.0")
        .author("Random Guys")
        .about("About info")
        .arg(player_arg())
        .get_matches()
}

const CLAP_PLAYER_ERROR: &'static str = "Clap failed at handling of players";

fn main() {
    let args = get_matches();

    let mut players = args.values_of("player").expect(CLAP_PLAYER_ERROR);

    let player1_path = players.next().expect(CLAP_PLAYER_ERROR);

    let mut builder = Engine::builder(player1_path);
    if let Some(player2_path) = players.next() {
        builder.with_player2(player2_path);
    }

    let mut filler = builder.finish();

    filler.run();

    let placements = filler.placement_counts();
    println!("Final Score: ");
    for (player, count) in placements {
        println!("<{}> -> {}", player, count);
    }
}
