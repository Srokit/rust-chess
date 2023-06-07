mod board;
mod game;
mod game_loop;
mod helpers;
mod piece;
mod position;
mod move_checker;

fn main() {
    println!("START Chess!");
    game_loop::start_game_loop();
    println!("Exiting Chess!");
}
