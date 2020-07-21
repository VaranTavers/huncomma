use std::io;
use std::io::Read;

use logos::Logos;

mod model;
mod detector;

use crate::detector::NaiveDetector;
use crate::model::Token;

fn main() -> io::Result<()> {
    let mut detector = NaiveDetector::new();

    let mut errors: Vec<(usize, usize, f64)> = Vec::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        let mut tokens = Token::lexer(buffer.as_str());

        let mut c_errors = detector.detect_errors(&mut tokens);
        errors.append(&mut c_errors);
    }

    for (r, c, _prob) in errors {
        println!("ln: {}, col: {} Potenciális vesszőhiba", r, c);
    }

    Ok(())
}
