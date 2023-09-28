use super::{Game, Color, Position};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs, ButtonArgs, ButtonState, Button, Key};
use std::collections::LinkedList;
use rand::Rng;

#[derive(PartialEq)]
enum Direction {
	Right,
	Left,
	Up,
	Down
}

pub struct Snake {
	pub parts: LinkedList<Position>,
	direction: Direction,
	color: Color,
	new_point: bool
}

impl Snake {
	const MOUTH_WIDTH: u32 = 2;
	const MOUTH_HEIGHT: u32 = Game::PIXEL_SIZE / 4;

	pub fn new() -> Self {
		Snake {
			parts: LinkedList::from([Game::get_random_position()]),
			direction: Snake::get_random_direction(),
			color: Color::RED,
			new_point: false
		}
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
		let mouth = self.get_mouth();

		let squares: Vec<graphics::types::Rectangle> = self.parts
			.iter()
			.map(|p| graphics::rectangle::square(p.x, p.y, Game::PIXEL_SIZE as f64))
			.collect();

		gl.draw(args.viewport(), |c, gl| {
			let transform = c.transform;

			squares.into_iter().for_each(|square| graphics::rectangle(self.color.as_array(), square, transform, gl));
			graphics::rectangle(Color::BLACK.as_array(), mouth, transform, gl);
		});
	}

	fn get_mouth(&mut self) -> [f64; 4] {
		let head = self.get_head_position().clone();
		return match self.direction {
			Direction::Right => {
				let x0 = head.x + (Game::PIXEL_SIZE - Snake::MOUTH_HEIGHT) as f64;
				let y0 = head.y + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let x1 = x0 + Snake::MOUTH_HEIGHT as f64;
				let y1 = y0 + Snake::MOUTH_WIDTH as f64;
				graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1)
			},
			Direction::Left => {
				let x0 = head.x;
				let y0 = head.y + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let x1 = x0 + Snake::MOUTH_HEIGHT as f64;
				let y1 = y0 + Snake::MOUTH_WIDTH as f64;
				graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1)
			}
			Direction::Down => {
				let x0 = head.x + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let y0 = head.y + (Game::PIXEL_SIZE - Snake::MOUTH_HEIGHT) as f64;
				let x1 = x0 + Snake::MOUTH_WIDTH as f64;
				let y1 = y0 + Snake::MOUTH_HEIGHT as f64;
				graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1)
			},
			Direction::Up => {
				let x0 = head.x + ((Game::PIXEL_SIZE / 2) as f64) - ((Snake::MOUTH_WIDTH / 2) as f64);
				let y0 = head.y;
				let x1 = x0 + Snake::MOUTH_WIDTH as f64;
				let y1 = y0 + Snake::MOUTH_HEIGHT as f64;
				graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1)
			}
		};
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		let mut new_head: Position = self.get_head_position().clone();

		let pixel_size = Game::PIXEL_SIZE as f64;
		match self.direction {
			Direction::Right => new_head.x += pixel_size,
			Direction::Left => new_head.x -= pixel_size,
			Direction::Up => new_head.y -= pixel_size,
			Direction::Down => new_head.y += pixel_size
		}

		// Bug somehow not working correctly
		if self.new_point {
			 self.new_point = false;
		} else {
			self.parts.pop_back();
		}
		self.parts.push_front(new_head);
	}

	pub fn add_point(&mut self) {
		self.new_point = true;
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

	pub fn get_head_position(&mut self) -> &Position {
		self.parts.front().unwrap()
	}

	fn get_random_direction() -> Direction {
		let mut rng = rand::thread_rng();
		let value = rng.gen_range(0..4);
		match value {
			0 => Direction::Right,
			1 => Direction::Left,
			2 => Direction::Up,
			3 => Direction::Down,
			_ => panic!("Value was out of range!")
		}
	}
}
