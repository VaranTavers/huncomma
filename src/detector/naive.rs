use crate::model::{PlainTextToken, Mistake};
use logos::Lexer;

// TODO: Read words from settings file

/// Detects if there isn't a comma before the given words. These words are generally preceded by a
/// comma, however for most of them there are exceptions.
///
/// Every word is given a probability, which means the following: what is the probability of actually
/// needing a comma before that word.
///
/// Exception: if there are two of the given words immediately after each other, the second one
/// doesn't require a comma before it (the first one still does)
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
            ("és", 0.20),
            ("hogy", 0.70),
            // a + dolgok
            ("ahol", 0.50),
            ("amikor", 0.50),
            // ami + toldalék
            ("ami", 0.50),
            ("amit", 0.50),
            ("amitől", 0.50),
            ("aminek", 0.50),
            ("amiért", 0.50),
            // amik + toldalék
            ("amik", 0.50),
            ("amiket", 0.50),
            ("amiknek", 0.50),
            ("amiktől", 0.50),
            ("amikért", 0.50),
            // aki + toldalék
            ("aki", 0.50),
            ("akit", 0.50),
            ("akinek", 0.50),
            ("akiért", 0.50),
            ("akitől", 0.50),
            // akik + toldalék
            ("akik", 0.50),
            ("akiket", 0.50),
            ("akiknek", 0.50),
            ("akiktől", 0.50),
            ("akikért", 0.50),
            // a többi
            ("de", 0.50),
            ("hiszen", 0.50),
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

    pub fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.col = 1;
        self.row = 1;
        self.is_last_token_in_vec = false;
        self.is_last_token_comma = false;

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {

            let index = self.words.iter().position(|a| a == &tokens.slice());

            if !self.is_last_token_comma && !self.is_last_token_in_vec {
                if let Some(pos) = index {
                    errors.push((self.row,
                                 self.col,
                                 Mistake::new_dyn(format!("a(z) \"{}\" szó elé általában vesszőt rakunk.", self.words[pos]), self.probs[pos])
                    ));
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
