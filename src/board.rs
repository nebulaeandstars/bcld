use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use strum::IntoEnumIterator;

use crate::bitboard::BitBoard;
use crate::bitboard::BitBoardType::{self, *};
use crate::piece::{Color, Piece};
use crate::{CastleAvailability, Square};

/// A trait representing anything that can represent a full game state.
pub trait GameState
{
    fn new() -> Self;
    fn start_of_game() -> Self;
    fn from_fen(s: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>;

    fn as_piece_array(&self) -> [Option<Piece>; 64];
    fn as_fen(&self) -> String;
}

/// A collection of BitBoards representing a full game state.
pub struct BitBoardState
{
    state:               HashMap<BitBoardType, BitBoard>,
    turn:                Color,
    castle_availability: CastleAvailability,
    en_passant_target:   Option<Square>,
    halfmove_clock:      u8,
    move_number:         u16,
}

impl BitBoardState {}

impl GameState for BitBoardState
{
    fn new() -> Self
    {
        let mut state = HashMap::new();
        for bitboard_type in BitBoardType::iter() {
            state.insert(bitboard_type, BitBoard::empty(bitboard_type));
        }

        BitBoardState { state, ..Default::default() }
    }

    fn start_of_game() -> Self
    {
        let mut state = HashMap::new();
        for bitboard_type in BitBoardType::iter() {
            let bitboard = BitBoard::default_for_type(bitboard_type);
            state.insert(bitboard_type, bitboard);
        }

        BitBoardState { state, ..Default::default() }
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

    fn as_fen(&self) -> String
    {
        let array = self.as_piece_array();
        let mut out = String::new();

        let mut tmp = String::new();
        let mut empty_squares = 0;

        for i in 1..=array.len() {
            if let Some(piece) = array[i - 1] {
                if empty_squares > 0 {
                    tmp.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                tmp.push_str(&format!("{}", piece));
            }
            else {
                empty_squares += 1;
            }
            if i % 8 == 0 {
                if empty_squares > 0 {
                    tmp.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                out.insert_str(0, &format!("{}/", tmp));
                tmp = String::new();
            }
        }

        let en_passant_target = match &self.en_passant_target {
            Some(square) => square.to_string(),
            None => String::from("-"),
        };

        out.pop();
        out.push_str(&format!(
            " {} {} {} {} {}",
            self.turn,
            self.castle_availability.to_string(),
            en_passant_target,
            self.halfmove_clock,
            self.move_number,
        ));

        out
    }

    fn from_fen(s: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    {
        let mut fen = s.split_whitespace();

        let pieces = fen.next().ok_or("could not find pieces in fen!")?;

        let turn =
            Color::from_str(fen.next().ok_or("could not find turn in fen!")?)?;

        let castle_availability = CastleAvailability::from_str(
            fen.next().ok_or("could not find castle availability in fen!")?,
        )?;

        let en_passant_target = Square::from_str(
            fen.next().ok_or("could not find en passant target in fen!")?,
        )
        .ok();

        let halfmove_clock = fen
            .next()
            .ok_or("could not find halfmove clock in fen!")?
            .parse::<u8>()?;

        let move_number = fen
            .next()
            .ok_or("could not find move number in fen!")?
            .parse::<u16>()?;

        let mut i: u8 = 0;
        let mut state: HashMap<BitBoardType, BitBoard> = HashMap::new();
        for piece in pieces.chars() {
            if piece == '/' {
                continue;
            }
            else if piece.is_numeric() {
                i += piece as u8 - b'0';
                continue;
            }
            else {
                let bitboard_type =
                    BitBoardType::from(Piece::from_str(&piece.to_string())?);

                let bitboard = state
                    .entry(bitboard_type)
                    .or_insert_with(|| BitBoard::empty(bitboard_type));

                let file = i % 8;
                let rank = 7 - (i / 8);

                let square_index = rank * 8 + file;
                bitboard.bits |= 1 << square_index;

                i += 1;
            }
        }

        Ok(Box::new(Self {
            state,
            turn,
            castle_availability,
            en_passant_target,
            halfmove_clock,
            move_number,
        }))
    }
}

impl Default for BitBoardState
{
    fn default() -> Self
    {
        let mut state = HashMap::new();
        for bitboard_type in BitBoardType::iter() {
            let bitboard = BitBoard::default_for_type(bitboard_type);
            state.insert(bitboard_type, bitboard);
        }

        BitBoardState {
            state,
            turn: Color::White,
            en_passant_target: None,
            castle_availability: CastleAvailability::default(),
            halfmove_clock: 0,
            move_number: 1,
        }
    }
}

impl fmt::Display for BitBoardState
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let array = self.as_piece_array();
        let mut out = String::new();
        let mut tmp = String::new();

        for i in 1..=array.len() {
            if let Some(piece) = array[i - 1] {
                tmp.push_str(&format!("{} ", piece));
            }
            else {
                tmp.push_str("  ");
            }
            if i % 8 == 0 {
                out.insert_str(0, &format!("{}\n", tmp));
                tmp = String::new();
            }
        }

        write!(f, "{}", out)
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
            assert_eq!(*bitboard, BitBoard::default_for_type(bitboard_type))
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

    #[test]
    fn test_board_display()
    {
        let board = BitBoardState::start_of_game();
        let mut target = String::new();
        target.push_str("r n b q k b n r \n");
        target.push_str("p p p p p p p p \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("P P P P P P P P \n");
        target.push_str("R N B Q K B N R \n");

        assert_eq!(format!("{}", board), target.to_string())
    }

    #[test]
    fn test_bitboard_state_to_fen()
    {
        let board = BitBoardState::start_of_game();
        let target = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        assert_eq!(board.as_fen(), target.to_string())
    }

    #[test]
    fn test_bitboard_state_from_fen()
    {
        let tests = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        ];

        for test in tests {
            let board = BitBoardState::from_fen(test).unwrap();
            assert_eq!(board.as_fen(), test.to_string())
        }
    }
}
