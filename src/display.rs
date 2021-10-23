/// Converts a given string buffer from text to unicode chess pieces.
pub fn convert_to_chess_pieces(s: &str) -> String
{
    let mut out = s.to_string();

    out = out.replace("P", "♙");
    out = out.replace("N", "♘");
    out = out.replace("B", "♗");
    out = out.replace("R", "♖");
    out = out.replace("Q", "♕");
    out = out.replace("K", "♔");
    out = out.replace("p", "♟︎");
    out = out.replace("n", "♞");
    out = out.replace("b", "♝");
    out = out.replace("r", "♜");
    out = out.replace("q", "♛");
    out = out.replace("k", "♚");

    out
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_convert_to_chess_pieces()
    {
        let mut source = String::new();
        source.push_str("r n b q k b n r \n");
        source.push_str("p p p p p p p p \n");
        source.push_str("                \n");
        source.push_str("                \n");
        source.push_str("                \n");
        source.push_str("                \n");
        source.push_str("P P P P P P P P \n");
        source.push_str("R N B Q K B N R \n");

        let mut target = String::new();
        target.push_str("♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n");
        target.push_str("♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("                \n");
        target.push_str("♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n");
        target.push_str("♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n");

        assert_eq!(convert_to_chess_pieces(&source), target)
    }
}
