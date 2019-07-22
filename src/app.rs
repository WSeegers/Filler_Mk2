use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;
use std::process::{Command, Stdio};
use std::io::{BufRead, Write, BufReader};

pub struct App {
    player1_sender: Sender<std::string::String>,
    player1_receiver: Receiver<std::string::String>,
    player2_sender: Sender<std::string::String>,
    player2_receiver: Receiver<std::string::String>,
    timeout: u64,
}

impl App {
    pub fn new(path1: String, path2: String, timeout: u64) -> App {
        let (p1_sender, p1_receiver) = App::spawn_child_process(path1);
        let (p2_sender, p2_receiver) = App::spawn_child_process(path2);

        App {
            player1_sender: p1_sender,
            player1_receiver: p1_receiver,
            player2_sender: p2_sender,
            player2_receiver: p2_receiver,
            timeout,
        }
    }

    pub fn p1_send(&self, message: String) {
        self.player1_sender
            .send(message)
            .expect("Error while sending message to player1");
    }

    pub fn p2_send(&self, message: String) {
        self.player2_sender
            .send(message)
            .expect("Error while sending message to player2");
    }

    pub fn p1_receive(&self) -> Result<String, RecvTimeoutError> {
        let s = self.player1_receiver
            .recv_timeout(Duration::from_secs(self.timeout))?;
        Ok(s)
    }

    pub fn p2_receive(&self) -> Result<String, RecvTimeoutError> {
        let s = self.player2_receiver
            .recv_timeout(Duration::from_secs(self.timeout))?;
        Ok(s)
    }
}

/* Helper functions */
impl App {
    fn spawn_child_process(path: String) -> (Sender<std::string::String>, Receiver<std::string::String>) {
        let (sender, receiver_internal) = mpsc::channel();
        let (sender_internal, receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut child_process = Command::new(&path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("Could not initialize player: {}", path));

            let child_in = child_process.stdin
                .as_mut()
                .unwrap_or_else(|| panic!("Could not retrieve stdin for: {}", path));
            let mut child_out = BufReader::new(child_process.stdout
                .as_mut()
                .unwrap_or_else(|| panic!("Could not retrieve stdout for: {}", path)));

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
