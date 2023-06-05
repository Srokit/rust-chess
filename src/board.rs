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
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                print!("{} ", self.pieces[i][j].toString());
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
            self.set_piece_at_row_col(1, i, piece::Piece::with_type(piece::PieceType::Pawn));
            self.set_piece_at_row_col(6, i, piece::Piece::with_type(piece::PieceType::Pawn));
        }

        // Place other pieces on both sides
        self.set_piece_at_row_col(0, 0, piece::Piece::with_type(piece::PieceType::Rook));
        self.set_piece_at_row_col(0, 1, piece::Piece::with_type(piece::PieceType::Knight));
        self.set_piece_at_row_col(0, 2, piece::Piece::with_type(piece::PieceType::Bishop));
        self.set_piece_at_row_col(0, 3, piece::Piece::with_type(piece::PieceType::Queen));
        self.set_piece_at_row_col(0, 4, piece::Piece::with_type(piece::PieceType::King));
        self.set_piece_at_row_col(0, 5, piece::Piece::with_type(piece::PieceType::Bishop));
        self.set_piece_at_row_col(0, 6, piece::Piece::with_type(piece::PieceType::Knight));
        self.set_piece_at_row_col(0, 7, piece::Piece::with_type(piece::PieceType::Rook));

        self.set_piece_at_row_col(7, 0, piece::Piece::with_type(piece::PieceType::Rook));
        self.set_piece_at_row_col(7, 1, piece::Piece::with_type(piece::PieceType::Knight));
        self.set_piece_at_row_col(7, 2, piece::Piece::with_type(piece::PieceType::Bishop));
        self.set_piece_at_row_col(7, 3, piece::Piece::with_type(piece::PieceType::Queen));
        self.set_piece_at_row_col(7, 4, piece::Piece::with_type(piece::PieceType::King));
        self.set_piece_at_row_col(7, 5, piece::Piece::with_type(piece::PieceType::Bishop));
        self.set_piece_at_row_col(7, 6, piece::Piece::with_type(piece::PieceType::Knight));
        self.set_piece_at_row_col(7, 7, piece::Piece::with_type(piece::PieceType::Rook));
    }
}
