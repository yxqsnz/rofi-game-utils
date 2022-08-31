use std::io::{Error, ErrorKind};

/// Creates a custom IO Error type.
pub fn io_custom(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}
