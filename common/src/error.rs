use std::{io, num};
use std::io::Error;

#[derive(Debug)]
pub enum CommonError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CommonError {
    fn from(err: Error) -> Self {
        CommonError::Io(err)
    }
}

impl From<num::ParseIntError> for CommonError {
    fn from(err: num::ParseIntError) -> Self {
        CommonError::Parse(err)
    }
}
