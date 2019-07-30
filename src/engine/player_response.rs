use crate::models::{Piece, Player};

pub struct PlayerResponse {
	pub player: Player,
	pub piece: Piece,
	pub raw_response: String,
	pub placement_count: u32,
}
