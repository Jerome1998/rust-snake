extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod color;

use graphics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use color::Color;

pub struct Game {
    gl: GlGraphics,
    rotation: f64,
    window: Window
}

impl Game {
    pub fn new(open_gl_version: OpenGL) -> Self {
        // Create a window.
        let window: Window = WindowSettings::new("Snake", [200, 200])
            .graphics_api(open_gl_version)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        Game {
            gl: GlGraphics::new(open_gl_version),
            rotation: 0.0,
            window
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
        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(Color::BLUE.as_array(), gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(Color::RED.as_array(), square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}