use crate::model::{PlainTextToken, Mistake};
use logos::Lexer;
use std::cell::Cell;

/// Detects pairs of words which usually require a comma between them, these words don't have to be
/// right next to each other! Each pair is given a probability, just like in the NaiveDetector.
///
/// All input will be converted to lowercase!
///
/// This doesn't detect if the comma is in the wrong place between the two words! (Not sure yet)
///
/// Example: ha ... akkor: Ha mész vásárolni, akkor ne felejts el tejet hozni!
/// (If you go shopping don't forget to buy milk!)
///
///
pub struct PairDetector<'a> {
    first_words: Vec<&'a str>,
    second_words: Vec<Vec<&'a str>>,
    probs: Vec<f64>,
    col: usize,
    row: usize,
    first_word_active: Vec<Cell<bool>>,
}

impl<'a> PairDetector<'a> {
    pub fn new() -> PairDetector<'a> {
        let word_probs = vec![
            ("mind", vec!["mind"], 0.80),
            ("azt", vec!["hogy", "akit", "amit"], 0.80),
            ("az", vec!["hogy"], 0.80),
            ("mivel", vec!["ezért"], 0.80),
            ("ha", vec!["akkor"], 0.80),
            ("abban", vec!["hogy"], 0.80),
            // Pairs starting with "olyan", may be separated later
            ("olyan", vec!["mint", "aki", "akit", "akiért", "ami", "amit", "amiért"], 0.50),
        ];
        PairDetector {
            first_words: word_probs.iter().map(|(a, _b, _c)| *a).collect(),
            second_words: word_probs.iter().map(|(_a, b, _c)| b.clone()).collect(),
            probs: word_probs.iter().map(|(_a, _b, c)| *c).collect(),
            col: 1,
            row: 1,
            first_word_active: vec![Cell::new(false); word_probs.len()],
        }
    }

    pub fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.col = 1;
        self.row = 1;
        self.first_word_active.iter().for_each(|a| a.set(false));

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {
            let lowercase = String::from(tokens.slice()).to_lowercase();
            // Checks for every active (which already appeared) first word if any of it's second words
            // is the current one. If it is, then there might be a missing comma.
            for (index, active) in self.first_word_active.iter().enumerate() {
                if active.get() {
                    let second_index = self.second_words[index].iter().position(|a| a == &lowercase.as_str());
                    if let Some(pos) = second_index {
                        errors.push(
                            (self.row, self.col,
                             Mistake::new_dyn(
                                 format!("a(z) \"{}\" és \"{}\" szavak közé általában vesszőt rakunk (általában a második elé).", self.first_words[index], self.second_words[index][pos]),
                                 self.probs[pos]
                             )
                        ));
                    }
                }
            }

            self.col += tokens.slice().chars().count() + 1;
            if token == PlainTextToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            let index = self.first_words.iter().position(|a| a == &lowercase.as_str());
            if let Some(pos) = index {
                self.first_word_active[pos].set(true);
            }

            // If it's a comma or a new sentence, then we don't need to check anymore if it is missing between words.
            if token == PlainTextToken::Comma || token == PlainTextToken::EndOfSentence {
                self.first_word_active.iter().for_each(|a| a.set(false));
            }
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

    #[test]
    fn comma_present() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Mind a tanárok, mind a diákok egyetértenek abban, hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn one_comma_missing() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Mind a tanárok mind a diákok egyetértenek abban, hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn both_commas_missing() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Mind a tanárok mind a diákok egyetértenek abban hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn example_correct() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Ha mész vásárolni, akkor ne felejts el tejet hozni!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn example_incorrect() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Ha mész vásárolni akkor ne felejts el tejet hozni!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_detection_over_sentences() {
        let mut sut = PairDetector::new();
        let mut tokens = PlainTextToken::lexer("Mind hősök voltak ők! Mind az a tizenhárom, kit várt a vérpad!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }
}
