use clap;
use std::path;

const PLAYER_ARG: &'static str = "player";
const JSON_ARG: &'static str = "json";
const VERBOSE_ARG: &'static str = "verbose";

const CLAP_PLAYER_ERROR: &'static str = "Clap failed at handling of players";

pub struct Arguments<'a> {
	matches: clap::ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
	pub fn new() -> Self {
		let matches = clap::App::new("Filler_mk2")
			.version("0.2.0")
			.author("https://github.com/WSeegers, https://github.com/GJSteyn")
			.about("A remake of 42's filler virtual machine")
			.arg(player_arg())
			.arg(json_arg())
			.arg(verbose_arg())
			.get_matches();

		Arguments { matches }
	}

	pub fn player_paths(&self) -> (&str, Option<&str>) {
		let mut players = self.matches.values_of(PLAYER_ARG).expect(CLAP_PLAYER_ERROR);
		let player1 = players.next().expect(CLAP_PLAYER_ERROR);
		let player2 = players.next();
		(player1, player2)
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
		.required(true)
		.max_values(2)
		.validator(validate_player_path)
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
		.help("displays each placement on the terminal")
}
