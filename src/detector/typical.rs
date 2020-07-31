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
}

impl TypicalStatus {
    fn new(words_len: usize) -> TypicalStatus {
        TypicalStatus {
            col: 1,
            row: 1,
            word_active: vec![Cell::new(false); words_len],
        }
    }
}

/// This detects if there isn't a comma after a word that is tipically introduces clauses.
///
/// Example: Reméljük, nem esett baja. (there is a "hogy" missing in the sentence)
///  We hope that he/she/it wasn't hurt.
pub struct TypicalDetector {
    settings: TypicalSettings,
    status: TypicalStatus,
}

impl TypicalDetector {
    fn new(settings: TypicalSettings) -> TypicalDetector {
        TypicalDetector {
            status: TypicalStatus::new(settings.words.len()),
            settings
        }
    }
}

impl Detector for TypicalDetector {
    fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.status = TypicalStatus::new(self.settings.words.len());

        self.detect_errors_in_row(tokens)
    }

    fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        unimplemented!()
    }
}