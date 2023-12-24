use core::fmt;
use std::{
    fmt::{Display, Formatter},
    io,
};

#[derive(Debug, Clone)]
pub enum Error {
    Fs(io::ErrorKind),
    NoDataDir,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
