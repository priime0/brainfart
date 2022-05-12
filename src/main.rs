use std::env;
use std::fs;
use std::process::exit;

mod lexer;
mod progstate;
mod token;
mod error;

use crate::progstate::ProgState;
use crate::token::Token;
use crate::error::BrainfartResult;

fn main(){
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
            let mut progstate: ProgState = ProgState::from_tokens(tokens);
            while !progstate.finished() {
                let result: BrainfartResult<()> = progstate.run();
                if let Err(e) = result {
                    return Err(e);
                }
            }
            Ok(())
        },
        Err(e) => {
            eprintln!("lex error");
            Err(e)
        },
    }
}
