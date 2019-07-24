use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use std::fmt;

use super::plateau::Player;

pub struct PlayerError {
    player: Player,
    msg: String,
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = match self.player {
            Player::Player1 => 1,
            Player::Player2 => 2
        };
        write!(f, "Player {} Error: {})", num, self.msg)
    }
}

pub struct ComError {
    msg: String,
}

impl fmt::Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct PlayerCom {
    player1_sender: Sender<std::string::String>,
    player1_receiver: Receiver<std::string::String>,
    player2_sender: Sender<std::string::String>,
    player2_receiver: Receiver<std::string::String>,
    timeout: u64,
}

impl PlayerCom {
    pub fn new(path1: String, path2: String, timeout: u64) -> Result<PlayerCom, ComError> {
        let (p1_sender, p1_receiver) = PlayerCom::spawn_child_process(path1, Player::Player1);
        let (p2_sender, p2_receiver) = PlayerCom::spawn_child_process(path2, Player::Player2);

        PlayerCom {
            player1_sender: p1_sender,
            player1_receiver: p1_receiver,
            player2_sender: p2_sender,
            player2_receiver: p2_receiver,
            timeout,
        }
    }

    pub fn p1_send(&self, message: String) -> Result<(), PlayerError> {
        match self.player1_sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(PlayerError {
                player: Player::Player1,
                msg: String::from("Error while sending message")
            })
        }
    }

    pub fn p2_send(&self, message: String) -> Result<(), PlayerError> {
        match self.player2_sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(PlayerError {
                player: Player::Player2,
                msg: String::from("Error while sending message")
            })
        }
    }

    pub fn p1_receive(&self) -> Result<String, PlayerError> {
        let s = self
            .player1_receiver
            .recv_timeout(Duration::from_secs(self.timeout));
        match s {
            Ok(s) => Ok(s),
            Err(_) => Err(PlayerError {
                player: Player::Player1,
                msg: String::from("Timed out")
            })
        }
    }

    pub fn p2_receive(&self) -> Result<String, PlayerError> {
        let s = self
            .player2_receiver
            .recv_timeout(Duration::from_secs(self.timeout));
        match s {
            Ok(s) => Ok(s),
            Err(_) => Err(PlayerError {
                player: Player::Player2,
                msg: String::from("Timed out")
            })
        }
    }
}

/* Helper functions */
impl PlayerCom {
    fn spawn_child_process(
        path: String,
        player_num: Player,
    ) -> Result<(Sender<std::string::String>, Receiver<std::string::String>), ComError> {
        let (sender, receiver_internal) = mpsc::channel();
        let (sender_internal, receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut child_process = match Command::new(&path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Ok(res) => res,
                Err(_) => return Err(ComError {
                    msg: String::from(format!("Could not initialize player: {}", path))
                })
            };

            let child_in = match child_process
                .stdin
                .as_mut()
            {
                Some(res) => res,
                None => return Err(ComError {
                    msg: String::from(format!("Could not retrieve stdin for: {}", path))
                })
            };

            let mut child_out = BufReader::new(
                match child_process
                    .stdout
                    .as_mut()
                {
                    Some(res) => res,
                    None => return Err(ComError {
                        msg: String::from(format!("Could not retrieve stdout for: {}", path))
                    })
                }
            );

            match player_num {
                Player::Player1 => {
                    match child_in
                        .write(format!("$$$ exec p1 : {}\n", path).as_bytes())
                    {
                        Ok(_) => (),
                        Err(_) => return Err(ComError {
                            msg: String::from(format!("Error initializing player 1"))
                        })
                    }
                }
                Player::Player2 => {
                    match child_in
                        .write(format!("$$$ exec p2 : {}\n", path).as_bytes())
                    {
                        Ok(_) => (),
                        Err(_) => return Err(ComError {
                            msg: String::from(format!("Error initializing player 2"))
                        })
                    }
                }
            }

            loop {
                let receive: String = receiver_internal
                    .recv()
                    .unwrap_or_else(|_| panic!("Error receiving message from: {}", path));
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

        (sender, receiver)
    }
}
