use super::models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

use super::player_com::PlayerCom;

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
        }
    }

    pub fn p1_move(&mut self) {                         // Need to return Result
        let piece = self.piece_bag.next();

        let msg = format!("{}{}", self.plateau, piece);

        self.player_com.p1_send(msg);
        let response = match self.player_com.p1_receive() {
            Ok(s) => s,
            Err(_) => {
                println!("Player1 timed out");
                self.set_winner(Winner::Player2);
                return;
            },
        };

        let placement = match Manager::coordinates_from_string(response) {
            Ok(point) => point,
            Err(s) => {
                println!("Player1: {}", s);
                self.set_winner(Winner::Player2);
                return;
            },
        };

        match self.plateau.place_piece(&piece, &placement, Player::Player1) {
            Err(msg) => {
                println!("Player1: {}", msg);
                self.set_winner(Winner::Player2);
            }
            Ok(_) => (),
        }

        println!("Aftermath: {}{}", self.plateau, piece);
        self.p1_move_count += 1;
    }

    pub fn p2_move(&mut self) {                         // Need to return Result
        let piece = self.piece_bag.next();

        let msg = format!("{}{}", self.plateau, piece);

        self.player_com.p2_send(msg);
        let response = match self.player_com.p2_receive() {
            Ok(s) => s,
            Err(_) => {
                println!("Player2 timed out");
                self.set_winner(Winner::Player1);
                return;
            },
        };

        let placement = match Manager::coordinates_from_string(response) {
            Ok(point) => point,
            Err(s) => {
                println!("Player2: {}", s);
                self.set_winner(Winner::Player1);
                return;
            },
        };

        match self.plateau.place_piece(&piece, &placement, Player::Player2) {
            Err(msg) => println!("Player2: {}", msg),
            Ok(_) => (),
        }

        self.p2_move_count += 1;
    }

    pub fn get_winner(&self) -> &Winner {
        &self.winner
    }

    fn set_winner(&mut self, winner: Winner) {
        self.winner = winner;
    }
}

/* Helper functions */
impl Manager {
    fn coordinates_from_string(input: String) -> Result<Point, String> {
        let coordinates = input.trim().split(" ");
        let vec = coordinates.collect::<Vec<&str>>();

        let cy = match vec.get(0) {
            Some(s) => s,
            None => return Err(String::from("Bad input from player")),
        };
        let cx = match vec.get(1) {
            Some(s) => s,
            None => return Err(String::from("Bad input from player")),
        };
        let x = match cx.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(String::from("Invalid coordinate x from player")),
        };
        let y = match cy.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(String::from("Invalid coordinate y from player")),
        };

        Ok(Point {x, y})
    }
}
