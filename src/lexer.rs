use crate::error::BrainfartError;
use crate::error::BrainfartResult;
use crate::token::Token;
use crate::token::TokenType;

/// Converts a String into a vector of Tokens, ignoring invalid characters
pub fn lex_string(string: String) -> BrainfartResult<Vec<Token>> {
    let mut line: u32 = 1;
    let mut col: u32 = 1;
    let mut tokens: Vec<Token> = vec![];
    let mut brace_balance: u32 = 0;
    for char in string.chars() {
        let opt_token_type: Option<TokenType> = lex_char(char);
        if let Some(token_type) = opt_token_type {
            let token_result = add_token(&mut tokens, token_type, &mut brace_balance, line, col);
            token_result?;
            col += 1;
        } else if char == '\n' || char == '\r' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    match brace_balance {
        0 => Ok(tokens),
        _ => Err(BrainfartError::UnmatchedOpenBracket),
    }
}

/// Adds a token to the tokens vector
fn add_token(
    tokens: &mut Vec<Token>,
    token_type: TokenType,
    brace_balance: &mut u32,
    line: u32,
    col: u32,
) -> BrainfartResult<()> {
    match token_type {
        TokenType::IfZero => {
            *brace_balance += 1;
        }
        TokenType::IfNonZero => {
            if *brace_balance == 0 {
                let token: Token = Token::from(token_type, line, col);
                return Err(BrainfartError::UnmatchedCloseBracket(token));
            }
            *brace_balance -= 1;
        }
        _ => (),
    }
    let token: Token = Token::from(token_type, line, col);
    tokens.push(token);
    Ok(())
}

/// Converts a character to a token type, if valid
fn lex_char(c: char) -> Option<TokenType> {
    match c {
        '>' => Some(TokenType::PointInc),
        '<' => Some(TokenType::PointDec),
        '+' => Some(TokenType::ValInc),
        '-' => Some(TokenType::ValDec),
        '.' => Some(TokenType::Output),
        ',' => Some(TokenType::Input),
        '[' => Some(TokenType::IfZero),
        ']' => Some(TokenType::IfNonZero),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex_char;
    use crate::lexer::lex_string;
    use crate::token::Token;
    use crate::token::TokenType;

    #[test]
    fn lex_string_char() {
        matches!(
            lex_string("+".to_string()).unwrap().as_slice(),
            &[Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 1
            }]
        );
    }

    #[test]
    fn lex_string_char_whitespace() {
        matches!(
            lex_string("  >\n ".to_string()).unwrap().as_slice(),
            &[Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 3
            }]
        );
    }

    #[test]
    fn lex_string_chars_whitespace() {
        matches!(
            lex_string("> ++ <\n-  ".to_string()).unwrap().as_slice(),
            &[
                Token {
                    ty: TokenType::PointInc,
                    line: 1,
                    col: 1,
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 1,
                    col: 3
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 1,
                    col: 4
                },
                Token {
                    ty: TokenType::PointDec,
                    line: 1,
                    col: 6
                },
                Token {
                    ty: TokenType::ValDec,
                    line: 2,
                    col: 1
                },
            ]
        );
    }

    #[test]
    fn lex_string_char_words() {
        matches!(
            lex_string("Observe the following:\n ,+++.".to_string())
                .unwrap()
                .as_slice(),
            &[
                Token {
                    ty: TokenType::Input,
                    line: 2,
                    col: 2,
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 2,
                    col: 3,
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 2,
                    col: 4,
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 2,
                    col: 4,
                },
                Token {
                    ty: TokenType::Output,
                    line: 2,
                    col: 5,
                },
            ]
        );
    }

    #[test]
    fn lex_point_inc() {
        matches!(lex_char('>').unwrap(), TokenType::PointInc);
    }

    #[test]
    fn lex_point_dec() {
        matches!(lex_char('>').unwrap(), TokenType::PointDec);
    }

    #[test]
    fn lex_val_inc() {
        matches!(lex_char('+').unwrap(), TokenType::ValInc);
    }

    #[test]
    fn lex_val_dec() {
        matches!(lex_char('-').unwrap(), TokenType::ValDec);
    }

    #[test]
    fn lex_output() {
        matches!(lex_char('.').unwrap(), TokenType::Output);
    }

    #[test]
    fn lex_input() {
        matches!(lex_char(',').unwrap(), TokenType::Input);
    }

    #[test]
    fn lex_if_zero() {
        matches!(lex_char('[').unwrap(), TokenType::IfZero);
    }

    #[test]
    fn lex_if_non_zero() {
        matches!(lex_char(']').unwrap(), TokenType::IfNonZero);
    }

    #[test]
    fn lex_none() {
        matches!(lex_char('a'), None);
        matches!(lex_char('d'), None);
        matches!(lex_char(' '), None);
        matches!(lex_char('\n'), None);
    }
}
