use std::fmt;

#[derive(PartialEq)]
enum Color
{
    White,
    Black,
}

enum PieceType
{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Piece
{
    color: Color,
    piece: PieceType,
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

#[cfg(test)]
mod tests
{
    use super::Color::*;
    use super::Piece;
    use super::PieceType::*;

    #[test]
    fn test_piece_display()
    {
        assert_eq!(Piece { color: White, piece: Pawn }.to_string(), "P");
        assert_eq!(Piece { color: Black, piece: Pawn }.to_string(), "p");
        assert_eq!(Piece { color: White, piece: Knight }.to_string(), "N");
        assert_eq!(Piece { color: Black, piece: Knight }.to_string(), "n");
        assert_eq!(Piece { color: White, piece: Bishop }.to_string(), "B");
        assert_eq!(Piece { color: Black, piece: Bishop }.to_string(), "b");
        assert_eq!(Piece { color: White, piece: Rook }.to_string(), "R");
        assert_eq!(Piece { color: Black, piece: Rook }.to_string(), "r");
        assert_eq!(Piece { color: White, piece: Queen }.to_string(), "Q");
        assert_eq!(Piece { color: Black, piece: Queen }.to_string(), "q");
        assert_eq!(Piece { color: White, piece: King }.to_string(), "K");
        assert_eq!(Piece { color: Black, piece: King }.to_string(), "k");
    }
}
