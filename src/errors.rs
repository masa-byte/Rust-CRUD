use core::fmt;

pub enum CustomError {
    NotFound(String),
    InternalServerError(String),
}

impl From<sqlx::Error> for CustomError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => CustomError::NotFound(format!("Error - Row not found error: {}", e)),
            _ => CustomError::InternalServerError(format!("Error - Database error: {}", e))
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotFound(msg) => write!(f, "{}", msg),
            CustomError::InternalServerError(msg) => write!(f, "{}", msg),
        }
    }
}
