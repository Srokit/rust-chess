#[derive(Copy, Clone)]
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
        let col = _letter_to_col(letC);
        let row = _num_to_row(numC);
        return Ok(Position::new(row, col)) ;
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push(_col_to_letter(self.col_));
        s.push(_row_to_num(self.row_));
        return s;
    }
}

const INV_ROW_OR_COL_VAL: u8 = 127;

fn _letter_to_col(letter: char) -> u8 {
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

fn _num_to_row(num: char) -> u8 {
    match num {
        // These are reversed because the board ascends from bottom to
        // top, but internal storage of pieces is from top to bottom.
        '1' => 7,
        '2' => 6,
        '3' => 5,
        '4' => 4,
        '5' => 3,
        '6' => 2,
        '7' => 1,
        '8' => 0,
        _ => INV_ROW_OR_COL_VAL,
    }
}

fn _col_to_letter(row: u8) -> char {
    match row {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        _ => 'X',
    }
}

fn _row_to_num(col: u8) -> char {
    match col {
        // Reversed same as above
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => 'X',
    }
}

fn _pos_str_is_valid(s: &String) -> bool {
    if s.len() != 2 {
        return false;
    }
    let letC = s.chars().nth(0).unwrap();
    let numC = s.chars().nth(1).unwrap();
    if _letter_to_col(letC) == INV_ROW_OR_COL_VAL {
        return false;
    }
    if _num_to_row(numC) == INV_ROW_OR_COL_VAL {
        return false;
    }
    return true;
}
