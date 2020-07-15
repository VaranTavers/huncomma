use crate::model::LogosToken;
use logos::Lexer;

pub struct LogosNaiveDetector<'a> {
    words: Vec<&'a str>,
    col: usize,
    row: usize,
}

impl<'a> LogosNaiveDetector<'a> {
    pub fn new() -> LogosNaiveDetector<'a> {
        let words = vec!["hogy", "ami", "aki", "ahol", "amikor", "amiért", "mert", "mint"];
        LogosNaiveDetector {
            words,
            col: 1,
            row: 1,
        }
    }

    pub fn new_from_words(words: Vec<&'a str>) -> LogosNaiveDetector<'a> {
        LogosNaiveDetector {
            words,
            col: 1,
            row: 1,
        }
    }

    pub fn detect_errors(&mut self, tokens: &mut Lexer<LogosToken>) -> Vec<(usize, usize)> {
        let mut errors = Vec::new();
        let mut commad = false;
        self.col = 1;
        self.row = 1;
        while let Some(token) = tokens.next() {
            if self.words.contains(&tokens.slice()) && !commad {
                errors.push((self.row, self.col));
            }
            self.col += tokens.span().len();
            if token == LogosToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            commad = token == LogosToken::Comma;
        }

        errors
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<LogosToken>) -> Vec<(usize, usize)> {
        let mut errors = Vec::new();
        let mut commad = false;
        while let Some(token) = tokens.next() {
            if self.words.contains(&tokens.slice()) && !commad {
                errors.push((self.row, self.col));
            }
            self.col += tokens.span().len();
            if token == LogosToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            commad = token == LogosToken::Comma;
        }

        errors
    }
}
