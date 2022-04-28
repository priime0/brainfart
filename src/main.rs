use std::fs;
use std::env;

mod token;
mod lexer;
mod progstate;

use crate::token::Token;
use crate::progstate::ProgState;

fn main() {
    let filenames: Vec<String> = env::args().skip(1).collect();
    for filename in filenames {
        run_file(filename);
    }
}

fn run_file(filename: String) {
    let contents = fs::read_to_string(filename.clone())
        .unwrap_or_else(|_| panic!("Encountered an error while attempting to read {}", filename));
    let tokens: Vec<Token> = lexer::lex_string(contents);
    let mut progstate: ProgState = ProgState::from_tokens(tokens);

    while !progstate.finished() {
        progstate.run();
    }
}
