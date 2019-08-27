extern crate serde;

use crate::models::{Piece, Player};

pub struct PlayerResponse {
    pub player: Player,
    pub piece: Piece,
    pub raw_response: Option<String>,
    pub placement_count: u32,
    pub error: Option<String>,
}
