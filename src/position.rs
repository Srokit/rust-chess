pub struct Position {
    row_: u8,
    col_: u8,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Position {
        Position {
            row_: row,
            col_: col,
        }
    }
    pub fn get_row(&self) -> u8 {
        self.row_
    }
    pub fn get_col(&self) -> u8 {
        self.col_
    }

    pub fn from_string(s: &String) -> Result<Position, String> {
        if !_pos_str_is_valid(s) {
            return Err(format!("Cannot parse position '{s}'").to_string());
        }
        let letC = s.chars().nth(0).unwrap();
        let numC = s.chars().nth(1).unwrap();
        let row = _letter_to_row(letC);
        let col = _num_to_col(numC);
        return Ok(Position::new(row, col)) ;
    }
}

const INV_ROW_OR_COL_VAL: u8 = 127;

fn _letter_to_row(letter: char) -> u8 {
    match letter {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
         _ => INV_ROW_OR_COL_VAL,
    }
}

fn _num_to_col(num: char) -> u8 {
    match num {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => INV_ROW_OR_COL_VAL,
    }
}

fn _pos_str_is_valid(s: &String) -> bool {
    if s.len() != 2 {
        return false;
    }
    let letC = s.chars().nth(0).unwrap();
    let numC = s.chars().nth(1).unwrap();
    if _letter_to_row(letC) == INV_ROW_OR_COL_VAL {
        return false;
    }
    if _num_to_col(numC) == INV_ROW_OR_COL_VAL {
        return false;
    }
    return true;
}