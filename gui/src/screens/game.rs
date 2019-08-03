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

static fragment_shader_src_red: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

static fragment_shader_src_green: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.0, 1.0, 0.0, 1.0);
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
    
    fn render(&mut self, plateau: &Plateau, target: &mut glium::Frame) {
        for (i, cell) in plateau.cells.iter().enumerate() {
            match cell {
                Cell::Empty => continue,
                _ => ()
            }

            let x: u32 = i as u32 % (self.board_height) * (self.square_size);
            let y: u32 = i as u32 / (self.board_height) * (self.square_size);

            self.draw_square(x, y, target, cell);
        }
    }

    fn normalize_x(&self, x: u32) -> f32 {
        ((x as f32 / self.width as f32) - 1.0) as f32
    }

    fn normalize_y(&self, y: u32) -> f32 {
        ((y as f32 / self.height as f32) - 1 as f32) as f32
    }

    fn draw_square(&self, x: u32, y: u32, target: &mut glium::Frame, cell: &Cell) {
        let start_x = self.normalize_x(x);
        let start_y = -self.normalize_y(y);
        let sq_width: f32 = self.square_size as f32 / self.width as f32 * 2.0;
        let vertex1 = Vertex { position: [start_x, start_y] };
        let vertex2 = Vertex { position: [ start_x + sq_width,  start_y] };
        let vertex3 = Vertex { position: [ start_x + sq_width, start_y - sq_width] };
        let vertex4 = Vertex { position: [ start_x, start_y - sq_width] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let disp = self.display.clone();
        let vertex_buffer = glium::VertexBuffer::new(&disp, &shape).unwrap();

        let ib_data: Vec<u16> = vec![0, 1, 3, 1, 2, 3];
        let indices = glium::IndexBuffer::new(
            &disp,
            glium::index::PrimitiveType::TrianglesList,
            &ib_data
        ).unwrap();

        let shader = match cell {
            Cell::Player1(_) => fragment_shader_src_red,
            Cell::Player2(_) => fragment_shader_src_green,
            _ => fragment_shader_src_green,
        };

        let program = glium::Program::from_source(&disp, vertex_shader_src, shader, None).unwrap();

        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
    }

    pub fn main_loop(&mut self) {
        let player1_start = Point { x: 4, y: 4 };
        let player2_start = Point { x: 44, y: 44 };

        let plat = match Plateau::new(50, 50, &player1_start, &player2_start) { Ok(plat) => plat,
            Err(msg) => panic!(msg),
        };

        let p_bag = PieceBag::new([5, 7], [5, 7]);

        let mut steve = match Engine::new(plat, p_bag, String::from("../resources/players/gsteyn.filler"), Some(String::from("../resources/players/gsteyn.filler")), 2) {
            Err(e) => panic!(e),
            Ok(engin) => engin,
        };
        
        let ERROR_THRESHOLD = 3;

        let mut errors: u8 = 0;

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.finish().unwrap();

        loop {
            let mut closed = false;
            while !closed {

                let mut target = self.display.draw();

                match steve.next_move() {
                    Ok(response) => {
                        errors = 0;
                        ()
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1;
                    }
                }

                let plat: &Plateau = steve.get_plateau();
                self.render(plat, &mut target);

                match errors {
                    e if e >= ERROR_THRESHOLD => break,
                    _ => (),
                }

                let width = &mut self.width;
                let height = &mut self.height;

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
                            glium::glutin::WindowEvent::Resized(size) => {
                                println!("w: {}, h: {}", size.width, size.height);
                                *width = size.width as u32;
                                *height = size.height as u32;
                                target.clear_color(0.0, 0.0, 1.0, 1.0);
                            },
                            _ => (),
                        },
                        _ => (),
                    }
                });

                target.finish().unwrap();
            }
        }
    }
}
