use crate::model::Token;

pub struct NaiveDetector<'a> {
    words: Vec<&'a str>,
}

impl<'a> NaiveDetector<'a> {
    pub fn new() -> NaiveDetector<'a> {
        let words = vec!["hogy", "ami", "aki", "ahol", "amikor", "amiért", "mert", "mint"];
        NaiveDetector {
            words
        }
    }

    pub fn new_from_words(words: Vec<&'a str>) -> NaiveDetector<'a> {
        NaiveDetector {
            words
        }
    }

    pub fn detect_errors(&mut self, tokens: &Vec<Token>) -> Vec<(usize, usize)> {
        let mut errors = Vec::new();
        tokens.iter().fold(false, |init, token| {
            if self.words.contains(&token.text) && !init {
                errors.push((token.row, token.col))
            }

            return token.is_comma;
        });

        errors
    }
}
