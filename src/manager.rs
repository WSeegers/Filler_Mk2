use super::models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

use super::player_com::PlayerCom;

const P1_EXEC: &str = "$$$ exec p1 : [gsteyn]\n"; // the player name should come from whatever reads the filenames / the cmdline args.
const P2_EXEC: &str = "$$$ exec p2 : [gsteyn]\n";

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

        let msg = match self.p1_move_count {
            0 => format!("{}{}{}", P1_EXEC, self.plateau, piece), // Consider implementing the 'exec' part inside player_com
            _ => format!("{}{}", self.plateau, piece),
        };

        self.player_com.p1_send(msg);
        let response = match self.player_com.p1_receive() {
            Ok(s) => s,
            Err(_) => {
                println!("Player1 timed out");
                self.set_winner(Winner::Player2);
                return;
            },
        };

        // println!("Response: {}", response);
        let placement = Manager::coordinates_from_string(response);
        // println!("Placement: {:?}", placement);

        match self.plateau.place_piece(&piece, &placement, Player::Player1) {
            Err(msg) => {
                println!("Player1: {}", msg);
                self.set_winner(Winner::Player2);
            }
            Ok(_) => (),
        }

        // println!("Aftermath: {}{}", self.plateau, piece);
        self.p1_move_count += 1;
    }

    pub fn p2_move(&mut self) {                         // Need to return Result
        let piece = self.piece_bag.next();

        let msg = match self.p2_move_count {
            0 => format!("{}{}{}", P2_EXEC, self.plateau, piece), // Consider implementing the 'exec' part inside player_com
            _ => format!("{}{}", self.plateau, piece),
        };

        self.player_com.p2_send(msg);
        let response = match self.player_com.p2_receive() {
            Ok(s) => s,
            Err(_) => {
                println!("Player2 timed out");
                self.set_winner(Winner::Player1);
                return;
            },
        };

        let placement = Manager::coordinates_from_string(response); // Proper error handling please

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
    fn coordinates_from_string(input: String) -> Point {
        let coordinates = input.trim().split(" ");
        let vec = coordinates.collect::<Vec<&str>>();

        let cx = vec.get(0).unwrap();                   // Errors need to be handled and propagated
        let cy = vec.get(1).unwrap();                   // Errors need to be handled and propagated
        let y = cx.parse::<u32>().expect("cx couldn't be parsed");
        let x = cy.parse::<u32>().expect("cy couldn't be parsed");

        Point {x, y}
    }
}
