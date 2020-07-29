use std::fs;

/// Contains the words that are used in the NaiveDetector.
///
/// Loads words from files.
#[derive(Clone)]
pub struct NaiveSettings {
    pub words: Vec<String>,
    pub probs: Vec<f64>,
}

impl NaiveSettings {
    pub fn new_from_file(filename: &str) -> NaiveSettings {
        let content = fs::read_to_string(filename).expect(format!("File not found: \"{}\"", filename).as_str());
        let rows = content.split("\n");

        let mut words = Vec::new();
        let mut probs = Vec::new();

        for row in rows {
            let cols = row.split(";").collect::<Vec<&str>>();

            if cols.len() > 1 {
                words.push(String::from(cols[0]));
                probs.push(cols[1].trim().parse::<f64>().unwrap());
            }
        }

        NaiveSettings {
            words,
            probs,
        }
    }
}