use std::error::Error;
use std::fmt;

#[derive(Debug)] // derive std::fmt::Debug on RecipeError
pub struct RecipeError {
    pub message: String,
}

impl fmt::Display for RecipeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RecipeError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}
