use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::token::Token;

pub type BrainfartResult<T> = Result<T, BrainfartError>;

#[derive(Debug)]
/// Possible errors that can be encountered during lexing or runtime. Some members store the token
/// where the error occurred for more precise reporting.
pub enum BrainfartError {
    UnmatchedOpenBracket,
    UnmatchedCloseBracket(Token),
    PointZeroDec(Token),
    ValZeroDec(Token),
    Io(Token),
}

impl Error for BrainfartError {}

impl Display for BrainfartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BrainfartError::UnmatchedOpenBracket => {
                write!(f, "ERROR: Missing matching closing bracket ]")
            }
            BrainfartError::UnmatchedCloseBracket(tok) => {
                write!(
                    f,
                    "ERROR line {} col {}: Encountered unmatched closing bracket ]",
                    tok.line, tok.col
                )
            }
            BrainfartError::PointZeroDec(tok) => {
                write!(
                    f,
                    "ERROR line {} col {}: Attempted to decrement pointer that is at index 0",
                    tok.line, tok.col
                )
            }
            BrainfartError::ValZeroDec(tok) => {
                write!(
                    f,
                    "ERROR line {} col {}: Attempted to decrement value that is 0",
                    tok.line, tok.col
                )
            }
            BrainfartError::Io(tok) => {
                write!(
                    f,
                    "ERROR line {} col {}: Failed to read character from input",
                    tok.line, tok.col
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{BrainfartError, BrainfartResult};
    use crate::token::{Token, TokenType};

    #[test]
    fn unmatched_open_error() {
        let err: BrainfartResult<()> = Err(BrainfartError::UnmatchedOpenBracket);
        match err {
            Ok(_) => panic!("unmatched_open_error had Ok result"),
            Err(e) => matches!(
                format!("{}", e).as_str(),
                "ERROR: Missing matching closing bracket"
            )
        };
    }

    #[test]
    fn unmatched_close_error() {
        let token: Token = Token {
            ty: TokenType::IfNonZero,
            line: 1,
            col: 1,
        };
        let err: BrainfartResult<()> = Err(BrainfartError::UnmatchedCloseBracket(token));
        match err {
            Ok(_) => panic!("unmatched_close_error had Ok result"),
            Err(e) => matches!(
                format!("{}", e).as_str(),
                "ERROR line 1 col 1: Encountered unmatched closing bracket ]"
            ),
        };
    }

    #[test]
    fn point_dec_error() {
        let token: Token = Token {
            ty: TokenType::PointDec,
            line: 3,
            col: 3
        };
        let err: BrainfartResult<()> = Err(BrainfartError::PointZeroDec(token));
        match err {
            Ok(_) => panic!("point_dec_error had Ok result"),
            Err(e) => matches!(
                format!("{}", e).as_str(),
                "ERROR line {} col {}: Attempted to decrement pointer that is at index 0"
            ),
        };
    }

    #[test]
    fn val_dec_error() {
        let token: Token = Token {
            ty: TokenType::ValDec,
            line: 2,
            col: 8
        };
        let err: BrainfartResult<()> = Err(BrainfartError::ValZeroDec(token));
        match err {
            Ok(_) => panic!("val_dec_error had Ok result"),
            Err(e) => matches!(
                format!("{}", e).as_str(),
                "ERROR line {} col {}: Attempted to decrement value that is 0"
            ),
        };
    }

    #[test]
    fn input_error() {
        let token: Token = Token {
            ty: TokenType::Input,
            line: 1,
            col: 2
        };
        let err: BrainfartResult<()> = Err(BrainfartError::Io(token));
        match err {
            Ok(_) => panic!("input_error had Ok result"),
            Err(e) => matches!(
                format!("{}", e).as_str(),
                "ERROR line {} col {}: Failed to read character from input"
            )
        };
    }
}
