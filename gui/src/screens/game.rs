use fillercore::models::*;

use fillercore::models::piece::{Piece, PieceBag};
use fillercore::models::plateau::{Plateau, Cell};
use fillercore::models::player::Player;
use fillercore::models::point::Point;

use fillercore::engine::Engine;

use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

static vertex_shader_src: &'static str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

static fragment_shader_src: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;


implement_vertex!(Vertex, position);

pub struct Game<'a> {
    display: &'a mut conrod::glium::Display,
    events_loop: &'a mut glium::glutin::EventsLoop,
    width: u32,
    height: u32,
    board_width: u32,
    board_height: u32,
    square_size: u32,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a mut conrod::glium::Display, events_loop: &'a mut glutin::EventsLoop, width: u32, height: u32, board_width: u32, board_height: u32) -> Self {

        Self {
            display,
            events_loop,
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

    fn normalize_x(&self, x: u32) -> f32 {
        ((x as f32 / self.width as f32) - 1.0) as f32
    }

    fn normalize_y(&self, y: u32) -> f32 {
        ((y as f32 / self.height as f32) - 1 as f32) as f32
    }

    fn draw_square(&self, x: u32, y: u32, target: &mut glium::Frame) {
        let start_x = self.normalize_x(x);
        let start_y = -self.normalize_y(y);
        let sq_width: f32 = self.square_size as f32 / self.width as f32 * 2.0;
        let vertex1 = Vertex { position: [start_x, start_y] };
        let vertex2 = Vertex { position: [ start_x + sq_width,  start_y] };
        let vertex3 = Vertex { position: [ start_x + sq_width, start_y - sq_width] };
        let vertex4 = Vertex { position: [ start_x, start_y - sq_width] };
        println!("startx: {}\n starty: {}\n sq_width: {}", start_x, start_y, sq_width);
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let disp = self.display.clone();
        let vertex_buffer = glium::VertexBuffer::new(&disp, &shape).unwrap();

        let ib_data: Vec<u16> = vec![0, 1, 3, 1, 2, 3];
        let indices = glium::IndexBuffer::new(
            &disp,
            glium::index::PrimitiveType::TrianglesList,
            &ib_data
        ).unwrap();

        let program = glium::Program::from_source(&disp, vertex_shader_src, fragment_shader_src, None).unwrap();

        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
    }

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

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.finish().unwrap();

        loop {
            let mut closed = false;
            while !closed {

                let mut target = self.display.draw();

                self.draw_square(50, 50, &mut target);
                self.draw_square(100, 100, &mut target);

                target.finish().unwrap();

                self.events_loop.poll_events(|ev| {
                    match ev {
                        glium::glutin::Event::WindowEvent { event, .. } => match event {
                            // Break from the loop upon `Escape`.
                            glium::glutin::WindowEvent::CloseRequested
                            | glium::glutin::WindowEvent::KeyboardInput {
                                input:
                                    glium::glutin::KeyboardInput {
                                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => closed = true,
                            _ => (),
                        },
                        _ => (),
                    }
                });
            }
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
