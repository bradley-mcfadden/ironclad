use crate::game::{Checker, Stone, PLAYER_A_ID, PLAYER_B_ID, EMPTY_PLAYER_ID};
use crate::vec::Vec2;

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 6;

const PLAYER_A_CHECK: [char; 4] = [' ', '░', '▒', '▓'];
const PLAYER_A_STONE: char = 'a';

const PLAYER_B_CHECK: [char; 4] = [' ', '▁', '▃', '▇'];
const PLAYER_B_STONE: char = 'b';

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
        self.place_checker_at(Vec2::new(6, 2), Checker::new(1, PLAYER_A_ID));
        self.place_checker_at(Vec2::new(6, 3), Checker::new(1, PLAYER_A_ID));
        self.place_checker_at(Vec2::new(7, 1), Checker::new(2, PLAYER_A_ID));
        self.place_checker_at(Vec2::new(7, 4), Checker::new(2, PLAYER_A_ID));
        self.place_checker_at(Vec2::new(7, 3), Checker::new(3, PLAYER_A_ID));
        self.place_checker_at(Vec2::new(7, 2), Checker::new(3, PLAYER_A_ID));

        self.place_checker_at(Vec2::new(1, 2), Checker::new(1, PLAYER_B_ID));
        self.place_checker_at(Vec2::new(1, 3), Checker::new(1, PLAYER_B_ID));
        self.place_checker_at(Vec2::new(0, 1), Checker::new(2, PLAYER_B_ID));
        self.place_checker_at(Vec2::new(0, 4), Checker::new(2, PLAYER_B_ID));
        self.place_checker_at(Vec2::new(0, 3), Checker::new(3, PLAYER_B_ID));
        self.place_checker_at(Vec2::new(0, 2), Checker::new(3, PLAYER_B_ID));
    }

    fn clear_board(&mut self) {
        self.checker_board.fill(Checker::new(0, EMPTY_PLAYER_ID));
        self.stone_board.fill(Stone::new(EMPTY_PLAYER_ID))
    }

    pub fn can_move_checker(&self, from: Vec2, to: Vec2) -> bool {
        true
    }

    pub fn place_checker_at(&mut self, pos: Vec2, checker: Checker) {
        let idx = Board::vec_to_checker_idx(pos);
        self.checker_board[idx] = checker
    }

    pub fn can_put_stone(&self, pos: Vec2) -> bool {
        true
    }

    pub fn can_move_stone(&self, from: Vec2, to: Vec2) -> bool {
        true
    }

    pub fn place_stone_at(&mut self, pos: Vec2, stone: Stone) {
        let idx = Board::vec_to_stone_idx(pos);
        self.stone_board[idx] = stone
    }

    pub fn fire_checker_at(&self, pos: Vec2) {}

    pub fn can_fire_at(&self, from: Vec2, pos: Vec2) -> bool {
        true
    }

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

    fn vec_to_stone_idx(pos: Vec2) -> usize {
        (pos.x + pos.y) as usize * (BOARD_HEIGHT + 1)
    }

    fn vec_to_checker_idx(pos: Vec2) -> usize {
        (pos.x + pos.y) as usize * BOARD_HEIGHT
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