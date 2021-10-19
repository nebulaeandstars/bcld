mod bitboard;
mod board;
mod piece;

use board::{BitBoardState, GameState};

pub fn run()
{
    let board = BitBoardState::start_of_game();

    println!("{}", board);
}
