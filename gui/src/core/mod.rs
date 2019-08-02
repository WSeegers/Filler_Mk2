mod eventloop;

pub mod renderer;
pub use renderer::main_loop;

pub enum Screen {
    Home,
    PSelect,
    Tournament,
    Game,
    Exit,
}
