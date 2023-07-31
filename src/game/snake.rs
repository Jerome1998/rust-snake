use super::{Game, Color};
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

impl Snake {
	const MOUTH_WIDTH: u32 = 2;
	const MOUTH_HEIGHT: u32 = Game::PIXEL_SIZE / 4;

	pub fn new() -> Self {
		Snake {
			pos_x: 0.0,
			pos_y: 0.0,
			direction: Direction::Right,
			color: Color::RED
		}
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let square = graphics::rectangle::square(self.pos_x, self.pos_y, Game::PIXEL_SIZE as f64);
		let mouth = self.get_mouth();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;

			graphics::rectangle(self.color.as_array(), square, transform, gl);
			graphics::rectangle(Color::BLACK.as_array(), mouth, transform, gl);
		});
	}

	fn get_mouth(&self) -> [f64; 4] {
		match self.direction {
			Direction::Right => {
				let x0 = self.pos_x + (Game::PIXEL_SIZE - Snake::MOUTH_HEIGHT) as f64;
				let y0 = self.pos_y + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let x1 = x0 + Snake::MOUTH_HEIGHT as f64;
				let y1 = y0 + Snake::MOUTH_WIDTH as f64;
				return graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1);
			},
			Direction::Left => {
				let x0 = self.pos_x;
				let y0 = self.pos_y + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let x1 = x0 + Snake::MOUTH_HEIGHT as f64;
				let y1 = y0 + Snake::MOUTH_WIDTH as f64;
				return graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1);
			}
			Direction::Down => {
				let x0 = self.pos_x + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let y0 = self.pos_y + (Game::PIXEL_SIZE - Snake::MOUTH_HEIGHT) as f64;
				let x1 = x0 + Snake::MOUTH_WIDTH as f64;
				let y1 = y0 + Snake::MOUTH_HEIGHT as f64;
				return graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1);
			},
			Direction::Up => {
				let x0 = self.pos_x + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let y0 = self.pos_y;
				let x1 = x0 + Snake::MOUTH_WIDTH as f64;
				let y1 = y0 + Snake::MOUTH_HEIGHT as f64;
				return graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1);
			}
		};
	}

	pub fn update(&mut self, args: &UpdateArgs) {
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

	pub fn button(&mut self, args: &ButtonArgs) {
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
