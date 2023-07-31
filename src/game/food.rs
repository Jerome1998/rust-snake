extern crate rand;

use super::{Game, Color};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

pub struct Food {
	pos_x: f64,
	pos_y: f64,
	color: Color,
	flash: bool
}

impl Food {
	pub fn new() -> Self {
		let mut rng = rand::thread_rng();
		let pos_x = (rng.gen_range(0..(Game::ROWS - 1)) * Game::PIXEL_SIZE) as f64;
		let pos_y = (rng.gen_range(0..(Game::COLS - 1)) * Game::PIXEL_SIZE) as f64;

		Food {
			pos_x,
			pos_y,
			color: Color::GREEN,
			flash: false
		}
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let square = graphics::rectangle::square(self.pos_x, self.pos_y, Game::PIXEL_SIZE as f64);

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