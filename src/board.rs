use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::bitboard::BitBoard;
use crate::bitboard::BitBoardType::{self, *};
use crate::piece::Piece;

/// A trait representing anything that can represent a full game state.
pub trait GameState
{
    fn new() -> Self;
    fn start_of_game() -> Self;
    fn as_piece_array(&self) -> [Option<Piece>; 64];
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

    fn as_piece_array(&self) -> [Option<Piece>; 64]
    {
        let mut pieces: [Option<Piece>; 64] = [None; 64];

        for (bitboard_type, bitboard) in self.state.iter() {
            match bitboard_type {
                WhitePawns | WhiteKnights | WhiteBishops | WhiteRooks
                | WhiteQueens | WhiteKings | BlackPawns | BlackKnights
                | BlackBishops | BlackRooks | BlackQueens | BlackKings => {
                    bitboard
                        .as_piece_array(bitboard_type.get_piece().unwrap())
                        .iter()
                        .enumerate()
                        .for_each(|(i, piece)| {
                            if piece.is_some() {
                                pieces[i] = *piece
                            }
                        });
                },
                _ => (),
            }
        }

        pieces
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use strum::IntoEnumIterator;

    use super::{BitBoardState, GameState};
    use crate::bitboard::{BitBoard, BitBoardType};
    use crate::piece::Piece;

    #[test]
    fn test_start_of_game_state()
    {
        let state = BitBoardState::start_of_game().state;
        for bitboard_type in BitBoardType::iter() {
            let bitboard = state.get(&bitboard_type).unwrap();
            assert_eq!(*bitboard, BitBoard::default_from_type(&bitboard_type))
        }
    }

    #[test]
    fn test_into_piece_array()
    {
        let array = BitBoardState::start_of_game().as_piece_array();

        let white_pawn = Piece::from_str("P").unwrap();
        let white_knight = Piece::from_str("N").unwrap();
        let white_rook = Piece::from_str("R").unwrap();
        let white_king = Piece::from_str("K").unwrap();
        let black_pawn = Piece::from_str("p").unwrap();
        let black_rook = Piece::from_str("r").unwrap();
        let black_bishop = Piece::from_str("b").unwrap();
        let black_queen = Piece::from_str("q").unwrap();

        assert_eq!(array[0], Some(white_rook));
        assert_eq!(array[7], Some(white_rook));
        assert_eq!(array[8], Some(white_pawn));
        assert_eq!(array[15], Some(white_pawn));
        assert_eq!(array[63], Some(black_rook));
        assert_eq!(array[56], Some(black_rook));
        assert_eq!(array[55], Some(black_pawn));
        assert_eq!(array[48], Some(black_pawn));

        assert_eq!(array[4], Some(white_king));
        assert_eq!(array[59], Some(black_queen));
        assert_eq!(array[1], Some(white_knight));
        assert_eq!(array[61], Some(black_bishop));

        assert_eq!(array[16], None);
        assert_eq!(array[47], None);
        assert_eq!(array[30], None);
        assert_eq!(array[40], None);
    }
}
