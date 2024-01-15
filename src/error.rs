use thiserror::Error;

#[derive(Debug, Error)]
#[error("{}" , self.message)]
pub struct RmORMError {
    message: String,
    kind: ErrorKind,
}
#[derive(Debug)]
enum ErrorKind {
    TestKind,
}

impl RmORMError {
    pub fn new(message: &str) -> RmORMError {
        RmORMError {
            // message: message.to_string(),
            message: String::from(message),
            kind: ErrorKind::TestKind,
        }
    }
    pub fn to_string(&self) -> String {
        let message = &self.message;
        message.to_string()
    }
}
// impl Display for RmORMError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
// }
