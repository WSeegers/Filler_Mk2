use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::PlayerResponse;
use crate::models::{Piece, Plateau, Player, Point};

pub type ComError = String;

pub struct PlayerCom {
    player: Player,
    placement_count: usize,
    sender: Sender<std::string::String>,
    receiver: Receiver<std::string::String>,
    timeout: usize,
}

impl PlayerCom {
    pub fn new(path: &str, timeout: usize, player: Player) -> Result<PlayerCom, ComError> {
        let (sender, receiver) = PlayerCom::spawn_player(path, player)?;
        Ok(PlayerCom {
            player,
            sender,
            receiver,
            timeout,
            placement_count: 0,
        })
    }

    pub fn request_placement(&mut self, plateau: &mut Plateau, piece: &Piece) -> PlayerResponse {
        let mut player_response = PlayerResponse {
            player: self.player,
            piece: piece.clone(),
            raw_response: None,
            placement_count: self.placement_count,
            error: None,
        };

        let msg = format!("{}{}", plateau, piece);
        if let Err(error_message) = self.send(msg) {
            player_response.error = Some(error_message);
            return player_response;
        }

        let raw_response = self.receive();
        if let Err(error_message) = raw_response {
            player_response.error = Some(error_message);
            return player_response;
        }
        let raw_response = raw_response.unwrap();

        let placement = Point::try_from(&raw_response);
        player_response.raw_response = Some(raw_response);
        if let Err(error_message) = placement {
            player_response.error = Some(error_message);
            return player_response;
        }

        if let Err(error_message) = plateau.place_piece(piece, &placement.unwrap(), self.player) {
            player_response.error = Some(error_message);
            return player_response;
        }

        self.placement_count += 1;
        player_response.placement_count = self.placement_count;

        player_response
    }

    fn send(&self, message: String) -> Result<(), String> {
        match self.sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error while sending message")),
        }
    }

    fn receive(&self) -> Result<String, String> {
        let s = self
            .receiver
            .recv_timeout(Duration::from_secs(self.timeout as u64));
        match s {
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Timed out")),
        }
    }

    fn spawn_player(
        path: &str,
        player_num: Player,
    ) -> Result<(Sender<std::string::String>, Receiver<std::string::String>), ComError> {
        let (sender, receiver_internal) = mpsc::channel();
        let (sender_internal, receiver) = mpsc::channel();

        let path = String::from(path);
        thread::spawn(move || {
            let mut child_process = Command::new(&path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("Could not initialize player: {}", path));

            let child_in = child_process
                .stdin
                .as_mut()
                .unwrap_or_else(|| panic!("Could not retrieve stdin for: {}", path));

            let mut child_out = BufReader::new(
                child_process
                    .stdout
                    .as_mut()
                    .unwrap_or_else(|| panic!("Could not retrieve stdout for: {}", path)),
            );

            match player_num {
                Player::Player1 => write!(child_in, "$$$ exec p1 : {}\n", path)
                    .expect("Error initializing player 2"),
                Player::Player2 => write!(child_in, "$$$ exec p2 : {}\n", path)
                    .expect("Error initializing player 2"),
            };

            loop {
                let receive: String = receiver_internal
                    .recv()
                    .unwrap_or_else(|_| String::from(""));
                child_in
                    .write(receive.as_bytes())
                    .unwrap_or_else(|_| panic!("Error writing to: {}", path));
                let mut line = String::new();
                child_out
                    .read_line(&mut line)
                    .unwrap_or_else(|_| panic!("Error while reading line from: {}", path));
                sender_internal
                    .send(line)
                    .unwrap_or_else(|_| panic!("Error while sending message from: {}", path));
            }
        });

        Ok((sender, receiver))
    }

    pub fn placement_count(&self) -> usize {
        self.placement_count
    }

    pub fn player(&self) -> Player {
        self.player
    }
}
