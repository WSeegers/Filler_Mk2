#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate ttf_noto_sans;
extern crate find_folder;

mod core;
mod screens;

fn main() {
    core::main_loop();
}
