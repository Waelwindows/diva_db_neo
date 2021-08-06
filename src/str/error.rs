use core::str::Utf8Error;

use crate::util::read::offset::ParseOffsetError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReadStringError {
    ParseOffsetError(ParseOffsetError),
    OffsetOutOfRange,
    MissingNullTerminator,
    Utf8Error(Utf8Error),
}


impl From<ParseOffsetError> for ReadStringError {
    fn from(err: ParseOffsetError) -> Self {
        Self::ParseOffsetError(err)
    }
}

impl From<Utf8Error> for ReadStringError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}
