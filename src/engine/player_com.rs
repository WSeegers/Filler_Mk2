use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::player_error::PlayerError;
use crate::models::Player;

pub type ComError = String;

pub struct PlayerCom {
    sender: Sender<std::string::String>,
    receiver: Receiver<std::string::String>,
    timeout: u64,
}

impl PlayerCom {
    pub fn new(path1: String, timeout: u64, player: Player) -> Result<PlayerCom, ComError> {
        let (sender, receiver) = PlayerCom::spawn_player(path1, player)?;

        Ok(PlayerCom {
            sender,
            receiver,
            timeout,
        })
    }

    pub fn send(&self, message: String) -> Result<(), PlayerError> {
        match self.sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(PlayerError::new(
                Player::Player1,
                String::from("Error while sending message"),
            )),
        }
    }

    pub fn receive(&self) -> Result<String, PlayerError> {
        let s = self
            .receiver
            .recv_timeout(Duration::from_secs(self.timeout));
        match s {
            Ok(s) => Ok(s),
            Err(_) => Err(PlayerError::new(Player::Player1, String::from("Timed out"))),
        }
    }

    fn spawn_player(
        path: String,
        player_num: Player,
    ) -> Result<(Sender<std::string::String>, Receiver<std::string::String>), ComError> {
        let (sender, receiver_internal) = mpsc::channel();
        let (sender_internal, receiver) = mpsc::channel();

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
                Player::Player1 => child_in
                    .write(format!("$$$ exec p1 : {}\n", path).as_bytes())
                    .unwrap_or_else(|_| panic!("Error initializing player 1")),
                Player::Player2 => child_in
                    .write(format!("$$$ exec p2 : {}\n", path).as_bytes())
                    .unwrap_or_else(|_| panic!("Error initializing player 2")),
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
}
