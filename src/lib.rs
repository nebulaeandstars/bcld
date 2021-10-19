mod bitboard;
mod board;
mod piece;

pub fn add(a: isize, b: isize) -> isize { a + b }
pub fn run() { println!("Hello, world!") }

#[cfg(test)]
mod tests
{
    use super::add;

    #[test]
    fn test_add()
    {
        assert_eq!(add(2, 3), 5);
    }
}
