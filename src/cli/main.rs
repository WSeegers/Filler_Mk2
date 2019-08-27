extern crate chrono;
extern crate clap;
extern crate fillercore;

use engine::Engine;
use fillercore::engine;
use std::path;

use path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

const PLAYER_ARG: &'static str = "player";
fn player_arg<'a>() -> clap::Arg<'a, 'a> {
    clap::Arg::with_name(PLAYER_ARG)
        .long("player")
        .short("p")
        .takes_value(true)
        .multiple(true)
        .value_name("PLAYER_PATH")
        .required(true)
        .max_values(2)
        .validator(validate_player_path)
}

fn validate_json_dir(path: String) -> Result<(), String> {
    let path = path::Path::new(&path);

    match path.exists() {
        false => Err(String::from("Path invalid: Path not found")),
        true if !path.is_dir() => Err(String::from("Path invalid: Path is not a directory")),
        true => Ok(()),
    }
}

const JSON_ARG: &'static str = "json";
fn json_arg<'a>() -> clap::Arg<'a, 'a> {
    clap::Arg::with_name(JSON_ARG)
        .long("json")
        .takes_value(true)
        .value_name("OUTPUT_DIR")
        .min_values(0)
        .max_values(1)
        .validator(validate_json_dir)
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Filler_mk2")
        .version("0.1.0")
        .author("Random Guys")
        .about("About info")
        .arg(player_arg())
        .arg(json_arg())
        .get_matches()
}

const CLAP_PLAYER_ERROR: &'static str = "Clap failed at handling of players";

fn main() {
    let args = get_matches();

    let mut players = args.values_of(PLAYER_ARG).expect(CLAP_PLAYER_ERROR);

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

    if args.is_present(JSON_ARG) {
        let dir = args.value_of(JSON_ARG).unwrap_or("./");
        write_replay(dir, &filler);
    }
}

use chrono::prelude::*;

fn write_replay(file_dir: &str, filler_engine: &Engine) {
    let player_names = filler_engine.player_names();
    let mut filename = format!("{}_", Utc::now().timestamp());

    filename = filename + player_names[0].split('.').next().unwrap();
    if let Some(player_name) = player_names.get(1) {
        filename = filename + "_vs_";
        filename = filename + player_name.split('.').next().unwrap();
    }
    filename = filename + ".json";

    let path = Path::new(file_dir).join(filename);
    let mut file = match File::create(&path) {
        //TODO: Better error handling
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    write!(file, "{}", filler_engine.replay()).expect("Failed to write file");
}
