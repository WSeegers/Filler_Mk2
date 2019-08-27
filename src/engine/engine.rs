use super::{PlayerCom, PlayerResponse};
use crate::models::{PieceBag, Plateau, Player};
use serde_json::json;

use std::path::Path;

/// Number of errors that may occure in a row before game ends
const ERROR_THRESHOLD: usize = 6;

const DEFAULT_TIMEOUT: usize = 2;

pub struct Engine {
    player_names: Vec<String>,
    players: Vec<PlayerCom>,
    plateau: Plateau,
    piece_bag: PieceBag,
    move_count: usize,
    player_count: usize,
    history: Vec<PlayerResponse>,
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
        let mut players =
            vec![PlayerCom::new(self.players[0], DEFAULT_TIMEOUT, Player::Player1).unwrap()];

        if let Some(player_path) = self.players.get(1) {
            let player2 = PlayerCom::new(*player_path, DEFAULT_TIMEOUT, Player::Player2).unwrap();
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

        let player_names = self
            .players
            .iter()
            .map(|player_path| {
                Path::new(*player_path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_owned()
                    .to_string()
            })
            .collect();

        Engine {
            player_names,
            player_count: players.len(),
            players,
            plateau,
            piece_bag,
            move_count: 0,
            history: vec![],
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

    pub fn run(&mut self) {
        let mut errors: usize = 0;
        loop {
            let response = self.next_move();

            match &response.error {
                None => {
                    print!(
                        "<got ({}): {}",
                        &response.player,
                        &response.raw_response.as_ref().unwrap()
                    );
                    print!("{}", response.piece);
                    print!("{}", self.plateau());
                    errors = 0;
                    ()
                }
                Some(e) => {
                    println!("{}: {}", response.player, e);
                    errors += 1;
                }
            }
            match errors {
                e if e >= ERROR_THRESHOLD => break,
                _ => (),
            }
            self.history.push(response);
        }
    }

    pub fn next_move(&mut self) -> PlayerResponse {
        let player_com = &mut self.players[self.move_count % self.player_count];
        self.move_count += 1;

        let piece = self.piece_bag.next();
        player_com.request_placement(&mut self.plateau, &piece)
    }

    pub fn plateau(&self) -> &Plateau {
        &self.plateau
    }

    pub fn placement_counts(&self) -> Vec<(Player, usize)> {
        self.players
            .iter()
            .map(|player_com| (player_com.player(), player_com.placement_count()))
            .collect()
    }

    pub fn player_names(&self) -> &Vec<String> {
        &self.player_names
    }

    pub fn replay(&self) -> String {
        // let history = serde_json::to_string_pretty(&self.history).unwrap();
        json!({
        "players": self.player_names,
        "plateau": json!({
            "width": self.plateau.width(),
            "height": self.plateau.height(),
            "player1_start": self.plateau.player_start(Player::Player1),
            "player2_start": self.plateau.player_start(Player::Player2),
        }),
        "history": self.history
        })
        .to_string()
    }
}
