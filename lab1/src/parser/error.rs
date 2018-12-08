use lalrpop_util::ParseError;
use std::error::Error;
use std::fmt;
use std::fmt::Display; 
use core::fmt::Debug;

#[derive(Debug)]
pub enum ParserError {
    /// syntax error when lalrpop does parsing (simple case for now)
    /// error string saved
    SyntaxError(String),
    UnknownVariable(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParserError::*;

        match self {
            SyntaxError(s) => write!(f, "syntax error: {}", s),
            UnknownVariable(s) => write!(f, "unknown variable {} used in context", s),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // simple case for now
        None
    }
}

impl<L, T, E> From<ParseError<L, T, E>> for ParserError
where
    L: Debug + Display,
    T: Debug + Display,
    E: Debug + Display,
{
    fn from(e: ParseError<L, T, E>) -> Self {
        ParserError::SyntaxError(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;
