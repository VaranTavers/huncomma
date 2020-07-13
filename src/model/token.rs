pub struct Token<'a> {
    pub text: &'a str,
    pub is_comma: bool,
    pub is_full_stop: bool,
    pub row: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    pub fn new(text: &'a str, row: usize, col: usize) -> Token<'a> {
        Token {
            text,
            is_comma: text.ends_with(","),
            is_full_stop: text.ends_with("."),
            row,
            col
        }
    }

}
