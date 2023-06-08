mod board;
mod check_move;
mod game;
mod game_loop;
mod helpers;
mod piece;
mod position;

fn main() {
    println!("START Chess!");
    game_loop::start_game_loop();
    println!("Exiting Chess!");
}
