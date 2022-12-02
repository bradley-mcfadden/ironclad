pub mod board;
pub mod game;
pub mod vec;

use game::PlayerFactory;
use game::{PLAYER_A_ID, PLAYER_B_ID, STARTING_STONES, Game};


/**
 * Creates two ConsolePlayers and plays games until the program is forcefully terminated (for now).
 */
pub fn run() {
    let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
    let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
    let mut game = Game::new(&mut player_a, &mut player_b);

    loop {
        game.play();
        game.reset();
    }
}