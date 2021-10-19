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
            WhiteKnights => 0b01000010,
            WhiteBishops => 0b00100100,
            WhiteRooks => 0b10000001,
            WhiteQueens => 0b00001000,
            WhiteKings => 0b00010000,
            BlackPawns => 0b11111111 << (8 * 6),
            BlackKnights => 0b01000010 << (8 * 7),
            BlackBishops => 0b00100100 << (8 * 7),
            BlackRooks => 0b10000001 << (8 * 7),
            BlackQueens => 0b00001000 << (8 * 7),
            BlackKings => 0b00010000 << (8 * 7),
        };

        BitBoard::new(bits)
    }

    pub fn into_piece_array(&self, piece: Piece) -> [Option<Piece>; 64]
    {
        let mut pieces = [None; 64];
        let mut bits = self.bits;

        for i in 0..64 {
            if bits & 1 == 1 {
                pieces[i] = Some(piece);
            }
            bits >>= 1;
        }

        pieces
    }
}

#[cfg(test)]
mod tests
{
    use crate::bitboard::BitBoard;
    use crate::piece::Color::*;
    use crate::piece::Piece;
    use crate::piece::PieceType::*;

    #[test]
    fn test_empty_bitboard_returns_no_pieces_in_array()
    {
        let array = BitBoard::empty()
            .into_piece_array(Piece { color: White, piece: Pawn });

        for piece in array.iter() {
            assert!(piece.is_none())
        }
    }

    #[test]
    fn test_bitboard_returns_correct_pieces_in_array()
    {
        let white_bishop = Piece { color: White, piece: Bishop };

        let array = BitBoard::new(0b00100100).into_piece_array(white_bishop);

        assert_eq!(array[2], Some(white_bishop));
        assert_eq!(array[5], Some(white_bishop));
        assert_eq!(array[6], None);
        assert_eq!(array[4], None);
        assert_eq!(array[0], None);
    }
}
