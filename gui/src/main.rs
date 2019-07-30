#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate ttf_noto_sans;
extern crate find_folder;

mod gui;
use gui::main_loop;

fn main() {
    gui::main_loop();
}
