#[macro_use]
extern crate lazy_static;

use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    Io,
    Json,
    InvalidWProfX,
    Quiche,
    LoadFail,
    Timeout(Duration),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Self {
        Error::Io
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Self {
        Error::Json
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod activity;
pub mod dependency;
pub mod resource;
pub mod mahimahi;