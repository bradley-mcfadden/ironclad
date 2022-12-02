use std::fmt::{Display, Error, Formatter};
use std::vec::{Vec};

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::game::{Checker, Stone, PLAYER_A_ID, PLAYER_B_ID, EMPTY_PLAYER_ID};
use crate::vec::{Vec2, UP, LEFT, RIGHT, DOWN};

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 6;

const PLAYER_A_CHECK: [char; 4] = ['.', 'A', 'B', 'C'];
const PLAYER_A_STONE: char = 'a';

const PLAYER_B_CHECK: [char; 4] = ['.', '1', '2', '3'];
const PLAYER_B_STONE: char = 'b';

const EMPTY_STONE: char = '.';
const EMPTY_CHECKER: char = '_';

#[derive(Clone, Debug)]
pub enum MoveError {
    // Thrown when move index is out of bounds.
    IndexError(String),
    // Thrown when trying to place a piece on an occupied square.
    OccupiedError,
    // Thrown when Rule of Negation is broken ie trying to place a stone on a square with checker.
    NegationError,
}

#[derive(Clone, Debug)]
pub enum FireError {
    // Move was out not a valid board index
    IndexError,
    // No attacker pieces are in range
    NoAttackersError,
}

#[derive(Clone, Debug)]
pub enum SlideError {
    // Thrown when move index is out of bounds.
    IndexError,
    // Thrown when slide is blocked
    BlockedError
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn as_vec(&self) -> Vec2 {
        match self {
            Direction::Up => crate::vec::UP,
            Direction::Down => crate::vec::DOWN,
            Direction::Left => crate::vec::LEFT,
            Direction::Right => crate::vec::RIGHT
        }
    }
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            MoveError::IndexError(msg) => write!(f, "{}", msg),
            MoveError::OccupiedError => write!(f, "{}", "Space is already occupied"),
            MoveError::NegationError => write!(f, "{}", "Cannot place a stone on a square with a checker.")
        }
    }
}

pub struct Board {
    checker_board: [Checker; BOARD_WIDTH * BOARD_HEIGHT],
    stone_board: [Stone; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)],
    rng: StdRng
}

