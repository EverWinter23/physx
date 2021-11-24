mod components;
mod constants;
mod game;
mod physx;
mod systems;
mod timer;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;

fn main() {
    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    game.init();
    while game.is_running {
        game.process_input();
        game.update();
        game.render();
    }
}
