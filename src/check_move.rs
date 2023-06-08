use crate::board;
use crate::piece;
use crate::position;

/**
 * NOTE: All moves are considered forward (up the board for White player) if the column
 *       number decreases between to and from because of the way the board is represented.
 *       AKA: to_pos.row - from_pos.row < 0 -> means forward
 */

const WHITE_PAWN_STARTING_ROW: u8 = 6;

pub struct CheckMoveInfo {
    pub from: position::Position,
    pub to: position::Position,
    pub from_piece: Option<piece::Piece>,
    pub to_piece: Option<piece::Piece>,
    pub is_capturing_piece: bool,
}

impl CheckMoveInfo {
    pub fn new(board: &board::Board, from: &position::Position, to: &position::Position) -> CheckMoveInfo {
        CheckMoveInfo {
            from: *from,
            to: *to,
            from_piece: board.copy_piece_at_position(from),
            to_piece: board.copy_piece_at_position(to),
            is_capturing_piece: board.copy_piece_at_position(to).is_some(),
        }
    }
}

pub fn check_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    if !_any_type(move_info) {
        return false;
    }
    let from_piece_type = board.copy_piece_at_position(&(move_info.from)).unwrap().get_piece_type();
    match from_piece_type {
        piece::PieceType::Pawn => return _pawn_move(board, move_info),
        piece::PieceType::Rook => return _rook_move(board, move_info),
        piece::PieceType::Bishop => return _bishop_move(board, move_info),
        piece::PieceType::Knight => return _knight_move(board, move_info),
        piece::PieceType::Queen => return _queen_move(board, move_info),
        piece::PieceType::King => return _king_move(board, move_info),
        // TODO(Stan): Account for other types
        _ => return true,
    }
}

fn _any_type(move_info: &CheckMoveInfo) -> bool {
    if move_info.from_piece.is_none() {
        return false;
    }

    let from_piece = move_info.from_piece.unwrap();
    let to_piece = move_info.to_piece;

    // Player always plays White for now
    if from_piece.get_color() == piece::Color::Black {
        return false;
    }
    // King cannot be captured
    if move_info.is_capturing_piece && to_piece.unwrap().get_piece_type() == piece::PieceType::King {
        return false;
    }
    // Cannot capture own pieces
    if move_info.is_capturing_piece && to_piece.unwrap().get_color() == from_piece.get_color() {
        return false;
    }
    // Cannot stay put
    if move_info.from == move_info.to {
        return false;
    }

    // Rest will be checked in piece specific move checkers
    true
}

fn _pawn_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());
    let maybe_to_piece = board.copy_piece_at_position(&to);
    let is_capturing_piece = maybe_to_piece.is_some();

    // Move forward
    if diff_c == 0 {
        // Move forward 1
        // Refer to NOTE at top if this seems backwards
        if diff_r == -1 && !is_capturing_piece{
            return true;
        }
        // Move forward 2 from starting position
        if diff_r == -2 && from.get_row() == WHITE_PAWN_STARTING_ROW && !is_capturing_piece {
            // There cannot be a piece in the way
            let next_r = from.get_row() - 1;
            let next_pos = position::Position::new(next_r, from.get_col());
            if board.copy_piece_at_position(&next_pos).is_some() {
                return false;
            }
            return true;
        }
    }
    // Capture
    if diff_r == -1 && diff_c.abs() == 1 && is_capturing_piece {
        return true;
    }

    // Other cases must be invalid
    false
}

fn _rook_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());

    // Can only move 1 direction
    if diff_c != 0 && diff_r != 0 {
        return false;
    }

    let direction_c = if diff_c > 0 {1} else if diff_c < 0 {-1} else {0};
    let direction_r= if diff_r > 0 {1} else if diff_r < 0 {-1} else {0};

    // Is there something in the way
    let mut row = from.get_row() as i8;
    let mut col = from.get_col() as i8;
    loop {
        // Move to next position
        col += direction_c;
        row += direction_r;
        let curr_pos = position::Position::new(row as u8, col as u8);
        if curr_pos == to {
            break;
        }
        let curr_piece = board.copy_piece_at_position(&curr_pos);
        if curr_piece.is_some() {
            return false;
        }
    }

    // Other cases must be valid
    true
}

fn _bishop_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());

    // Can only move diagonally
    if diff_c.abs() != diff_r.abs() {
        return false;
    }

    let direction_c = if diff_c > 0 {1} else {-1};
    let direction_r= if diff_r > 0 {1} else {-1};

    // Is there something in the way
    let mut row = from.get_row() as i8;
    let mut col = from.get_col() as i8;
    loop {
        // Move to next position
        col += direction_c;
        row += direction_r;
        let curr_pos = position::Position::new(row as u8, col as u8);
        if curr_pos == to {
            break;
        }
        let curr_piece = board.copy_piece_at_position(&curr_pos);
        if curr_piece.is_some() {
            return false;
        }
    }

    // Other cases must be valid
    true
}

fn _knight_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());

    // Can only move in L shape
    if diff_c.abs() == 2 && diff_r.abs() == 1 {
        return true;
    }
    if diff_c.abs() == 1 && diff_r.abs() == 2 {
        return true;
    }

    // Other cases must be invalid
    false
}

fn _queen_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());

    // Can only move 1 direction (both rook and bishop)
    if (diff_c != 0 && diff_r != 0) && diff_c.abs() != diff_r.abs() {
        return false;
    }

    let direction_c = if diff_c > 0 {1} else if diff_c < 0 {-1} else {0};
    let direction_r= if diff_r > 0 {1} else if diff_r < 0 {-1} else {0};

    // Is there something in the way
    let mut row = from.get_row() as i8;
    let mut col = from.get_col() as i8;
    loop {
        // Move to next position
        col += direction_c;
        row += direction_r;
        let curr_pos = position::Position::new(row as u8, col as u8);
        if curr_pos == to {
            break;
        }
        let curr_piece = board.copy_piece_at_position(&curr_pos);
        if curr_piece.is_some() {
            return false;
        }
    }

    // Other cases must be valid
    true
}

fn _king_move(board: &board::Board, move_info: &CheckMoveInfo) -> bool {
    let from = move_info.from;
    let to = move_info.to;
    let diff_c = _diff_u8_as_i8(from.get_col(), to.get_col());
    let diff_r = _diff_u8_as_i8(from.get_row(), to.get_row());

    // Can only move 1 space
    if diff_c.abs() > 1 || diff_r.abs() > 1 {
        return false;
    }

    // Other cases must be valid
    true
}

// Helpers
fn _diff_u8_as_i8(a: u8, b: u8) -> i8 {
    (b as i8) - (a as i8)
}
