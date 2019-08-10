use event::MouseButton;
use ggez::*;

struct State {
	dt: std::time::Duration,
	fps: f64,
	mouse_down: bool,
	mouse_pos: [f32; 4],
}

impl State {
	fn new() -> Self {
		State {
			dt: std::time::Duration::new(0, 0),
			mouse_down: false,
			mouse_pos: [0.0; 4],
			fps: 0.0,
		}
	}
}

impl event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.dt = timer::delta(ctx);
		self.fps = timer::fps(ctx);
		// println!("{}", 1.0 / (self.dt.as_nanos() as f64 / 1000000000_f64));
		// println!("fps: {}", self.fps);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

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

	let config = conf::Conf::new();
	let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello_gzzzz", "ItRI")
		.conf(config)
		.build()
		.expect("failed to build");

	event::run(ctx, event_loop, state).expect("Starting loop failed");

	println!("Hello, world!");
}
