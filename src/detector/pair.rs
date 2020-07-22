use crate::model::PlainTextToken;
use logos::Lexer;
use std::cell::Cell;

/// Detects pairs of words which usually require a comma between them, these words don't have to be
/// right next to each other! Each pair is given a probability, just like in the NaiveDetector.
///
/// This doesn't detect if the comma is in the wrong place between the two words! (Not sure yet)
///
/// Example: ha ... akkor: Ha kimész, akkor tudsz hozni nekem egy hamburgert?
/// (If you go out then can you bring me a hamburger?)
///
///
pub struct PairDetector<'a> {
    first_words: Vec<&'a str>,
    second_words: Vec<&'a str>,
    probs: Vec<f64>,
    col: usize,
    row: usize,
    first_word_active: Vec<Cell<bool>>,
}

impl<'a> PairDetector<'a> {
    pub fn new() -> PairDetector<'a> {
        let word_probs = vec![
            ("és", "", 0.50),
            ("hogy", "", 0.70),
            ("ami", "", 0.50),
            ("aki", "", 0.50),
            ("ahol", "", 0.50),
            ("amikor", "", 0.50),
            ("amiért", "", 0.50),
            ("mert", "", 0.50),
            ("mint", "", 0.80),
            ("illetve", "", 1.00),
            ("amint", "", 1.00),
            ("valamint", "", 1.00),
            ("ha", "", 1.00),
        ];
        PairDetector {
            first_words: word_probs.iter().map(|(a, _b, _c)| *a).collect(),
            second_words: word_probs.iter().map(|(_a, b, _c)| *b).collect(),
            probs: word_probs.iter().map(|(_a, _b, c)| *c).collect(),
            col: 1,
            row: 1,
            first_word_active: vec![Cell::new(false); word_probs.len()],
        }
    }

    pub fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, f64)> {
        self.col = 1;
        self.row = 1;

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, f64)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {

            // TODO: Actually to do it
        }

        self.row += 1;

        errors
    }
}
#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::PairDetector;
    use crate::model::PlainTextToken;

    #[test]
    fn empty_str() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }
}
