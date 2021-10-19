mod bitboard;
mod board;
mod piece;

use board::{BitBoardState, GameState};

pub struct Square
{
    file: u8,
    rank: u8,
}

pub enum CastleAvailability
{
    Full,
    QueensideOnly,
    KingsideOnly,
    None,
}

pub fn run()
{
    let board = BitBoardState::start_of_game();

    println!("{}", board);
}
