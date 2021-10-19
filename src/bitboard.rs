/// Refer to https://en.wikipedia.org/wiki/Bitboard

pub struct BitBoard
{
    board: u64,
}

impl BitBoard
{
    pub fn new(bits: u64) -> Self { BitBoard { board: bits } }
}
