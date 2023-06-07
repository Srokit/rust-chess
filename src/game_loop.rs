use std::io;

use crate::game;
use crate::helpers;
use crate::move_checker;
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
    let game = game::Game::new();
    let move_checker = move_checker::MoveChecker::new(game.get_board());

    // Initial show
    game.show();

    loop {

        println!("Enter your move (e.g. A1<enter>H8) : ");
        let read_res = read_to_and_from_positions();

        if read_res.1 {
            println!("yuh1");
            break;
        }
        if read_res.0.is_none() {
            println!("yuh2");
            continue;
        }
        let (from, to) = read_res.0.unwrap();
        if !move_checker.is_move_valid(&from, &to) {
            helpers::printerr(move_checker.get_last_err_str());
            println!("yuh4");
            continue;
        }

        // Show after move
        game.show();
    }
}
