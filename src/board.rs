use crate::piece;

const BOARD_SIZE: usize = 8;

pub struct Board {
    // pieces: [[piece::Piece; BOARD_SIZE]; BOARD_SIZE],
    pieces: Vec<Vec<piece::Piece>>
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            pieces: Vec::new(),
        };
        for _ in 0..BOARD_SIZE {
            let mut vec = Vec::new();
            for j in 0..BOARD_SIZE {
                vec.push(piece::Piece::new());
            }
            b.pieces.push(vec);
        }

        b._put_pieces_in_starting_positions();

        b
    }
    pub fn show(&self) {
        for r in 0..BOARD_SIZE {
            // Initial space
            print!(" ");
            for c in 0..BOARD_SIZE {
                print!("{} ", self.pieces[r][c].to_string());
            }
            println!("");
        }
    }

    pub fn set_piece_at_row_col(&mut self, row: usize, col: usize, piece: piece::Piece) {
        self.pieces[row][col] = piece;
    }

    fn _put_pieces_in_starting_positions(&mut self) {
        // Place pawns on both sides
        for i in 0..BOARD_SIZE {
            self.set_piece_at_row_col(1, i, piece::Piece::with_type_and_color(piece::PieceType::Pawn, piece::Color::Black));
            self.set_piece_at_row_col(6, i, piece::Piece::with_type_and_color(piece::PieceType::Pawn, piece::Color::White));
        }

        // Place other pieces on both sides

        // Black is on top (lower row numbers)
        self.set_piece_at_row_col(0, 0,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Rook,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 1,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Knight,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 2,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Bishop,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 3,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Queen,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 4,
            piece::Piece::with_type_and_color(
                    piece::PieceType::King,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 5,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Bishop,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 6,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Knight,
                    piece::Color::Black));
    self.set_piece_at_row_col(0, 7,
            piece::Piece::with_type_and_color(
                    piece::PieceType::Rook,
                    piece::Color::Black));

        // White is on bottom (higher row numbers)
        self.set_piece_at_row_col(7, 0,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Rook,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 1,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Knight,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 2,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Bishop,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 3,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Queen,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 4,
                piece::Piece::with_type_and_color(
                        piece::PieceType::King,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 5,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Bishop,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 6,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Knight,
                        piece::Color::White));
        self.set_piece_at_row_col(7, 7,
                piece::Piece::with_type_and_color(
                        piece::PieceType::Rook,
                        piece::Color::White));
    }
}
