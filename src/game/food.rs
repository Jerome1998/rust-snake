extern crate rand;

use super::{Game, Color, Position};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

pub struct Food {
	pub position: Position,
	color: Color,
	flash: bool
}

impl Food {
	pub fn new() -> Self {
		let mut rng = rand::thread_rng();
		let pos_x = (rng.gen_range(0..Game::COLS) * Game::PIXEL_SIZE) as f64;
		let pos_y = (rng.gen_range(0..Game::ROWS) * Game::PIXEL_SIZE) as f64;
		let position = Position {
			x: pos_x,
			y: pos_y
		};

		Food {
			position,
			color: Color::GREEN,
			flash: false
		}
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let square = graphics::rectangle::square(self.position.x, self.position.y, Game::PIXEL_SIZE as f64);

		let color = if self.flash {
			Color::YELLOW.as_array()
		} else {
			self.color.as_array()
		};

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;

			graphics::rectangle(color, square, transform, gl);
		})
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		self.flash = !self.flash;
	}
} 