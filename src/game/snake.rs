use super::{Game, RenderObject, Color};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct Snake {
	x: f64,
	y: f64,
	color: Color
}

impl RenderObject for Snake {
	fn new() -> Self {
		Snake {
			x: 0.0,
			y: 0.0,
			color: Color::RED
		}
	}

	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		let square = graphics::rectangle::square(self.x, self.y, Game::SQUARE_SIZE);

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;

			graphics::rectangle(self.color.as_array(), square, transform, gl)
		});
	}
}
