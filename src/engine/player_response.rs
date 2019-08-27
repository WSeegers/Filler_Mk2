extern crate serde;

use crate::models::{Piece, Player};
use serde::Serialize;

#[derive(Serialize)]
pub struct PlayerResponse {
    pub player: Player,
    pub piece: Piece,
    pub raw_response: Option<String>,
    pub placement_count: usize,
    pub error: Option<String>,
}
