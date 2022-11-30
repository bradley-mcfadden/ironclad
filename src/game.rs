
use std::vec::Vec;
use crate::{board::Board, vec::Vec2};

pub const EMPTY_PLAYER_ID: i32 = -1;
pub const PLAYER_A_ID: i32 = 1;
pub const PLAYER_B_ID: i32 = 2;
const STARTING_STONES: i32 = 32;

/**
 * Game is responsible for the main loop of the game.
 * It gets possible moves from the board, passes them to the Player structs,
 * and passes the selected moves to Board to be applied.
 * Each turn, it checks for a winner.
 */
pub struct Game<'a> {
    pub board: Board,
    pub players: [&'a Player<'a>; 2],
    pub last_two_slides_a: [Option<Intent>; 2],
    pub last_two_slides_b: [Option<Intent>; 2]
}

impl<'a> Game<'a> {
    pub fn new(player_a: &'a mut Player<'a>, player_b: &'a mut Player<'a>) -> Game<'a> {
        Game {
            board: Board::new(),
            players: [
                player_a,
                player_b,
            ],
            last_two_slides_a: [None; 2],
            last_two_slides_b: [None; 2]
        }
    }

    /**
     * play the game, alternating turns between players until a winner is determined, then
     * return that winner.
     * @ret Reference to winning player.
     */
    pub fn play(&mut self) -> i32 {
        loop {
            for p_num in 0..2 {
                let player = self.players[p_num];
                let move_checkers = self.checker_moves_for(player.id);
                let fire_checkers = self.checker_fires_for(player.id);
                let place_stones = self.stone_places_for(player.id);
                let slide_stones = self.stone_slides_for(player.id);
                let chosen_move = player.choose_move(
                    move_checkers, fire_checkers, place_stones, slide_stones
                );
                self.apply_move(chosen_move);

                let win_state = self.check_for_win();
                if win_state.is_some() {
                    return win_state.unwrap().id;
                }
            }
        }
    }

    /**
     * check_for_win
     * Determine if the board is in a winning state, returning the winning player or None.
     * ret - Winning player or none.
     */
    pub fn check_for_win(&self) -> Option<&Player<'a>> {
        None
    }

    /**
     * checker_moves_for
     * Get the legal moves for all check pieces of the player.
     * player - Id of player to get moves for.
     * ret - Vector of Intent.MoveChecker.
     */
    pub fn checker_moves_for(&self, player: i32) -> Vec<Intent> {
        vec![]
    }

    /**
     * checker_fires_for
     * Get all legal attack moves for player.
     * player - Id of player to get attack moves for.
     * ret - Vector of Intent.FireChecker
     */
    pub fn checker_fires_for(&self, player: i32) -> Vec<Intent> {
        vec![]
    }

    /**
     * stone_places_for
     * Get all legal moves where a stone is placed for the player.
     * player - Id of player to get stone place moves for.
     * ret - Vector of Intent.PlaceStone
     */
    pub fn stone_places_for(&self, player: i32) -> Vec<Intent> {
        vec![]
    }
    
    /**
     * stone_slides_for
     * Get all legal moves where a stone is slid for the player.
     * player - Id of player to get stone slide moves for.
     * ret - Vector of Intent.SlideStone.
     */
    pub fn stone_slides_for(&self, player: i32) -> Vec<Intent> {
        vec![]
    }

    /**
     * apply_move
     * Apply the move to the game state.
     * intent - Intent specifying action to be taken.
     */
    pub fn apply_move(&mut self, intent: Intent) {

    }

    /* 
     * Helper function returning if a non-straight line of stones 
     * of the same color proceeds from one side of the board to another.
     * Returns reference to winner or none.
     */
    // fn check_for_stone_win
    /*
     * Helper function returing if a checker has reached the opposite side of
     * the board it started on.
     * Returns reference to winner or none.
     */
    // fn check_for_checker_win
    /*
     * Helper function returning winner if law of circularity (circular slide move)
     * has been violated.
     * Returns reference to winner or none.
     */
    // fn check_for_circularity_win
    /*
     * Helper function returning empty neighbour positions around a checker position.
     * Returns an array of Vec2.
     */
    // fn empty_checker_n_at
    /*
     * Helper function returning empty stone directions around a stone positions.
     * Returns an array of Direction.
     */
    // fn empty_stone_n_at
    /*
     * Helper function returning valid stone placement positions (empty and not bordering)
     * a square with a checker.
     * Returns an array of Vec2 
     */
    // fn valid_stone_places
}

pub struct Player<'a> {
    pub id: i32,
    pub stones: i32,
    pub decider: &'a dyn Decide
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    MoveChecker(Vec2, Vec2),
    FireChecker(Vec2),
    PlaceStone(Vec2),
    SlideStone(Vec2)
}

pub trait Decide {
    fn choose_move(
        &self, move_checkers: Vec<Intent>, fire_checkers: Vec<Intent>, 
        place_stones: Vec<Intent>, slide_stones: Vec<Intent>
    ) -> Intent;
}

/**
 * ConsolePlayer is a player that makes it moves from the console.
 * Player will be printed a list of options, and selects a move to make
 * from the list.
 * Class is responsible for presenting moves to the player, and collecting
 * the player's intent after they make a decision.
 */
pub struct ConsolePlayer;

impl Decide for ConsolePlayer {
    fn choose_move(
        &self, move_checkers: Vec<Intent>, fire_checkers: Vec<Intent>, 
        place_stones: Vec<Intent>, slide_stones: Vec<Intent>
    ) -> Intent {
        Intent::PlaceStone(Vec2::new(0, 0))
    }
}

impl<'a> Player<'a> {
    pub fn new(_id: i32, nstones: i32, decide: &'a dyn Decide) -> Player<'a> {
        Player {
            id: _id,
            stones: nstones,
            decider: decide
        }
    }

    pub fn set_stones(&mut self, n: i32) {
        self.stones = n
    }

    pub fn get_stone(&mut self) -> Option<Stone> {
        if self.stones <= 0 {
            None
        } else {
            self.stones -= 1;
            Some(Stone::new(self.id))
        }
    }
}

impl<'a> Decide for Player<'a> {
    fn choose_move(
        &self, move_checkers: Vec<Intent>, fire_checkers: Vec<Intent>, 
        place_stones: Vec<Intent>, slide_stones: Vec<Intent>
    ) -> Intent {
        self.decider.choose_move(move_checkers, fire_checkers, place_stones, slide_stones)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checker {
    pub height: usize,
    pub owner: i32
}

impl Checker {
    pub fn new(h: usize, o: i32) -> Checker {
        Checker { height: h, owner: o }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stone {
    pub owner: i32
}

impl Stone {
    pub fn new(o: i32) -> Stone {
        Stone { owner: o }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn player_get_stone() {
        let mut player = Player::new(1, 1, &ConsolePlayer);
        match player.get_stone() {
            None => panic!("Expecting to get a stone!"),
            Some(stone) => assert_eq!(stone.owner, 1)
        } 
        assert_eq!(player.stones, 0);
        match player.get_stone() {
            None => (),
            Some(_) => panic!("Expecting to not get a stone!")
        }
    }

    #[test]
    pub fn check_for_win() {
        panic!()
    }

    #[test]
    pub fn checker_moves_for() {
        panic!()
    }

    #[test]
    pub fn checker_fires_for()  {
        panic!()
    }

    #[test]
    pub fn stone_places_for() {
        panic!()
    }
    
    #[test]
    pub fn stone_slides_for() {
        panic!()
    }

    #[test]
    pub fn apply_move() {

    }
}