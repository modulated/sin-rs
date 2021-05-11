use std::fmt;
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    UnknownInstruction,
    TokenParseError,
    NoEndInstruction
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error {kind,message}
    }

    // pub fn UnknownInstruction(message: String) -> Error {
    //     Self::new(ErrorKind::UnknownInstruction, message)
    // }

    // pub fn TokenParseError(message: String) -> Error {
    //     Self::new(ErrorKind::TokenParseError, message)
    // }

    // pub fn NoEndInstruction(message: String) -> Error {
    //     Self::new(ErrorKind::NoEndInstruction, message)
    // }    
}

impl std::fmt::Display for Error {    
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::fmt::Display for ErrorKind {    
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}