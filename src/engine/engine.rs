use super::{PlayerCom, PlayerError, PlayerResponse};
use crate::models::{PieceBag, Plateau, Player};

pub struct Engine {
	players: Vec<PlayerCom>,
	plateau: Plateau,
	piece_bag: PieceBag,
	move_count: u32,
	player_count: u32,
}

impl Engine {
	pub fn new(
		plateau: Plateau,
		piece_bag: PieceBag,
		player1_path: String,
		player2_path: Option<String>,
		time_out: u64,
	) -> Result<Self, String> {
		let mut players = vec![PlayerCom::new(player1_path, time_out, Player::Player1)?];
		match player2_path {
			None => (),
			Some(path) => players.push(PlayerCom::new(path, time_out, Player::Player2)?),
		};

		Ok(Engine {
			player_count: players.len() as u32,
			players,
			plateau,
			piece_bag,
			move_count: 0,
		})
	}

	pub fn next_move(&mut self) -> Result<PlayerResponse, PlayerError> {
		let player_com = &mut self.players[(self.move_count % self.player_count) as usize];
		self.move_count += 1;

		let piece = self.piece_bag.next();
		player_com.request_placement(&mut self.plateau, &piece)
	}

	pub fn get_plateau(&self) -> &Plateau {
		&self.plateau
	}

	pub fn placement_counts(&self) -> Vec<(Player, u32)> {
		self.players
			.iter()
			.map(|player_com| (player_com.player, player_com.placement_count()))
			.collect()
	}
}
