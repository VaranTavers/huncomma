
use std::io;
use std::io::Read;

use logos::Logos;

mod model;
mod detector;
mod traits;

use crate::detector::{NaiveDetector, PairDetector};
use crate::model::{PlainTextToken, Mistake, NaiveSettings, PairSettings};
use crate::traits::Detector;

fn main() -> io::Result<()> {
    let mut naive_detector = NaiveDetector::new(NaiveSettings::new_from_file("naive.csv"));
    let mut pair_detector = PairDetector::new(PairSettings::new_from_file("pair.csv"));

    let mut errors: Vec<(usize, usize, Mistake)> = Vec::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        let mut tokens = PlainTextToken::lexer(buffer.as_str());

        let mut c_errors = naive_detector.detect_errors(&mut tokens.clone());
        errors.append(&mut c_errors);
        c_errors = pair_detector.detect_errors(&mut tokens.clone());
        errors.append(&mut c_errors);
    }

    for (r, c, mistake) in errors {
        if mistake.prob > 0.30 {
            println!("ln: {}, col: {} potenciális vesszőhiba: {}", r, c, mistake.get_str());
        }
    }

    Ok(())
}
