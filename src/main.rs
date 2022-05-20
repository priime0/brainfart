use std::env;
use std::fs;
use std::process::exit;

mod error;
mod expr;
mod lexer;
mod parser;
mod progstate;
mod token;

use crate::error::BrainfartResult;
use crate::expr::Expr;
use crate::parser::parse_tokens;
use crate::progstate::ProgState;
use crate::token::Token;

fn main() {
    let filenames: Vec<String> = env::args().skip(1).collect();
    for filename in filenames {
        let result: BrainfartResult<()> = run_file(filename);
        match result {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

fn run_file(filename: String) -> BrainfartResult<()> {
    let contents = fs::read_to_string(filename.clone())
        .unwrap_or_else(|_| panic!("Encountered an error while attempting to read {}", filename));
    let tokens_result: BrainfartResult<Vec<Token>> = lexer::lex_string(contents);
    match tokens_result {
        Ok(tokens) => {
            let exprs_result: BrainfartResult<Vec<Expr>> = parse_tokens(tokens);
            match exprs_result {
                Ok(exprs) => ProgState::default().run(&exprs),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
