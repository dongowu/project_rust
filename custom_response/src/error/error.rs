use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct CustomError{
    message: String,
    code :i8,
}
impl CustomError {
    pub fn new(message:&str,code :i8)->Self{
        Self{
            message:message.to_string(),
            code,
        }
    }
}