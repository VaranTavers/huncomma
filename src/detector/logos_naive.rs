use crate::model::LogosToken;
use logos::Lexer;

pub struct LogosNaiveDetector<'a> {
    words: Vec<&'a str>,
    col: usize,
    row: usize,
}

impl<'a> LogosNaiveDetector<'a> {
    pub fn new() -> LogosNaiveDetector<'a> {
        let words = vec!["hogy", "ami", "aki", "ahol", "amikor", "amiért", "mert", "mint", "illetve", "amint", "valamint"];
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
        self.col = 1;
        self.row = 1;

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<LogosToken>) -> Vec<(usize, usize)> {
        let mut errors = Vec::new();
        let mut is_last_token_comma = false;
        let mut is_last_token_in_vec = false;
        while let Some(token) = tokens.next() {
            let is_current_token_in_vec = self.words.contains(&tokens.slice());
            if is_current_token_in_vec && !is_last_token_comma && !is_last_token_in_vec {
                errors.push((self.row, self.col));
            }
            self.col += tokens.span().len();
            if token == LogosToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            is_last_token_in_vec = is_current_token_in_vec;
            is_last_token_comma = token == LogosToken::Comma;
        }

        self.row += 1;

        errors
    }
}
