use crate::error::{BrainfartError, BrainfartResult};
use crate::expr::{Expr, ExprType, LoopBlock};

use std::io;

/// A ProgState represents the state/context of the program, with a list of the commands to go
/// through, a table of the current data stored by the program, the locations of the current
/// command and current data pointer, as well as a stack to keep track of loops.
#[derive(Debug)]
pub struct ProgState {
    data: Vec<u32>,
    data_index: usize,
}

impl ProgState {
    /// Generate the default ProgState, with an empty cell array and the data pointer pointing to
    /// the first cell.
    pub fn default() -> Self {
        let mut data: Vec<u32> = vec![0];
        let data_index = 0;
        data.resize(data.capacity(), 0);
        ProgState { data, data_index }
    }

    /// Run the provided vector of Exprs with the current ProgState.
    pub fn run(&mut self, exprs: &[Expr]) -> BrainfartResult<()> {
        for expr in exprs {
            let result = match &expr.ty {
                ExprType::Set(val) => self.run_set(*val),
                ExprType::Add(val) => self.run_add(*val),
                ExprType::Sub(val) => self.run_sub(expr, *val),
                ExprType::MoveRight(val) => self.run_move_right(*val),
                ExprType::MoveLeft(val) => self.run_move_left(expr, *val),
                ExprType::Output(val) => self.run_output(*val),
                ExprType::Input(val) => self.run_input(expr, *val),
                ExprType::LoopBlock(lb) => self.run_loop_block(&**lb),
            };

            if let Err(e) = result {
                return Err(e);
            }
        }

        Ok(())
    }

    /// Set the current pointer's location of this ProgState to the given value.
    fn run_set(&mut self, val: u32) -> BrainfartResult<()> {
        self.data[self.data_index] = val;
        Ok(())
    }

    /// Add the given value to the current pointer's location of this ProgState.
    fn run_add(&mut self, val: u32) -> BrainfartResult<()> {
        self.data[self.data_index] += val;
        Ok(())
    }

    /// Subtract the given value from the current pointer's location of this ProgState.
    fn run_sub(&mut self, expr: &Expr, val: u32) -> BrainfartResult<()> {
        let curr_val = self.data[self.data_index];
        if curr_val < val {
            let err_token = expr.tokens[curr_val as usize];
            Err(BrainfartError::ValZeroDec(err_token))
        } else {
            self.data[self.data_index] -= val;
            Ok(())
        }
    }

    /// Move the data pointer's location to the right the given number of times.
    fn run_move_right(&mut self, val: u32) -> BrainfartResult<()> {
        self.data_index += val as usize;

        if self.data_index >= self.data.capacity() {
            let add_space: usize = self.data_index - self.data.len() + 1;
            self.data.reserve(add_space);
            self.data.resize(self.data.capacity(), 0);
        }

        Ok(())
    }

    /// Move the data pointer's location to the left the given number of times.
    fn run_move_left(&mut self, expr: &Expr, val: u32) -> BrainfartResult<()> {
        let dec_val = val as usize;
        if self.data_index < dec_val {
            let err_token = expr.tokens[self.data_index as usize];
            Err(BrainfartError::PointZeroDec(err_token))
        } else {
            self.data_index -= dec_val;
            Ok(())
        }
    }

    /// Output the value at the current pointer's location the given number of times.
    fn run_output(&mut self, val: u32) -> BrainfartResult<()> {
        let char_val = self.data[self.data_index];
        match char::from_u32(char_val) {
            Some(c) => {
                for _ in 0..val {
                    print!("{}", c);
                }
            }
            None => {
                print!(" ");
            }
        };
        Ok(())
    }

    /// Input a user-entered value into the current pointer's location the given number of times.
    fn run_input(&mut self, expr: &Expr, val: u32) -> BrainfartResult<()> {
        for _ in 0..val {
            let mut input_string = String::new();
            let read_result = io::stdin().read_line(&mut input_string);
            match read_result {
                Ok(_) => {
                    let input = input_string.chars().next().unwrap();
                    let cell_val = input as u32;
                    self.data[self.data_index] = cell_val;
                }
                Err(_) => {
                    let token = expr.tokens.get(0).unwrap();
                    return Err(BrainfartError::Io(*token));
                }
            }
        }
        Ok(())
    }

    /// Run the expressions contained in the LoopBlock, and keep looping while the current pointer
    /// location does not equal zero after every iteration.
    fn run_loop_block(&mut self, lb: &LoopBlock) -> BrainfartResult<()> {
        loop {
            if self.data[self.data_index] == 0 {
                break;
            }
            let result = self.run(&lb.exprs);
            if let Err(e) = result {
                return Err(e);
            }
        }
        Ok(())
    }
}
