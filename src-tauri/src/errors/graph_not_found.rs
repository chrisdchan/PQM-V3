use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct GraphNotFoundError {}

impl Error for GraphNotFoundError {
    fn description(&self) -> &str {
        "Graph Not Found"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for GraphNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Graph Not Found")
    }
}
