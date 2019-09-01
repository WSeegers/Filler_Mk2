extern crate chrono;
extern crate clap;
extern crate fillercore;

mod arguments;
use arguments::Arguments;

use engine::{Engine, GameManager};
use fillercore::engine;
use std::path;

use path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args = Arguments::new();

    let players = args.player_paths();

    if players.len() <= 2 {
        let mut builder = Engine::builder(&players[0]);
        if let Some(player2) = players.get(1) {
            builder.with_player2(player2);
        }

        if args.verbose() {
            builder.verbose();
        }

        let mut filler = builder.finish();

        filler.run();
        if let Some(json_dir) = args.json_path() {
            write_replay(json_dir, &filler);
        }
    } else {
        GameManager::new(players).run();
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

    let path = Path::new(file_dir).join(filename).with_extension("json");
    let mut file = match File::create(&path) {
        //TODO: Better error handling
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    write!(file, "{}", filler_engine.replay()).expect("Failed to write file");
}
