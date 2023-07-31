use super::{Game, RenderObject, Color};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs, ButtonArgs, ButtonState, Button, Key};

#[derive(PartialEq)]
pub enum Direction {
	Right,
	Left,
	Up,
	Down
}

pub struct Snake {
	pos_x: f64,
	pos_y: f64,
	direction: Direction,
	color: Color
}

impl RenderObject for Snake {
	fn new() -> Self {
		Snake {
			pos_x: 0.0,
			pos_y: 0.0,
			direction: Direction::Right,
			color: Color::RED
		}
	}

	fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let square = graphics::rectangle::square(self.pos_x, self.pos_y, Game::PIXEL_SIZE as f64);

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;

			graphics::rectangle(self.color.as_array(), square, transform, gl)
		});
	}

	fn update(&mut self, args: &UpdateArgs) {
		if self.direction == Direction::Right {
			self.pos_x += Game::PIXEL_SIZE as f64;
		}
		if self.direction == Direction::Left {
			self.pos_x -= Game::PIXEL_SIZE as f64;
		}
		if self.direction == Direction::Up {
			self.pos_y -= Game::PIXEL_SIZE as f64;
		}
		if self.direction == Direction::Down {
			self.pos_y += Game::PIXEL_SIZE as f64;
		}
	}

	fn button(&mut self, args: &ButtonArgs) {
		if args.state == ButtonState::Press {
			match args.button {
				Button::Keyboard(Key::Right) => self.direction = Direction::Right,
				Button::Keyboard(Key::Left) => self.direction = Direction::Left,
				Button::Keyboard(Key::Up) => self.direction = Direction::Up,
				Button::Keyboard(Key::Down) => self.direction = Direction::Down,
				_ => ()
			}
		}
	}
}