impl Board {
    /**
     * new creates a new Board in its initial state.
     * @ret New board, in Ironclad's start state.
     */
    pub fn new() -> Board {
        let mut board = Board {
            checker_board: [Checker{height: 0, owner: EMPTY_PLAYER_ID}; BOARD_WIDTH * BOARD_HEIGHT],
            stone_board: [Stone{owner: EMPTY_PLAYER_ID}; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)],
            rng: StdRng::from_entropy() 
        };
        board.place_start_pieces();
        board
    }

    /**
     * Create a new Board with the given seed for the random number generator.
     * @seed Array of 32 u8's as a seed.
     */
    pub fn from_seed(seed: [u8; 32]) -> Board {
        let mut board = Board {
            checker_board: [Checker{height: 0, owner: EMPTY_PLAYER_ID}; BOARD_WIDTH * BOARD_HEIGHT],
            stone_board: [Stone{owner: EMPTY_PLAYER_ID}; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)],
            rng: StdRng::from_seed(seed) 
        };
        board.place_start_pieces();
        board
    }

    /**
     * reset the board to the game's initial state.
     */
    pub fn reset(&mut self) {
        self.clear_board();
        self.place_start_pieces()
    }

    fn place_start_pieces(&mut self) {
        self.place_checker_at(Vec2::new(6, 2), Checker::new(1, PLAYER_A_ID)).unwrap();
        self.place_checker_at(Vec2::new(6, 3), Checker::new(1, PLAYER_A_ID)).unwrap();
        self.place_checker_at(Vec2::new(7, 1), Checker::new(2, PLAYER_A_ID)).unwrap();
        self.place_checker_at(Vec2::new(7, 4), Checker::new(2, PLAYER_A_ID)).unwrap();
        self.place_checker_at(Vec2::new(7, 3), Checker::new(3, PLAYER_A_ID)).unwrap();
        self.place_checker_at(Vec2::new(7, 2), Checker::new(3, PLAYER_A_ID)).unwrap();

        self.place_checker_at(Vec2::new(1, 2), Checker::new(1, PLAYER_B_ID)).unwrap();
        self.place_checker_at(Vec2::new(1, 3), Checker::new(1, PLAYER_B_ID)).unwrap();
        self.place_checker_at(Vec2::new(0, 1), Checker::new(2, PLAYER_B_ID)).unwrap();
        self.place_checker_at(Vec2::new(0, 4), Checker::new(2, PLAYER_B_ID)).unwrap();
        self.place_checker_at(Vec2::new(0, 3), Checker::new(3, PLAYER_B_ID)).unwrap();
        self.place_checker_at(Vec2::new(0, 2), Checker::new(3, PLAYER_B_ID)).unwrap();
    }

    fn clear_board(&mut self) {
        self.checker_board.fill(Checker::new(0, EMPTY_PLAYER_ID));
        self.stone_board.fill(Stone::new(EMPTY_PLAYER_ID))
    }

    /**
     * move_checker from @from to @to, returning Ok if the move is accepted, and an error otherwise.
     * @from Position of checker to move.
     * @to Position to move checker to.
     * @ret Ok if move is legal, or a MoveError if something went wrong.
     */
    pub fn move_checker(&mut self, from: Vec2, to: Vec2) -> Result<(), MoveError> {
        for vec in vec![from, to].iter() {
            if !Board::is_checker_vec_valid(*vec) {
                return Err(MoveError::IndexError(String::from("{vec} is not a valid checker position")));
            }
        }
        let to_checker = self.checker_at_unsafe(to);
        if to_checker.owner != EMPTY_PLAYER_ID {
            return Err(MoveError::OccupiedError);
        }

        let to_idx = Board::vec_to_checker_idx(to);
        let from_idx = Board::vec_to_checker_idx(from);

        self.checker_board.swap(to_idx, from_idx);

        Ok(())
    }

    /**
     * slide_stone
     * Slide a stone in a Direction as many squares as possible.
     * @from Position of stone to slide.
     * @dir Direction to move stone in.
     * @ret Ok if slide is legal, or SlideError if something went wrong.
     */
    pub fn slide_stone(&mut self, from: Vec2, dir: Direction) -> Result<(), SlideError> {
        if !Board::is_stone_vec_valid(from) {
            return Err(SlideError::IndexError)
        }
        let target = from + dir.as_vec();
        if !Board::is_stone_vec_valid(target) {
            return Err(SlideError::BlockedError)
        }
        let target_idx = Board::vec_to_stone_idx(target);
        if self.stone_board[target_idx].owner != EMPTY_PLAYER_ID {
            return Err(SlideError::BlockedError)
        }
        let mut last_free_position = target;
        loop {
            let next_target = last_free_position + dir.as_vec();
            if !Board::is_stone_vec_valid(next_target) {
                break;
            }
            let idx = Board::vec_to_stone_idx(next_target);
            if self.stone_board[idx].owner != EMPTY_PLAYER_ID {
                break;
            }
            last_free_position = next_target;
        }
        let new_idx = Board::vec_to_stone_idx(last_free_position);
        let old_idx  = Board::vec_to_stone_idx(from);
        self.stone_board.swap(new_idx, old_idx);

        Ok(())
    }

    /**
     * TODO: test me
     * slide_stone_result
     * Get the result of sliding the stone from @from in @dir, or an error if the move is invalid.
     * @from Position of stone to slide.
     * @dir Direction to move stone in.
     * @ret Ok if slide is legal, with location stone ends at, or SlideError if something went wrong.
     */
    pub fn slide_stone_result(&self, from: Vec2, dir: Direction) -> Result<Vec2, SlideError> {
        if !Board::is_stone_vec_valid(from) {
            return Err(SlideError::IndexError)
        }
        let target = from + dir.as_vec();
        if !Board::is_stone_vec_valid(target) {
            return Err(SlideError::BlockedError)
        }
        let target_idx = Board::vec_to_stone_idx(target);
        if self.stone_board[target_idx].owner != EMPTY_PLAYER_ID {
            return Err(SlideError::BlockedError)
        }
        let mut last_free_position = target;
        loop {
            let next_target = last_free_position + dir.as_vec();
            if !Board::is_stone_vec_valid(next_target) {
                break;
            }
            let idx = Board::vec_to_stone_idx(next_target);
            if self.stone_board[idx].owner != EMPTY_PLAYER_ID {
                break;
            }
            last_free_position = next_target;
        }

        Ok(last_free_position)
    }

    /**
     * fire_checker_at
     * Player of id @player attacks the checker at @pos, with all possible pieces in range, or errors.
     * @player Player id 
     * @pos Square to attempt to attack.
     * @return Ok if no error, or one of the error types if something went wrong.
     */
    pub fn fire_checker_at(&mut self, pos: Vec2) -> Result<(), FireError> {
        if !Board::is_checker_vec_valid(pos) {
            return Err(FireError::IndexError);
        }
        let checker_idx = Board::vec_to_checker_idx(pos);
        let checker = self.checker_board[checker_idx];

        // Check neighbourhood for attackers
        let mut attackers = 0;
        let dirs = vec![UP, DOWN, LEFT, RIGHT, UP + LEFT, UP + RIGHT, DOWN + LEFT, DOWN + RIGHT];
        for dir in dirs.iter() {
            for scale_factor in 1..3 {
                let offset = dir.scale(scale_factor);
                let neighbour_pos = pos + offset;
                if !Board::is_checker_vec_valid(neighbour_pos) {
                    continue;
                }
                let neighbour_idx = Board::vec_to_checker_idx(neighbour_pos);
                let neigh = self.checker_board[neighbour_idx];
                if neigh.owner != checker.owner && neigh.owner != EMPTY_PLAYER_ID {
                    attackers += 1;
                }
            }
        }
        if attackers == 0 {
            return Err(FireError::NoAttackersError)
        }
        // Get terrain bonus
        let mut terrain_bonus = 0;
        for stone_pos in Board::stone_neighbours_of_checker(pos).iter() {
            let idx = Board::vec_to_stone_idx(*stone_pos);
            if let Some(s) = self.stone_board.get(idx) {
                if s.owner != EMPTY_PLAYER_ID {
                    terrain_bonus += 1;
                }
            }
        }
        // For each attack, roll a die
        let mut dmg = 0;
        for _ in 0..attackers {
            // If die > terrain bonus, checker takes 1 damage
            let roll = self.rng.next_u32() % 6 + 1;
            if roll >= terrain_bonus {
                dmg += 1;
            }
        }
        let new_height = checker.height.checked_sub(dmg).unwrap_or(0);
        if new_height == 0 {
            self.checker_board[checker_idx] = Checker::new(0, EMPTY_PLAYER_ID);
        } else {
            self.checker_board[checker_idx] = Checker::new(new_height, checker.owner);
        }
        Ok(())
    }

    /**
     * TODO: Test me
     * can_fire_checker_at
     * Determine if any attacking units are in range of @pos, of opposite id.
     * @player Player id 
     * @pos Square to attempt to attack.
     * @return Ok if and number of attackers, or one of the error types if something went wrong.
     */
    pub fn can_fire_checker_at(&self, pos: Vec2) -> Result<u32, FireError> {
        if !Board::is_checker_vec_valid(pos) {
            return Err(FireError::IndexError);
        }
        let checker_idx = Board::vec_to_checker_idx(pos);
        let checker = self.checker_board[checker_idx];

        // Check neighbourhood for attackers
        let mut attackers = 0;
        let dirs = vec![UP, DOWN, LEFT, RIGHT, UP + LEFT, UP + RIGHT, DOWN + LEFT, DOWN + RIGHT];
        for dir in dirs.iter() {
            for scale_factor in 1..3 {
                let offset = dir.scale(scale_factor);
                let neighbour_pos = pos + offset;
                if !Board::is_checker_vec_valid(neighbour_pos) {
                    continue;
                }
                let neighbour_idx = Board::vec_to_checker_idx(neighbour_pos);
                let neigh = self.checker_board[neighbour_idx];
                if neigh.owner != checker.owner && neigh.owner != EMPTY_PLAYER_ID {
                    attackers += 1;
                }
            }
        }
        if attackers == 0 {
            return Err(FireError::NoAttackersError)
        }
        Ok(attackers)
    }

    /**
     * place_checker_at places a checker at a position, or returns a MoveError if a rule is
     * violated.
     * 
     * @pos - Position to place the checker at, should be within [[0,0], [BOARD_WIDTH,BOARD_HEIGHT])
     * @checker - Checker to put at the position.
     * @return - Ok if checker was placed, or one of the MoveError types if the checker was not placed.
     */
    pub fn place_checker_at(&mut self, pos: Vec2, checker: Checker) -> Result<(), MoveError> {
        // Check index of move is ok
        if !Board::is_checker_vec_valid(pos) {
            return Err(MoveError::IndexError(String::from("{pos} not within 0,0 and {BOARD_WIDTH-1},{BOARD_HEIGHT-1}")))
        }
        let idx = Board::vec_to_checker_idx(pos);
        let current_piece = &self.checker_board[idx];

        // Do not allow to placing a non-empty piece in a non-empty slot
        if current_piece.owner != EMPTY_PLAYER_ID && checker.owner != EMPTY_PLAYER_ID {
            return Err(MoveError::OccupiedError);
        }
        self.checker_board[idx] = checker;
        Ok(())
    }

    /**
     * place_stone_at places a stone at a position, or returns a MoveError if a rule is broken.
     * @pos - Position to place the checker at, should be within [0, 0], [BOARD_WIDTH+1, BOARD_HEIGHT+1]
     * @stone - Stone to put at the position.
     * @return - Ok if checker was placed, or one of the MoveError types if the stone was not placed.
     */
    pub fn place_stone_at(&mut self, pos: Vec2, stone: Stone) -> Result<(), MoveError> {
        if !Board::is_stone_vec_valid(pos) {
            return Err(MoveError::IndexError(String::from("{pos} not within 0,0 and {BOARD_WIDTH},{BOARD_HEIGHT}")))  
        }
        let idx = Board::vec_to_stone_idx(pos);
        let current_piece: Stone = self.stone_board[idx];
        if current_piece.owner != EMPTY_PLAYER_ID {
            return Err(MoveError::OccupiedError);
        }
        // Check for rule of negation
        for neighbour in Board::checker_neigbours_of_stone(pos) {
            let checker = self.checker_at_unsafe(neighbour);
            if checker.owner != EMPTY_PLAYER_ID {
                return Err(MoveError::NegationError)
            }
        } 

        self.stone_board[idx] = stone;
        Ok(())
    }

    /**
     * checker_neighbours_of_stone
     * Returns a list of all valid checker neighbours of the stone at @pos.
     * Since stones surround check squares, for a vector pos, the possible
     * valid neighbours are {(pos), (pos.x - 1, pos.y), (pos.x, pos.y - 1), (pos.x - 1, pos.y - 1)}
     * Neighbours that do not represent a valid board position are filtered out.
     * 
     */
    pub fn checker_neigbours_of_stone(pos: Vec2) -> Vec<Vec2>  {
        let mut neighbours: Vec<Vec2> = Vec::new();
        let mut out: Vec<Vec2> = Vec::new();
        neighbours.push(pos.clone());
        neighbours.push(pos.left().up());
        neighbours.push(pos.left());
        neighbours.push(pos.up());

        for pos in neighbours.iter().rev() {
            if Board::is_checker_vec_valid(*pos) {
                out.push(*pos);
            }
        }

        out
    }

    /**
     * stone_neighbours_of_checker
     * Returns a list of all valid stone neighbours of the checker at @pos.
     * Since stones surround check squares, for a vector pos, the possible
     * valid neighbours are {(pos), (pos.x + 1, pos.y), (pos.x, pos.y + 1), (pos.x + 1, pos.y + 1)}
     * Neighbours that do not represent a valid board position are filtered out.
     */
    pub fn stone_neighbours_of_checker(pos: Vec2) -> Vec<Vec2> {
        let mut neighbours: Vec<Vec2> = Vec::new();
        let mut out: Vec<Vec2> = Vec::new();
        neighbours.push(pos.clone());
        neighbours.push(pos.right().down());
        neighbours.push(pos.right());
        neighbours.push(pos.down());

        for pos in neighbours.iter() {
            if Board::is_stone_vec_valid(*pos) {
                out.push(*pos);
            }
        }

        out
    }

    /**
     * checker_neighbours
     * Given the checker position @pos, return up to 8 neighbours of the square.
     */
    pub fn checker_neighbours(pos: Vec2) -> Vec<Vec2> {
        let mut neighbours: Vec<Vec2> = Vec::new();
        neighbours.push(pos.down().left());
        neighbours.push(pos.down().right());
        neighbours.push(pos.left());
        neighbours.push(pos.right());
        neighbours.push(pos.up().left());
        neighbours.push(pos.up().right());
        neighbours.push(pos.down());
        neighbours.push(pos.up());

        let mut out: Vec<Vec2> = Vec::new();
        for pos in neighbours.iter() {
            if Board::is_checker_vec_valid(*pos) {
                out.push(*pos)
            }
        }
        out
    }

    /**
     * TODO: Test me
     * stone_neighbours
     * Given the stone position @pos, return up to 4 neighbours of the square.
     */
    pub fn stone_neighbours(pos: Vec2) -> Vec<Vec2> {
        let mut neighbours: Vec<Vec2> = Vec::new();
        neighbours.push(pos.down());
        neighbours.push(pos.up());
        neighbours.push(pos.left());
        neighbours.push(pos.right());

        let mut out: Vec<Vec2> = Vec::new();
        for pos in neighbours.iter() {
            if Board::is_stone_vec_valid(*pos) {
                out.push(*pos)
            }
        }
        out
    } 
    
    /**
     * checker_at returns the Checker on the board at the provided position or an error.
     * @pos Vec2 instance that should be between [0, 0] and [BOARD_WIDTH - 1, BOARD_HEIGHT - 1].
     * @ret Ok containing the Checker, or an Err if position is not a valid checker index.
     */
    pub fn checker_at<'a>(&'a self, pos: Vec2) -> Result<&'a Checker, ()> {
        if !Board::is_checker_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_checker_idx(pos); 
            Ok(&self.checker_board[idx])
        }
    }

    pub fn mut_checker_at<'a>(&'a mut self, pos: Vec2) -> Result<&'a mut Checker, ()> {
        if !Board::is_checker_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_checker_idx(pos); 
            Ok(&mut self.checker_board[idx])
        }
    }

    /**
     * stone_at returns the Stone on the board at the provided position, or an error
     * if the position was not in range.
     * @pos Vec2 instance that should be between [0, 0] and [BOARD_WIDTH, BOARD_HEIGHT] inclusive.
     * @ret Ok containing the stone, or an Err if position is not a valid stone index. 
     */
    pub fn stone_at<'a>(&'a self, pos: Vec2) -> Result<&'a Stone, ()> {
        if !Board::is_stone_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_stone_idx(pos); 
            Ok(&self.stone_board[idx])
        }
    }

    pub fn mut_stone_at<'a>(&'a mut self, pos: Vec2) -> Result<&'a mut Stone, ()> {
        if !Board::is_stone_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_stone_idx(pos); 
            Ok(&mut self.stone_board[idx])
        }
    }

    // Use when @pos has already been bounds-checked
    fn checker_at_unsafe<'a>(&'a self, pos: Vec2) -> &'a Checker {
        let idx: usize = Board::vec_to_checker_idx(pos); 
        &self.checker_board[idx]
    }

    // Use when @pos has already been bounds-checked
    fn stone_at_unsafe<'a>(&'a self, pos: Vec2) -> &'a Stone {
        let idx: usize = Board::vec_to_stone_idx(pos); 
        &self.stone_board[idx]
    }

    /**
     * vec_to_stone_idx
     * @pos - Position to convert into an index.
     * @return - Index into inner stone array.
     */
    pub fn vec_to_stone_idx(pos: Vec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * (BOARD_WIDTH + 1)
    }

    fn is_checker_vec_valid(pos: Vec2) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x >= BOARD_WIDTH as i32 || pos.y >= BOARD_HEIGHT as i32)
    }

    fn is_stone_vec_valid(pos: Vec2) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x >= (BOARD_WIDTH + 1) as i32 || pos.y >= (BOARD_HEIGHT + 1) as i32)
    }

    /**
     * vec_to_checker_idex
     * @pos - Position to convert into an index.
     * @return - Index into inner checker array.
     */
    pub fn vec_to_checker_idx(pos: Vec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * BOARD_WIDTH
    }

    /**
     * stones_for_player returns the positions of all stones belonging
     * to @player.
     * @player Player id to match against stone.owner.
     * @ret Vec containing positions of stones owned by player.
     */
    pub fn stones_for_player(&self, player: i32) -> Vec<Vec2> {
        let mut stones: Vec<Vec2> = Vec::new();
        for x in 0 ..=BOARD_WIDTH {
            for y in 0..=BOARD_HEIGHT {
                let pos = Vec2::new(x as i32, y as i32);
                if self.stone_at_unsafe(pos).owner == player {
                    stones.push(pos);
                } 
            }
        }
        stones
    }

    /**
     * checkers_for_player returns the positions of all checkers belonging
     * to @player.
     * @player Player ID to match against checker.owner.
     * @ret Vec containing positions of checkers owned by player.
     */
    pub fn checkers_for_player(&self, player: i32) -> Vec<Vec2> {
        let mut checkers: Vec<Vec2> = Vec::new();
        for x in 0 .. BOARD_WIDTH {
            for y in 0 .. BOARD_HEIGHT {
                let pos = Vec2::new(x as i32, y as i32);
                if self.checker_at_unsafe(pos).owner == player {
                    checkers.push(pos);
                } 
            }
        }
        checkers
    }

    /**
     * empty_stones returns all possible stone positions that do not contain 
     * a stone.
     * @ret Vec containing positions on stone board not containing a stone.
     */
    pub fn empty_stones(&self) -> Vec<Vec2> {
        self.stones_for_player(EMPTY_PLAYER_ID)
    }

    /**
     * as_string
     * Stones and checker rows are printed interlaced.
     * ret - String representation of pieces on the board. 
     */
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        for yi in 0..=BOARD_HEIGHT as i32 {
            for xi in 0..=BOARD_WIDTH as i32 {
                let idx = Board::vec_to_stone_idx(Vec2::new(xi, yi));
                let stone = self.stone_board[idx];
                    let mut draw_char: char = EMPTY_STONE;
                    if stone.owner == PLAYER_A_ID {
                        draw_char = PLAYER_A_STONE;
                    } else if stone.owner == PLAYER_B_ID {
                        draw_char = PLAYER_B_STONE;
                    }
                    string.push(draw_char);
                    string.push(' ');
                    // print!("{} ", draw_char);
            }
            string.push_str("\n");
            if yi >= BOARD_HEIGHT as i32 { continue; }
            
            for xi in 0..BOARD_WIDTH as i32{
                let idx = Board::vec_to_checker_idx(Vec2::new(xi, yi));
                let checker = self.checker_board[idx];
                let mut draw_char: char = EMPTY_CHECKER;
                if checker.owner == PLAYER_A_ID {
                    draw_char = PLAYER_A_CHECK[checker.height as usize];
                } else if checker.owner == PLAYER_B_ID {
                    draw_char = PLAYER_B_CHECK[checker.height as usize];
                }
                // print!(" {}", draw_char);
                string.push(' ');
                string.push(draw_char);
            }
            string.push('\n');
        }
        string
    }
}

