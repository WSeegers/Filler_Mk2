#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate find_folder;
extern crate ttf_noto_sans;

extern crate nfd;

#[macro_use]
extern crate glium;

extern crate fillercore;

mod core;
mod screens;

use crate::core::controller;

fn main() {
    controller::main_loop();
}
