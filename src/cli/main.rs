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

mod arguments;
use arguments::Arguments;

fn main() {
    let args = Arguments::new();

    let (player1, player2) = args.player_paths();

    let mut builder = Engine::builder(player1);
    if let Some(player2_) = player2 {
        builder.with_player2(player2_);
    }

    if args.verbose() {
        builder.verbose();
    }

    let mut filler = builder.finish();

    filler.run();

    if let Some(json_dir) = args.json_path() {
        write_replay(json_dir, &filler);
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
