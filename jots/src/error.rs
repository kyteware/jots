use core::fmt;
use std::{io, fmt::{Display, Formatter}};

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    Fs(io::ErrorKind),
    NoDataDir,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}