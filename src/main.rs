use std::io;
use std::io::Read;

mod model;
mod lexer;
mod detector;

use crate::lexer::PlainTextLexer;
use crate::detector::NaiveDetector;

fn main() -> io::Result<()> {
    let mut lexer = PlainTextLexer::new();
    let mut detector = NaiveDetector::new();

    let mut errors: Vec<(usize, usize)> = Vec::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        let tokens = lexer.tokenize_row(buffer.as_str());

        let mut c_errors = detector.detect_errors(&tokens);
        errors.append(&mut c_errors);
    }

    for (r, c) in errors {
        println!("ln: {}, col: {} Potenciális vesszőhiba", r, c);
    }

    Ok(())
}
