use super::{PlayerCom, PlayerError, PlayerResponse};
use crate::models::{PieceBag, Plateau, Player};

use std::path::Path;

/// Number of errors that may occure in a row before game ends
const ERROR_THRESHOLD: usize = 6;

const DEFAULT_TIMEOUT: usize = 2;

pub struct Engine {
    players: Vec<PlayerCom>,
    plateau: Plateau,
    piece_bag: PieceBag,
    move_count: usize,
    player_count: usize,
}

pub struct EngineBuilder<'a> {
    players: Vec<&'a str>,
    plateau: Option<Plateau>,
    piece_bag: Option<PieceBag>,
}

impl<'a> EngineBuilder<'a> {
    pub fn with_player2(&mut self, player_path: &'a str) -> &Self {
        self.players.push(player_path);
        self
    }

    pub fn with_plateau(&mut self, plateau: Plateau) -> &Self {
        self.plateau = Some(plateau);
        self
    }

    pub fn with_piecebag(&mut self, piece_bag: PieceBag) -> &Self {
        self.piece_bag = Some(piece_bag);
        self
    }

    pub fn finish(&mut self) -> Engine {
        let mut players = vec![PlayerCom::new(
            String::from(self.players[0]),
            DEFAULT_TIMEOUT as u64,
            Player::Player1,
        )
        .unwrap()];

        if let Some(player_path) = self.players.get(1) {
            let player2 = PlayerCom::new(
                String::from(*player_path),
                DEFAULT_TIMEOUT as u64,
                Player::Player2,
            )
            .unwrap();
            players.push(player2);
        }

        let plateau = match self.plateau.take() {
            Some(set_plateau) => set_plateau,
            None => Plateau::default(),
        };

        let piece_bag = match self.piece_bag.take() {
            Some(set_piece_bag) => set_piece_bag,
            None => PieceBag::default(),
        };

        Engine {
            player_count: players.len(),
            players,
            plateau,
            piece_bag,
            move_count: 0,
        }
    }
}

impl Engine {
    pub fn builder<'a>(player_path: &'a str) -> EngineBuilder {
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
            player_count: players.len(),
            players,
            plateau,
            piece_bag,
            move_count: 0,
        })
    }

    pub fn run(&mut self) {
        let mut errors: usize = 0;
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
        let player_com = &mut self.players[self.move_count % self.player_count];
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
