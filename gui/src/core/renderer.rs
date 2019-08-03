use super::eventloop::EventLoop;
use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

use std::path::Path;
use ttf_noto_sans;

use crate::screens::{Game, Home, PlayerSelect};
use conrod::Widget;

use super::Screen;

use fillercore::models::point::Point;

static TITLE: &str = "filler_mk2";

const INITIAL_WINDOW_WIDTH: f32 = 800.0;
const INITIAL_WINDOW_HEIGHT: f32 = 500.0;
const INITIAL_BOARD_WIDTH: u32 = 50;
const INITIAL_BOARD_HEIGHT: u32 = 50;

widget_ids!(struct Ids {
    home,
    player_select,
    game,
});

pub fn main_loop() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((INITIAL_WINDOW_WIDTH as u32, INITIAL_WINDOW_HEIGHT as u32).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let mut display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui =
        conrod::UiBuilder::new([INITIAL_WINDOW_WIDTH as f64, INITIAL_WINDOW_HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let font_path = Path::new("src/assets/fonts/blocks.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let home_id = Ids::new(ui.widget_id_generator()).home;
    let player_select_id = Ids::new(ui.widget_id_generator()).player_select;
    let game_id = Ids::new(ui.widget_id_generator()).game;

    let mut screen = Screen::Home;
    let mut window_width = INITIAL_WINDOW_WIDTH;
    let mut window_height = INITIAL_WINDOW_HEIGHT;
    let mut board_width = INITIAL_BOARD_WIDTH;
    let mut board_height = INITIAL_BOARD_HEIGHT;
    let mut p1_start = Point { x: 4, y: 4 };
    let mut p2_start = Point {
        x: (board_width - 4) as i32,
        y: (board_height - 4) as i32,
    };
    let mut p1_path = Some(String::from("../resources/players/gsteyn.filler"));
    let mut p2_path = Some(String::from("../resources/players/gsteyn.filler"));

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
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
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        {
            match screen {
                Screen::Home => Home::new(&mut screen).set(home_id, &mut ui.set_widgets()),
                Screen::PSelect => PlayerSelect::new(
                    &mut screen,
                    &mut p1_path,
                    &mut p2_path,
                    &mut p1_start,
                    &mut p2_start,
                    &mut board_width,
                    &mut board_height,
                )
                .set(player_select_id, &mut ui.set_widgets()),
                Screen::Game => {
                    let mut game = Game::new(
                        &mut screen,
                        &mut display,
                        &mut events_loop,
                        &mut window_width,
                        &mut window_height,
                        board_width,
                        board_width,
                        p1_start,
                        p2_start,
                    );
                    game.main_loop();
                }
                Screen::Exit => break 'main,
                _ => Home::new(&mut screen).set(home_id, &mut ui.set_widgets()),
            }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
