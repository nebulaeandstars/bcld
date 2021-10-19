use std::error::Error;

use strum::Display;
/// Refer to https://en.wikipedia.org/wiki/Bitboard
use strum::EnumIter;
use BitBoardType::*;

use crate::piece::Color::*;
use crate::piece::Piece;
use crate::piece::PieceType::*;

#[derive(EnumIter, Display, Hash, PartialEq, Eq)]
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

impl BitBoardType
{
    pub fn get_piece(&self) -> Result<Piece, Box<dyn Error>>
    {
        match &self {
            WhitePawns => Ok(Piece { color: White, piece: Pawn }),
            WhiteKnights => Ok(Piece { color: White, piece: Knight }),
            WhiteBishops => Ok(Piece { color: White, piece: Bishop }),
            WhiteRooks => Ok(Piece { color: White, piece: Rook }),
            WhiteQueens => Ok(Piece { color: White, piece: Queen }),
            WhiteKings => Ok(Piece { color: White, piece: King }),
            BlackPawns => Ok(Piece { color: Black, piece: Pawn }),
            BlackKnights => Ok(Piece { color: Black, piece: Knight }),
            BlackBishops => Ok(Piece { color: Black, piece: Bishop }),
            BlackRooks => Ok(Piece { color: Black, piece: Rook }),
            BlackQueens => Ok(Piece { color: Black, piece: Queen }),
            BlackKings => Ok(Piece { color: Black, piece: King }),
            _ =>
                Err(format!("BitBoardType {} has no associated Piece!", &self)
                    .into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BitBoard
{
    pub bits: u64,
}

impl BitBoard
{
    pub fn new(bits: u64) -> Self { BitBoard { bits } }
    pub fn empty() -> Self { Self::new(0) }
    pub fn default_from_type(bitboard_type: &BitBoardType) -> Self
    {
        let bits: u64 = match bitboard_type {
            WhitePawns => 0b11111111 << 8,
            WhiteKnights => 0b11111111 << (8 * 6),
            WhiteBishops => 0b01000010,
            WhiteRooks => 0b01000010 << (8 * 7),
            WhiteQueens => 0b00100100,
            WhiteKings => 0b00100100 << (8 * 7),
            BlackPawns => 0b10000001,
            BlackKnights => 0b10000001 << (8 * 7),
            BlackBishops => 0b00010000,
            BlackRooks => 0b00010000 << (8 * 7),
            BlackQueens => 0b00001000,
            BlackKings => 0b00001000 << (8 * 7),
        };

        BitBoard::new(bits)
    }
}
