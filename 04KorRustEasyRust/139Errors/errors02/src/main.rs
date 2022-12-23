use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum CompanyError {
    CouldntConnect,
    NotEnoughData,
    UserTimeOut,
}
impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "got a CompanyError")
    }
}

#[derive(Debug)]
struct BaseError;

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "got a BaseError")
    }
}

impl Error for CompanyError {}

impl Error for BaseError {}

fn main() {}
