use crate::piece;
use crate::position;

const BOARD_SIZE: u8 = 8;

pub struct Board {
    // pieces: [[piece::Piece; BOARD_SIZE]; BOARD_SIZE],
    pieces: Vec<Vec<Option<piece::Piece>>>
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            pieces: Vec::new(),
        };
        for _ in 0..BOARD_SIZE {
            let mut vec = Vec::new();
            for j in 0..BOARD_SIZE {
                vec.push(None);
            }
            b.pieces.push(vec);
        }

        b._put_pieces_in_starting_positions();

        return b
    }
    pub fn show(&self) {
        for r in 0..BOARD_SIZE {
            // Initial space
            print!(" ");
            let rUSize = r as usize;
            for c in 0..BOARD_SIZE {
                let cUSize = c as usize;
                if self.pieces[rUSize][cUSize].is_none() {
                    print!("--- ");
                    continue;
                } else {
                    print!("{} ", self.pieces[rUSize][cUSize].as_ref().unwrap().to_string());
                }
            }
            println!("");
        }
    }

    pub fn set_piece_at_position(&mut self, pos: &position::Position, piece: Option<piece::Piece>) {
        if piece.is_none() {
            self._set_non_piece_at_row_col(pos.get_row(), pos.get_col());
            return;
        }
        self._set_piece_at_row_col(pos.get_row(), pos.get_col(), piece.unwrap());
    }

    pub fn copy_piece_at_position(&self, pos: &position::Position) -> Option<piece::Piece> {
        let optional = self._get_piece_at_row_col(pos.get_row(), pos.get_col());
        if optional.is_none() {
            return None;
        }
        return Some(*(optional.unwrap()));
    }

    pub fn _set_piece_at_row_col(&mut self, row: u8, col: u8, piece: piece::Piece) {
        self.pieces[row as usize][col as usize] = Some(piece);
    }

    pub fn _set_non_piece_at_row_col(&mut self, row: u8, col: u8) {
        self.pieces[row as usize][col as usize] = None;
    }

    pub fn _get_piece_at_row_col(&self, row: u8, col: u8) -> Option<&piece::Piece> {
        let rowUSize = row as usize;
        let colUSize = col as usize;
        if self.pieces[rowUSize][colUSize].is_none() {
            return None;
        }
        return self.pieces[rowUSize][colUSize].as_ref();
    }

    fn _put_pieces_in_starting_positions(&mut self) {
        // Place pawns on both sides
        for i in 0..BOARD_SIZE {
            self._set_piece_at_row_col(1, i, piece::Piece::with_type_and_color(piece::PieceType::Pawn, piece::Color::Black));
            self._set_piece_at_row_col(6, i, piece::Piece::with_type_and_color(piece::PieceType::Pawn, piece::Color::White));
        }

        // Place other pieces on both sides

        // Black is on top (lower row numbers)
        self._set_piece_at_row_col(0, 0,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Rook,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 1,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Knight,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 2,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Bishop,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 3,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Queen,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 4,
            piece::Piece::with_type_and_color(
                    piece::PieceType::King,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 5,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Bishop,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 6,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Knight,
                    piece::Color::Black));
    self._set_piece_at_row_col(0, 7,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Rook,
                    piece::Color::Black));

        // White is on bottom (higher row numbers)
        self._set_piece_at_row_col(7, 0,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Rook,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 1,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Knight,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 2,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Bishop,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 3,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Queen,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 4,
                piece::Piece::with_type_and_color(
                        piece::PieceType::King,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 5,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Bishop,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 6,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Knight,
                        piece::Color::White));
        self._set_piece_at_row_col(7, 7,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Rook,
                        piece::Color::White));
    }
}
