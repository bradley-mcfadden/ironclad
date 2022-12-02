
use std::vec::Vec;

use crate::vec::Vec2;
use crate::board::{Board, Direction, BOARD_WIDTH, BOARD_HEIGHT};

pub const EMPTY_PLAYER_ID: i32 = -1;
pub const PLAYER_A_ID: i32 = 1;
pub const PLAYER_B_ID: i32 = 2;
pub const STARTING_STONES: i32 = 32;


/**
 * Game is responsible for the main loop of the game.
 * It gets possible moves from the board, passes them to the Player structs,
 * and passes the selected moves to Board to be applied.
 * Each turn, it checks for a winner.
 */
pub struct Game<'a> {
    board: Board,
    players: [&'a mut Player<'a>; 2],
    // most recent stored at 1
    last_two_slides_a: [Option<Intent>; 2],
    last_two_slides_b: [Option<Intent>; 2],
}

impl<'a> Game<'a> {
    /**
     * new - Create a new instance of the game.
     * player_a - Player that moves first.
     * player_b - Player that moves second.
     * ret - New game instance.
     */
    pub fn new(player_a: &'a mut Player<'a>, player_b: &'a mut Player<'a>) -> Game<'a> {
        Game {
            board: Board::new(),
            players: [
                player_a,
                player_b,
            ],
            last_two_slides_a: [None; 2],
            last_two_slides_b: [None; 2],
        }
    }

    /**
     * reset players and board to their initial state.
     */
    pub fn reset(&mut self) {
        self.board.reset();
        for player in self.players.iter_mut() {
            player.reset();
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
                let player_id = self.players[p_num].id;
                let move_checkers = self.checker_moves_for(player_id);
                let fire_checkers = self.checker_fires_for(player_id);
                let place_stones = self.stone_places_for(player_id);
                let slide_stones = self.stone_slides_for(player_id);
                let chosen_move = self.players[p_num].choose_move(
                    move_checkers, fire_checkers, place_stones, slide_stones
                );
                self.apply_move(player_id, chosen_move);
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
     * @ret - Winning player or none.
     */
    pub fn check_for_win(&self) -> Option<&Player<'a>> {
        let checks = [
            self.check_for_checker_win(),
            self.check_for_circularity_win(),
            self.check_for_stone_win(),
        ];
        for check in checks {
            if let Some(winner) = check {
                if winner == PLAYER_A_ID {
                    return Some(&self.players[0]);
                } else if winner == PLAYER_B_ID {
                    return Some(&self.players[1]);
                }
            }
        }
        None
    }

    /**
     * checker_moves_for
     * Get the legal moves for all check pieces of the player.
     * player - Id of player to get moves for.
     * ret - Vector of Intent.MoveChecker.
     */
    pub fn checker_moves_for(&self, player: i32) -> Vec<Intent> {
        let checkers = self.board.checkers_for_player(player);
        let mut moves: Vec<Intent> = Vec::new();
        for checker_position in checkers.iter() {
            for neighbour_position in self.empty_checker_n_at(*checker_position).iter() {
                moves.push(Intent::MoveChecker(*checker_position, *neighbour_position));
            }
        }
        moves
    }

    /**
     * checker_fires_for
     * Get all legal attack moves for player.
     * player - Id of player to get attack moves for.
     * ret - Vector of Intent.FireChecker
     */
    pub fn checker_fires_for(&self, player: i32) -> Vec<Intent> {
        let other_player = match player {
            PLAYER_A_ID => PLAYER_B_ID,
            _ => PLAYER_B_ID
        };

        let mut moves: Vec<Intent> = Vec::new();
        let other_checkers = self.board.checkers_for_player(other_player);
        for checker_pos in other_checkers.iter() {
            if let Ok(_num) = self.board.can_fire_checker_at(*checker_pos) {
                moves.push(Intent::FireChecker(*checker_pos));
            }
        }
        moves
    }

    /**
     * stone_places_for
     * Get all legal moves where a stone can be placed for the player.
     * player - Id of player to get stone place moves for.
     * ret - Vector of Intent.PlaceStone
     */
    pub fn stone_places_for(&self, _player: i32) -> Vec<Intent> {
        let mut moves: Vec<Intent> = Vec::new();
        for stone_pos in self.valid_stone_places().iter() {
            moves.push(Intent::PlaceStone(*stone_pos));
        }
        moves
    }
    
    /**
     * stone_slides_for
     * Get all legal moves where a stone is slid for the player.
     * player - Id of player to get stone slide moves for.
     * ret - Vector of Intent.SlideStone.
     */
    pub fn stone_slides_for(&self, player: i32) -> Vec<Intent> {
        let mut moves: Vec<Intent> = Vec::new();
        let stone_positions = self.board.stones_for_player(player);
        for stone_position in stone_positions.iter() {
            for direction in self.empty_stone_n_at(*stone_position).iter() {
                moves.push(Intent::SlideStone(*stone_position, *direction));
            }
        }
        moves
    }

    /**
     * apply_move
     * Apply the move to the game state, using current player as the player executing the move.
     * intent - Intent specifying action to be taken.
     */
    pub fn apply_move(&mut self, current_player: i32, intent: Intent) {
        match intent {
            Intent::FireChecker(position) => {
                self.board
                    .fire_checker_at(position)
                    .unwrap();
            },
            Intent::MoveChecker(from, to) => {
                self.board
                    .move_checker(from, to)
                    .unwrap();
            },
            Intent::PlaceStone(at) => {
                self.board
                    .place_stone_at(at, Stone::new(current_player))
                    .unwrap();
            },
            Intent::SlideStone(from, direction) => {
                self.board
                    .slide_stone(from, direction)
                    .unwrap();
            }
        }
    }

    /* 
     * Helper function returning if a non-straight line of stones 
     * of the same color proceeds from one side of the board to another.
     * Returns reference to winner or none.
     */
    fn check_for_stone_win(&self) -> Option<i32> {
        // check player A stones first
        // start at top row of the board, work down
        for player_id in [PLAYER_A_ID, PLAYER_B_ID] {
            let mut visited: Vec<Vec2> = Vec::new();
            let mut frontier: Vec<Vec2> = Vec::new();
            for xi in 0..=BOARD_WIDTH as i32 {
                let position = Vec2::new(xi, 0);
                let stone = self.board.stone_at(position).unwrap();
                if stone.owner == player_id {
                    frontier.push(position);
                }
            }
            // While the frontier is not empty
            while let Some(position) = frontier.pop() {
                // If we reached the other side of the board, then this player has won.
                if position.y == BOARD_HEIGHT as i32 {
                    return Some(player_id);
                }
                visited.push(position);
                // Get all neighbouring squares
                for neighbour in Board::stone_neighbours(position) {
                    let stone = self.board.stone_at(neighbour).unwrap();
                    // Add to frontier if not visited and stones are owned by current player
                    if !visited.contains(&neighbour) && stone.owner == player_id {
                        frontier.push(neighbour);
                    }
                }
            }
        }
        None
    }
    /*
     * Helper function returing if a checker has reached the opposite side of
     * the board it started on.
     * Returns reference to winner or none.
     */
    fn check_for_checker_win(&self) -> Option<i32> {
        for yi in 0..BOARD_HEIGHT as i32 {
             // check if any player B checkers in column 7
            let position_b = Vec2::new(BOARD_WIDTH as i32, yi);
            if self.board.checker_at(position_b).unwrap().owner == PLAYER_B_ID {
                return Some(PLAYER_B_ID);
            }
            // check if any player A checkers are in column 0
            let position_a = Vec2::new(0, yi);
            if self.board.checker_at(position_a).unwrap().owner == PLAYER_A_ID {
                return Some(PLAYER_A_ID);
            }
        }
        None
    }
    /*
     * Helper function returning winner if law of circularity (circular slide move)
     * has been violated.
     * Returns reference to winner or none.
     */
    fn check_for_circularity_win(&self) -> Option<i32> {
        if let Some(Intent::SlideStone(from1, dir1)) = self.last_two_slides_a[0] {
            if let Some(Intent::SlideStone(_, dir2)) = self.last_two_slides_a[1] {
                let mid_position = self.board.slide_stone_result(from1, dir1).unwrap();
                let end_position = self.board.slide_stone_result(mid_position, dir2).unwrap();
                if from1 == end_position {
                    return Some(PLAYER_B_ID);
                }
            }
        }
        if let Some(Intent::SlideStone(from1, dir1)) = self.last_two_slides_b[0] {
            if let Some(Intent::SlideStone(_, dir2)) = self.last_two_slides_b[1] {
                let mid_position = self.board.slide_stone_result(from1, dir1).unwrap();
                let end_position = self.board.slide_stone_result(mid_position, dir2).unwrap();
                if from1 == end_position {
                    return Some(PLAYER_A_ID);
                }
            }
        }
        None
    }

    /*
     * Helper function returning empty neighbour positions around a checker position.
     * Returns an array of Vec2.
     */
    fn empty_checker_n_at(&self, pos: Vec2) -> Vec<Vec2> {
        let mut empty_neighbours: Vec<Vec2> = Vec::new();
        for npos in Board::checker_neighbours(pos).iter() {
            if self.board.checker_at(*npos).unwrap().owner == EMPTY_PLAYER_ID {
                empty_neighbours.push(pos);
            }
        }
        empty_neighbours
    }
    /*
     * Helper function returning empty stone directions around a stone position.
     * Returns an array of Direction.
     */
    fn empty_stone_n_at(&self, pos: Vec2) -> Vec<Direction> {
        let mut empty_directions: Vec<Direction> = Vec::new();
        let directions = [
            Direction::Up, Direction::Down, Direction::Left, Direction::Right
        ];
        for dir in directions.iter() {
            let npos = pos + dir.as_vec();
            if let Ok(stone) = self.board.stone_at(npos) {
                if stone.owner == EMPTY_PLAYER_ID {
                    empty_directions.push(*dir);
                }
            }
        }
        empty_directions
    }
    /*
     * Helper function returning valid stone placement positions (empty and not bordering
     * a square with a checker).
     * Returns an array of Vec2 
     */
    fn valid_stone_places(&self) -> Vec<Vec2> {
        let mut valid_pos : Vec<Vec2> = Vec::new();
        for pos in self.board.empty_stones().iter() {
            let mut is_valid = true;
            for cpos in Board::checker_neigbours_of_stone(*pos).iter() {
                if self.board.stone_at(*cpos).unwrap().owner != EMPTY_PLAYER_ID {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                valid_pos.push(*pos);
            }
        }
        valid_pos
    }
}

pub struct Player<'a> {
    pub id: i32,
    pub stones: i32,
    pub decider: &'a dyn Decide,
    pub max_stones: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    MoveChecker(Vec2, Vec2),
    FireChecker(Vec2),
    PlaceStone(Vec2),
    SlideStone(Vec2, Direction)
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

impl ConsolePlayer {
    pub fn new() -> ConsolePlayer {
        ConsolePlayer{}
    }
}

impl Decide for ConsolePlayer {
    fn choose_move(
        &self, move_checkers: Vec<Intent>, fire_checkers: Vec<Intent>, 
        place_stones: Vec<Intent>, slide_stones: Vec<Intent>
    ) -> Intent {
        Intent::PlaceStone(Vec2::new(0, 0))
    }
}

impl<'a> Player<'a> {
    /**
     * new - Create a new instance.
     * nstones - Number of stones the player has.
     * decide - Object that decides what moves to take.
     */
    pub fn new(_id: i32, nstones: i32, decide: &'a dyn Decide) -> Player<'a> {
        Player {
            id: _id,
            stones: nstones,
            decider: decide,
            max_stones: nstones
        }
    }

    /** 
     * get_stone - Remove a stone from the pile.
     * ret - Some if there are stones left, otherwise None.
     */
    pub fn get_stone(&mut self) -> Option<Stone> {
        if self.stones <= 0 {
            None
        } else {
            self.stones -= 1;
            Some(Stone::new(self.id))
        }
    }

    /**
     * reset player to its initial state, with max number of stonse in pile.
     */
    pub fn reset(&mut self) {
        self.stones = self.max_stones;
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

/**
 * PlayerFactory constructs different instances of the player.
 */
pub struct PlayerFactory;

impl<'a> PlayerFactory {
    pub fn console_player(id: i32, nstones: i32) -> Player<'a> {
        Player::new(id, nstones, &ConsolePlayer{})
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

    #[test]
    pub fn reset() {

    }

    mod player {
        #[test]
        pub fn reset() {

        }

        #[test]
        pub fn get_stone() {

        }
    }
}