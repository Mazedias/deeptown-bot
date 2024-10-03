use std::fmt;

pub struct WebRequestError {
    pub code: usize,
    pub message: String,
}

impl fmt::Display for WebRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            100 => "Requested data not recieved!",
            _ => "Error occured!",
        };
        
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for WebRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {} }}", self.code, self.message
        )
    }
}