/// Contains the words that are used in the NaiveDetector.
///
/// Loads words from files.
#[derive(Clone)]
pub struct NaiveSettings {
    pub words: Vec<String>,
    pub probs: Vec<f64>,
}

impl NaiveSettings {
    pub fn new_from_string(content: String) -> NaiveSettings {
        let rows = content.split('\n');

        let mut words = Vec::new();
        let mut probs = Vec::new();

        for row in rows {
            let cols = row.split(';').collect::<Vec<&str>>();

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