use std::io::Error;
use std::{io, num};

#[derive(Debug)]
pub enum CommonError {
    Io(io::Error),
    IntParse(num::ParseIntError),
    Parse(&'static str),
}

impl From<io::Error> for CommonError {
    fn from(err: Error) -> Self {
        CommonError::Io(err)
    }
}

impl From<num::ParseIntError> for CommonError {
    fn from(err: num::ParseIntError) -> Self {
        CommonError::IntParse(err)
    }
}
