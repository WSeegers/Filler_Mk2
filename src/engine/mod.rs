mod player_com;
pub mod player_error;

use player_com::PlayerCom;
use player_error::PlayerError;
use std::convert::TryFrom;

use crate::models::{Piece, PieceBag, Plateau, Player, Point};

pub struct Engine {
	player1_com: PlayerCom,
	player2_com: Option<PlayerCom>,
	next_player: Player,
	plateau: Plateau,
	piece_bag: PieceBag,
	p1_move_count: u32,
	p2_move_count: u32,
	p1_last_move: Option<String>,
	p2_last_move: Option<String>,
	current_piece: Option<Piece>,
}

impl Engine {
	pub fn new(
		plateau: Plateau,
		piece_bag: PieceBag,
		player1_path: String,
		player2_path: Option<String>,
		time_out: u64,
	) -> Result<Self, String> {
		let player1_com = PlayerCom::new(player1_path, time_out, Player::Player1)?;
		let player2_com = match player2_path {
			None => None,
			Some(path) => Some(PlayerCom::new(path, time_out, Player::Player2)?),
		};
		Ok(Engine {
			player1_com,
			player2_com,
			next_player: Player::Player1,
			plateau,
			piece_bag,
			p1_move_count: 0,
			p2_move_count: 0,
			p1_last_move: None,
			p2_last_move: None,
			current_piece: None,
		})
	}

	pub fn next_move(&mut self) -> Result<Player, PlayerError> {
		let player_moved = match self.player2_com {
			None => self.p1_move()?,
			Some(_) => match self.next_player {
				Player::Player1 => {
					self.next_player = Player::Player2;
					self.p1_move()?
				}
				Player::Player2 => {
					self.next_player = Player::Player1;
					self.p2_move()?
				}
			},
		};
		Ok(player_moved)
	}

	fn p1_move(&mut self) -> Result<Player, PlayerError> {
		self.set_next_piece();
		let piece = self.current_piece.as_ref().unwrap();

		let msg = format!("{}{}", self.plateau, piece);

		self.player1_com.send(msg)?;
		let received = self.player1_com.receive()?;

		let placement = match Point::try_from(&received) {
			Ok(point) => point,
			Err(msg) => return Err(PlayerError::new(Player::Player1, msg)),
		};

		match self
			.plateau
			.place_piece(&piece, &placement, Player::Player1)
		{
			Ok(_) => (),
			Err(msg) => return Err(PlayerError::new(Player::Player1, msg)),
		}

		self.p1_last_move = Some(received);
		self.p1_move_count += 1;
		Ok(Player::Player1)
	}

	fn p2_move(&mut self) -> Result<Player, PlayerError> {
		let piece = self.current_piece.as_ref().unwrap();

		let msg = format!("{}{}", self.plateau, piece);

		self.player2_com.as_ref().unwrap().send(msg)?;
		let received = self.player2_com.as_ref().unwrap().receive()?;

		let placement = match Point::try_from(&received) {
			Ok(point) => point,
			Err(msg) => return Err(PlayerError::new(Player::Player2, msg)),
		};

		match self
			.plateau
			.place_piece(&piece, &placement, Player::Player2)
		{
			Ok(_) => (),
			Err(msg) => return Err(PlayerError::new(Player::Player2, msg)),
		}

		self.p2_last_move = Some(received);
		self.p2_move_count += 1;
		Ok(Player::Player2)
	}

	pub fn get_plateau(&self) -> &Plateau {
		&self.plateau
	}

	pub fn get_move_counts(&self) -> (u32, u32) {
		(self.p1_move_count, self.p2_move_count)
	}

	pub fn get_current_piece(&self) -> &Option<Piece> {
		&self.current_piece
	}

	pub fn get_p1_last_move(&self) -> &Option<String> {
		&self.p1_last_move
	}

	pub fn get_p2_last_move(&self) -> &Option<String> {
		&self.p2_last_move
	}

	fn set_next_piece(&mut self) {
		self.current_piece = Some(self.piece_bag.next());
	}
}
