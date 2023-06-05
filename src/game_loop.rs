use std::io;

use crate::game;
use crate::helpers;

const EXIT_STRING: &str = "exit";

pub fn start_game_loop() {
    let mut move_line: String;
    let mut game = game::Game::new();
    loop {
        // Overwrite the previous move_line
        move_line = String::new();
        println!("Enter your move: ");
        match io::stdin().read_line(&mut move_line) {
            Err(_) => {
                helpers::printerr("Couldn't read from cmdline");
                continue
            },
            _ => {},
        }
        move_line = move_line.trim().to_string();
        if move_line.eq(EXIT_STRING) {
            break;
        }
        println!("Your text: {}", move_line);
        game.show();
    }
}