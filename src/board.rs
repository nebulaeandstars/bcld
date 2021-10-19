use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::bitboard::{BitBoard, BitBoardType};

/// A trait representing anything that can represent a full game state.
pub trait GameState
{
    fn new() -> Self;
    fn start_of_game() -> Self;
}

/// A collection of BitBoards representing a full game state.
#[allow(dead_code)]
pub struct BitBoardState
{
    state: HashMap<BitBoardType, BitBoard>,
}

impl BitBoardState {}

impl GameState for BitBoardState
{
    fn new() -> Self
    {
        let mut state = HashMap::new();
        for bitboard_type in BitBoardType::iter() {
            state.insert(bitboard_type, BitBoard::empty());
        }

        BitBoardState { state }
    }

    fn start_of_game() -> Self
    {
        let mut state = HashMap::new();
        for bitboard_type in BitBoardType::iter() {
            let bitboard = BitBoard::default_from_type(&bitboard_type);
            state.insert(bitboard_type, bitboard);
        }

        BitBoardState { state }
    }
}

#[cfg(test)]
mod tests
{
    use super::BitBoardState;
}
