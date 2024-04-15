use thiserror::Error;

#[derive(Debug, Error)]
#[error("{}" , self.message)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}
#[derive(Debug)]
enum ErrorKind {
    TestKind,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: String::from(message),
            kind: ErrorKind::TestKind,
        }
    }
}


