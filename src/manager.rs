use super::models::*;

use piece::PieceBag;
use plateau::{Plateau, Player};
use point::Point;

use super::player_com::PlayerCom;

const P1_EXEC: &str = "$$$ exec p1 : [gsteyn]\n"; // the player name should come from whatever reads the filenames / the cmdline args.
const P2_EXEC: &str = "$$$ exec p2 : [gsteyn]\n";

pub enum Winner {
    Player1,
    Player2,
    None,
}

pub struct Manager {
    plateau: Plateau,
    piece_bag: PieceBag,
    player_com: PlayerCom,
    pub winner: Winner,
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
        // println!("{}{}", piece, self.plateau);

        let msg = match self.p1_move_count {
            0 => format!("{}{}{}", P1_EXEC, self.plateau, piece),
            _ => format!("{}{}", self.plateau, piece),
        };

        self.player_com.p1_send(msg);
        let response = match self.player_com.p1_receive() {
            Ok(s) => s,
            Err(_) => panic!("Player1 error response"), // Replace with return of Result value
        };

        // println!("Response: {}", response);
        let placement = Manager::coordinates_from_string(response);
        // println!("Placement: {:?}", placement);

        match self.plateau.place_piece(&piece, &placement, Player::Player1) {
            Err(msg) => println!("Player1: {}", msg),
            Ok(_) => (),
        }

        self.p1_move_count += 1;
    }

    pub fn p2_move(&mut self) {                         // Need to return Result
        let piece = self.piece_bag.next();

        let msg = match self.p1_move_count {
            0 => format!("{}{}{}", P2_EXEC, self.plateau, piece),
            _ => format!("{}{}", self.plateau, piece),
        };

        self.player_com.p2_send(msg);
        let response = match self.player_com.p2_receive() {
            Ok(s) => s,
            Err(_) => panic!("Player2 error response"), // Replace with return of Result value
        };

        let placement = Manager::coordinates_from_string(response);

        match self.plateau.place_piece(&piece, &placement, Player::Player2) {
            Err(msg) => println!("Player2: {}", msg),
            Ok(_) => (),
        }

        self.p2_move_count += 1;
    }

    fn coordinates_from_string(input: String) -> Point {
        let coordinates = input.trim().split(" ");
        let vec = coordinates.collect::<Vec<&str>>();

        let cx = vec.get(0).unwrap();                   // Errors need to be handled and propagated
        let cy = vec.get(1).unwrap();                   // Errors need to be handled and propagated
        let x = cx.parse::<u32>().expect("cx couldn't be parsed");
        let y = cy.parse::<u32>().expect("cy couldn't be parsed");

        Point {x, y}
    }
}
