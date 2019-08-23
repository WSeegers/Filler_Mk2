use crate::models::{Piece, Player, Point};

use std::convert::TryFrom;
use serde_json::{json, Value};

pub struct PlayerResponse {
    pub player: Player,
    pub piece: Piece,
    pub raw_response: String,
    pub placement_count: u32,
}

impl PlayerResponse {
    pub fn as_json(&self) -> Value {
        let point = match Point::try_from(&self.raw_response) {
            Ok(point) => point,
            Err(_) => panic!("Nope on playerResponse JSON")
        };

        let player = match self.player {
            Player::Player1 => 1,
            Player::Player2 => 2
        };

        json!({
            "player": player,
            "piece": self.piece.as_json(),
            "place": point,
        })
    }
}
