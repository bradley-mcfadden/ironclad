
use crate::board::Board;

pub const EMPTY_PLAYER_ID: i32 = -1;
pub const PLAYER_A_ID: i32 = 1;
pub const PLAYER_B_ID: i32 = 2;
const STARTING_STONES: i32 = 32;

pub struct Game {
    pub board: Board,
    pub players: [Player; 2], 
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            players: [
                Player::new(PLAYER_A_ID, STARTING_STONES),
                Player::new(PLAYER_B_ID, STARTING_STONES)
            ]
        }
    }
}

pub struct Player {
    pub id: i32,
    pub stones: i32,
}

impl Player {
    pub fn new(_id: i32, nstones: i32) -> Player {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checker {
    pub height: i32,
    pub owner: i32
}

impl Checker {
    pub fn new(h: i32, o: i32) -> Checker {
        Checker { height: h, owner: o }
    }
}

#[derive(Clone, Copy)]
pub struct Stone {
    pub owner: i32
}

impl Stone {
    pub fn new(o: i32) -> Stone {
        Stone { owner: o }
    }
}