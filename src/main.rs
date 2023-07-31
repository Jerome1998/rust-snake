mod game;

use opengl_graphics::OpenGL;
use game::Game;

fn main() {
  let mut game = Game::new(OpenGL::V4_5); 
  game.enter_render_loop();
}
