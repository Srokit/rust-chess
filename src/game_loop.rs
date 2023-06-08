use std::io;

use crate::game;
use crate::helpers;
use crate::position;

const EXIT_STRING: &str = "exit";

fn read_move() -> Option<String> {
    let mut move_str = String::new();
    return match io::stdin().read_line(&mut move_str) {
        Err(_) => {
            helpers::printerr("Couldn't read from cmdline");
            None
        },
        _ => {
            return Some(move_str.trim().to_string());
        },
    }
}

fn read_to_and_from_positions() -> (Option<(position::Position, position::Position)>, bool) {
    let move1 = read_move();
    if move1.is_none() {
        return (None, false);
    }

    let move1 = move1.unwrap();
    
    if move1.eq(EXIT_STRING) {
        return (None, true);
    }

    let from: position::Position;
    match position::Position::from_string(&move1) {
        Err(err) => {
            helpers::printerr(&err);
            return (None, false);
        },
        Ok(pos) => { from = pos; },
    };

    let move2 = read_move();
    if move2.is_none() {
        return (None, false);
    }
    let move2 = move2.unwrap();

    if move2.eq(EXIT_STRING) {
        return (None, true);
    }

    let to: position::Position;
    match position::Position::from_string(&move2) {
        Err(err) => {
            helpers::printerr(&err);
            return (None, false);
        },
        Ok(pos) => { to = pos; },
    };
    return (Some((from, to)), false);
}

pub fn start_game_loop() {
    let mut game = game::Game::new();

    // Initial show
    game.show();

    loop {

        println!("Enter your move (e.g. A1<enter>H8) : ");
        let read_res = read_to_and_from_positions();

        if read_res.1 {
            break;
        }
        if read_res.0.is_none() {
            continue;
        }
        let (from, to) = read_res.0.unwrap();
        if !game.is_move_valid(&from, &to) {
            let fromStr = from.to_string();
            let toStr = to.to_string();
            helpers::printerr((format!("Invalid move from '{fromStr}' to '{toStr}'")).as_str());
            continue;
        }
        game.move_piece(&from, &to);

        // Show after move
        game.show();
    }
}
