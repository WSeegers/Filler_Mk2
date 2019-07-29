use crate::models::Player;
use std::fmt;

#[derive(Debug)]
pub struct PlayerError {
    player: Player,
    msg: String,
}

impl PlayerError {
    pub fn new(player: Player, msg: String) -> Self {
        Self { player, msg }
    }
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = match self.player {
            Player::Player1 => 1,
            Player::Player2 => 2,
        };
        write!(f, "Player {} - Error: {}", num, self.msg)
    }
}
