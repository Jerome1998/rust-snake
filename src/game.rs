extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod color;
mod snake;
mod food;
mod position;

use color::Color;
use snake::Snake;
use food::Food;
use position::Position;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, ButtonEvent, ButtonArgs};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::PistonWindow;
use rand::Rng;

pub struct Game {
  gl: GlGraphics,
  window: PistonWindow,
  snake: Snake,
  food: Food
}

impl Game {
  pub const PIXEL_SIZE: u32 = 20;
  const COLS: u32 = 30;
  const ROWS: u32 = 20;

  const WIDTH: u32 = Game::COLS * Game::PIXEL_SIZE;
  const HEIGHT: u32 = Game::ROWS * Game::PIXEL_SIZE;

  pub fn new(open_gl_version: OpenGL) -> Self {
    // Create a window.
    let mut window_settings = WindowSettings::new("Snake", [Game::WIDTH, Game::HEIGHT]);
    window_settings.set_resizable(false);
    window_settings.set_decorated(false);

    let window: PistonWindow = window_settings
      .graphics_api(open_gl_version)
      .exit_on_esc(true)
      .build()
      .unwrap();

    // Create a new game and run it.
    Game {
      gl: GlGraphics::new(open_gl_version),
      window,
      snake: Snake::new(),
      food: Food::new()
    }
  }

  pub fn enter_render_loop(&mut self) {
    let mut events = Events::new(EventSettings::new().ups(4));
    while let Some(e) = events.next(&mut self.window) {
      if self.game_over() {
        break;
      }

      if let Some(args) = e.render_args() {
        self.render(&args);
      }

      if let Some(args) = e.update_args() {
        self.update(&args);
      }

      if let Some(args) = e.button_args() {
        self.button(&args);
      }
    }

    println!("Game over!");
    println!("Score: {}", self.snake.parts.len())
  }

  pub fn get_random_position() -> Position {
    let mut rng = rand::thread_rng();
    let pos_x = (rng.gen_range(0..Game::COLS) * Game::PIXEL_SIZE) as f64;
    let pos_y = (rng.gen_range(0..Game::ROWS) * Game::PIXEL_SIZE) as f64;

    Position {
      x: pos_x,
      y: pos_y
    }
  }

  fn render(&mut self, args: &RenderArgs) {
    self.gl.draw(args.viewport(), |_, gl| {
      // Clear the screen.
      graphics::clear(Color::BLUE.as_array(), gl);

      self.food.render(gl, &args);
      self.snake.render(gl, &args);
    });
  }

  fn update(&mut self, args: &UpdateArgs) {
    let snake_head = self.snake.get_head_position().clone();

    // snake eats
    if snake_head == self.food.position {
      self.snake.add_point();
      self.food = Food::new();
    }

    self.snake.update(args);
    self.food.update(args);
  }

  fn button(&mut self, args: &ButtonArgs) {
    self.snake.button(args);
  }

  fn game_over(&mut self) -> bool {
    let head_position = self.snake.get_head_position().clone();

    // snakes eats itself
    let tail = self.snake.parts.clone().split_off(1);
    if tail.iter().any(|position| head_position.eq(position)) {
      println!("You ate yourself");
      return true;
    }

    // max points
    if self.snake.parts.len() == (Game::ROWS * Game::COLS) as usize {
      println!("You can't eat any more, you win!");
      return true;
    }

    // out of boundaries
    if head_position.x >= Game::WIDTH as f64 || head_position.y >= Game::HEIGHT as f64
      || head_position.x < 0.0 || head_position.y < 0.0 {
      println!("You got out of the boundaries");
        return true;
    }

    false
  }
}