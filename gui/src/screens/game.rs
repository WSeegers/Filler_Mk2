use fillercore::models::*;

use fillercore::models::piece::{Piece, PieceBag};
use fillercore::models::plateau::{Plateau, Cell};
use fillercore::models::player::Player;
use fillercore::models::point::{Point, TryFrom};

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
    width: f32,
    height: f32,
    board_width: u32,
    board_height: u32,
    rect_width: f32,
    rect_height: f32,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a mut conrod::glium::Display, events_loop: &'a mut glutin::EventsLoop, width: f32, height: f32, board_width: u32, board_height: u32) -> Self {

        Self {
            display,
            events_loop,
            width,
            height,
            board_width,
            board_height,
            rect_width: width / board_width as f32,
            rect_height: height / board_height as f32,
        }
    }
    
    fn draw_plateau(&mut self, plateau: &Plateau, target: &mut glium::Frame) {
        for (i, cell) in plateau.cells.iter().enumerate() {
            match cell {
                Cell::Empty => continue,
                _ => ()
            }

            let x: f32 = i as f32 % (self.board_height as f32);
            let y: f32 = i as f32 / (self.board_height as f32);

            self.draw_rect(x * self.rect_width, y * self.rect_height, target, cell);
        }
    }

    fn draw_piece(&mut self, piece: Piece, pos: Point, player: &Player, target: &mut glium::Frame) {
        let cell = match player {
            Player::Player1 => Cell::Player1(true),
            Player::Player2 => Cell::Player2(true),
        };

        for (i, block) in piece.cells.iter().enumerate() {
            if (*block) {
                let x: f32 = i as f32 % piece.width as f32 + pos.x as f32;
                let y: f32 = i as f32 / piece.height as f32 + pos.y as f32;

                self.draw_rect(x * self.rect_width, y * self.rect_height, target, &cell);
            }
        }
    }

    fn normalize_x(&self, x: f32) -> f32 {
        (x / self.width) * 2.0 - 1.0
    }

    fn normalize_y(&self, y: f32) -> f32 {
        (y / self.height) * 2.0 - 1.0
    }

    fn draw_rect(&self, x: f32, y: f32, target: &mut glium::Frame, cell: &Cell) {
        let start_x = self.normalize_x(x);
        let start_y = -self.normalize_y(y);
        let rect_width: f32 = self.rect_width / self.width * 1.5;
        let rect_height: f32 = self.rect_height / self.height * 1.5;
        let vertex1 = Vertex { position: [start_x, start_y] };
        let vertex2 = Vertex { position: [ start_x + rect_width,  start_y] };
        let vertex3 = Vertex { position: [ start_x + rect_width, start_y - rect_height] };
        let vertex4 = Vertex { position: [ start_x, start_y - rect_height] };
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
        let player2_start = Point { x: 94, y: 94 };

        let plat = match Plateau::new(self.board_width, self.board_height, &player1_start, &player2_start) {
            Ok(plat) => plat,
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
                        let pos = Point::try_from(&response.raw_response).unwrap();
                        self.draw_piece(response.piece, pos, &response.player, &mut target);
                        ()
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1;
                    }
                }

                let plat: &Plateau = steve.get_plateau();
                // self.draw_plateau(plat, &mut target);

                match errors {
                    e if e >= ERROR_THRESHOLD => break,
                    _ => (),
                }

                let width = &mut self.width;
                let height = &mut self.height;
                let rect_width = &mut self.rect_width;
                let rect_height = &mut self.rect_height;
                let board_width = &mut self.board_width;
                let board_height = &mut self.board_height;
                let mut reset = false;

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
                                *width = size.width as f32;
                                *height = size.height as f32;
                                *rect_width = *width / *board_width as f32;
                                *rect_height = *height / *board_height as f32;
                                target.clear_color(0.0, 0.0, 1.0, 1.0);
                                reset = true;
                            },
                            _ => (),
                        },
                        _ => (),
                    }
                });

                if (reset) {
                    self.draw_plateau(plat, &mut target);
                }

                target.finish().unwrap();
            }
        }
    }
}
