use crate::model::{PlainTextToken, Mistake, NaiveSettings};
use logos::Lexer;
use crate::traits::Detector;

/// Contains the status of the current NaiveForwardDetector (row, column, active_word)
///
/// Generally you shouldn't bother with it.
struct NaiveForwardStatus {
    col: usize,
    row: usize,
    active_word: Option<usize>
}

impl NaiveForwardStatus {
    pub fn new() -> NaiveForwardStatus {
        NaiveForwardStatus {
            col: 1,
            row: 1,
            active_word: None,
        }
    }
}

/// Detects if there isn't a comma after the given words. These words are generally followed by a
/// comma, however for most of them there are exceptions.
///
/// Every word is given a probability, which means the following: what is the probability of actually
/// needing a comma after that word.
///
/// Example: greetings that are not adjectives: szia, helló; but not: "kedves" "tisztelt"

pub struct NaiveForwardDetector {
    settings: NaiveSettings,
    status: NaiveForwardStatus,
}

impl NaiveForwardDetector {
    pub fn new(settings: NaiveSettings) -> NaiveForwardDetector {
        NaiveForwardDetector {
            settings,
            status: NaiveForwardStatus::new(),
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
                format!("a(z) \"{}\" szó után általában vesszőt teszünk.", self.settings.words[pos]),
                self.settings.probs[pos]
            )
        )
    }

    fn is_token_word(&self, token: &PlainTextToken) -> bool {
        *token == PlainTextToken::Number || *token == PlainTextToken::Text
    }
}

impl Detector for NaiveForwardDetector {

    fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.status = NaiveForwardStatus::new();

        self.detect_errors_in_row(tokens)
    }

    fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();

        while let Some(token) = tokens.next() {
            let lowercase = String::from(tokens.slice()).to_lowercase();
            let index = self.settings.words.iter().position(|a| a == &lowercase.as_str());

            if let Some(pos) = self.status.active_word {
                if self.is_token_word(&token) {
                    errors.push(self.get_mistake_for_word(pos));
                }
            }

            self.move_cursor_forward(&token, tokens);

            if token != PlainTextToken::NewLine {
                self.status.active_word = index;
            }
        }

        self.status.row += 1;

        errors
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::NaiveForwardDetector;
    use crate::model::{PlainTextToken, NaiveSettings};
    use crate::traits::Detector;

    #[test]
    fn empty_str() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_in_sight() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Mi van?");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_required_if_terminated() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Szia!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Szia, meghoztuk a tudod... Hmmm...");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn semicolon_provided() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Szia; meghoztuk a tudod... Hmmm...");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_missing() {
        let mut sut = NaiveForwardDetector::new(NaiveSettings { words: vec![String::from("szia")], probs: vec![1.0] });
        let mut tokens = PlainTextToken::lexer("Szia meghoztuk a tudod... Hmmm...");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

}
