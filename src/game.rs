use crate::board;
use crate::position;
use crate::check_move;

pub struct Game {
    board: board::Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
        }
    }
    pub fn show(&mut self) {
        self.board.show();
    }
    pub fn move_piece(&mut self, from: &position::Position, to: &position::Position) -> bool {
        let pieceOpt = self.board.copy_piece_at_position(from);
        if pieceOpt.is_none() {
            // Inv move
            return false;
        }
        self.board.set_piece_at_position(from, None);
        self.board.set_piece_at_position(to, pieceOpt);

        true
    }
    pub fn is_move_valid(&self, from: &position::Position, to: &position::Position) -> bool {
        let info = check_move::CheckMoveInfo {from: *from, to: *to};
        check_move::check_move(&self.board, &info)
    }
}
