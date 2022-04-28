use crate::token::TokenType;
use crate::token::Token;

use std::io;

/// A ProgState represents the state of the program, with a list of the commands to go through, a
/// table of the current data stored by the program, the locations of the current command and
/// current data pointer, as well as a stack to keep track of loops.
#[derive(Debug)]
pub struct ProgState {
    commands: Vec<Token>,
    data: Vec<u32>,
    command_index: usize,
    data_index: usize,
    loop_stack: Vec<usize>
}

impl ProgState {
    /// Given a vector of tokens, create a default ProgState with no data, default indices, and an
    /// empty loop stack.
    pub fn from_tokens(tokens: Vec<Token>) -> ProgState {
        let mut data_vec: Vec<u32> = vec!(0);
        data_vec.resize(data_vec.capacity(), 0);
        ProgState {
            commands: tokens,
            data: data_vec,
            command_index: 0,
            data_index: 0,
            loop_stack: vec!()
        }
    }

    /// Has the ProgState finished running?
    pub fn finished(&self) -> bool {
        self.command_index >= self.commands.len()
    }

    /// Run a single command, assuming that the ProgState is not finished
    pub fn run(&mut self) {
        let curr_command: &Token = &self.commands[self.command_index];
        match curr_command.r#type {
            TokenType::PointInc => self.run_point_inc(),
            TokenType::PointDec => self.run_point_dec(),
            TokenType::ValInc => self.run_val_inc(),
            TokenType::ValDec => self.run_val_dec(),
            TokenType::Output => self.run_output(),
            TokenType::Input => self.run_input(),
            TokenType::IfZero => self.run_if_zero(),
            TokenType::IfNonZero => self.run_if_non_zero(),
        }
    }

    /// Run the pointer increment command on this ProgState
    fn run_point_inc(&mut self) {
        self.data_index += 1;
        self.command_index += 1;

        // Allocate more capacity to the data vector if needed
        if self.data_index >= self.data.capacity() {
            let add_space: usize = self.data_index - self.data.len() + 1;
            self.data.reserve(add_space);
            self.data.resize(self.data.capacity(), 0);
        }
    }

    /// Run the pointer decrement command on this ProgState
    fn run_point_dec(&mut self) {
        if self.data_index == 0 {
            panic!("Attempted to decrement pointer that is at index 0");
        }

        self.data_index -= 1;
        self.command_index += 1;
    }

    /// Run the data increment command on this ProgState
    fn run_val_inc(&mut self) {
        self.data[self.data_index] += 1;
        self.command_index += 1;
    }

    /// Run the data decrement command on this ProgState
    fn run_val_dec(&mut self) {
        if self.data[self.data_index] == 0 {
            panic!("Attempted to decrement pointer that is at index 0");
        }

        self.data[self.data_index] -= 1;
        self.command_index += 1;
    }

    /// Run the data output command on this ProgState
    fn run_output(&mut self) {
        let val: u32 = self.data[self.data_index];
        match char::from_u32(val) {
            Some(c) => print!("{}", c),
            None => print!(" ")
        }

        self.command_index += 1;
    }

    /// Run the data input command on this ProgState
    fn run_input(&mut self) {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let val: u32 = match input.trim().parse::<u32>() {
            Ok(i) => i,
            Err(_) => panic!("Failed to read number from input")
        };

        self.data[self.data_index] = val;
        self.command_index += 1;
    }

    /// Run the if zero command on this ProgState
    fn run_if_zero(&mut self) {
        let val: u32 = self.data[self.data_index];
        if val == 0 {
            let mut curr: &TokenType = &self.commands[self.command_index].r#type;
            while curr != &TokenType::IfNonZero {
                self.command_index += 1;
                if self.command_index == self.commands.len() {
                    panic!("Could not find closing IfNonZero token \"]\"");
                }
                curr = &self.commands[self.command_index].r#type;
            }
            self.command_index += 1;
        }
        else {
            self.loop_stack.push(self.command_index);
            self.command_index += 1;
        }
    }

    /// Run the if non zero command on this ProgState
    fn run_if_non_zero(&mut self) {
        let val: u32 = self.data[self.data_index];
        if val != 0 {
            self.command_index = self.loop_stack.pop().unwrap();
        }
        else {
            self.loop_stack.pop();
            self.command_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::progstate::ProgState;
    use crate::token::TokenType;
    use crate::token::Token;

    #[test]
    fn progstate_from() {
        let tokens: Vec<Token> = vec![
            Token::from(TokenType::PointInc, 1, 1),
            Token::from(TokenType::ValInc, 1, 2)
        ];
        let result: ProgState = ProgState::from_tokens(tokens);
        matches!(result.commands.as_slice(), &[
            Token {
                r#type: TokenType::PointInc,
                line: 1,
                col: 1
            },
            Token {
                r#type: TokenType::ValInc,
                line: 1,
                col: 2
            }
        ]);
        matches!(result.data.as_slice(), &[0]);
        matches!(result.command_index, 0);
        matches!(result.data_index, 0);
        matches!(result.loop_stack.as_slice(), &[]);
    }
}