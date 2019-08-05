mod eventloop;

pub mod controller;
pub use controller::main_loop;

pub enum Screen {
    Home,
    PSelect,
    Tournament,
    Game,
    Exit,
}
