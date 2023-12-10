#[derive(Debug)]
pub struct RSparkError {
    message: String,
}

impl RSparkError {
    pub fn new(message: &str) -> RSparkError {
        RSparkError {
            message: message.to_string(),
        }
    }
    pub fn to_string(&self) -> String{
       let message = &self.message;
        message.to_string()
    }
}

