/// A TokenType is a valid "command" in bf that either changes the state of the program or performs
/// an input/output side-effect.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    /// A Token that represents incrementing the pointer position
    PointInc,
    /// A Token that represents decrementing the pointer position
    PointDec,
    /// A Token that represents incrementing the value located at the pointer
    ValInc,
    /// A Token that represents decrementing the value located at the pointer
    ValDec,
    /// A Token that represents outputting the value contained at the pointer
    Output,
    /// A Token that represents receiving input and inserting it into the location of the pointer
    Input,
    /// A Token that introduces the start of a "while not zero" loop at the pointer location,
    /// jumping to a corresponding IfNonZero token if the pointer location's value is zero
    IfZero,
    /// A Token that closes the "while not zero" loop at the pointer location, jumping to its
    /// corresponding IfZero token if the pointer location's value is zero
    IfNonZero
}

/// A Token stores a TokenType and where it was encountered in the source file
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub col: u32,
}

impl Token {
    /// Produce a Token from the given arguments
    pub fn from(ty: TokenType, line: u32, col: u32) -> Self {
        Token {
            ty,
            line,
            col
        }
    }
}
