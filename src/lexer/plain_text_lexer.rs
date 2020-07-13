use crate::model::Token;

pub struct PlainTextLexer {
    row: usize,
    col: usize,

}

impl<'a> PlainTextLexer {
    pub fn new() -> PlainTextLexer {
        PlainTextLexer {
            row: 1,
            col: 1,
        }
    }
    pub fn tokenize(&mut self, text: &'a str) -> Vec<Token<'a>> {
        let words: Vec<&str> = text.split(" ").collect();
        let mut tokens = Vec::new();

        self.col = 1;
        self.row = 1;

        for word in words.iter() {
            if word.contains("\n") {
                let sp = word.split("\n").collect::<Vec<&str>>();
                for s in sp {
                    tokens.push(Token::new(s, self.row, self.col));
                    self.col = 1;
                    self.row += 1;
                }
            } else {
                tokens.push(Token::new(word, self.row, self.col));
                self.col += word.len() + 1;
            }
        }

        tokens
    }

    pub fn tokenize_row(&mut self, row: &'a str) -> Vec<Token<'a>> {
        let words: Vec<&str> = row.split(" ").collect();
        let mut tokens = Vec::new();

        self.col = 1;

        for word in words.iter() {
                tokens.push(Token::new(word, self.row, self.col));
                self.col += word.len() + 1;
        }

        self.row += 1;

        tokens
    }

}
#[cfg(test)]
mod tests {
    use crate::lexer::PlainTextLexer;

    #[test]
    fn simple_tokenize() {
        let mut sut = PlainTextLexer::new();
        let v = sut.tokenize("Kicsi kutya tarka!");
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn detect_is_comma() {
        let mut sut = PlainTextLexer::new();
        let v = sut.tokenize("Kicsi, kutya tarka!");
        assert!(v[0].is_comma)
    }

    #[test]
    fn detect_text() {
        let mut sut = PlainTextLexer::new();
        let v = sut.tokenize("Kicsi kutya tarka!");
        assert!(!v[0].is_comma && !v[0].is_full_stop)
    }

    #[test]
    fn detect_full_stop() {
        let mut sut = PlainTextLexer::new();
        let v = sut.tokenize("Kicsi. kutya tarka!");
        assert!(v[0].is_full_stop)
    }

}
