#[derive(Debug)]
pub struct RmORMError {
    message: String,
}

impl RmORMError {
    pub fn new(message: &str) -> RmORMError {
        RmORMError {
            message: message.to_string(),
        }
    }
    pub fn to_string(&self) -> String{
       let message = &self.message;
        message.to_string()
    }
}

