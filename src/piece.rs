use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color
{
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType
{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece
{
    pub color: Color,
    pub piece: PieceType,
}

impl FromStr for Piece
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        use Color::*;
        use PieceType::*;

        if s.len() != 1 {
            return Err(format!(
                "Cannot parse Piece from str of length {}!",
                s.len()
            )
            .into());
        }
        let c = s.chars().next().unwrap();

        let color = match c.is_uppercase() {
            true => White,
            false => Black,
        };

        let piece: Result<PieceType, Self::Err> = match c.to_ascii_uppercase() {
            'P' => Ok(Pawn),
            'N' => Ok(Knight),
            'B' => Ok(Bishop),
            'R' => Ok(Rook),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            _ => Err(format!("Cannot parse Piece from char {}", c).into()),
        };

        Ok(Piece { color, piece: piece? })
    }
}

impl fmt::Display for Color
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        use Color::*;

        write!(f, "{}", match &self {
            White => "w",
            Black => "b",
        })
    }
}

impl FromStr for Color
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s.to_lowercase().as_str() {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(format!("{} is not a valid color!", s).into()),
        }
    }
}

impl fmt::Display for PieceType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        use PieceType::*;

        write!(f, "{}", match self {
            Pawn => "P",
            Knight => "N",
            Bishop => "B",
            Rook => "R",
            Queen => "Q",
            King => "K",
        })
    }
}

impl fmt::Display for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        use Color::*;

        write!(f, "{}", match &self.color {
            White => self.piece.to_string(),
            Black => self.piece.to_string().to_lowercase(),
        })
    }
}

impl fmt::Debug for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use super::Color::{self, *};
    use super::Piece;
    use super::PieceType::*;

    #[test]
    fn test_piece_display()
    {
        assert_eq!(Piece { color: White, piece: Pawn }.to_string(), "P");
        assert_eq!(Piece { color: White, piece: Knight }.to_string(), "N");
        assert_eq!(Piece { color: White, piece: Bishop }.to_string(), "B");
        assert_eq!(Piece { color: Black, piece: Rook }.to_string(), "r");
        assert_eq!(Piece { color: Black, piece: Queen }.to_string(), "q");
        assert_eq!(Piece { color: Black, piece: King }.to_string(), "k");
    }

    #[test]
    fn test_piece_from_string()
    {
        assert_eq!(Piece::from_str("p").unwrap(), Piece {
            color: Black,
            piece: Pawn,
        });
        assert_eq!(Piece::from_str("n").unwrap(), Piece {
            color: Black,
            piece: Knight,
        });
        assert_eq!(Piece::from_str("b").unwrap(), Piece {
            color: Black,
            piece: Bishop,
        });
        assert_eq!(Piece::from_str("R").unwrap(), Piece {
            color: White,
            piece: Rook,
        });
        assert_eq!(Piece::from_str("Q").unwrap(), Piece {
            color: White,
            piece: Queen,
        });
        assert_eq!(Piece::from_str("K").unwrap(), Piece {
            color: White,
            piece: King,
        });
    }

    #[test]
    fn test_color_from_string()
    {
        assert_eq!(Color::from_str("w").unwrap(), White);
        assert_eq!(Color::from_str("b").unwrap(), Black);
        assert!(Color::from_str("invalid_color").is_err());
        assert!(Color::from_str("").is_err());
    }
}
