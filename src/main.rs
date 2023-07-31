mod game;

use opengl_graphics::OpenGL;
use game::Game;

fn main() {
    let open_gl = OpenGL::V4_5;
    let mut game = Game::new(open_gl); 
    game.enter_render_loop();
}
