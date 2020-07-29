use std::fs;

/// Contains the words that are used in the NaiveDetector.
///
/// Loads words from files.
#[derive(Clone)]
pub struct PairSettings {
    pub first_words: Vec<String>,
    pub second_words: Vec<Vec<String>>,
    pub probs: Vec<f64>,
}

impl PairSettings {
    pub fn new_from_file(filename: &str) -> PairSettings {
        let content = fs::read_to_string(filename).expect(format!("File not found: \"{}\"", filename).as_str());
        let rows = content.split("\n");

        let mut first_words = Vec::new();
        let mut second_words = Vec::new();
        let mut probs = Vec::new();

        for row in rows {
            let cols = row.split(";").collect::<Vec<&str>>();

            if cols.len() > 2 {
                let seconds = cols[2].split(" ").map(|a| String::from(a)).collect();

                first_words.push(String::from(cols[0]));
                second_words.push(seconds);
                probs.push(cols[1].trim().parse::<f64>().unwrap());
            }
        }

        PairSettings {
            first_words,
            second_words,
            probs,
        }
    }
}