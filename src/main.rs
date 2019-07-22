mod plateau;
mod app;

use plateau::{Cell, Plateau};
use app::App;

fn main() {
    // let mut p = Plateau::new(30, 14);

    // p.set(0, 0, Cell::Player1);
    // p.set(0, 1, Cell::Player2);
    // p.set(5, 5, Cell::Player2);

    // println!("{}", p);

    let app = App::new(String::from("./a.out"), String::from("./a.out"), 2);
    app.p1_send(String::from("Hey\n"));
    let s: Option<String> = match app.p1_receive() {
        Ok(s) => {
            print!("{}", s);
            Some(s)
        },
        Err(_) => {
            println!("Player took too long to respond");
            None
        },
    };
}
