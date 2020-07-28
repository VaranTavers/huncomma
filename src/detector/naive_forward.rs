use crate::model::{PlainTextToken, Mistake};
use logos::Lexer;

// TODO: Read words from settings file

/// Detects if there isn't a comma after the given words. These words are generally followed by a
/// comma, however for most of them there are exceptions.
///
/// Every word is given a probability, which means the following: what is the probability of actually
/// needing a comma after that word.
///
/// Example: greetings that are not adjectives: szia, helló; but not: "kedves" "tisztelt"

pub struct NaiveForwardDetector<'a> {
    words: Vec<&'a str>,
    probs: Vec<f64>,
    col: usize,
    row: usize,
    active_word: Option<usize>
}

impl<'a> NaiveForwardDetector<'a> {
    pub fn new() -> NaiveForwardDetector<'a> {
        let word_probs = vec![
            ("szia", 1.00),
            ("helló", 1.00),
            ("szeva", 1.00),
            ("üdvözöllek", 1.00),
            ("üdvözöletem", 0.30),
        ];
        NaiveForwardDetector {
            words: word_probs.iter().map(|(a, _b)| *a).collect(),
            probs: word_probs.iter().map(|(_a, b)| *b).collect(),
            col: 1,
            row: 1,
            active_word: None
        }
    }

    pub fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.col = 1;
        self.row = 1;
        self.active_word = None;

        self.detect_errors_in_row(tokens)
    }

    pub fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();

        while let Some(token) = tokens.next() {
            let lowercase = String::from(tokens.slice()).to_lowercase();
            let index = self.words.iter().position(|a| a == &lowercase.as_str());

            if let Some(pos) = self.active_word {
                if token != PlainTextToken::Comma && token != PlainTextToken::EndOfSentence && token != PlainTextToken::NewLine {
                    errors.push((self.row,
                                 self.col,
                                 Mistake::new_dyn(format!("a(z) \"{}\" szó után általában vesszőt teszünk.", self.words[pos]), self.probs[pos])
                    ));
                }
            }

            self.col += tokens.slice().chars().count() + 1;
            if token == PlainTextToken::NewLine {
                self.col = 1;
                self.row += 1;
            }

            if token != PlainTextToken::NewLine {
                self.active_word = index;
            }

        }

        self.row += 1;

        errors
    }
}
#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::NaiveForwardDetector;
    use crate::model::PlainTextToken;

    #[test]
    fn empty_str() {
        let mut sut = NaiveForwardDetector::new();
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_in_sight() {
        let mut sut = NaiveForwardDetector::new();
        let mut tokens = PlainTextToken::lexer("Mi van?");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_comma_required_if_terminated() {
        let mut sut = NaiveForwardDetector::new();
        let mut tokens = PlainTextToken::lexer("Szia!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = NaiveForwardDetector::new();
        let mut tokens = PlainTextToken::lexer("Szia, meghoztuk a tudod... Hmmm...");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_missing() {
        let mut sut = NaiveForwardDetector::new();
        let mut tokens = PlainTextToken::lexer("Szia meghoztuk a tudod... Hmmm...");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

}
