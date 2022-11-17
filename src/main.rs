use std::ops::Add;

const EMPTY_PLAYER_ID: i32 = -1;
const PLAYER_A_ID: i32 = 1;
const PLAYER_B_ID: i32 = 2;
const STARTING_STONES: i32 = 32;

struct Game {
    board: Board,
    players: [Player; 2], 
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            players: [
                Player::new(PLAYER_A_ID, STARTING_STONES),
                Player::new(PLAYER_B_ID, STARTING_STONES)
            ]
        }
    }
}

struct Player {
    id: i32,
    stones: i32,
}

impl Player {
    fn new(_id: i32, nstones: i32) -> Player {
        Player {
            id: _id,
            stones: nstones
        }
    }

    fn set_stones(&mut self, n: i32) {
        self.stones = n
    }

    fn get_stone(&mut self) -> Option<Stone> {
        if self.stones <= 0 {
            None
        } else {
            self.stones -= 1;
            Some(Stone::new(self.id))
        }
    }
}
const PLAYER_A_CHECK: [char; 4] = [' ', '░', '▒', '▓'];
const PLAYER_A_STONE: char = 'a';

const PLAYER_B_CHECK: [char; 4] = [' ', '▁', '▃', '▇'];
const PLAYER_B_STONE: char = 'b';

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 6;

struct Board {
    checker_board: [Checker; BOARD_WIDTH * BOARD_HEIGHT],
    stone_board: [Stone; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)]
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            checker_board: [Checker{height: 0, owner: EMPTY_PLAYER_ID}; BOARD_WIDTH * BOARD_HEIGHT],
            stone_board: [Stone{owner: EMPTY_PLAYER_ID}; (BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)] 
        };
        board.place_start_pieces();
        
        board
    }

    fn reset(&mut self) {
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

    fn can_move_checker(&self, from: Vec2, to: Vec2) -> bool {
        true
    }

    fn place_checker_at(&mut self, pos: Vec2, checker: Checker) {
        let idx = Board::vec_to_checker_idx(pos);
        self.checker_board[idx] = checker
    }

    fn can_put_stone(&self, pos: Vec2) -> bool {
        true
    }

    fn can_move_stone(&self, from: Vec2, to: Vec2) -> bool {
        true
    }

    fn place_stone_at(&mut self, pos: Vec2, stone: Stone) {
        let idx = Board::vec_to_stone_idx(pos);
        self.stone_board[idx] = stone
    }

    fn fire_checker_at(&self, pos: Vec2) {}

    fn can_fire_at(&self, from: Vec2, pos: Vec2) -> bool {
        true
    }

    /*
     * Do a bounds check and return an Option
     */
    fn checker_at<'a>(&'a self, pos: Vec2) -> &'a Checker {
        let idx: usize = Board::vec_to_checker_idx(pos); 
        &self.checker_board[idx]
    }

    /*
     * Bounds check and return an Option
     */
    fn stone_at<'a>(&'a self, pos: Vec2) -> &'a Stone {
        let idx: usize = Board::vec_to_stone_idx(pos);
        &self.stone_board[idx]
    }

    fn vec_to_stone_idx(pos: Vec2) -> usize {
        (pos.x + pos.y) as usize * (BOARD_HEIGHT + 1)
    }

    fn vec_to_checker_idx(pos: Vec2) -> usize {
        (pos.x + pos.y) as usize * BOARD_HEIGHT
    }

    fn print(&self) {
        for yi in 0 .. BOARD_HEIGHT as i32 * 2 + 1 {
            let turn = yi % 2;
            for xi in 0 .. BOARD_WIDTH as i32 * 2 + 1 {
                if (xi % 2 == turn) {
                    continue;
                }
                let idx = Board::vec_to_stone_idx(Vec2::new(xi >> 2, yi >> 2)); 
                if (turn == 0) {
                    // print row of stones
                    let stone = self.stone_board[idx];
                    
                    let mut draw_char: char = ' ';
                    if stone.owner == PLAYER_A_ID {
                        draw_char = PLAYER_A_STONE;
                    } else if stone.owner == PLAYER_B_ID {
                        draw_char = PLAYER_B_STONE;
                    }
                    print!("{} ", draw_char);
                } else if (turn == 1) {
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

#[derive(Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32
}

const UP: Vec2 = Vec2 {x:0, y:-1};
const DOWN: Vec2 = Vec2 {x:0, y:1};
const LEFT: Vec2 = Vec2 {x:-1, y:0};
const RIGHT: Vec2 = Vec2 {x:1, y:0};

impl Vec2 {
    fn new(xi: i32, yi: i32) -> Vec2 {
        Vec2{x: xi, y: yi}
    }

    fn left(self) -> Vec2 {
        self + LEFT
    }

    fn right(self) -> Vec2 {
        self + RIGHT
    }

    fn up(self) -> Vec2 {
        self + UP
    }

    fn down(self) -> Vec2 {
        self + DOWN
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Clone, Copy)]
struct Checker {
    height: i32,
    owner: i32
}

impl Checker {
    fn new(h: i32, o: i32) -> Checker {
        Checker { height: h, owner: o }
    }
}

#[derive(Clone, Copy)]
struct Stone {
    owner: i32
}

impl Stone {
    fn new(o: i32) -> Stone {
        Stone { owner: o }
    }
}


fn main() {
    println!("Hello, world!");
}
