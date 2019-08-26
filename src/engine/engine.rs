use super::{PlayerCom, PlayerError, PlayerResponse};
use crate::models::{PieceBag, Plateau, Player};

use std::path::Path;

/// Number of errors that may occure in a row before game ends
const ERROR_THRESHOLD: u8 = 6;

pub struct Engine {
    players: Vec<PlayerCom>,
    plateau: Plateau,
    piece_bag: PieceBag,
    move_count: u32,
    player_count: u32,
}

pub struct EngineBuilder<'a> {
    players: Vec<&'a Path>,
    plateau: Option<Plateau>,
    piece_bag: Option<PieceBag>,
}

impl<'a> EngineBuilder<'a> {
    fn with_player(mut self, player_path: &'a Path) -> Self {
        self.players.push(player_path);
        self
    }

    fn with_plateau(mut self, plateau: Plateau) -> Self {
        self.plateau = Some(plateau);
        self
    }

    fn with_piecebag(mut self, piece_bag: PieceBag) -> Self {
        self.piece_bag = Some(piece_bag);
        self
    }
}

impl Engine {
    pub fn builder<'a>(player_path: &'a Path) -> EngineBuilder {
        EngineBuilder {
            players: vec![player_path],
            plateau: None,
            piece_bag: None,
        }
    }

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

    pub fn run(&mut self) {
        let mut errors: u8 = 0;
        loop {
            match &self.next_move() {
                Ok(response) => {
                    print!("<got ({}): {}", response.player, response.raw_response);
                    print!("{}", response.piece);
                    print!("{}", self.plateau());
                    errors = 0;
                    ()
                }
                Err(e) => {
                    println!("{}", e);
                    errors += 1;
                }
            }
            match errors {
                e if e >= ERROR_THRESHOLD => break,
                _ => (),
            }
        }
    }

    pub fn next_move(&mut self) -> Result<PlayerResponse, PlayerError> {
        let player_com = &mut self.players[(self.move_count % self.player_count) as usize];
        self.move_count += 1;

        let piece = self.piece_bag.next();
        player_com.request_placement(&mut self.plateau, &piece)
    }

    pub fn plateau(&self) -> &Plateau {
        &self.plateau
    }

    pub fn placement_counts(&self) -> Vec<(Player, u32)> {
        self.players
            .iter()
            .map(|player_com| (player_com.player, player_com.placement_count()))
            .collect()
    }
}
