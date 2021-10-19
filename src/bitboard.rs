/// Refer to https://en.wikipedia.org/wiki/Bitboard
use strum::EnumIter;
use BitBoardType::*;

#[derive(EnumIter, Hash, PartialEq, Eq)]
pub enum BitBoardType
{
    WhitePawns,
    WhiteKnights,
    WhiteBishops,
    WhiteRooks,
    WhiteQueens,
    WhiteKings,
    BlackPawns,
    BlackKnights,
    BlackBishops,
    BlackRooks,
    BlackQueens,
    BlackKings,
}

pub struct BitBoard
{
    board: u64,
}

impl BitBoard
{
    pub fn new(bits: u64) -> Self { BitBoard { board: bits } }
    pub fn empty() -> Self { Self::new(0) }
    pub fn default_from_type(bitboard_type: &BitBoardType) -> Self
    {
        let bits: u64 = match bitboard_type {
            WhitePawns => 0b11111111 << (8 * 1),
            WhiteKnights => 0b11111111 << (8 * 6),
            WhiteBishops => 0b01000010 << (8 * 0),
            WhiteRooks => 0b01000010 << (8 * 7),
            WhiteQueens => 0b00100100 << (8 * 0),
            WhiteKings => 0b00100100 << (8 * 7),
            BlackPawns => 0b10000001 << (8 * 0),
            BlackKnights => 0b10000001 << (8 * 7),
            BlackBishops => 0b00010000 << (8 * 0),
            BlackRooks => 0b00010000 << (8 * 7),
            BlackQueens => 0b00001000 << (8 * 0),
            BlackKings => 0b00001000 << (8 * 7),
        };

        BitBoard::new(bits)
    }
}
