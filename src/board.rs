use std::fmt::{Display, Error, Formatter};
use std::vec::Vec;

use crate::game::{Checker, Stone, PLAYER_A_ID, PLAYER_B_ID, EMPTY_PLAYER_ID};
use crate::vec::Vec2;

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

    /**
     * place_checker_at places a checker at a position, or returns a MoveError if a rule is
     * violated.
     * 
     * @pos - Position to place the checker at, should be within [[0,0], [BOARD_WIDTH,BOARD_HEIGHT])
     * @checker - Checker to put at the position.
     * @return - Ok if checker was placed, or one of the MoveError types if the checker was not placed.
     */
    pub fn place_checker_at(&mut self, pos: Vec2, checker: Checker) -> Result<(), MoveError> {
        let idx = Board::vec_to_checker_idx(pos);
        let current_piece = self.checker_board[idx];

        // Do not allow to placing a non-empty piece in a non-empty slot
        if current_piece.owner != EMPTY_PLAYER_ID && checker.owner != EMPTY_PLAYER_ID {
            return Err(MoveError::OccupiedError);
        }
        if idx < BOARD_WIDTH * BOARD_HEIGHT {
            self.checker_board[idx] = checker;
            return Ok(())
        } else {
            return Err(MoveError::IndexError(String::from("{pos} not within 0,0 and {BOARD_WIDTH-1},{BOARD_HEIGHT-1}")))
        }
    }

    // pub fn can_move_stone(&self, from: Vec2, to: Vec2) -> bool {
    //     true
    // }

    pub fn place_stone_at(&mut self, pos: Vec2, stone: Stone) -> Result<(), MoveError> {
        let idx = Board::vec_to_stone_idx(pos);
        let current_piece: Stone = self.stone_board[idx];
        if current_piece.owner != EMPTY_PLAYER_ID {
            return Err(MoveError::OccupiedError);
        }
        // Check for rule of negation
        // TODO: 

        if idx < (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1) {
            self.stone_board[idx] = stone;
            return Ok(())
        } else {
            return Err(MoveError::IndexError(String::from("{pos} not within 0,0 and {BOARD_WIDTH},{BOARD_HEIGHT}")))
        }
    }

    /**
     * checker_neighbours_of_stone
     * Returns a list of all valid checker neighbours of stone
     * 
     */
    pub fn checker_neigbours_of_stone() -> Vec<Vec2>  {
        return Vec::new()
    }

    pub fn stone_neighbours_of_checker() -> Vec<Vec2> {
        return Vec::new()
    }

    pub fn fire_checker_at(&self, pos: Vec2) {}

    // pub fn can_fire_at(&self, from: Vec2, pos: Vec2) -> bool {
    //     true
    // }

    /*
        * Do a bounds check and return an Option
        */
    pub fn checker_at<'a>(&'a self, pos: Vec2) -> &'a Checker {
        let idx: usize = Board::vec_to_checker_idx(pos); 
        &self.checker_board[idx]
    }

    /*
        * Bounds check and return an Option
        */
    pub fn stone_at<'a>(&'a self, pos: Vec2) -> &'a Stone {
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
                assert_eq!(stone.owner, EMPTY_PLAYER_ID)
            }
        }
        // On the checkerboard,
        // [0, 1] and [0, 4] should have 2-stack player B pieces
        let b2 = Checker::new(2, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(0, 1)), b2);
        assert_eq!(*board.checker_at(Vec2::new(0, 4)), b2);
        // [0, 2] and [0, 3] should have 3-stack player B pieces
        let b3 = Checker::new(3, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(0, 2)), b3);
        assert_eq!(*board.checker_at(Vec2::new(0, 3)), b3);
        // [1, 2] and [1, 3] should have 1-stack player B pieces
        let b1 = Checker::new(1, PLAYER_B_ID);
        assert_eq!(*board.checker_at(Vec2::new(1, 2)), b1);
        assert_eq!(*board.checker_at(Vec2::new(1, 3)), b1);
        // [7, 1] and [7, 4] should have 2-stack player A pieces
        let a2 = Checker::new(2, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(7, 1)), a2);
        assert_eq!(*board.checker_at(Vec2::new(7, 4)), a2);
        // [7, 2] and [7, 3] should have 3-stack player A pieces
        let a3 = Checker::new(3, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(7, 2)), a3);
        assert_eq!(*board.checker_at(Vec2::new(7, 3)), a3);
        // [6, 2] and [6, 3] should have 1-stack player A pieces
        let a1 = Checker::new(1, PLAYER_A_ID);
        assert_eq!(*board.checker_at(Vec2::new(6, 2)), a1);
        assert_eq!(*board.checker_at(Vec2::new(6, 3)), a1);           
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
            Err(MoveError::NegationError) => panic!("Expected an IndexError, got a NegationError"),
            Err(MoveError::IndexError(_)) => panic!("Expected an OccupiedError, got an IndexError"),
            Ok(()) => panic!("Expected an OccupiedError, no error")
        }
    }

    
}