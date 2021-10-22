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
                'K' => Ok(out.white.0 = true),
                'Q' => Ok(out.white.1 = true),
                'k' => Ok(out.black.0 = true),
                'q' => Ok(out.black.1 = true),
                _ =>
                    Err(format!("unknown symbol {} in castle availability!", c)
                        .into()),
            };
            result?;
        }

        Ok(out)
    }
}

impl FromStr for Square
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        unimplemented!();
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

impl std::fmt::Display for Square
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut out = String::new();
        out.push('-');

        write!(f, "{}", out)
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

    use super::CastleAvailability;

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
}
