#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate ttf_noto_sans;
extern crate find_folder;

extern crate nfd;

mod core;
mod screens;

use crate::core::renderer;

fn main() {
    renderer::main_loop();
}
