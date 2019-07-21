use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::{BufRead, Write, BufReader};

pub struct App {
    player1_sender: std::sync::mpsc::Sender<std::string::String>,
    player1_receiver: std::sync::mpsc::Receiver<std::string::String>,
    player2_sender: std::sync::mpsc::Sender<std::string::String>,
    player2_receiver: std::sync::mpsc::Receiver<std::string::String>,
    timeout: u64,
}

impl App {
    pub fn new(path1: String, path2: String, timeout: u64) -> App {
        let (p1_sender, p1_receiver_internal) = mpsc::channel();
        let (p1_sender_internal, p1_receiver) = mpsc::channel();
        let (p2_sender, p2_receiver_internal) = mpsc::channel();
        let (p2_sender_internal, p2_receiver) = mpsc::channel();

        let p1_thread = thread::spawn(move || {
            let mut child_process = Command::new(path1)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Could not initialize player1");

            let child_in = child_process.stdin
                .as_mut().
                expect("Could not retrieve player1 stdin");
            let mut child_out = BufReader::new(child_process.stdout
                .as_mut()
                .expect("Could not retrieve player1 stdout"));

            loop {
                let mut receive: String = p1_receiver_internal
                    .recv()
                    .expect("Something went wrong when player1 received message");
                child_in
                    .write(receive.as_bytes())
                    .expect("Something went wrong writing to player1");
                let mut line = String::new();
                child_out
                    .read_line(&mut line)
                    .expect("Something went wrong reading line from player1");
                p1_sender_internal
                    .send(line)
                    .expect("Something went wrong while sending message from player1");
            }
        });

        let p2_thread = thread::spawn(move || {
            let mut child_process = Command::new(path2)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Could not initialize player2");

            let child_in = child_process.stdin
                .as_mut()
                .expect("Could not retrieve player2 stdin");
            let mut child_out = BufReader::new(child_process.stdout
                .as_mut()
                .expect("Could not retrieve player2 stdout"));

            loop {
                let mut receive: String = p2_receiver_internal
                    .recv()
                    .expect("Something went wrong when player2 received message");
                child_in
                    .write(receive.as_bytes())
                    .expect("Something went wrong writing to player2");
                let mut line = String::new();
                child_out
                    .read_line(&mut line)
                    .expect("Something went wrong reading line from player2");
                p2_sender_internal
                    .send(line)
                    .expect("Something went wrong while sending message from player2");
            }
        });

        App {
            player1_sender: p1_sender,
            player1_receiver: p1_receiver,
            player2_sender: p2_sender,
            player2_receiver: p2_receiver,
            timeout
        }
    }

    pub fn p1_send(&self, message: String) {
        self.player1_sender
            .send(message)
            .expect("Something went wrong sending message to player1");
    }

    pub fn p2_send(&self, message: String) {
        self.player2_sender
            .send(message)
            .expect("Something went wrong sending message to player2");
    }

    pub fn p1_receive(&self) -> String {
        let s = self.player1_receiver
            .recv_timeout(Duration::from_secs(self.timeout))
            .expect("Something went wrong while receiving player1 message");
        s
    }

    pub fn p2_receive(&self) -> String {
        let s = self.player2_receiver
            .recv_timeout(Duration::from_secs(self.timeout))
            .expect("Something went wrong while receiving player 2 message");
        s
    }
}
