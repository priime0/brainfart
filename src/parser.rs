use std::slice::Iter;

use crate::error::{BrainfartError, BrainfartResult};
use crate::expr::{Expr, ExprType, LoopBlock};
use crate::token::{Token, TokenType};

/// Parse tokens produced by the lexer to produce a vector of Exprs.
pub fn parse_tokens(tokens: Vec<Token>) -> BrainfartResult<Vec<Expr>> {
    let mut exprs: Vec<Expr> = vec![];
    let mut tokens_iter = tokens.iter();

    while let Some(token) = tokens_iter.next() {
        match token.ty {
            TokenType::PointInc => parse_point_inc(&mut exprs, *token),
            TokenType::PointDec => parse_point_dec(&mut exprs, *token),
            TokenType::ValInc => parse_val_inc(&mut exprs, *token),
            TokenType::ValDec => parse_val_dec(&mut exprs, *token)?,
            TokenType::Output => parse_output(&mut exprs, *token),
            TokenType::Input => parse_input(&mut exprs, *token),
            TokenType::IfZero => parse_loop_block(&mut exprs, &mut tokens_iter)?,
            TokenType::IfNonZero => (),
        }
    }

    Ok(exprs)
}

/// Given a Token of type PointInc, add to the vector of Exprs.
fn parse_point_inc(exprs: &mut Vec<Expr>, token: Token) {
    if exprs.is_empty() {
        push_new_move_right(exprs, token);
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];

        if let ExprType::MoveRight(x) = prev.ty {
            prev.ty = ExprType::MoveRight(x + 1);
            prev.tokens.push(token);
        } else {
            push_new_move_right(exprs, token);
        }
    }
}

/// Given a Token of type PointDec, add to the vector of Exprs. If the previous Expr is a
/// MoveRight, then decrement its value or pop it if its value is 1 (cancelling).
fn parse_point_dec(exprs: &mut Vec<Expr>, token: Token) {
    if exprs.is_empty() {
        push_new_move_left(exprs, token);
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];
        let prev_type: &ExprType = &prev.ty;
        match prev_type {
            ExprType::MoveRight(x) => {
                if x == &1 {
                    exprs.pop();
                } else {
                    prev.ty = ExprType::MoveRight(x - 1);
                    prev.tokens.pop();
                }
            }
            ExprType::MoveLeft(x) => {
                prev.ty = ExprType::MoveLeft(x + 1);
                prev.tokens.push(token);
            }
            _ => push_new_move_left(exprs, token),
        }
    }
}

/// Given a Token of type ValInc, add to the vector of Exprs.
fn parse_val_inc(exprs: &mut Vec<Expr>, token: Token) {
    if exprs.is_empty() {
        push_new_add(exprs, token);
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];
        let prev_type: &ExprType = &prev.ty;

        match prev_type {
            ExprType::Add(x) => {
                prev.ty = ExprType::Add(x + 1);
                prev.tokens.push(token);
            }
            ExprType::Set(x) => {
                prev.ty = ExprType::Set(x + 1);
                prev.tokens.push(token);
            }
            _ => push_new_add(exprs, token),
        };
    }
}

/// Given a Token of type ValDec, add to the vector of Exprs. If the previous Expr is a ValInc,
/// then decrement its value or pop it if its value is 1 (cancelling).
fn parse_val_dec(exprs: &mut Vec<Expr>, token: Token) -> BrainfartResult<()> {
    if exprs.is_empty() {
        push_new_sub(exprs, token);
        Ok(())
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];
        let prev_type: &ExprType = &prev.ty;
        match prev_type {
            ExprType::Add(x) => {
                if x == &1 {
                    exprs.pop();
                } else {
                    prev.ty = ExprType::Add(x - 1);
                    prev.tokens.pop();
                }
            }
            ExprType::Sub(x) => {
                prev.ty = ExprType::Sub(x + 1);
                prev.tokens.push(token);
            }
            ExprType::Set(x) => {
                if x == &0 {
                    return Err(BrainfartError::ValZeroDec(token));
                }

                prev.ty = ExprType::Set(x - 1);
                prev.tokens.push(token);
            }
            _ => push_new_sub(exprs, token),
        };

        Ok(())
    }
}

/// Given a Token of type Output, add to the vector of Exprs.
fn parse_output(exprs: &mut Vec<Expr>, token: Token) {
    if exprs.is_empty() {
        push_new_output(exprs, token);
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];
        let prev_type: &ExprType = &prev.ty;
        match prev_type {
            ExprType::Output(x) => {
                prev.ty = ExprType::Output(x + 1);
                prev.tokens.push(token);
            }
            _ => push_new_output(exprs, token),
        }
    }
}

