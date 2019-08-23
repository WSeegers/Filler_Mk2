use super::PlayerResponse;
use crate::models::Point;

use serde_json::json;

use std::fmt;

pub struct Replay {
    move_count: u32,
    map: serde_json::Value,
    player_1: Point,
    player_2: Point,
    moves: Vec<serde_json::Value>
}

impl Replay {
    pub fn new(player_1: &Point, player_2: &Point, map_w: u32, map_h: u32) -> Replay {
        let map = json!({
            "w": map_w,
            "h": map_h
        });

        Replay {
            move_count: 0,
            map,
            player_1: player_1.clone(),
            player_2: player_2.clone(),
            moves: Vec::new(),
        }
    }

    pub fn next_move(&mut self, response: &PlayerResponse) {
        let entry = json!({
            "count": self.move_count,
            "action": response.as_json()
        });
        self.move_count += 1;

        self.moves.push(entry);
    }
}

impl fmt::Display for Replay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let all = json!({
            "move_count": self.move_count,
            "map": self.map,
            "p1": self.player_1,
            "p2": self.player_2,
            "moves": self.moves,
        });

        write!(f, "{}", all.to_string())?;
        Ok(())
    }
}
