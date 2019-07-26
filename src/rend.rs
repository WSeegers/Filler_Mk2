extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use super::models::*;

use piece::{Piece, PieceBag};
use plateau::{Plateau, Player, Cell};
use point::Point;

pub struct App {
    gl: GlGraphics,
    window: Window,
    events: Events,
    width: u32,
    height: u32,
    board_width: u32,
    board_height: u32,
    square_size: u32,
    rotation: f64
}

impl App {
    pub fn new(width: u32, height: u32, board_width: u32, board_height: u32) -> Self {
        let opengl = OpenGL::V3_2;

        let mut window: Window = WindowSettings::new("test", [width, height]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
        let mut events = Events::new(EventSettings::new());

        Self {
            gl: GlGraphics::new(opengl),
            window,
            events,
            width,
            height,
            board_width,
            board_height,
            square_size: width / board_width,
            rotation: 0.0
        }
    }
    
    fn render(&mut self, args: &RenderArgs, plateau: &Plateau) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });

        for (i, item) in plateau.cells.iter().enumerate() {
            // println!("The {}th item is {}", i, item);
            match item {
                Cell::Empty => continue,
                _ => ()
            }

            let x = i % (self.board_height as usize) * (self.square_size as usize);
            let y = i / (self.board_height as usize) * (self.square_size as usize);
            let square = rectangle::square(x as f64, y as f64, (self.square_size as f64 / 1.5) as f64);
            // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            self.gl.draw(args.viewport(), |c, gl| {

                // let transform = c.transform.trans(x as f64, y as f64).trans(-25.0, -25.0);
                // let transform = c.transform.trans(x as f64, y as f64);
                let transform = c.transform;

                let color = match item {
                    Cell::Player1 => RED,
                    Cell::Player2 => BLUE,
                    Cell::Empty => BLACK,
                };
                rectangle(color, square, transform, gl);
            })
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    pub fn main_loop(&mut self, plateau: &Plateau) {
        if let Some(e) = self.events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r, plateau);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }
}