/// Given a Token of type Input, add to the vector of Exprs.
fn parse_input(exprs: &mut Vec<Expr>, token: Token) {
    if exprs.is_empty() {
        push_new_input(exprs, token);
    } else {
        let last_index: usize = &exprs.len() - 1;
        let prev: &mut Expr = &mut exprs[last_index];
        let prev_type: &ExprType = &prev.ty;
        match prev_type {
            ExprType::Input(x) => {
                prev.ty = ExprType::Input(x + 1);
                prev.tokens.push(token);
            }
            _ => push_new_input(exprs, token),
        }
    }
}

/// Given a Token of type IfZero, parse a LoopBlock and add to the vector of Exprs.
fn parse_loop_block(exprs: &mut Vec<Expr>, tokens: &mut Iter<'_, Token>) -> BrainfartResult<()> {
    let mut lb_exprs: Vec<Expr> = vec![];

    while let Some(token) = tokens.next() {
        match token.ty {
            TokenType::PointInc => parse_point_inc(&mut lb_exprs, *token),
            TokenType::PointDec => parse_point_dec(&mut lb_exprs, *token),
            TokenType::ValInc => parse_val_inc(&mut lb_exprs, *token),
            TokenType::ValDec => parse_val_dec(&mut lb_exprs, *token)?,
            TokenType::Output => parse_output(&mut lb_exprs, *token),
            TokenType::Input => parse_input(&mut lb_exprs, *token),
            TokenType::IfZero => parse_loop_block(&mut lb_exprs, tokens)?,
            TokenType::IfNonZero => {
                if lb_exprs.len() == 1 {
                    let expr: &Expr = &lb_exprs[0];
                    if let ExprType::Sub(1) = expr.ty {
                        let expr_token: Token = expr.tokens[0];
                        let set_expr = Expr {
                            ty: ExprType::Set(0),
                            tokens: vec![expr_token],
                        };
                        exprs.push(set_expr);
                        return Ok(());
                    }
                }
                break;
            }
        }
    }

    let loop_block = LoopBlock { exprs: lb_exprs };
    let boxed_loop_block = Box::new(loop_block);
    let loop_block_expr = Expr {
        ty: ExprType::LoopBlock(boxed_loop_block),
        tokens: vec![],
    };
    exprs.push(loop_block_expr);

    Ok(())
}

/// Push a new Expr with the given ExprType containing the given token.
fn push_new_expr(exprs: &mut Vec<Expr>, ty: ExprType, token: Token) {
    let expr: Expr = Expr {
        ty,
        tokens: vec![token],
    };
    exprs.push(expr);
}

/// Push a new Expr of type MoveRight containing the given token.
fn push_new_move_right(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::MoveRight(1), token);
}

/// Push a new Expr of type MoveLeft containing the given token.
fn push_new_move_left(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::MoveLeft(1), token);
}

/// Push a new Expr of type Add containing the given token.
fn push_new_add(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::Add(1), token);
}

/// Push a new Expr of type Sub containing the given token.
fn push_new_sub(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::Sub(1), token);
}

/// Push a new Expr of type Output containing the given token.
fn push_new_output(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::Output(1), token);
}

/// Push a new Expr of type Input containing the given token.
fn push_new_input(exprs: &mut Vec<Expr>, token: Token) {
    push_new_expr(exprs, ExprType::Input(1), token);
}

#[cfg(test)]
mod tests {
    use crate::error::BrainfartResult;
    use crate::expr::{Expr, ExprType, LoopBlock};
    use crate::parser::{
        parse_input, parse_loop_block, parse_output, parse_point_dec, parse_point_inc,
        parse_tokens, parse_val_dec, parse_val_inc,
    };
    use crate::token::{Token, TokenType};

