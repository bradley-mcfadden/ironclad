/**
 * Main class that enforces the rules of the game, collects player
 * input, and applies moves to the board.
 */

use std::io::{self, Write};
use std::fmt::{
    Display,
    Formatter,
};
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
    pub board: Board,
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
        for i in 0..2 {
            self.last_two_slides_a[i] = None;
            self.last_two_slides_b[i] = None;
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
                println!("\n{}", self.board);
                let player_id = self.players[p_num].id;
                let move_checkers = self.checker_moves_for(player_id);
                let fire_checkers = self.checker_fires_for(player_id);
                let place_stones = self.stone_places_for(player_id);
                let slide_stones = self.stone_slides_for(player_id);
                let chosen_move = self.players[p_num].choose_move(
                    move_checkers, fire_checkers, place_stones, slide_stones
                );
                println!("Player {} chose to {}", player_id, chosen_move);
                self.apply_move(player_id, chosen_move);
                let win_state = self.check_for_win();
                if win_state.is_some() {
                    return win_state.unwrap();
                }
            }
        }
    }

    /**
     * check_for_win
     * Determine if the board is in a winning state, returning the winning player or None.
     * @ret - Winning player or none.
     */
    pub fn check_for_win(&mut self) -> Option<i32> {
        let checks = [
            self.check_for_checker_win(),
            self.check_for_circularity_win(),
            self.check_for_stone_win(),
        ];
        for check in checks {
            if let Some(winner) = check {
                if winner == PLAYER_A_ID {
                    return Some(PLAYER_A_ID);
                } else if winner == PLAYER_B_ID {
                    return Some(PLAYER_B_ID);
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
            _ => PLAYER_A_ID
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
                match current_player {
                    PLAYER_A_ID => self.players[0].get_stone(),
                    PLAYER_B_ID => self.players[1].get_stone(),
                    _ => None
                };
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
        match current_player {
            PLAYER_A_ID => {
                if self.last_two_slides_a.len() == 2 {
                    self.last_two_slides_a.swap(0, 1);
                    self.last_two_slides_a[1] = None;
                }
                for i in 0..2 {
                    if let None = self.last_two_slides_a[i] {
                        self.last_two_slides_a[i] =  Some(intent);
                    }
                }

            },
            PLAYER_B_ID => {
                if self.last_two_slides_b.len() == 2 {
                    self.last_two_slides_b.swap(0, 1);
                    self.last_two_slides_b[1] = None;
                }
                for i in 0..2 {
                    if let None = self.last_two_slides_b[i] {
                        self.last_two_slides_b[i] =  Some(intent);
                    }
                }
            },
            _ => ()
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
            let position_a = Vec2::new(0, yi);
            let checker_at_a = self.board.checker_at(position_a).unwrap().owner;
            if checker_at_a == PLAYER_A_ID {
                return Some(PLAYER_A_ID);
            }
            // check if any player A checkers are in column 0
            let position_b = Vec2::new(BOARD_WIDTH as i32 - 1, yi);
            let checker_at_b = self.board.checker_at(position_b).unwrap().owner;
            if checker_at_b == PLAYER_B_ID {
                return Some(PLAYER_B_ID);
            }
        }
        None
    }

    /*
     * Helper function returning winner if law of circularity (circular slide move)
     * has been violated.
     * Returns reference to winner or none.
     */
    fn check_for_circularity_win(&mut self) -> Option<i32> {
        if let Some(Intent::SlideStone(from1, dir1)) = self.last_two_slides_a[0] {
            if let Some(Intent::SlideStone(_, dir2)) = self.last_two_slides_a[1] {
                let mid_position = self.board.slide_stone(from1, dir1).unwrap();
                let end_position = self.board.slide_stone(mid_position, dir2).unwrap();
                if from1 == end_position {
                    return Some(PLAYER_B_ID);
                }
                
                if let Ok(mid_position) = self.board.slide_stone(end_position, dir2.inverse()) {
                    self.board.slide_stone(mid_position, dir1.inverse()).unwrap();
                }
            }
        }
        if let Some(Intent::SlideStone(from1, dir1)) = self.last_two_slides_b[0] {
            if let Some(Intent::SlideStone(_, dir2)) = self.last_two_slides_b[1] {
                let mid_position = self.board.slide_stone(from1, dir1).unwrap();
                let end_position = self.board.slide_stone(mid_position, dir2).unwrap();
                if from1 == end_position {
                    return Some(PLAYER_A_ID);
                }

                if let Ok(mid_position) = self.board.slide_stone(end_position, dir2.inverse()) {
                    self.board.slide_stone(mid_position, dir1.inverse()).unwrap();
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
                empty_neighbours.push(*npos);
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
                if self.board.checker_at(*cpos).unwrap().owner != EMPTY_PLAYER_ID {
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

impl Display for Intent {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Intent::MoveChecker(from, to) => write!(formatter, "MoveChecker from {} to {}", from, to),
            Intent::FireChecker(at) => write!(formatter, "FireChecker at {}", at),
            Intent::PlaceStone(at) => write!(formatter, "PlaceStone at {}", at),
            Intent::SlideStone(from, direction) => write!(formatter, "SlideStone from {} toward {}", from, direction)
        }
    }
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

        let chosen_move: Option<Intent> = None;
        while let None = chosen_move {
            print!("\nWhat would you like to do? (Type your choice, then press ENTER)\n");
            println!("M - Move checker");
            println!("A - Attack checker");
            println!("P - Place stone");
            println!("S - Slide stone");

            print!("Enter a letter: ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            while let Err(_) = io::stdin().read_line(&mut line){
                print!("Enter a letter: ");
            }
            
            let choice = line.chars().collect::<Vec<char>>()[0];
            match choice {
                'M' => {
                    let mut idx = 0;
                    for move_checker in move_checkers.iter() {
                        if let Intent::MoveChecker(from, to) = move_checker {
                            println!("{idx} - move checker {from} to {to}");
                        }
                        idx += 1;
                    }

                    let mut line = String::new();
                    loop {
                        print!("Enter the number of your choice: ");
                        io::stdout().flush().unwrap();
                        while let Err(_) = io::stdin().read_line(&mut line) {}
                        if let Ok(idx) = line.trim().parse::<usize>() { 
                            if idx < move_checkers.len() {
                                return move_checkers[idx];
                            }
                        }
                    }

                },
                'A' => {
                    let mut idx = 0;
                    for fire_checker in fire_checkers.iter() {
                        if let Intent::FireChecker(at) = fire_checker {
                            println!("{idx} - attack checker at {at}");
                        }
                        idx += 1;
                    }

                    let mut line = String::new();
                    loop {
                        print!("Enter the number of your choice: ");
                        io::stdout().flush().unwrap();
                        while let Err(_) = io::stdin().read_line(&mut line) {}
                        if let Ok(idx) = line.trim().parse::<usize>() { 
                            if idx < fire_checkers.len() {
                                return fire_checkers[idx];
                            }
                        }
                    }

                },
                'P' => {
                    let mut idx = 0;
                    for place_stone in place_stones.iter() {
                        if let Intent::PlaceStone(at) = place_stone {
                            println!("{idx} - place stone at {at}");
                        }
                        idx += 1;
                    }

                    let mut line = String::new();
                    loop {
                        print!("Enter the number of your choice: ");
                        io::stdout().flush().unwrap();
                        while let Err(_) = io::stdin().read_line(&mut line) {}
                        if let Ok(idx) = line.trim().parse::<usize>() { 
                            if idx < place_stones.len() {
                                return place_stones[idx];
                            }
                        }
                    }

                },
                'S' => {
                    let mut idx = 0;
                    for slide_stone in slide_stones.iter() {
                        if let Intent::SlideStone(from, dir) = slide_stone {
                            println!("{idx} - slide stone from {from} {dir}");
                        }
                        idx += 1;
                    }

                    let mut line = String::new();
                    loop {
                        print!("Enter the number of your choice: ");
                        io::stdout().flush().unwrap();
                        while let Err(_) = io::stdin().read_line(&mut line) {}
                        if let Ok(idx) = line.trim().parse::<usize>() { 
                            if idx < slide_stones.len() {
                                return slide_stones[idx];
                            }
                        }
                    }
                },
                _ => {
                    continue;
                },
            }
        } 



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
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let mut game = Game::new(&mut player_a, &mut player_b);
        // Stones wins
        // -- normal case, straight line down the middle of the board
        for player_id in [PLAYER_A_ID, PLAYER_B_ID] {
            for yi in 0..=(BOARD_HEIGHT as i32) {
                let place_position = Vec2::new(4, yi);
                game.apply_move(player_id, Intent::PlaceStone(place_position));
            }
            assert_eq!(game.check_for_win(), Some(player_id));
            game.reset();
        }

        // Checker wins
        game.reset();
        assert_eq!(game.check_for_win(), None);

        game.reset();
        game.apply_move(PLAYER_A_ID, Intent::MoveChecker(Vec2::new(7, 1), Vec2::new(7, 0)));
        for xi in (1..=7).rev() {
            let from_position = Vec2::new(xi, 0);
            let to_position = Vec2::new(xi - 1, 0);
            game.apply_move(PLAYER_A_ID, Intent::MoveChecker(from_position, to_position));
        }

        assert_eq!(game.check_for_win(), Some(PLAYER_A_ID));

        game.reset();
        game.apply_move(PLAYER_B_ID, Intent::MoveChecker(Vec2::new(0, 1), Vec2::new(0, 0)));
        for xi in 0..=6 {
            let from_position = Vec2::new(xi, 0);
            let to_position = Vec2::new(xi + 1, 0);
            game.apply_move(PLAYER_B_ID, Intent::MoveChecker(from_position, to_position));
        }
        assert_eq!(game.check_for_win(), Some(PLAYER_B_ID));

        // Circularity wins
        game.reset();
        let from_position = Vec2::new(4, 0);
        let to_position = Vec2::new(4, BOARD_HEIGHT as i32);
        game.apply_move(PLAYER_A_ID, Intent::PlaceStone(from_position));
        game.apply_move(PLAYER_A_ID, Intent::SlideStone(from_position, Direction::Down));
        game.apply_move(PLAYER_A_ID, Intent::SlideStone(to_position, Direction::Up));
        assert_eq!(game.check_for_win(), Some(PLAYER_B_ID));

        game.reset();
        game.apply_move(PLAYER_B_ID, Intent::PlaceStone(from_position));
        game.apply_move(PLAYER_B_ID, Intent::SlideStone(from_position, Direction::Down));
        game.apply_move(PLAYER_B_ID, Intent::SlideStone(to_position, Direction::Up));
        assert_eq!(game.check_for_win(), Some(PLAYER_A_ID));
    }

    #[test]
    pub fn checker_moves_for() {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let game = Game::new(&mut player_a, &mut player_b);

        {
            let expected_a_moves = vec![
                Intent::MoveChecker(Vec2::new(7, 1), Vec2::new(7, 0)), Intent::MoveChecker(Vec2::new(7, 1), Vec2::new(6, 1)),
                Intent::MoveChecker(Vec2::new(7, 1), Vec2::new(6, 0)), Intent::MoveChecker(Vec2::new(7, 2), Vec2::new(6, 1)),
                Intent::MoveChecker(Vec2::new(7, 3), Vec2::new(6, 4)), Intent::MoveChecker(Vec2::new(7, 4), Vec2::new(7, 5)), 
                Intent::MoveChecker(Vec2::new(7, 4), Vec2::new(6, 4)), Intent::MoveChecker(Vec2::new(7, 4), Vec2::new(6, 5)),
                Intent::MoveChecker(Vec2::new(6, 2), Vec2::new(6, 1)), Intent::MoveChecker(Vec2::new(6, 2), Vec2::new(5, 2)),
                Intent::MoveChecker(Vec2::new(6, 2), Vec2::new(5, 1)), Intent::MoveChecker(Vec2::new(6, 2), Vec2::new(5, 3)),
                Intent::MoveChecker(Vec2::new(6, 3), Vec2::new(6, 4)), Intent::MoveChecker(Vec2::new(6, 3), Vec2::new(5, 2)),
                Intent::MoveChecker(Vec2::new(6, 3), Vec2::new(5, 4)), Intent::MoveChecker(Vec2::new(6, 3), Vec2::new(5, 3)),
            ];
            let actual_a_moves = game.checker_moves_for(PLAYER_A_ID);

            for move_a in actual_a_moves.iter() {
                println!("{move_a}");
            }
            
            for move_a in expected_a_moves.iter() {
                // println!("{move_a}");
                assert!(actual_a_moves.contains(&move_a));
            }
        }
        {
            let expected_b_moves = vec![
                Intent::MoveChecker(Vec2::new(0, 1), Vec2::new(0, 0)), Intent::MoveChecker(Vec2::new(0, 1), Vec2::new(1, 1)),
                Intent::MoveChecker(Vec2::new(0, 1), Vec2::new(1, 0)), Intent::MoveChecker(Vec2::new(0, 2), Vec2::new(1, 1)),
                Intent::MoveChecker(Vec2::new(0, 3), Vec2::new(1, 4)), Intent::MoveChecker(Vec2::new(0, 4), Vec2::new(0, 5)), 
                Intent::MoveChecker(Vec2::new(0, 4), Vec2::new(1, 4)), Intent::MoveChecker(Vec2::new(0, 4), Vec2::new(1, 5)),
                Intent::MoveChecker(Vec2::new(1, 2), Vec2::new(1, 1)), Intent::MoveChecker(Vec2::new(1, 2), Vec2::new(2, 2)),
                Intent::MoveChecker(Vec2::new(1, 2), Vec2::new(2, 1)), Intent::MoveChecker(Vec2::new(1, 2), Vec2::new(2, 3)),
                Intent::MoveChecker(Vec2::new(1, 3), Vec2::new(1, 4)), Intent::MoveChecker(Vec2::new(1, 3), Vec2::new(2, 3)),
                Intent::MoveChecker(Vec2::new(1, 3), Vec2::new(2, 4)), Intent::MoveChecker(Vec2::new(1, 3), Vec2::new(2, 2)),
            ];
            let actual_b_moves = game.checker_moves_for(PLAYER_B_ID);
            for move_b in expected_b_moves.iter() {
                println!("{move_b}");
                assert!(actual_b_moves.contains(&move_b));
            }
        }
    }

    #[test]
    pub fn checker_fires_for()  {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let mut game = Game::new(&mut player_a, &mut player_b);

        {
            let fireable_positions = vec![Vec2::new(5, 2), Vec2::new(5, 3), Vec2::new(5, 5)];
            for pos in fireable_positions.iter() {
                game.board.place_checker_at(*pos, Checker::new(1, PLAYER_B_ID)).unwrap();
            }

            let expected = fireable_positions.iter().map(|pos| Intent::FireChecker(*pos)).collect::<Vec<Intent>>();
            let actual = game.checker_fires_for(PLAYER_A_ID);

            println!("Actual moves for Player A:");
            for move_a in actual.iter() {
                println!("{move_a}");
            }
            
            for move_a in expected.iter() {
                assert!(actual.contains(move_a)); 
            }
        }
        game.reset();
        {
            let fireable_positions = vec![Vec2::new(3, 2), Vec2::new(3, 3), Vec2::new(3, 5)];
            for pos in fireable_positions.iter() {
                game.board.place_checker_at(*pos, Checker::new(1, PLAYER_A_ID)).unwrap();
            }

            let expected = fireable_positions.iter().map(|pos| Intent::FireChecker(*pos)).collect::<Vec<Intent>>();
            let actual = game.checker_fires_for(PLAYER_B_ID);
            
            println!("Actual moves for Player B:");
            for move_b in actual.iter() {
                println!("{move_b}");
            }

            for move_b in expected.iter() {
                assert!(actual.contains(move_b)); 
            }
        }
    }

    #[test]
    pub fn stone_places_for() {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let game = Game::new(&mut player_a, &mut player_b);
        
        for player in vec![PLAYER_A_ID, PLAYER_B_ID] {
            // The expected number is 37, because the 6 checkers on each side border 2*13 unique squares, and 63 - 26 = 37
            assert_eq!(game.stone_places_for(player).len(), 37);
        }
    }
    
    #[test]
    pub fn stone_slides_for() {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let mut game = Game::new(&mut player_a, &mut player_b);

        let stone_location = Vec2::new(0, 0);
        game.apply_move(PLAYER_A_ID, Intent::PlaceStone(stone_location));

        let expected = vec![
            Intent::SlideStone(stone_location, Direction::Down), Intent::SlideStone(stone_location, Direction::Right),    
        ];
        let actual = game.stone_slides_for(PLAYER_A_ID);
        for move_actual in actual.iter() {
            println!("{move_actual}");
        }
        for stone_slide in expected.iter() {
            assert!(actual.contains(stone_slide));
        }
    }

    #[test]
    pub fn apply_move() {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);

        // Moving a checker
        let mut game = Game::new(&mut player_a, &mut player_b);

        let from_position = Vec2::new(7, 1);
        let to_position = Vec2::new(7, 0);
        
        println!("{}", &game.board);
        game.apply_move(PLAYER_A_ID, Intent::MoveChecker(from_position, to_position)); 
        println!("{}", &game.board);

        assert_eq!(game.board.checker_at(to_position).unwrap().owner, PLAYER_A_ID);
        assert_eq!(game.board.checker_at(from_position).unwrap().owner, EMPTY_PLAYER_ID);
        
        // Placing a stone
        game.reset();
        let stone_position = Vec2::new(4, 4);
        game.apply_move(PLAYER_A_ID, Intent::PlaceStone(stone_position));
        assert_eq!(game.board.stone_at(stone_position).unwrap().owner, PLAYER_A_ID);
        assert_eq!(game.players[0].stones, STARTING_STONES - 1);

        // Sliding a stone
        game.apply_move(PLAYER_A_ID, Intent::SlideStone(stone_position, Direction::Up));
        assert_eq!(game.board.stone_at(stone_position).unwrap().owner, EMPTY_PLAYER_ID);
        assert_eq!(game.board.stone_at(Vec2::new(4, 0)).unwrap().owner, PLAYER_A_ID);

        // Firing at a checker
        let fire_position = Vec2::new(2, 2);
        game.board.place_checker_at(fire_position, Checker::new(1, PLAYER_A_ID)).unwrap();
        game.apply_move(PLAYER_B_ID, Intent::FireChecker(fire_position));
        assert_eq!(game.board.checker_at(fire_position).unwrap().owner, EMPTY_PLAYER_ID);
    }

    #[test]
    pub fn reset() {
        let mut player_a = PlayerFactory::console_player(PLAYER_A_ID, STARTING_STONES);
        let mut player_b = PlayerFactory::console_player(PLAYER_B_ID, STARTING_STONES);
        let mut game = Game::new(&mut player_a, &mut player_b);

        // Place some stones for a, b
        game.apply_move(PLAYER_A_ID, Intent::PlaceStone(Vec2::new(4, 4)));
        game.apply_move(PLAYER_B_ID, Intent::PlaceStone(Vec2::new(4, 3)));
        // Place checkers for a, b
        game.apply_move(PLAYER_A_ID, Intent::MoveChecker(Vec2::new(7, 1), Vec2::new(7, 0)));
        game.apply_move(PLAYER_B_ID, Intent::MoveChecker(Vec2::new(0, 1), Vec2::new(0, 0)));
        // reset
        game.reset();
        // Assert that stone_counters reset, stones not on board
        assert_eq!(game.board.stone_at(Vec2::new(4, 4)).unwrap().owner, EMPTY_PLAYER_ID);
        assert_eq!(game.board.stone_at(Vec2::new(4, 3)).unwrap().owner, EMPTY_PLAYER_ID);
        assert_eq!(game.players[0].stones, STARTING_STONES);
        assert_eq!(game.players[1].stones, STARTING_STONES);

        // Assert that checkers not on board.
        assert_eq!(game.board.checker_at(Vec2::new(7, 0)).unwrap().owner, EMPTY_PLAYER_ID);
        assert_eq!(game.board.checker_at(Vec2::new(0, 0)).unwrap().owner, EMPTY_PLAYER_ID);

        // Assert that last moves are empty
        for i in 0..2 {
            assert_eq!(game.last_two_slides_a[i], None);
            assert_eq!(game.last_two_slides_b[i], None);
        }
    }

    mod player {
        use crate::game::PlayerFactory;

        #[test]
        pub fn reset() {
            // Number of stones should reset
            let mut player = PlayerFactory::console_player(2, 5);
            player.get_stone();
            player.get_stone();
            player.reset();
            assert_eq!(player.stones, 5);
        }

        #[test]
        pub fn get_stone() {
            let nstones = 5;
            let id = 2;
            let mut player = PlayerFactory::console_player(id, nstones);
            let mut stone_counter = 0;
            while let Some(stone) = player.get_stone() {
                stone_counter += 1;
                assert_eq!(stone.owner, id);
            }
            assert_eq!(player.stones, 0);
            assert_eq!(stone_counter, nstones);
            if let Some(_stone) = player.get_stone() {
                panic!("Expected None, got Some");
            }
            assert_eq!(player.stones, 0);
        }
    }
}