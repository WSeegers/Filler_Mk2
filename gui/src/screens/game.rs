use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
// use glutin_window::GlutinWindow as Window;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use glium::glutin::Window;
use opengl_graphics::{GlGraphics, OpenGL};

use fillercore::models::*;

use fillercore::models::piece::{Piece, PieceBag};
use fillercore::models::plateau::{Plateau, Cell};
use fillercore::models::player::Player;
use fillercore::models::point::Point;

use fillercore::engine::Engine;

widget_ids!(struct Ids {
    canvas,
});

pub struct State {
    ids: Ids,
}

pub struct Game<'a> {
    display: &'a mut conrod::glium::Display,
    events_loop: &'a mut glium::glutin::EventsLoop,
    events: Events,
    width: u32,
    height: u32,
    board_width: u32,
    board_height: u32,
    square_size: u32,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a mut conrod::glium::Display, events_loop: &'a mut glium::glutin::EventsLoop, width: u32, height: u32, board_width: u32, board_height: u32) -> Self {
        let mut events = Events::new(EventSettings::new());

        Self {
            display,
            events_loop,
            events,
            width,
            height,
            board_width,
            board_height,
            square_size: width / board_width,
        }
    }
    
    // fn render(&mut self, args: &RenderArgs, plateau: &Plateau) {
    //     use graphics::*;

    //     const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    //     const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    //     const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    //     self.gl.draw(args.viewport(), |c, gl| {
    //         clear(BLACK, gl);
    //     });

    //     for (i, item) in plateau.cells.iter().enumerate() {
    //         // println!("The {}th item is {}", i, item);
    //         match item {
    //             Cell::Empty => continue,
    //             _ => ()
    //         }

    //         let x = i % (self.board_height as usize) * (self.square_size as usize);
    //         let y = i / (self.board_height as usize) * (self.square_size as usize);
    //         let square = rectangle::square(x as f64, y as f64, (self.square_size as f64 / 1.5) as f64);
    //         // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

    //         self.gl.draw(args.viewport(), |c, gl| {

    //             // let transform = c.transform.trans(x as f64, y as f64).trans(-25.0, -25.0);
    //             // let transform = c.transform.trans(x as f64, y as f64);
    //             let transform = c.transform;

    //             let color = match item {
    //                 Cell::Player1(true) => RED,
    //                 Cell::Player2(true) => BLUE,
    //                 Cell::Empty => BLACK,
    //             };
    //             rectangle(color, square, transform, gl);
    //         })
    //     }
    // }

    pub fn main_loop(&mut self) {
        // let player1_start = Point { x: 4, y: 4 };
        // let player2_start = Point { x: 44, y: 44 };

        // let plat = match Plateau::new(50, 50, &player1_start, &player2_start) {
        //     Ok(plat) => plat,
        //     Err(msg) => panic!(msg),
        // };
        // // --------------------------------------------

        // let p_bag = PieceBag::new([5, 7], [5, 7]);

        // let mut steve = match Engine::new(plat, p_bag, String::from("../gsteyn.filler"), Some(String::from("../gsteyn.filler")), 2) {
        //     Err(e) => panic!(e),
        //     Ok(engin) => engin,
        // };
        
        // let ERROR_THRESHOLD = 3;

        // let mut errors: u8 = 0;
        loop {
            use glium::{glutin, Surface};

            let mut closed = false;
            println!("HERE!!!!!!!!!!!!!!");
            while !closed {
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
                target.finish().unwrap();

                self.events_loop.poll_events(|ev| {
                    match ev {
                        glutin::Event::WindowEvent { event, .. } => match event {
                            glutin::WindowEvent::CloseRequested => closed = true,
                            _ => (),
                        },
                        _ => (),
                    }
                });
            }
            // match steve.next_move() {
            //     Ok(response) => {
            //         print!("<got ({}): {}", response.player, response.raw_response);
            //         print!("{}", response.piece);
            //         print!("{}", steve.get_plateau());
            //         errors = 0;

            //         if let Some(e) = self.events.next(&mut self.window) {
            //             if let Some(r) = e.render_args() {
            //                 self.render(&r, &plat);
            //             }
            //         }
            //     }
            //     Err(e) => {
            //         println!("{}", e);
            //         errors += 1;
            //     }
            // }
            // match errors {
            //     e if e >= ERROR_THRESHOLD => break,
            //     _ => (),
            // }
        }
    }
}

// impl Widget for Game {
//     type State = State;
//     type Style = ();
//     type Event = ();

//     fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
//         State {
//             ids: Ids::new(id_gen),
//         }
//     }

//     fn style(&self) -> Self::Style {}

//     fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
//         let widget::UpdateArgs { state, ui, id, .. } = args;

//         widget::Canvas::new()
//             .parent(id)
//             .color(color::DARK_CHARCOAL)
//             .border(0.0)
//             .wh_of(id)
//             .set(state.ids.canvas, ui);
//     }
// }
