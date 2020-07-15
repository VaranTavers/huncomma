use std::io;
use std::io::Read;

use logos::Logos;

mod model;
mod lexer;
mod detector;

use crate::detector::LogosNaiveDetector;
use crate::model::LogosToken;

fn main() -> io::Result<()> {
    let mut detector = LogosNaiveDetector::new();

    let mut errors: Vec<(usize, usize)> = Vec::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        let mut tokens = LogosToken::lexer(buffer.as_str());

        let mut c_errors = detector.detect_errors(&mut tokens);
        errors.append(&mut c_errors);
    }

    for (r, c) in errors {
        println!("ln: {}, col: {} Potenciális vesszőhiba", r, c);
    }

    Ok(())
}
