pub struct Piece {
    ptype: PieceType,
}

impl Piece {
    pub fn new() -> Piece {
        Piece{ptype: PieceType::Pawn}
    }
    pub fn with_type(ptype: PieceType) -> Piece {
        Piece{ptype: ptype}
    }
    pub fn toString(&self) -> String {
        pieceToString(self)
    }
    pub fn parse(s: String) -> Result<Piece, String> {
        Ok(Piece{ptype: parsePieceType(s).expect("")})
    }
}

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

fn pieceToString(piece: &Piece) -> String {
    match piece.ptype {
        PieceType::Pawn => "Pawn".to_string(),
        PieceType::Rook => "Rook".to_string(),
        PieceType::Knight => "Knight".to_string(),
        PieceType::Bishop => "Bishop".to_string(),
        PieceType::Queen => "Queen".to_string(),
        PieceType::King => "King".to_string(),
    }
}

fn parsePieceType(s: String) -> Result<PieceType, String> {
    let s = s.trim();
    match s {
        "Pawn" => Ok(PieceType::Pawn),
        "Rook" => Ok(PieceType::Rook),
        "Knight" => Ok(PieceType::Knight),
        "Bishop" => Ok(PieceType::Bishop),
        "Queen" => Ok(PieceType::Queen),
        "King" => Ok(PieceType::King),
        _ => Err(String::from("Invalid piece type")),
    }
}
