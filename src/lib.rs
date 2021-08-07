#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::convert::TryFrom;
use core::iter::FromIterator;

pub mod str;
pub(crate) mod util;

/// Implements a zero copy database baed on iterators
pub trait IteratorDatabase: Iterator + Sized {
    type ParseError;

    fn parse(i: &[u8]) -> Result<Self, Self::ParseError>;
}

pub trait Database
where
    Self: TryFrom<Self::Iterator>,
    <Self as TryFrom<Self::Iterator>>::Error:
        From<<Self::Iterator as IteratorDatabase>::ParseError>,
    Self: FromIterator<<Self::Iterator as Iterator>::Item>,
{
    type Iterator: IteratorDatabase;

    fn parse(i: &[u8]) -> Result<Self, <Self as TryFrom<Self::Iterator>>::Error> {
        let iter = Self::Iterator::parse(i)?;
        Self::try_from(iter)
    }
}
