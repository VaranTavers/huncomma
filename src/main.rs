
use std::io;
use std::io::Read;

use logos::Logos;

mod model;
mod detector;
mod traits;

use crate::detector::{NaiveDetector, PairDetector, NaiveForwardDetector};
use crate::model::{PlainTextToken, Mistake, NaiveSettings, PairSettings};
use crate::traits::Detector;

fn main() -> io::Result<()> {
    let mut detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(NaiveDetector::new(NaiveSettings::new_from_file("naive.csv"))),
        Box::new(NaiveForwardDetector::new(NaiveSettings::new_from_file("naive_forward.csv"))),
        Box::new(PairDetector::new(PairSettings::new_from_file("pair.csv"))),
    ];

    let mut errors: Vec<(usize, usize, Mistake)> = Vec::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        let mut tokens = PlainTextToken::lexer(buffer.as_str());

        for detector in detectors.iter_mut() {
            let mut c_errors = detector.detect_errors(&mut tokens.clone());
            errors.append(&mut c_errors);
        }
    }

    for (r, c, mistake) in errors {
        if mistake.prob > 0.30 {
            println!("ln: {}, col: {} potenciális vesszőhiba: {}", r, c, mistake.get_str());
        }
    }

    Ok(())
}
