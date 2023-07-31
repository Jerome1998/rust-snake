extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod color;
mod snake;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use color::Color;
use snake::Snake;

pub struct Game {
  gl: GlGraphics,
  rotation: f64,
  window: Window,
  game_objects: Vec<Box<dyn RenderObject>>
}

impl Game {
  pub const SQUARE_SIZE: f64 = 20_f64;

  pub fn new(open_gl_version: OpenGL) -> Self {
    // Create a window.
    let window: Window = WindowSettings::new("Snake", [200, 200])
      .graphics_api(open_gl_version)
      .exit_on_esc(true)
      .build()
      .unwrap();

    // Create a new game and run it.
    let snake = Snake::new();

    let game_objects: Vec<Box<dyn RenderObject>> = vec![
      Box::new(snake)
    ];

    Game {
      gl: GlGraphics::new(open_gl_version),
      rotation: 0.0,
      window,
      game_objects
    }
  }

  pub fn enter_render_loop(&mut self) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut self.window) {
      if let Some(args) = e.render_args() {
        self.render(&args);
      }

      if let Some(args) = e.update_args() {
        self.update(&args);
      }
    }
  }

  fn render(&mut self, args: &RenderArgs) {
    self.gl.draw(args.viewport(), |_, gl| {
      // Clear the screen.
      graphics::clear(Color::BLUE.as_array(), gl);

      for render_object in self.game_objects.iter_mut() {
        render_object.render(gl, &args);
      }
    });
  }

  fn update(&mut self, args: &UpdateArgs) {
    // Rotate 2 radians per second.
    self.rotation += 2.0 * args.dt;
  }
}

pub trait RenderObject {
  fn new() -> Self where Self: Sized;
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
}