use std::io;

use crate::game;
use crate::helpers;

const EXIT_STRING: &str = "exit";

fn read_move(move_str: &mut String) -> bool {
    match io::stdin().read_line(move_str) {
        Err(_) => {
            helpers::printerr("Couldn't read from cmdline");
            return false;
        },
        _ => {},
    }
    return true;
}

pub fn start_game_loop() {
    let mut move1: String;
    let mut move2: String;
    let mut game = game::Game::new();
    loop {
        // Overwrite the previous move_line
        move1 = String::new();
        move2 = String::new();
        println!("Enter your move (e.g. A1<enter>H8) : ");
        if !read_move(&mut move1) {
            break;
        }
        if !read_move(&mut move2) {
            break;
        }
        if move1.eq(EXIT_STRING) || move2.eq(EXIT_STRING) {
            break;
        }
        game.show();
    }
}
