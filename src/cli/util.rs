extern crate chrono;

use chrono::prelude::*;
use fillercore::engine::Engine;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn write_replay(file_dir: &str, filler_engine: &Engine) {
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
