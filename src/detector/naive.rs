use crate::model::{PlainTextToken, Mistake, NaiveSettings};
use logos::Lexer;
use crate::traits::Detector;

/// Contains the status of the current NaiveDetector (row, column, is_last_token_comma, is_last_token_in_vec)
///
/// Generally you shouldn't bother with it.
struct NaiveStatus {
    col: usize,
    row: usize,
    is_last_token_comma: bool,
    is_last_token_in_vec: bool,
}

impl NaiveStatus {
    pub fn new() -> NaiveStatus {
        NaiveStatus {
            col: 1,
            row: 1,
            is_last_token_comma: false,
            is_last_token_in_vec: false,
        }
    }
}

/// Detects if there isn't a comma before the given words. These words are generally preceded by a
/// comma, however for most of them there are exceptions.
///
/// Every word is given a probability, which means the following: what is the probability of actually
/// needing a comma before that word.
///
/// Exception: if there are two of the given words immediately after each other, the second one
/// doesn't require a comma before it (the first one still does)
pub struct NaiveDetector {
    settings: NaiveSettings,
    status: NaiveStatus,
}

impl NaiveDetector {
    pub fn new(settings: NaiveSettings) -> NaiveDetector {
        NaiveDetector {
            settings,
            status: NaiveStatus::new(),
        }
    }

    fn move_cursor_forward(&mut self, current_token: &PlainTextToken, tokens: &Lexer<PlainTextToken>) {
        self.status.col += tokens.slice().chars().count() + 1;
        if *current_token == PlainTextToken::NewLine {
            self.status.col = 1;
            self.status.row += 1;
        }
    }

    fn get_mistake_for_word(&self, pos: usize) -> (usize, usize, Mistake) {
        (
            self.status.row,
            self.status.col,
            Mistake::new_dyn(
                format!("a(z) \"{}\" szó elé általában vesszőt teszünk.", self.settings.words[pos]),
                self.settings.probs[pos]
            )
        )
    }
}

impl Detector for NaiveDetector {
    fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.status = NaiveStatus::new();

        self.detect_errors_in_row(tokens)
    }

    fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {
            let index = self.settings.words.iter().position(|a| a == &tokens.slice());

            if !self.status.is_last_token_comma && !self.status.is_last_token_in_vec {
                if let Some(pos) = index {
                    errors.push(self.get_mistake_for_word(pos));
                }
            }

            self.move_cursor_forward(&token, tokens);

            self.status.is_last_token_in_vec = index.is_some();
            if token != PlainTextToken::NewLine {
                self.status.is_last_token_comma = token == PlainTextToken::Comma;
            }
        }

        self.status.row += 1;

        errors
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::NaiveDetector;
    use crate::model::{PlainTextToken, NaiveSettings};
    use crate::traits::Detector;

    #[test]
    fn empty_str() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: Vec::new(), probs: Vec::new() });
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_in_sight() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy"), String::from("ha")], probs: vec![1.0, 1.0] });
        let mut tokens = PlainTextToken::lexer("Ki kopog? Mi kopog? Egy fekete holló!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani, hogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn semicolon_provided() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani; hogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_duplicate_words() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Nem értem, hogy hogy kellene ezt csinálni.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_one_line() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani hogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_next_line() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani\nhogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_multiple_lines() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Azt szeretném mondani\n\n\n\nhogy minden jól sikerült.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_duplicate_words() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Nem értem hogy hogy kellene ezt csinálni.");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_comma_multiple_error_one_line() {
        let mut sut = NaiveDetector::new(NaiveSettings { words: vec![String::from("hogy"), String::from("ha")], probs: vec![1.0, 1.0] });
        let mut tokens = PlainTextToken::lexer("Nem értem hogy kellene ezt csinálni. Elmagyarázod ha szépen megkérlek?");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 2);
    }
}
