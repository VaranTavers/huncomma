use std::cell::Cell;
use crate::traits::Detector;
use logos::Lexer;
use crate::model::{Mistake, PlainTextToken, TypicalSettings};

/// Contains the status of a TypicalDetector (row, column, first_word_active)
///
/// Generally you shouldn't bother with it.
struct TypicalStatus {
    pub col: usize,
    pub row: usize,
    pub word_active: Vec<Cell<bool>>,
    pub comma_active: bool,
}

impl TypicalStatus {
    fn new(words_len: usize) -> TypicalStatus {
        TypicalStatus {
            col: 1,
            row: 1,
            word_active: vec![Cell::new(false); words_len],
            comma_active: false,
        }
    }
}

/// This detects if there isn't a comma after a word that is tipically introduces clauses.
///
/// Example: Reméljük, nem esett baja. (there is an implicit "hogy")
///  We hope that he/she/it wasn't hurt.
pub struct TypicalDetector {
    settings: TypicalSettings,
    status: TypicalStatus,
}

impl TypicalDetector {
    pub fn new(settings: TypicalSettings) -> TypicalDetector {
        TypicalDetector {
            status: TypicalStatus::new(settings.words.len()),
            settings
        }
    }

    fn move_cursor_forward(&mut self, current_token: &PlainTextToken, tokens: &Lexer<PlainTextToken>) {
        self.status.col += tokens.slice().chars().count() + 1;
        if *current_token == PlainTextToken::NewLine {
            self.status.col = 1;
            self.status.row += 1;
        }
    }

    fn get_mistake_for_word(&self, pos1: usize) -> (usize, usize, Mistake) {
        (
            self.status.row,
            self.status.col,
            Mistake::new_dyn(
                format!("mondatokba, melyekben szerepel a(z) \"{}\" szó, gyakran teszünk vesszőt.", self.settings.words[pos1]),
                self.settings.probs[pos1]
            )
        )
    }

    fn set_active_word(&mut self, word: &str) {
        let index = self.settings.words.iter().position(|a| a == word);
        if let Some(pos) = index {
            self.status.word_active[pos].set(true);
        }
    }
}

impl Detector for TypicalDetector {
    fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.status = TypicalStatus::new(self.settings.words.len());

        self.detect_errors_in_row(tokens)
    }

    fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {
            let lowercase = String::from(tokens.slice()).to_lowercase();

            if token == PlainTextToken::EndOfSentence && !self.status.comma_active {
                for (pos, _) in self.status.word_active.iter().enumerate().filter(|(_, a)| a.get()) {
                    errors.push(self.get_mistake_for_word(pos));
                }
            }

            self.move_cursor_forward(&token, tokens);
            self.set_active_word(lowercase.as_str());

            if token == PlainTextToken::Comma {
                self.status.comma_active = true;
            }
            if token == PlainTextToken::EndOfSentence {
                self.status.comma_active = false;
                self.status.word_active.iter().for_each(|a| a.set(false));
            }
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::TypicalDetector;
    use crate::model::{PlainTextToken, TypicalSettings};
    use crate::traits::Detector;

    #[test]
    fn empty_str() {
        let mut sut = TypicalDetector::new(TypicalSettings { words: Vec::new(), probs: Vec::new()});
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = TypicalDetector::new(TypicalSettings { words: vec![String::from("remélem"),], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Remélem, jól van.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided_before() {
        let mut sut = TypicalDetector::new(TypicalSettings { words: vec![String::from("remélem"),], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Jól van, remélem.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn semicolon_provided() {
        let mut sut = TypicalDetector::new(TypicalSettings { words: vec![String::from("remélem"),], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Remélem; jól van.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_missing() {
        let mut sut = TypicalDetector::new(TypicalSettings { words: vec![String::from("remélem"),], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Remélem jól van.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }
}