use clap;
// use std::error::Error;
// use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::path::{self, PathBuf};

const APP_NAME: &'static str = "Filler_mk2";
const ABOUT: &'static str = "A remake of 42's filler virtual machine";

const PLAYER_ARG: &'static str = "player";
const JSON_ARG: &'static str = "json";
const VERBOSE_ARG: &'static str = "verbose";
const PLAYER_DIR_ARG: &'static str = "players_dir";

pub struct Arguments<'a> {
	matches: clap::ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
	pub fn new() -> Self {
		let matches = clap::App::new(APP_NAME)
			.version(clap::crate_version!())
			.author(clap::crate_authors!())
			.about(ABOUT)
			.arg(player_arg())
			.arg(json_arg())
			.arg(verbose_arg())
			.arg(players_dir_arg())
			.get_matches();

		Arguments { matches }
	}

	pub fn player_paths(&self) -> Vec<String> {
		let mut players = vec![];
		match self.matches.values_of(PLAYER_ARG) {
			Some(p) => {
				let p = p.map(|player| String::from(player));
				players.extend(p);
			}
			None => (),
		}

		match self.matches.values_of(PLAYER_DIR_ARG) {
			Some(p) => {
				for dir in p {
					let path = PathBuf::from(dir);
					let files = fs::read_dir(path).unwrap();
					let player_paths = files
						.filter(|player_path| {
							if let Ok(dir_entry) = player_path {
								let path = dir_entry.path();
								match validate_player_path(String::from(path.to_string_lossy())) {
									Ok(_) => true,
									Err(_) => false,
								}
							} else {
								false
							}
						})
						.map(|player_path| {
							String::from(player_path.unwrap().path().to_string_lossy())
						});
					players.extend(player_paths);
				}
			}
			None => (),
		}

		if players.len() < 1 {
			clap::Error::with_description("Not enough players", clap::ErrorKind::TooFewValues)
				.exit();
		}

		players
	}

	pub fn json_path(&self) -> Option<&str> {
		match self.matches.is_present(JSON_ARG) {
			true => Some(self.matches.value_of(JSON_ARG).unwrap_or("./")),
			false => None,
		}
	}

	pub fn verbose(&self) -> bool {
		self.matches.is_present(VERBOSE_ARG)
	}
}

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

fn validate_player_dir(path: String) -> Result<(), String> {
	let path = path::Path::new(&path);

	match path.exists() {
		false => Err(String::from("Path invalid: Path not found")),
		true if !path.is_dir() => Err(String::from("Path invalid: Path is not directory")),
		true => Ok(()),
	}
}

fn validate_json_dir(path: String) -> Result<(), String> {
	let path = path::Path::new(&path);

	match path.exists() {
		false => Err(String::from("Path invalid: Path not found")),
		true if !path.is_dir() => Err(String::from("Path invalid: Path is not a directory")),
		true => Ok(()),
	}
}

fn player_arg<'a>() -> clap::Arg<'a, 'a> {
	clap::Arg::with_name(PLAYER_ARG)
		.long("player")
		.short("p")
		.takes_value(true)
		.multiple(true)
		.value_name("PLAYER_PATH")
		.validator(validate_player_path)
}

fn players_dir_arg<'a>() -> clap::Arg<'a, 'a> {
	clap::Arg::with_name(PLAYER_DIR_ARG)
		.long("players")
		.takes_value(true)
		.multiple(true)
		.value_name("PLAYERS_PATH_DIR")
		.validator(validate_player_dir)
		.help("a directory where .filler files will be searched for")
}

fn json_arg<'a>() -> clap::Arg<'a, 'a> {
	clap::Arg::with_name(JSON_ARG)
		.long("json")
		.takes_value(true)
		.value_name("OUTPUT_DIR")
		.min_values(0)
		.max_values(1)
		.validator(validate_json_dir)
		.help("folder where the a replay of the game will be saved in json")
}

fn verbose_arg<'a>() -> clap::Arg<'a, 'a> {
	clap::Arg::with_name(VERBOSE_ARG)
		.long(VERBOSE_ARG)
		.help("displays each placement on the terminal, ignored with more than 2 players")
}
