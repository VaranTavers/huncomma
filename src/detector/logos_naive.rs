use crate::model::PlainTextToken;
use logos::Lexer;

pub struct NaiveDetector<'a> {
    words: Vec<&'a str>,
    probs: Vec<f64>,
    col: usize,
    row: usize,
    is_last_token_comma: bool,
    is_last_token_in_vec: bool,
}

impl<'a> NaiveDetector<'a> {
    pub fn new() -> NaiveDetector<'a> {
        let word_probs = vec![
            ("hogy", 0.70),
            ("ami", 0.50),
            ("aki", 0.50),
            ("ahol", 0.50),
            ("amikor", 0.50),
            ("amiért", 0.50),
            ("mert", 0.50),
            ("mint", 0.80),
            ("illetve", 1.00),
            ("amint", 1.00),
            ("valamint", 1.00),
            ("ha", 1.00),
        ];
        NaiveDetector {
            words: word_probs.iter().map(|(a, _b)| *a).collect(),
            probs: word_probs.iter().map(|(_a, b)| *b).collect(),
            col: 1,
            row: 1,
            is_last_token_comma: false,
            is_last_token_in_vec: false,
        }
    }

    pub fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, f64)> {
        self.col = 1;
        self.row = 1;
        self.is_last_token_in_vec = false;
        self.is_last_token_comma = false;

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, f64)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {

            let index = self.words.iter().position(|a| a == &tokens.slice());

            if !self.is_last_token_comma && !self.is_last_token_in_vec {
                if let Some(pos) = index {
                    errors.push((self.row, self.col, self.probs[pos]));
                }
            }
            self.col += tokens.slice().chars().count() + 1;
            if token == PlainTextToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            self.is_last_token_in_vec = index.is_some();
            if token != PlainTextToken::NewLine {
                self.is_last_token_comma = token == PlainTextToken::Comma;
            }
        }

        self.row += 1;

        errors
    }
}
#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::NaiveDetector;
    use crate::model::PlainTextToken;

    #[test]
    fn empty_str() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_in_sight() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Ki kopog? Mi kopog? Egy fekete holló!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani, hogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_duplicate_words() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Nem értem, hogy hogy kellene ezt csinálni.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_one_line() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani hogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_next_line() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani\nhogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_multiple_lines() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani\n\n\n\nhogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_duplicate_words() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Nem értem hogy hogy kellene ezt csinálni.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_multiple_error_one_line() {
        let mut sut = NaiveDetector::new();
        let mut tokens = PlainTextToken::lexer("Nem értem hogy kellene ezt csinálni. Elmagyarázod ha szépen megkérlek?");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 2);
    }
}
