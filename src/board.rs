use std::fmt::{Display, Error, Formatter};
use std::vec::{Vec};

use crate::game::{Checker, Stone, PLAYER_A_ID, PLAYER_B_ID, EMPTY_PLAYER_ID};
use crate::vec::{Vec2};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 6;

const PLAYER_A_CHECK: [char; 4] = [' ', '░', '▒', '▓'];
const PLAYER_A_STONE: char = 'a';

const PLAYER_B_CHECK: [char; 4] = [' ', '▁', '▃', '▇'];
const PLAYER_B_STONE: char = 'b';

#[derive(Clone, Debug)]
pub enum MoveError {
    // Thrown when move index is out of bounds.
    IndexError(String),
    // Thrown when trying to place a piece on an occupied square.
    OccupiedError,
    // Thrown when Rule of Negation is broken ie trying to place a stone on a square with checker.
    NegationError,
}

pub enum FireError {
    // There is no unit on the attacked square
    EmptyError,
    // Move was out not a valid board index
    IndexError,
    // No attacker pieces are in range
    NoAttackersError,
}

pub enum SlideError {
    // Thrown when move index is out of bounds.
    // Thrown when slide is blocked
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn as_vec(&self) -> Vec2 {
        match self {
            UP => crate::vec::UP,
            DOWN => crate::vec::DOWN,
            LEFT => crate::vec::LEFT,
            RIGHT => crate::vec::RIGHT
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
    stone_board: [Stone; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)]
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            checker_board: [Checker{height: 0, owner: EMPTY_PLAYER_ID}; BOARD_WIDTH * BOARD_HEIGHT],
            stone_board: [Stone{owner: EMPTY_PLAYER_ID}; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)] 
        };
        board.place_start_pieces();
        
        board
    }

    // TODO:
    // function that returns stones for player
    // function that returns checkers for a player
    // function that returns valid stone placement squares


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

    pub fn can_move_checker(&self, from: Vec2, to: Vec2) -> bool {
        true
    }

    pub fn slide_stone(&self, from: Vec2, dir: Direction) -> Result<(), SlideError> {
        Ok(())
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
        return Ok(())
    }

    // pub fn can_move_stone(&self, from: Vec2, to: Vec2) -> bool {
    //     true
    // }

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
        return Ok(())
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

        let mut out: Vec<Vec2> = Vec::new();
        for pos in neighbours.iter() {
            if Board::is_checker_vec_valid(*pos) {
                out.push(*pos)
            }
        }
        return out
    }

    /**
     * fire_checker_at
     * @player attacks the checker at @pos, with all possible pieces in range, or errors.
     * @player Player id 
     * @pos Square to attempt to attack.
     * @return Ok if no error, or one of the error types if something went wrong.
     */
    pub fn fire_checker_at(&self, player: i32, pos: Vec2) -> Result<(), FireError> {
        Ok(())
    }

    // pub fn can_fire_at(&self, from: Vec2, pos: Vec2) -> bool {
    //     true
    // }

    /*
     * Do a bounds check and return an Option
     */
    pub fn checker_at<'a>(&'a self, pos: Vec2) -> Result<&'a Checker, ()> {
        if !Board::is_checker_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_checker_idx(pos); 
            Ok(&self.checker_board[idx])
        }
    }

    /*
     * Bounds check and return an Option
     */
    pub fn stone_at<'a>(&'a self, pos: Vec2) -> Result<&'a Stone, ()> {
        if !Board::is_stone_vec_valid(pos) {
            Err(())
        } else {
            let idx: usize = Board::vec_to_stone_idx(pos); 
            Ok(&self.stone_board[idx])
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
        if pos.x < 0 || pos.y < 0 {
            return false;
        }
        let idx = Board::vec_to_checker_idx(pos);
        return idx < BOARD_WIDTH * BOARD_HEIGHT
    }

    fn is_stone_vec_valid(pos: Vec2) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return false;
        }
        let idx = Board::vec_to_stone_idx(pos);
        return idx < (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)
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

    pub fn stones_for_player(player: i32) -> Vec<Vec2> {
        Vec::new()
    }

    pub fn checkers_for_player(player: i32) -> Vec<Vec2> {
        Vec::new()
    }

    pub fn empty_stones() -> Vec<Vec2> {
        Vec::new()
    }

    pub fn print(&self) {
        for yi in 0 .. BOARD_HEIGHT as i32 * 2 + 1 {
            let turn = yi % 2;
            for xi in 0 .. BOARD_WIDTH as i32 * 2 + 1 {
                if xi % 2 == turn {
                    continue;
                }
                let idx = Board::vec_to_stone_idx(Vec2::new(xi >> 2, yi >> 2)); 
                if turn == 0 {
                    // print row of stones
                    let stone = self.stone_board[idx];
                    
                    let mut draw_char: char = ' ';
                    if stone.owner == PLAYER_A_ID {
                        draw_char = PLAYER_A_STONE;
                    } else if stone.owner == PLAYER_B_ID {
                        draw_char = PLAYER_B_STONE;
                    }
                    print!("{} ", draw_char);
                } else if turn == 1 {
                    let checker = self.checker_board[idx];
                    
                    let mut draw_char: char = ' ';
                    if checker.owner == PLAYER_A_ID {
                        draw_char = PLAYER_A_CHECK[checker.height as usize];
                    } else if checker.owner == PLAYER_B_ID {
                        draw_char = PLAYER_B_CHECK[checker.height as usize];
                    }
                    print!("{} ", draw_char);
                }
            } 
            print!("\n")
        }
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
        // Board should be free of stones
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let stone = board.stone_at(
                    Vec2::new(x as i32, y as i32)
                );
                assert_eq!(stone.unwrap().owner, EMPTY_PLAYER_ID)
            }
        }
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
    
}