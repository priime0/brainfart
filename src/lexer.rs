use crate::token::TokenType;
use crate::token::Token;

/// Converts a String into a vector of Tokens, ignoring invalid characters
pub fn lex_string(string: String) -> Vec<Token> {
    let mut line: u32 = 1;
    let mut col: u32 = 1;
    let mut tokens: Vec<Token> = vec!();
    let mut brace_balance: u32 = 0;
    for char in string.chars() {
        let opt_token_type: Option<TokenType> = lex_char(char);
        if let Some(token_type) = opt_token_type {
            match token_type {
                TokenType::IfZero => {
                    brace_balance += 1;
                },
                TokenType::IfNonZero => {
                    if brace_balance == 0 {
                        panic!("Encountered non-matched closing brace ] at line {} col {}", line, col);
                    }
                    brace_balance -= 1;
                },
                _ => (),
            }
            let token: Token = Token::from(token_type, line, col);
            tokens.push(token);
            col += 1;
        }
        else if char == '\n' || char == '\r' {
            line += 1;
            col = 1;
        }
        else {
            col += 1;
        }
    }

    if brace_balance != 0 {
        panic!("Missing matching closing brace ]");
    }

    tokens
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
    use crate::token::TokenType;
    use crate::token::Token;
    use crate::lexer::lex_char;
    use crate::lexer::lex_string;

    #[test]
    fn lex_string_char() {
        matches!(lex_string("+".to_string()).as_slice(), &[
            Token {
                r#type: TokenType::ValInc,
                line: 1,
                col: 1
            }
        ]);
    }

    #[test]
    fn lex_string_char_whitespace() {
        matches!(lex_string("  >\n ".to_string()).as_slice(), &[
            Token {
                r#type: TokenType::PointInc,
                line: 1,
                col: 3
            }
        ]);
    }

    #[test]
    fn lex_string_chars_whitespace() {
        matches!(lex_string("> ++ <\n-  ".to_string()).as_slice(), &[
            Token {
                r#type: TokenType::PointInc,
                line: 1,
                col: 1,
            },
            Token {
                r#type: TokenType::ValInc,
                line: 1,
                col: 3
            },
            Token {
                r#type: TokenType::ValInc,
                line: 1,
                col: 4
            },
            Token {
                r#type: TokenType::PointDec,
                line: 1,
                col: 6
            },
            Token {
                r#type: TokenType::ValDec,
                line: 2,
                col: 1
            },
        ]);
    }

    #[test]
    fn lex_string_char_words() {
        matches!(lex_string("Observe the following:\n ,+++.".to_string()).as_slice(), &[
            Token {
                r#type: TokenType::Input,
                line: 2,
                col: 2,
            },
            Token {
                r#type: TokenType::ValInc,
                line: 2,
                col: 3,
            },
            Token {
                r#type: TokenType::ValInc,
                line: 2,
                col: 4,
            },
            Token {
                r#type: TokenType::ValInc,
                line: 2,
                col: 4,
            },
            Token {
                r#type: TokenType::Output,
                line: 2,
                col: 5,
            },
        ]);
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
