use crate::board;
use crate::position;

pub struct MoveChecker<'a> {
    board: &'a board::Board,
    errStr: String,
}

impl MoveChecker<'_> {
    pub fn new(board: &board::Board) -> MoveChecker {
        MoveChecker {
            board: board,
            errStr: String::new(),
        }
    }
    pub fn is_move_valid(&self, from: &position::Position, to: &position::Position) -> bool {
        // TODO(Stan): Implement this
        return true;
    }

    pub fn get_last_err_str(&self) -> &String {
        return &self.errStr;
    }
}
