mod bitboard;
mod board;
mod piece;

use std::str::FromStr;

use board::{BitBoardState, GameState};

pub struct Square
{
    file: u8,
    rank: u8,
}

pub struct CastleAvailability
{
    white: (bool, bool),
    black: (bool, bool),
}


impl FromStr for Square
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() != 2 {
            return Err(format!("invalid square {}!", s).into());
        }

        let mut s = s.chars();
        let file = s.next().unwrap() as u8 - b'a' + 1;
        let rank = s.next().unwrap() as u8 - b'1' + 1;

        Ok(Square { file, rank })
    }
}

impl FromStr for CastleAvailability
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() > 4 {
            return Err("invalid castle availability length!".into());
        }

        let mut out =
            CastleAvailability { white: (false, false), black: (false, false) };

        for c in s.chars() {
            let result: Result<(), Self::Err> = match c {
                'K' => {
                    out.white.0 = true;
                    Ok(())
                },
                'Q' => {
                    out.white.1 = true;
                    Ok(())
                },
                'k' => {
                    out.black.0 = true;
                    Ok(())
                },
                'q' => {
                    out.black.1 = true;
                    Ok(())
                },
                _ =>
                    Err(format!("unknown symbol {} in castle availability!", c)
                        .into()),
            };
            result?;
        }

        Ok(out)
    }
}

impl std::fmt::Display for Square
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{}{}",
            char::from_u32((self.file + b'a' - 1) as u32).unwrap(),
            self.rank
        )
    }
}

impl std::fmt::Display for CastleAvailability
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut out = String::new();

        if self.white.0 {
            out.push('K')
        }
        if self.white.1 {
            out.push('Q')
        }
        if self.black.0 {
            out.push('k')
        }
        if self.black.1 {
            out.push('q')
        }
        if out.is_empty() {
            out.push('-')
        }

        write!(f, "{}", out)
    }
}

impl Default for CastleAvailability
{
    fn default() -> Self
    {
        CastleAvailability { white: (true, true), black: (true, true) }
    }
}

pub fn run()
{
    let board = BitBoardState::start_of_game();

    println!("{}", board);
}

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use super::{CastleAvailability, Square};

    #[test]
    fn test_castle_availability_from_string()
    {
        let tests = vec!["KQkq", "Kkq", "kq", "q"];

        for test in tests {
            assert_eq!(
                CastleAvailability::from_str(test).unwrap().to_string(),
                test
            )
        }

        assert!(CastleAvailability::from_str("invalid").is_err());
        assert!(CastleAvailability::from_str("KQKQKQ").is_err())
    }

    #[test]
    fn test_square_to_string()
    {
        assert_eq!(Square { file: 5, rank: 4 }.to_string(), "e4");
        assert_eq!(Square { file: 3, rank: 4 }.to_string(), "c4");
        assert_eq!(Square { file: 8, rank: 8 }.to_string(), "h8");
        assert_eq!(Square { file: 5, rank: 2 }.to_string(), "e2");
        assert_eq!(Square { file: 1, rank: 1 }.to_string(), "a1");
    }

    #[test]
    fn test_square_from_string()
    {
        let tests = vec!["e4", "c5", "a4", "g7", "b3"];

        for test in tests {
            assert_eq!(Square::from_str(test).unwrap().to_string(), test)
        }

        assert!(Square::from_str("invalid").is_err());
        assert!(Square::from_str("").is_err())
    }
}
