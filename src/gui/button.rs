use event::MouseButton;
use ggez::{event, graphics, Context, GameResult};

pub struct Button {
	draw_mode: graphics::DrawMode,
	bounds: graphics::Rect,
	color: graphics::Color,
	mouse_hover: bool,
	mouse_down: bool,
}

impl Button {
	pub fn new() -> Self {
		Button {
			draw_mode: graphics::DrawMode::fill(),
			bounds: graphics::Rect::new(10.0, 10.0, 100.0, 50.0),
			color: graphics::WHITE,
			mouse_hover: false,
			mouse_down: false,
		}
	}

	fn mouse_enter(&mut self) {
		self.color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);
		println!("Mouse Enter");
	}

	fn mouse_exit(&mut self) {
		self.color = graphics::WHITE;
		println!("Mouse Exit");
	}

	fn on_click(&mut self, _button: MouseButton) {
		println!("Mouse clicked");
	}

	fn is_in_bounds(&self, x: f32, y: f32) -> bool {
		y >= self.bounds.top()
			&& y <= self.bounds.bottom()
			&& x >= self.bounds.left()
			&& x <= self.bounds.right()
	}
}

impl event::EventHandler for Button {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let mesh = graphics::Mesh::new_rectangle(ctx, self.draw_mode, self.bounds, self.color)
			.expect("Failed to make button body");
		graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
		Ok(())
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
		if self.is_in_bounds(x, y) {
			self.mouse_down = true;
			println!("Button: Mouse down {:?}", button);
		}
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
		if self.mouse_down && self.is_in_bounds(x, y) {
			self.on_click(button);
		}
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		match self.is_in_bounds(x, y) {
			true if !self.mouse_hover => {
				self.mouse_enter();
				self.mouse_hover = true;
			}
			false if self.mouse_hover => {
				self.mouse_exit();
				self.mouse_hover = false;
			}
			_ => (),
		}
	}
}
