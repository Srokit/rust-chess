#[derive(Copy, Clone)]
pub struct Piece {
    ptype: PieceType,
    color: Color,
}

impl Piece {
    pub fn new() -> Piece {
        Piece{ptype: PieceType::Pawn,
              color: Color::White}
    }
    pub fn with_type_and_color(ptype: PieceType, color: Color) -> Piece {
        Piece{ptype: ptype,
              color: color}
    }
    pub fn to_string(&self) -> String {
        _piece_to_string(self)
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_piece_type(&self) -> PieceType {
        self.ptype
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

fn _piece_to_string(piece: &Piece) -> String {
    return format!("{}{}", _color_to_string(&piece.color), _piecetype_to_string(&piece.ptype));
}

fn _piecetype_to_string(pt: &PieceType) -> String {
    match pt {
        PieceType::Pawn => "Pa".to_string(),
        PieceType::Rook => "Ro".to_string(),
        PieceType::Knight => "Kn".to_string(),
        PieceType::Bishop => "Bi".to_string(),
        PieceType::Queen => "Qu".to_string(),
        PieceType::King => "Ki".to_string(),
    }
}

fn _color_to_string(color: &Color) -> String {
    match color {
        Color::White => "w".to_string(),
        Color::Black => "b".to_string(),
    }
}
