use crate::bitboard::BitBoard;

/// A trait representing anything that can represent a full game state.
pub trait Board
{
    fn new() -> Self;
    fn start_of_game() -> Self;
}

/// A collection of BitBoards representing a full game state.
#[allow(dead_code)]
pub struct BitBoardState
{
    white_pawns:   BitBoard,
    black_pawns:   BitBoard,
    white_knights: BitBoard,
    black_knights: BitBoard,
    white_bishops: BitBoard,
    black_bishops: BitBoard,
    white_rooks:   BitBoard,
    black_rooks:   BitBoard,
    white_queens:  BitBoard,
    black_queens:  BitBoard,
    white_kings:   BitBoard,
    black_kings:   BitBoard,
}

impl BitBoardState {}

impl Board for BitBoardState
{
    fn new() -> Self
    {
        BitBoardState {
            white_pawns:   BitBoard::new(0),
            black_pawns:   BitBoard::new(0),
            white_knights: BitBoard::new(0),
            black_knights: BitBoard::new(0),
            white_bishops: BitBoard::new(0),
            black_bishops: BitBoard::new(0),
            white_rooks:   BitBoard::new(0),
            black_rooks:   BitBoard::new(0),
            white_queens:  BitBoard::new(0),
            black_queens:  BitBoard::new(0),
            white_kings:   BitBoard::new(0),
            black_kings:   BitBoard::new(0),
        }
    }

    fn start_of_game() -> Self
    {
        BitBoardState {
            white_pawns:   BitBoard::new(0b11111111 << (8 * 1)),
            black_pawns:   BitBoard::new(0b11111111 << (8 * 6)),
            white_knights: BitBoard::new(0b01000010 << (8 * 0)),
            black_knights: BitBoard::new(0b01000010 << (8 * 7)),
            white_bishops: BitBoard::new(0b00100100 << (8 * 0)),
            black_bishops: BitBoard::new(0b00100100 << (8 * 7)),
            white_rooks:   BitBoard::new(0b10000001 << (8 * 0)),
            black_rooks:   BitBoard::new(0b10000001 << (8 * 7)),
            white_queens:  BitBoard::new(0b00010000 << (8 * 0)),
            black_queens:  BitBoard::new(0b00010000 << (8 * 7)),
            white_kings:   BitBoard::new(0b00001000 << (8 * 0)),
            black_kings:   BitBoard::new(0b00001000 << (8 * 7)),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::BitBoardState;
}
