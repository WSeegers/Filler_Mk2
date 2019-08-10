extern crate fillercore;

use event::MouseButton;
use ggez::*;

use engine::Engine;
use fillercore::{engine, models};
use models::{PieceBag, Plateau, Point};

struct State {
	dt: std::time::Duration,
	fps: f64,
	mouse_down: bool,
	mouse_pos: [f32; 4],
	engine: engine::Engine,
	errors: u8,
}

impl State {
	fn new() -> Self {
		let player1_start = Point { x: 4, y: 4 };
		let player2_start = Point { x: 94, y: 94 };
		let plat =
			Plateau::new(100, 100, &player1_start, &player2_start).expect("Plateau failed to init");

		let p_bag = PieceBag::new([5, 7], [5, 7]);

		let player1 = String::from("./resources/players/gsteyn.filler");
		let player2 = String::from("./resources/players/wseegers.filler");

		let engine =
			Engine::new(plat, p_bag, player1, Some(player2), 2).expect("Engine failed to init");

		let errors: u8 = 0;

		State {
			dt: std::time::Duration::new(0, 0),
			mouse_down: false,
			mouse_pos: [0.0; 4],
			fps: 0.0,
			engine,
			errors,
		}
	}
}

impl event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.dt = timer::delta(ctx);
		self.fps = timer::fps(ctx);
		println!("fps: {}", self.fps);

		let engine = &mut self.engine;
		match self.errors {
			e if e >= 5 => event::quit(ctx),
			_ => match engine.next_move() {
				Ok(_response) => {
					self.errors = 0;
					()
				}
				Err(e) => {
					println!("{}", e);
					self.errors += 1;
				}
			},
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		let plat = self.engine.get_plateau();

		let mut map_builder = graphics::MeshBuilder::new();

		for y in 0..plat.height as i32 {
			for x in 0..plat.width as i32 {
				let cell = plat.get(&Point::new(x as i32, y as i32));
				use fillercore::models::plateau::Cell;
				let color = match cell {
					Cell::Empty => continue,
					Cell::Player1(_) => graphics::Color::new(1.0, 0.0, 0.0, 1.0),
					Cell::Player2(_) => graphics::Color::new(0.0, 0.0, 1.0, 1.0),
				};

				map_builder.rectangle(
					graphics::DrawMode::fill(),
					graphics::Rect::new_i32((x * 10) + 1, (y * 10) + 1, 8, 8),
					color,
				);
			}
		}

		let map = map_builder.build(ctx).expect("Failed to build Map");
		graphics::draw(ctx, &map, graphics::DrawParam::default()).expect("Failed to draw map");
		graphics::present(ctx)?;
		Ok(())
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
		self.mouse_down = true;
		println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
		self.mouse_down = false;
		println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
		self.mouse_pos = [x, y, dx, dy];
	}
}

fn main() {
	let state = &mut State::new();

	let mut config = conf::Conf::new();
	config.window_mode.width = 1000.0;
	config.window_mode.height = 1000.0;
	let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello_gzzzz", "ItRI")
		.conf(config)
		.build()
		.expect("failed to build");

	event::run(ctx, event_loop, state).expect("Starting loop failed");
}