impl Display for Board {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(fmt, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Checker;

    #[test]
    fn vec_to_checker_idx() {
        assert_eq!(Board::vec_to_checker_idx(Vec2::new(1, 1)), 9);
        assert_eq!(Board::vec_to_checker_idx(Vec2::new(0, 1)), 8);
        assert_eq!(Board::vec_to_checker_idx(Vec2::new(1, 0)), 1);
        assert_eq!(Board::vec_to_checker_idx(Vec2::new(7, 5)), 47)
    }

    #[test]
    fn vec_to_stone_idx() {
        assert_eq!(Board::vec_to_stone_idx(Vec2::new(1, 1)), 10);
        assert_eq!(Board::vec_to_stone_idx(Vec2::new(0, 5)), 45);
        assert_eq!(Board::vec_to_stone_idx(Vec2::new(8, 6)), 62);
    }

    #[test]
    fn check_initial_board() {
        let board = Board::new();
        check_start_state(&board)
    }

    #[test]
    fn reset() {
        let mut board = Board::new();
        board.place_stone_at(Vec2::new(0, 0), Stone::new(PLAYER_A_ID)).unwrap();
        board.place_checker_at(Vec2::new(4, 4), Checker::new(2, PLAYER_A_ID)).unwrap();
        board.reset();
        check_start_state(&board)
    }

    fn check_start_state(board: &Board) {
        // Board should be free of stones
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let stone = board.stone_at(
                    Vec2::new(x as i32, y as i32)
                );
                assert_eq!(stone.unwrap().owner, EMPTY_PLAYER_ID)
            }
        }
        let empty_c = Checker::new(0, EMPTY_PLAYER_ID);
        // On the checkerboard,
        // [0, 1] and [0, 4] should have 2-stack player B pieces
        let b2 = Checker::new(2, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(0, 1)).unwrap(), b2);
        assert_eq!(*board.checker_at(Vec2::new(0, 4)).unwrap(), b2);
        // [0, 2] and [0, 3] should have 3-stack player B pieces
        let b3 = Checker::new(3, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(0, 2)).unwrap(), b3);
        assert_eq!(*board.checker_at(Vec2::new(0, 3)).unwrap(), b3);
        // [1, 2] and [1, 3] should have 1-stack player B pieces
        let b1 = Checker::new(1, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(1, 2)).unwrap(), b1);
        assert_eq!(*board.checker_at(Vec2::new(1, 3)).unwrap(), b1);
        // [7, 1] and [7, 4] should have 2-stack player A pieces
        let a2 = Checker::new(2, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(7, 1)).unwrap(), a2);
        assert_eq!(*board.checker_at(Vec2::new(7, 4)).unwrap(), a2);
        // [7, 2] and [7, 3] should have 3-stack player A pieces
        let a3 = Checker::new(3, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(7, 2)).unwrap(), a3);
        assert_eq!(*board.checker_at(Vec2::new(7, 3)).unwrap(), a3);
        // [6, 2] and [6, 3] should have 1-stack player A pieces
        let a1 = Checker::new(1, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(6, 2)).unwrap(), a1);
        assert_eq!(*board.checker_at(Vec2::new(6, 3)).unwrap(), a1);   
        
        // Row 0 should be empty
        for xoff in 0..8 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 0)).unwrap(), empty_c);
        }
        // 1,1 to 6,1 should be empty
        for xoff in 1..7 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 1)).unwrap(), empty_c);
        }
        // 2,2 to 5,2 should be empty
        for xoff in 2..6 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 2)).unwrap(), empty_c);
        }
        // 2,3 to 5,3 should be empty
        for xoff in 2..6 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 3)).unwrap(), empty_c);
        }
        // 1,4 to 6,4 should be empty
        for xoff in 1..7 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 4)).unwrap(), empty_c);
        }
        // Row 5 should be empty
        for xoff in 0..8 {
            assert_eq!(*board.checker_at(Vec2::new(xoff, 5)).unwrap(), empty_c);
        }
    }

    #[test]
    fn place_checker_at() {
        let c1 = Checker::new(1, PLAYER_A_ID);
        let mut board = Board::new();
        assert_eq!(board.place_checker_at(Vec2::new(1, 1), c1).unwrap(), ());
        match board.place_checker_at(Vec2::new(-1, -1), c1) {
            Err(MoveError::IndexError(_)) => (),
            Err(MoveError::NegationError) => panic!("Expected an IndexError, got a NegationError"),
            Err(MoveError::OccupiedError) => panic!("Expected an IndexError, got an OccupiedError"),
            Ok(()) => panic!("Expected an IndexError, no error")
        }
        match board.place_checker_at(Vec2::new(1, 1), c1) {
            Err(MoveError::OccupiedError) => (),
            Err(MoveError::NegationError) => panic!("Expected an OccupiedError, got a NegationError"),
            Err(MoveError::IndexError(_)) => panic!("Expected an OccupiedError, got an IndexError"),
            Ok(()) => panic!("Expected an OccupiedError, no error")
        }
        assert_eq!(*board.checker_at(Vec2::new(1, 1)).unwrap(), c1);
    }

    #[test]
    fn place_stone_at() {
        let c1 = Checker::new(1, PLAYER_A_ID);
        let s1 = Stone::new(PLAYER_A_ID);
        let s1_pos = Vec2::new(3, 3);
        let mut board = Board::new();
        assert_eq!(board.place_stone_at(s1_pos, s1).unwrap(), ());
        assert_eq!(*board.stone_at(s1_pos).unwrap(), s1);
        match board.place_stone_at(s1_pos, s1) {
            // occupied error
            Err(MoveError::OccupiedError) => (),
            Err(MoveError::NegationError) => panic!("Expected an OccupiedError, got a NegationError"),
            Err(MoveError::IndexError(_)) => panic!("Expected an OccupiedError, got an IndexError"),
            Ok(()) => panic!("Expected an OccupiedError, got no error")
        }
        board.place_checker_at(Vec2::new(4, 2), c1).unwrap();
        match board.place_stone_at(Vec2::new(4, 3), s1) {
            // expect negation error
            Err(MoveError::NegationError) => (),
            Err(MoveError::OccupiedError) => panic!("Expected a NegationError, got a OccupiedError"),
            Err(MoveError::IndexError(_)) => panic!("Expected a NegationError, got an IndexError"),
            Ok(()) => panic!("Expected an NegationError, got no error")
        }
        // index error
        match board.place_stone_at(Vec2::new(-1, -1), s1) {
            Err(MoveError::IndexError(_)) => (),
            Err(MoveError::OccupiedError) => panic!("Expected an IndexError, got an OccupiedError"),
            Err(MoveError::NegationError) => panic!("Expected an IndexError, got a NegationError"),
            Ok(()) => panic!("Expected an IndexError, got no error")
        }

    }
    
    #[test]
    fn checker_neighbours_of_stone() {
        let neighbours = Board::checker_neigbours_of_stone(Vec2::new(1, 1));
        assert!(neighbours.contains(&Vec2::new(0, 0)));
        assert!(neighbours.contains(&Vec2::new(0, 1)));
        assert!(neighbours.contains(&Vec2::new(1, 0)));
        assert!(neighbours.contains(&Vec2::new(1, 1)));

        // Top left corner
        let nw_neighbours = Board::checker_neigbours_of_stone(Vec2::new(0, 0));
        assert!(nw_neighbours.contains(&Vec2::new(0, 0)));

        let ne_neighbours = Board::checker_neigbours_of_stone(Vec2::new(BOARD_WIDTH as i32, 0));
        assert!(ne_neighbours.contains(&Vec2::new((BOARD_WIDTH - 1) as i32, 0)));

        let sw_neighbours = Board::checker_neigbours_of_stone(Vec2::new(0, BOARD_HEIGHT as i32));
        assert!(sw_neighbours.contains(&Vec2::new(0, (BOARD_HEIGHT - 1) as i32)));

        let se_neighbours = Board::checker_neigbours_of_stone(Vec2::new(BOARD_WIDTH as i32, BOARD_HEIGHT as i32));
        assert!(se_neighbours.contains(&Vec2::new((BOARD_WIDTH - 1) as i32, (BOARD_HEIGHT - 1) as i32)));

        // Edge
        let edge_neighbours = Board::checker_neigbours_of_stone(Vec2::new(0, 3));
        assert!(edge_neighbours.contains(&Vec2::new(0, 2)));
        assert!(edge_neighbours.contains(&Vec2::new(0, 3)));
    }

    #[test]
    fn stone_neighbours_of_checker() {
        let normal = Board::stone_neighbours_of_checker(Vec2::new(1, 1));
        assert!(normal.contains(&Vec2::new(1, 1)));
        assert!(normal.contains(&Vec2::new(1, 2)));
        assert!(normal.contains(&Vec2::new(2, 1)));
        assert!(normal.contains(&Vec2::new(2, 2)));

        // lower right corner
        let se_neighbours = Board::stone_neighbours_of_checker(Vec2::new((BOARD_WIDTH - 1) as i32, (BOARD_HEIGHT - 1) as i32));
        assert!(se_neighbours.contains(&Vec2::new((BOARD_WIDTH - 1) as i32, (BOARD_HEIGHT - 1) as i32)));
        assert!(se_neighbours.contains(&Vec2::new(BOARD_WIDTH as i32, (BOARD_HEIGHT - 1) as i32)));
        assert!(se_neighbours.contains(&Vec2::new((BOARD_WIDTH - 1) as i32, BOARD_HEIGHT as i32)));
        assert!(se_neighbours.contains(&Vec2::new(BOARD_WIDTH as i32, BOARD_HEIGHT as i32)));
    }

    #[test]
    fn checker_neighbours() {
        // Case 1: Middle of board, 8 neighbours
        let mid_v = Vec2::new(3, 3);
        let middle = Board::checker_neighbours(mid_v);
        assert!(middle.contains(&mid_v.left()));
        assert!(middle.contains(&mid_v.right()));
        assert!(middle.contains(&mid_v.left().up()));
        assert!(middle.contains(&mid_v.left().down()));
        assert!(middle.contains(&mid_v.right().up()));
        assert!(middle.contains(&mid_v.right().down()));
        assert!(middle.contains(&mid_v.up()));
        assert!(middle.contains(&mid_v.down()));
        // Case 2: Corner, 3 neighbours
        let cor_v = Vec2::new(0, 0);
        let corner = Board::checker_neighbours(cor_v);
        assert!(corner.contains(&cor_v.down()));
        assert!(corner.contains(&cor_v.right()));
        assert!(corner.contains(&cor_v.down().right()));
        // Case 3: Edge, 5 neighbours
        let edge_v = Vec2::new(0, 3);
        let edge = Board::checker_neighbours(edge_v);
        assert!(edge.contains(&edge_v.down()));
        assert!(edge.contains(&edge_v.down().right()));
        assert!(edge.contains(&edge_v.right()));
        assert!(edge.contains(&edge_v.up()));
        assert!(edge.contains(&edge_v.up().right()));
    }

    #[test]
    fn stone_neighbours() {
        // Case 1: Middle of board, 4 neighbours
        let middle_vector = Vec2::new(3, 3);
        let middle_neighbours = Board::stone_neighbours(middle_vector);
        assert!(middle_neighbours.contains(&middle_vector.up()));
        assert!(middle_neighbours.contains(&middle_vector.down()));
        assert!(middle_neighbours.contains(&middle_vector.left()));
        assert!(middle_neighbours.contains(&middle_vector.right()));
        // Case 2: Corner of board, 2 neighbours
        let corner_vector = Vec2::new(0, 0);
        let corner_neighbours = Board::stone_neighbours(corner_vector);
        assert!(corner_neighbours.contains(&corner_vector.down()));
        assert!(corner_neighbours.contains(&corner_vector.right()));
        // Case 3: Edge of board, 3 neighbours
        let edge_vector = Vec2::new(0, 3);
        let edge_neighbours = Board::stone_neighbours(edge_vector);
        assert!(edge_neighbours.contains(&edge_vector.down()));
        assert!(edge_neighbours.contains(&edge_vector.right()));
        assert!(edge_neighbours.contains(&edge_vector.up()));
    }

    #[test]
    fn stones_for_player() {
        // Create board, place some stones, verify that list contains all placed stones
        let mut board = Board::new();
        let positions = vec![Vec2::new(0, 0), Vec2::new(5, 5), Vec2::new(4, 2)];
        for pos in positions.iter() {
            board.place_stone_at(*pos, Stone::new(PLAYER_A_ID)).unwrap();
        }
        let stones = board.stones_for_player(PLAYER_A_ID);
        for pos in positions.iter() {
            assert!(stones.contains(pos))
        }
        assert!(stones.len() == positions.len())
    }

    #[test]
    fn checkers_for_player() {
        let board = Board::new();
        let checkers_a = board.checkers_for_player(PLAYER_B_ID);
        let positions = vec![
            Vec2::new(0, 1), Vec2::new(0, 2), Vec2::new(0, 3), 
            Vec2::new(0, 4), Vec2::new(1, 2), Vec2::new(1, 3)
        ];
        for pos in positions.iter() {
            assert!(checkers_a.contains(pos));
        }
        assert!(positions.len() == checkers_a.len())
    }

    #[test]
    fn empty_stones() {
        let board = Board::new();
        let stones = board.empty_stones();
        
        assert!(stones.len() == (BOARD_HEIGHT + 1) * (BOARD_WIDTH + 1));
        for x in 0..=BOARD_WIDTH as i32 {
            for y in 0..=BOARD_HEIGHT as i32 {
                let pos = Vec2::new(x, y);
                assert!(stones.contains(&pos));
            }
        }
    }

    #[test]
    fn slide_stone() {
        let mut board = Board::new();
        match board.slide_stone(Vec2::new(-1, -1), Direction::Up) {
            Err(SlideError::IndexError) => (),
            Err(SlideError::BlockedError) => panic!("Expected an IndexError, got a BlockedError"),
            Ok(_) => panic!("Expected an IndexError, got no error")
        }

        // board.place_stone_at(Vec2::new(2, 2), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(4, 2), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 1), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 3), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 2), Stone::new(PLAYER_B_ID)).unwrap();
        match board.slide_stone(Vec2::new(3, 2), Direction::Up) {
            Err(SlideError::BlockedError) => (),
            Err(SlideError::IndexError) => panic!("Expected a BlockedError, got an IndexError"),
            Ok(_) => panic!("Expected a BlockedError, got an IndexError")
        }

        // Normal case -- does it stop when hitting the edge of the board, and does it move?
        let board_edge_pos = Vec2::new(4, 2);
        board.slide_stone(board_edge_pos, Direction::Up).unwrap();
        // Previous position should be empty
        match board.stone_at(board_edge_pos) {
            Ok(stone) => assert_eq!(stone.owner, EMPTY_PLAYER_ID),
            Err(_) => panic!("Expected to slide stone at 2,2 upward, got error instead")
        }
        // Board edge position should be occupied
        match board.stone_at(Vec2::new(4, 0)) {
            Ok(stone) => assert_eq!(stone.owner, PLAYER_B_ID),
            Err(_) => panic!("Expected to slide stone at 4,2 upward, got error instead")
        }
        // Normal case -- does it stop when hitting another stone?
        let board_hit_pos = Vec2::new(4, 4);
        board.place_stone_at(board_hit_pos, Stone::new(PLAYER_B_ID)).unwrap();
        board.slide_stone(board_hit_pos, Direction::Up).unwrap();
        // (4, 4) should be empty
        assert_eq!(board.stone_at(board_hit_pos).unwrap().owner, EMPTY_PLAYER_ID);
        // Stone should slide to (4,1), because it is blocked by (4,0)
        assert_eq!(board.stone_at(Vec2::new(4, 1)).unwrap().owner, PLAYER_B_ID);

    }

    #[test]
    fn slide_stone_result() {
        let mut board = Board::new();
        match board.slide_stone_result(Vec2::new(-1, -1), Direction::Up) {
            Err(SlideError::IndexError) => (),
            Err(SlideError::BlockedError) => panic!("Expected an IndexError, got a BlockedError"),
            Ok(_) => panic!("Expected an IndexError, got no error")
        }

        // board.place_stone_at(Vec2::new(2, 2), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(4, 2), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 1), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 3), Stone::new(PLAYER_B_ID)).unwrap();
        board.place_stone_at(Vec2::new(3, 2), Stone::new(PLAYER_B_ID)).unwrap();
        match board.slide_stone_result(Vec2::new(3, 2), Direction::Up) {
            Err(SlideError::BlockedError) => (),
            Err(SlideError::IndexError) => panic!("Expected a BlockedError, got an IndexError"),
            Ok(_) => panic!("Expected a BlockedError, got an IndexError")
        }

        // Normal case -- does it stop when hitting the edge of the board, and does it move?
        let board_edge_pos = Vec2::new(4, 2);
        match board.slide_stone_result(board_edge_pos, Direction::Up) {
            Err(SlideError::BlockedError) => panic!("Expected to slide stone at 2,2 upward, got BlockedError instead"),
            Err(SlideError::IndexError) => panic!("Expected to slide stone at 2,2 upward, got IndexError instead"),
            Ok(pos) => assert_eq!(pos, Vec2::new(4, 0))
        }
        
        // Normal case -- does it stop when hitting another stone?
        let board_hit_pos = Vec2::new(4, 4);
        board.place_stone_at(board_hit_pos, Stone::new(PLAYER_B_ID)).unwrap();
        match board.slide_stone_result(board_hit_pos, Direction::Up) {
            Err(SlideError::BlockedError) => panic!("Expected to slide stone at 2,2 upward, got BlockedError instead"),
            Err(SlideError::IndexError) => panic!("Expected to slide stone at 2,2 upward, got IndexError instead"),
            // Stone should slide to (4,3), because it is blocked by (4,2)
            Ok(pos) => assert_eq!(pos, Vec2::new(4, 3))
        }
    }

    #[test]
    fn can_fire_checker_at() {
        // Seed me
        let mut board = Board::new();
        // Normal case, checker takes damage and dies
        board.place_checker_at(Vec2::new(5, 2), Checker::new(3, PLAYER_B_ID)).unwrap();
        match board.can_fire_checker_at(Vec2::new(5, 2)) {
            Ok(num_attackers) => assert_eq!(num_attackers, 4),
            Err(_) => panic!("Got error when expecting no error")
        }
        // IndexError case
        match board.can_fire_checker_at(Vec2::new(-1, -1)) {
            Ok(_) => panic!("Expected an IndexError, got no error"),
            Err(FireError::NoAttackersError) => panic!("Expected an IndexError, got no a NoAttackersError"),
            Err(FireError::IndexError) => ()
        }
        
        board.reset();
        // Nothing in range case
        match board.can_fire_checker_at(Vec2::new(7, 1)) {
            Ok(_) => panic!("Expected a NoAttackersError, got no error"),
            Err(FireError::NoAttackersError) => (),
            Err(FireError::IndexError) => panic!("Expected an IndexError, got an IndexError")
        }
    }

    #[test]
    fn fire_checker_at() {
        // Seed me
        let mut board = Board::from_seed([0; 32]);
        // Normal case, checker takes damage and dies
        board.place_checker_at(Vec2::new(5, 2), Checker::new(3, PLAYER_B_ID)).unwrap();
        board.fire_checker_at(Vec2::new(5, 2)).unwrap();
        let post_fire = board.checker_at(Vec2::new(5, 2)).unwrap();
        assert_eq!(post_fire.height, 0);
        assert_eq!(post_fire.owner, EMPTY_PLAYER_ID);

        // Place stones, normal case with terrain, expect a certain result based on RNG rolls
        let stone_pos = vec![Vec2::new(4, 2), Vec2::new(4, 3), Vec2::new(5, 2), Vec2::new(5, 3)];
        for pos in stone_pos.iter() {
            board.place_stone_at(*pos, Stone::new(PLAYER_B_ID)).unwrap();
        }
        board.place_checker_at(Vec2::new(4, 2), Checker::new(3, PLAYER_B_ID)).unwrap();
        board.fire_checker_at(Vec2::new(4, 2)).unwrap();
        let victim = board.checker_at(Vec2::new(4, 2)).unwrap();
        assert_eq!(victim.height, 2); 
        assert_eq!(victim.owner, PLAYER_B_ID);

        // IndexError case
        match board.fire_checker_at(Vec2::new(-1, -1)) {
            Ok(_) => panic!("Expected an IndexError, got no error"),
            Err(FireError::NoAttackersError) => panic!("Expected an IndexError, got no a NoAttackersError"),
            Err(FireError::IndexError) => ()
        }
        board.reset();
        // Nothing in range case
        match board.fire_checker_at(Vec2::new(7, 1)) {
            Ok(_) => panic!("Expected a NoAttackersError, got no error"),
            Err(FireError::NoAttackersError) => (),
            Err(FireError::IndexError) => panic!("Expected an IndexError, got an IndexError")
        }

    }

    #[test]
    fn move_checker() {
        let mut board = Board::new();
        // Normal case
        let start = Vec2::new(1, 2);
        let end = Vec2::new(2, 2);
        let start_checker = board.checker_at(start).unwrap().clone();
        board.move_checker(start, end).unwrap();
        // Start should not be occupied
        assert!(board.checker_at(start).unwrap().owner == EMPTY_PLAYER_ID);
        // End should contain start
        assert!(*board.checker_at(end).unwrap() == start_checker);

        // Trying to move to an occupied square
        match board.move_checker(Vec2::new(1, 3), Vec2::new(0, 3)) {
            Ok(_) => panic!("Expected an OccupiedError, got no error"),
            Err(MoveError::IndexError(_)) => panic!("Expected an OccupiedError, got IndexError"),
            Err(MoveError::NegationError) => panic!("Expected am OccupiedError, got NegationError"),
            Err(MoveError::OccupiedError) => ()
        }
        // Trying to move out of bounds
        match board.move_checker(Vec2::new(0, 2), Vec2::new(-1, 2)) {
            Ok(_) => panic!("Expected an IndexError, got no error"),
            Err(MoveError::IndexError(_)) => (),
            Err(MoveError::NegationError) => panic!("Expected a IndexError, got an NegationError"),
            Err(MoveError::OccupiedError) => panic!("Expected an IndexError, got an OccupiedError")
        }
    }

    #[test]
    fn as_string() {
        let mut board = Board::new();
        board.place_stone_at(Vec2::new(0, 0), Stone::new(PLAYER_A_ID)).unwrap();
        board.place_stone_at(Vec2::new(0,  6), Stone::new(PLAYER_B_ID)).unwrap();
        let expected = 
"a . . . . . . . . 
 _ _ _ _ _ _ _ _
. . . . . . . . . 
 2 _ _ _ _ _ _ B
. . . . . . . . . 
 3 1 _ _ _ _ A C
. . . . . . . . . 
 3 1 _ _ _ _ A C
. . . . . . . . . 
 2 _ _ _ _ _ _ B
. . . . . . . . . 
 _ _ _ _ _ _ _ _
b . . . . . . . . 
";
        let rep = board.as_string();
        assert_eq!(rep, expected);
    }
}