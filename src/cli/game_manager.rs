use super::engine::Engine;
use super::util;
use std::fmt::{self, Display};

pub struct GameManager<'a> {
	players: Vec<Player>,
	json_dir: Option<&'a str>,
}

impl<'a> GameManager<'a> {
	pub fn new(player_names: Vec<String>) -> Self {
		let players = player_names
			.iter()
			.map(|player_name| Player::new(player_name.clone()))
			.collect();

		GameManager {
			players,
			json_dir: None,
		}
	}

	pub fn with_replays(&mut self, json_dir: &'a str) -> &Self {
		self.json_dir = Some(json_dir);
		self
	}

	pub fn run(&mut self) {
		let total_players = self.players.len();

		// Iterators fight with the borrow checker =/
		for i in 0..total_players - 1 {
			for j in i + 1..total_players {
				let mut game_builder = Engine::builder(self.players[i].player_path());
				{
					let player2 = self.players[j].player_path();
					game_builder.with_player2(player2);
				}
				let mut game = game_builder.finish();
				let results = game.run();

				// Check for replay dir
				if let Some(json_dir) = self.json_dir {
					util::write_replay(json_dir, &game);
				}

				// This was born out of frustration
				// Data is badly organised to be honest
				if results[0].placements > results[1].placements {
					self.players
						.get_mut(i)
						.unwrap()
						.win(true, results[0].placements);
					self.players
						.get_mut(j)
						.unwrap()
						.win(false, results[1].placements);
				} else {
					self.players
						.get_mut(i)
						.unwrap()
						.win(false, results[0].placements);
					self.players
						.get_mut(j)
						.unwrap()
						.win(true, results[1].placements);
				}
			}
		}

		use std::cmp::Ordering;
		self.players.sort_by(|a, b| {
			let a = a.wins();
			let b = b.wins();
			if a == b {
				Ordering::Equal
			} else if a > b {
				Ordering::Less
			} else {
				Ordering::Greater
			}
		});
		for player in self.players.iter() {
			println!("{}", player);
		}
	}
}

pub struct Player {
	player_path: String,
	total_placements: usize,
	wins: usize,
	loses: usize,
}

impl Player {
	fn new(player_path: String) -> Self {
		Player {
			player_path,
			total_placements: 0,
			wins: 0,
			loses: 0,
		}
	}

	fn player_path(&self) -> &String {
		&self.player_path
	}

	fn win(&mut self, did_win: bool, placements: usize) {
		match did_win {
			true => self.wins += 1,
			false => self.loses += 1,
		}
		self.total_placements += placements;
	}

	fn wins(&self) -> usize {
		self.wins
	}
}

impl Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Player {
			player_path,
			total_placements,
			wins,
			loses,
		} = self;
		write!(
			f,
			"{} => Wins: {}, Loses: {}, Total Placements: {}",
			player_path, wins, loses, total_placements
		)
	}
}
