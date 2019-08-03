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

const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 500;

widget_ids!(struct Ids {
    home,
    player_select,
    game,
});

pub fn main_loop() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let mut display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui =
        conrod::UiBuilder::new([INITIAL_WINDOW_WIDTH as f64, INITIAL_WINDOW_HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let font_path = Path::new("src/assets/fonts/blocks.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    // ui.fonts.insert(conrod::text::FontCollection::from_bytes(ttf_noto_sans::REGULAR).unwrap().into_font().unwrap());

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let home_id = Ids::new(ui.widget_id_generator()).home;
    let player_select_id = Ids::new(ui.widget_id_generator()).player_select;
    let game_id = Ids::new(ui.widget_id_generator()).game;
    let mut screen = Screen::Home;

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
                Screen::PSelect => PlayerSelect::new().set(player_select_id, &mut ui.set_widgets()),
                Screen::Game => {
                    let p1_start = Point { x: 4, y: 4 };
                    let p2_start = Point { x: 94, y: 94 };
                    let mut game = Game::new(
                        &mut screen,
                        &mut display,
                        &mut events_loop,
                        800.0,
                        500.0,
                        100,
                        100,
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
