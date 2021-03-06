use std::convert::TryFrom;
use std::error::Error;

use strum::Display;
/// Refer to https://en.wikipedia.org/wiki/Bitboard
use strum::EnumIter;
use BitBoardType::*;

use crate::piece::Color::*;
use crate::piece::Piece;
use crate::piece::PieceType::*;

#[non_exhaustive]
#[derive(EnumIter, Display, Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
    AllPieces,
}

impl From<Piece> for BitBoardType
{
    fn from(piece: Piece) -> Self
    {
        match piece {
            Piece { color: White, piece: Pawn } => WhitePawns,
            Piece { color: White, piece: Knight } => WhiteKnights,
            Piece { color: White, piece: Bishop } => WhiteBishops,
            Piece { color: White, piece: Rook } => WhiteRooks,
            Piece { color: White, piece: Queen } => WhiteQueens,
            Piece { color: White, piece: King } => WhiteKings,
            Piece { color: Black, piece: Pawn } => BlackPawns,
            Piece { color: Black, piece: Knight } => BlackKnights,
            Piece { color: Black, piece: Bishop } => BlackBishops,
            Piece { color: Black, piece: Rook } => BlackRooks,
            Piece { color: Black, piece: Queen } => BlackQueens,
            Piece { color: Black, piece: King } => BlackKings,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BitBoard
{
    pub board_type: BitBoardType,
    pub bits:       u64,
}

impl BitBoard
{
    pub fn new(board_type: BitBoardType, bits: u64) -> Self
    {
        BitBoard { board_type, bits }
    }
    pub fn empty(board_type: BitBoardType) -> Self { Self::new(board_type, 0) }

    pub fn default_for_type(board_type: BitBoardType) -> Self
    {
        let bits: u64 = match board_type {
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
            AllPieces => 0xffff00000000ffff,
        };

        BitBoard::new(board_type, bits)
    }

    pub fn as_piece_array(&self)
        -> Result<[Option<Piece>; 64], Box<dyn Error>>
    {
        let piece = Piece::try_from(self.board_type)?;

        let mut pieces = [None; 64];
        let mut bits = self.bits;

        for p in pieces.iter_mut() {
            if bits & 1 == 1 {
                *p = Some(piece);
            }
            bits >>= 1;
        }

        Ok(pieces)
    }
}

#[cfg(test)]
mod tests
{
    use super::BitBoardType::*;
    use crate::bitboard::BitBoard;
    use crate::piece::Color::*;
    use crate::piece::Piece;
    use crate::piece::PieceType::*;

    #[test]
    fn test_empty_bitboard_returns_no_pieces_in_array()
    {
        let array = BitBoard::empty(WhitePawns).as_piece_array().unwrap();

        for piece in array.iter() {
            assert!(piece.is_none())
        }
    }

    #[test]
    fn test_bitboard_returns_correct_pieces_in_array()
    {
        let white_bishop = Piece { color: White, piece: Bishop };

        let array =
            BitBoard::new(WhiteBishops, 0b00100100).as_piece_array().unwrap();

        assert_eq!(array[2], Some(white_bishop));
        assert_eq!(array[5], Some(white_bishop));
        assert_eq!(array[6], None);
        assert_eq!(array[4], None);
        assert_eq!(array[0], None);
    }
}
