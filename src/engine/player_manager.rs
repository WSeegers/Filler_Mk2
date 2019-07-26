use std::fmt;

use crate::models::{piece, plateau, point};

use piece::{Piece, PieceBag};
use plateau::{Plateau, Player};
use point::Point;

use super::player_com::{ComError, PlayerCom, PlayerError};

#[derive(PartialEq)]
pub enum Winner {
    Player1,
    Player2,
    None,
}

pub struct Manager {
    plateau: Plateau,
    piece_bag: PieceBag,
    player_com: PlayerCom,
    winner: Winner,
    p1_move_count: u32,
    p2_move_count: u32,
    p1_last_move: Option<String>,
    p2_last_move: Option<String>,
    current_piece: Option<Piece>,
}

impl Manager {
    pub fn new(plateau: Plateau, piece_bag: PieceBag, player_com: PlayerCom) -> Manager {
        Manager {
            plateau,
            piece_bag,
            player_com,
            winner: Winner::None,
            p1_move_count: 0,
            p2_move_count: 0,
            p1_last_move: None,
            p2_last_move: None,
            current_piece: None,
        }
    }

    pub fn p1_move(&mut self) -> Result<(), PlayerError> {
        self.set_next_piece();
        let piece = self.current_piece.as_ref().unwrap();

        let msg = format!("{}{}", self.plateau, piece);

        self.player_com.p1_send(msg)?;
        let received = self.player_com.p1_receive()?;

        let placement = match Manager::coordinates_from_string(&received) {
            Ok(point) => point,
            Err(msg) => return Err(PlayerError::new(Player::Player1, msg)),
        };

        match self
            .plateau
            .place_piece(&piece, &placement, Player::Player1)
        {
            Ok(_) => (),
            Err(msg) => return Err(PlayerError::new(Player::Player1, msg)),
        }

        self.p1_last_move = Some(received);
        self.p1_move_count += 1;
        Ok(())
    }

    pub fn p2_move(&mut self) -> Result<(), PlayerError> {
        self.set_next_piece();
        let piece = self.current_piece.as_ref().unwrap();

        let msg = format!("{}{}", self.plateau, piece);

        self.player_com.p2_send(msg)?;
        let received = self.player_com.p2_receive()?;

        let placement = match Manager::coordinates_from_string(&received) {
            Ok(point) => point,
            Err(msg) => return Err(PlayerError::new(Player::Player2, msg)),
        };

        match self
            .plateau
            .place_piece(&piece, &placement, Player::Player2)
        {
            Ok(_) => (),
            Err(msg) => return Err(PlayerError::new(Player::Player2, msg)),
        }

        self.p2_last_move = Some(received);
        self.p2_move_count += 1;
        Ok(())
    }

    pub fn get_plateau(&self) -> &Plateau {
        &self.plateau
    }

    pub fn get_winner(&self) -> &Winner {
        &self.winner
    }

    pub fn set_winner(&mut self, winner: Winner) {
        self.winner = winner;
    }

    pub fn get_move_counts(&self) -> (u32, u32) {
        (self.p1_move_count, self.p2_move_count)
    }

    pub fn get_current_piece(&self) -> &Option<Piece> {
        &self.current_piece
    }

    pub fn get_p1_last_move(&self) -> &Option<String> {
        &self.p1_last_move
    }

    pub fn get_p2_last_move(&self) -> &Option<String> {
        &self.p2_last_move
    }
}

/* Helper functions */
impl Manager {
    fn coordinates_from_string(input: &String) -> Result<Point, String> {
        let coordinates = input.trim().split(" ");
        let vec = coordinates.collect::<Vec<&str>>();

        let cy = match vec.get(0) {
            Some(s) => s,
            None => return Err(format!("Bad input: {}", input)),
        };
        let cx = match vec.get(1) {
            Some(s) => s,
            None => return Err(format!("Bad input: {}", input)),
        };
        let x = match cx.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(format!("Invalid x coordinate: {}", input)),
        };
        let y = match cy.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(format!("Invalid y coordinate: {}", input)),
        };

        Ok(Point { x, y })
    }

    fn set_next_piece(&mut self) {
        self.current_piece = Some(self.piece_bag.next());
    }
}
