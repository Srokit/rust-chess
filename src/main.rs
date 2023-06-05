mod game_loop;
mod helpers;
mod game;
mod board;
mod piece;

fn main() {
    println!("START Chess!");
    game_loop::start_game_loop();
    println!("Exiting Chess!");
}