    #[test]
    fn parse_point_inc_new() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(1),
            tokens: vec![Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::PointInc,
            line: 1,
            col: 2,
        };
        parse_point_inc(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::Add(1),
                    tokens: vec![Token {
                        ty: TokenType::ValInc,
                        line: 1,
                        col: 1
                    }]
                },
                Expr {
                    ty: ExprType::MoveRight(1),
                    tokens: vec![Token {
                        ty: TokenType::PointInc,
                        line: 1,
                        col: 2
                    }]
                }
            ]
        );
    }

    #[test]
    fn parse_point_inc_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::PointInc,
            line: 2,
            col: 1,
        };
        parse_point_inc(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::MoveRight(2),
                tokens: vec![
                    Token {
                        ty: TokenType::PointInc,
                        line: 1,
                        col: 1,
                    },
                    Token {
                        ty: TokenType::PointInc,
                        line: 2,
                        col: 1,
                    }
                ]
            },]
        );
    }

    #[test]
    fn parse_point_dec_add() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(1),
            tokens: vec![Token {
                ty: TokenType::ValInc,
                line: 3,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::PointDec,
            line: 3,
            col: 2,
        };
        parse_point_dec(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::Add(1),
                    tokens: vec![Token {
                        ty: TokenType::ValInc,
                        line: 3,
                        col: 1,
                    }]
                },
                Expr {
                    ty: ExprType::MoveLeft(1),
                    tokens: vec![Token {
                        ty: TokenType::PointDec,
                        line: 3,
                        col: 2,
                    }],
                }
            ]
        );
    }

    #[test]
    fn parse_point_dec_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveLeft(2),
            tokens: vec![
                Token {
                    ty: TokenType::PointDec,
                    line: 5,
                    col: 1,
                },
                Token {
                    ty: TokenType::PointDec,
                    line: 5,
                    col: 2,
                },
            ],
        }];
        let token: Token = Token {
            ty: TokenType::PointDec,
            line: 5,
            col: 3,
        };
        parse_point_dec(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::MoveLeft(3),
                tokens: vec![
                    Token {
                        ty: TokenType::PointDec,
                        line: 5,
                        col: 1,
                    },
                    Token {
                        ty: TokenType::PointDec,
                        line: 5,
                        col: 2,
                    },
                    Token {
                        ty: TokenType::PointDec,
                        line: 5,
                        col: 3
                    }
                ],
            }]
        );
    }

    #[test]
    fn parse_point_dec_cancel() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(2),
            tokens: vec![
                Token {
                    ty: TokenType::PointInc,
                    line: 3,
                    col: 3,
                },
                Token {
                    ty: TokenType::PointInc,
                    line: 4,
                    col: 1,
                },
            ],
        }];
        let token: Token = Token {
            ty: TokenType::PointDec,
            line: 4,
            col: 2,
        };
        parse_point_dec(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::MoveRight(1),
                tokens: vec![Token {
                    ty: TokenType::PointInc,
                    line: 3,
                    col: 3,
                }]
            }]
        );
    }

    #[test]
    fn parse_point_dec_pop() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 3,
                col: 3,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::PointDec,
            line: 4,
            col: 3,
        };
        parse_point_dec(&mut exprs, token);
        assert_eq!(exprs, vec![]);
    }

    #[test]
    fn parse_val_inc_new() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::ValInc,
            line: 1,
            col: 2,
        };
        parse_val_inc(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::MoveRight(1),
                    tokens: vec![Token {
                        ty: TokenType::PointInc,
                        line: 1,
                        col: 1,
                    }],
                },
                Expr {
                    ty: ExprType::Add(1),
                    tokens: vec![Token {
                        ty: TokenType::ValInc,
                        line: 1,
                        col: 2,
                    }],
                }
            ]
        );
    }

    #[test]
    fn parse_val_inc_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(1),
            tokens: vec![Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::ValInc,
            line: 2,
            col: 1,
        };
        parse_val_inc(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::Add(2),
                tokens: vec![
                    Token {
                        ty: TokenType::ValInc,
                        line: 1,
                        col: 1,
                    },
                    Token {
                        ty: TokenType::ValInc,
                        line: 2,
                        col: 1,
                    }
                ]
            }]
        );
    }

    #[test]
    fn parse_val_dec_add() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::ValDec,
            line: 1,
            col: 2,
        };

        if let Err(e) = parse_val_dec(&mut exprs, token) {
            panic!("{}", e);
        }

        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::MoveRight(1),
                    tokens: vec![Token {
                        ty: TokenType::PointInc,
                        line: 1,
                        col: 1,
                    }],
                },
                Expr {
                    ty: ExprType::Sub(1),
                    tokens: vec![Token {
                        ty: TokenType::ValDec,
                        line: 1,
                        col: 2,
                    }]
                }
            ]
        );
    }

    #[test]
    fn parse_val_dec_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Sub(1),
            tokens: vec![Token {
                ty: TokenType::ValDec,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::ValDec,
            line: 3,
            col: 3,
        };

        if let Err(e) = parse_val_dec(&mut exprs, token) {
            panic!("{}", e);
        }

        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::Sub(2),
                tokens: vec![
                    Token {
                        ty: TokenType::ValDec,
                        line: 1,
                        col: 1,
                    },
                    Token {
                        ty: TokenType::ValDec,
                        line: 3,
                        col: 3,
                    }
                ],
            }]
        );
    }

    #[test]
    fn parse_val_dec_cancel() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(2),
            tokens: vec![
                Token {
                    ty: TokenType::ValInc,
                    line: 1,
                    col: 1,
                },
                Token {
                    ty: TokenType::ValInc,
                    line: 1,
                    col: 2,
                },
            ],
        }];
        let token: Token = Token {
            ty: TokenType::ValDec,
            line: 1,
            col: 3,
        };

        if let Err(e) = parse_val_dec(&mut exprs, token) {
            panic!("{}", e);
        }

        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::Add(1),
                tokens: vec![Token {
                    ty: TokenType::ValInc,
                    line: 1,
                    col: 1
                }]
            }]
        );
    }

    #[test]
    fn parse_val_dec_pop() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(1),
            tokens: vec![Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::ValDec,
            line: 2,
            col: 1,
        };

        if let Err(e) = parse_val_dec(&mut exprs, token) {
            panic!("{}", e);
        }

        assert_eq!(exprs, vec![]);
    }

    #[test]
    fn parse_output_add() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 10,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::Output,
            line: 10,
            col: 2,
        };
        parse_output(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::MoveRight(1),
                    tokens: vec![Token {
                        ty: TokenType::PointInc,
                        line: 10,
                        col: 1,
                    }],
                },
                Expr {
                    ty: ExprType::Output(1),
                    tokens: vec![Token {
                        ty: TokenType::Output,
                        line: 10,
                        col: 2,
                    }]
                }
            ]
        );
    }

    #[test]
    fn parse_output_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Output(1),
            tokens: vec![Token {
                ty: TokenType::Output,
                line: 1,
                col: 3,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::Output,
            line: 1,
            col: 4,
        };
        parse_output(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::Output(2),
                tokens: vec![
                    Token {
                        ty: TokenType::Output,
                        line: 1,
                        col: 3,
                    },
                    Token {
                        ty: TokenType::Output,
                        line: 1,
                        col: 4,
                    }
                ]
            },]
        );
    }

    #[test]
    fn parse_input_add() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::MoveRight(1),
            tokens: vec![Token {
                ty: TokenType::PointInc,
                line: 10,
                col: 1,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::Input,
            line: 10,
            col: 2,
        };
        parse_input(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::MoveRight(1),
                    tokens: vec![Token {
                        ty: TokenType::PointInc,
                        line: 10,
                        col: 1,
                    }],
                },
                Expr {
                    ty: ExprType::Input(1),
                    tokens: vec![Token {
                        ty: TokenType::Input,
                        line: 10,
                        col: 2,
                    }]
                }
            ]
        );
    }

    #[test]
    fn parse_input_append() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Input(1),
            tokens: vec![Token {
                ty: TokenType::Input,
                line: 1,
                col: 3,
            }],
        }];
        let token: Token = Token {
            ty: TokenType::Input,
            line: 1,
            col: 4,
        };
        parse_input(&mut exprs, token);
        assert_eq!(
            exprs,
            vec![Expr {
                ty: ExprType::Input(2),
                tokens: vec![
                    Token {
                        ty: TokenType::Input,
                        line: 1,
                        col: 3,
                    },
                    Token {
                        ty: TokenType::Input,
                        line: 1,
                        col: 4,
                    }
                ]
            },]
        );
    }

    #[test]
    fn parse_loop_block_add() {
        let mut exprs: Vec<Expr> = vec![Expr {
            ty: ExprType::Add(1),
            tokens: vec![Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 1,
            }],
        }];
        let _already_consumed_token: Token = Token {
            ty: TokenType::IfZero,
            line: 1,
            col: 2,
        };
        let tokens: Vec<Token> = vec![
            Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 3,
            },
            Token {
                ty: TokenType::IfNonZero,
                line: 1,
                col: 4,
            },
        ];
        let mut tokens_iter = tokens.iter();

        if let Err(e) = parse_loop_block(&mut exprs, &mut tokens_iter) {
            panic!("{}", e);
        }

        assert_eq!(
            exprs,
            vec![
                Expr {
                    ty: ExprType::Add(1),
                    tokens: vec![Token {
                        ty: TokenType::ValInc,
                        line: 1,
                        col: 1
                    }]
                },
                Expr {
                    ty: ExprType::LoopBlock(Box::new(LoopBlock {
                        exprs: vec![Expr {
                            ty: ExprType::MoveRight(1),
                            tokens: vec![Token {
                                ty: TokenType::PointInc,
                                line: 1,
                                col: 3,
                            }],
                        }],
                    })),
                    tokens: vec![],
                }
            ]
        );
    }

    #[test]
    fn parse_set() {
        let tokens: Vec<Token> = vec![
            Token {
                ty: TokenType::IfZero,
                line: 1,
                col: 1,
            },
            Token {
                ty: TokenType::ValDec,
                line: 1,
                col: 2,
            },
            Token {
                ty: TokenType::IfNonZero,
                line: 1,
                col: 3,
            },
        ];
        let result: BrainfartResult<Vec<Expr>> = parse_tokens(tokens);
        match result {
            Ok(exprs) => {
                assert_eq!(
                    exprs,
                    vec![Expr {
                        ty: ExprType::Set(0),
                        tokens: vec![Token {
                            ty: TokenType::ValDec,
                            line: 1,
                            col: 2
                        },],
                    }]
                );
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn parse_set_one() {
        let tokens: Vec<Token> = vec![
            Token {
                ty: TokenType::IfZero,
                line: 1,
                col: 1,
            },
            Token {
                ty: TokenType::ValDec,
                line: 1,
                col: 2,
            },
            Token {
                ty: TokenType::IfNonZero,
                line: 1,
                col: 3,
            },
            Token {
                ty: TokenType::ValInc,
                line: 1,
                col: 4,
            },
        ];
        let result: BrainfartResult<Vec<Expr>> = parse_tokens(tokens);
        match result {
            Ok(exprs) => {
                assert_eq!(
                    exprs,
                    vec![Expr {
                        ty: ExprType::Set(1),
                        tokens: vec![
                            Token {
                                ty: TokenType::ValDec,
                                line: 1,
                                col: 2
                            },
                            Token {
                                ty: TokenType::ValInc,
                                line: 1,
                                col: 4
                            }
                        ],
                    }]
                );
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    // LoopBlock MoveLeft case [<]
    fn parse_lb_mvl() {
        let tokens = vec![
            Token {
                ty: TokenType::IfZero,
                line: 1,
                col: 1,
            },
            Token {
                ty: TokenType::PointDec,
                line: 1,
                col: 2,
            },
            Token {
                ty: TokenType::IfNonZero,
                line: 1,
                col: 3,
            },
            Token {
                ty: TokenType::PointDec,
                line: 1,
                col: 4,
            },
        ];
        let result = parse_tokens(tokens);
        match result {
            Ok(exprs) => {
                assert_eq!(
                    exprs,
                    vec![
                        Expr {
                            ty: ExprType::LoopBlock(Box::new(LoopBlock {
                                exprs: vec![Expr {
                                    ty: ExprType::MoveLeft(1),
                                    tokens: vec![Token {
                                        ty: TokenType::PointDec,
                                        line: 1,
                                        col: 2,
                                    }],
                                }],
                            })),
                            tokens: vec![],
                        },
                        Expr {
                            ty: ExprType::MoveLeft(1),
                            tokens: vec![Token {
                                ty: TokenType::PointDec,
                                line: 1,
                                col: 4,
                            }],
                        }
                    ]
                );
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    // LoopBlock MoveRight case [<]
    fn parse_lb_mvr() {
        let tokens = vec![
            Token {
                ty: TokenType::IfZero,
                line: 1,
                col: 1,
            },
            Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 2,
            },
            Token {
                ty: TokenType::IfNonZero,
                line: 1,
                col: 3,
            },
            Token {
                ty: TokenType::PointInc,
                line: 1,
                col: 4,
            },
        ];
        let result = parse_tokens(tokens);
        match result {
            Ok(exprs) => {
                assert_eq!(
                    exprs,
                    vec![
                        Expr {
                            ty: ExprType::LoopBlock(Box::new(LoopBlock {
                                exprs: vec![Expr {
                                    ty: ExprType::MoveRight(1),
                                    tokens: vec![Token {
                                        ty: TokenType::PointInc,
                                        line: 1,
                                        col: 2,
                                    }],
                                }],
                            })),
                            tokens: vec![],
                        },
                        Expr {
                            ty: ExprType::MoveRight(1),
                            tokens: vec![Token {
                                ty: TokenType::PointInc,
                                line: 1,
                                col: 4,
                            }],
                        }
                    ]
                );
            }
            Err(e) => panic!("{}", e),
        }
    }
}
