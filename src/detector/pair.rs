use crate::model::{PlainTextToken, Mistake, PairSettings};
use logos::Lexer;
use std::cell::Cell;
use crate::traits::Detector;

/// Contains the status of the current PairDetector (row, column, first_word_active)
///
/// Generally you shouldn't bother with it.
struct PairStatus {
    pub col: usize,
    pub row: usize,
    pub first_word_active: Vec<Cell<bool>>,
}

impl PairStatus {
    pub fn new(words: usize) -> PairStatus {
        PairStatus {
            col: 1,
            row: 1,
            first_word_active: vec![Cell::new(false); words],
        }
    }
}

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
pub struct PairDetector {
    settings: PairSettings,
    status: PairStatus,
}

impl Detector for PairDetector {
    fn detect_errors(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        self.status = PairStatus::new(self.settings.first_words.len());

        self.detect_errors_in_row(tokens)
    }

    fn detect_errors_in_row(&mut self, tokens: &mut Lexer<PlainTextToken>) -> Vec<(usize, usize, Mistake)> {
        let mut errors = Vec::new();
        while let Some(token) = tokens.next() {
            let lowercase = String::from(tokens.slice()).to_lowercase();

            // Checks for every active (which already appeared) first word if any of it's second words
            // is the current one. If it is, then there might be a missing comma.
            for (index, active) in self.status.first_word_active.iter().enumerate() {
                if active.get() {
                    let second_index = self.settings.second_words[index].iter().position(|a| a == &lowercase.as_str());
                    if let Some(pos) = second_index {
                        errors.push(self.get_mistake_for_word(index, pos));
                    }
                }
            }

            self.move_cursor_forward(&token, tokens);
            self.set_active_word(lowercase.as_str());

            // If it's a comma or a new sentence, then we don't need to check anymore if it is missing between words.
            if token == PlainTextToken::Comma || token == PlainTextToken::EndOfSentence {
                self.status.first_word_active.iter().for_each(|a| a.set(false));
            }
        }

        self.status.row += 1;

        errors
    }
}

impl PairDetector {
    pub fn new(settings: PairSettings) -> PairDetector {
        PairDetector {
            status: PairStatus::new(settings.first_words.len()),
            settings,
        }
    }

    fn move_cursor_forward(&mut self, current_token: &PlainTextToken, tokens: &Lexer<PlainTextToken>) {
        self.status.col += tokens.slice().chars().count() + 1;
        if *current_token == PlainTextToken::NewLine {
            self.status.col = 1;
            self.status.row += 1;
        }
    }

    fn get_mistake_for_word(&self, pos1: usize, pos2: usize) -> (usize, usize, Mistake) {
        (
            self.status.row,
            self.status.col,
            Mistake::new_dyn(
                format!("a(z) \"{}\" és \"{}\" szavak közé általában vesszőt teszünk (általában a második elé).", self.settings.first_words[pos1], self.settings.second_words[pos1][pos2]),
                self.settings.probs[pos1]
            )
        )
    }

    fn set_active_word(&mut self, word: &str) {
        let index = self.settings.first_words.iter().position(|a| a == word);
        if let Some(pos) = index {
            self.status.first_word_active[pos].set(true);
        }
    }
}
#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::detector::PairDetector;
    use crate::model::{PlainTextToken, PairSettings};
    use crate::traits::Detector;

    #[test]
    fn empty_str() {
        let mut sut = PairDetector::new(PairSettings { first_words: Vec::new(), second_words: Vec::new(), probs: Vec::new()});
        let mut tokens = PlainTextToken::lexer("");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comma_provided() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("mind"), String::from("abban")], second_words: vec![vec![String::from("mind")], vec![String::from("hogy")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Mind a tanárok, mind a diákok egyetértenek abban, hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn semicolon_provided() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("mind"), String::from("abban")], second_words: vec![vec![String::from("mind")], vec![String::from("hogy")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Mind a tanárok, mind a diákok egyetértenek abban; hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn one_comma_missing() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("mind"), String::from("abban")], second_words: vec![vec![String::from("mind")], vec![String::from("hogy")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Mind a tanárok mind a diákok egyetértenek abban, hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn both_commas_missing() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("mind"), String::from("abban")], second_words: vec![vec![String::from("mind")], vec![String::from("hogy")]], probs: vec![1.0, 0.8]});
        let mut tokens = PlainTextToken::lexer("Mind a tanárok mind a diákok egyetértenek abban hogy változásra van szükség!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn example_correct() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("ha")], second_words: vec![vec![String::from("akkor")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Ha mész vásárolni, akkor ne felejts el tejet hozni!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn example_incorrect() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("ha")], second_words: vec![vec![String::from("akkor")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Ha mész vásárolni akkor ne felejts el tejet hozni!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn no_detection_over_sentences() {
        let mut sut = PairDetector::new(PairSettings { first_words: vec![String::from("mind"), String::from("abban")], second_words: vec![vec![String::from("mind")], vec![String::from("hogy")]], probs: vec![1.0]});
        let mut tokens = PlainTextToken::lexer("Mind hősök voltak ők. Mind az a tizenhárom, kit várt a vérpad!");
        let errors = sut.detect_errors(&mut tokens);

        assert_eq!(errors.len(), 0);
    }
}
