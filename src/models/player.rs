use super::constants::*;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Serialize)]
pub enum Player {
    Player1,
    Player2,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Player::{Player1, Player2};
        let c = match self {
            Player1 => PLAYER1,
            Player2 => PLAYER2,
        };
        write!(f, "{}", c)
    }
}
