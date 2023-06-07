use crate::board;

pub struct Game {
    board: board::Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
        }
    }
    pub fn show(&self) {
        self.board.show();
    }
    pub fn get_board(&self) -> &board::Board {
        return &self.board;
    }
}
