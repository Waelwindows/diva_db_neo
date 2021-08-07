use core::convert::TryFrom;
use core::mem::size_of;
use core::num::TryFromIntError;

use crate::util::Endianness;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseOffsetError {
    NotEnoughBytes,
    OffsetOverflow(TryFromIntError),
}

// N shouldn't be a trait input due to min_generics.
pub trait ToOffset<const N: usize>: Sized {
    fn from_bytes(input: [u8; N], endian: Endianness) -> Self;
    fn from_slice(input: &[u8], endian: Endianness) -> Option<Self> {
        let bytes = input.get(..N)?;
        let bytes = TryFrom::try_from(bytes).ok()?;
        Some(Self::from_bytes(bytes, endian))
    }

    fn parse_offset(input: &[u8], endian: Endianness) -> Result<usize, ParseOffsetError>
    where
        usize: TryFrom<Self, Error = TryFromIntError>,
    {
        let val = Self::from_slice(input, endian).ok_or(ParseOffsetError::NotEnoughBytes)?;
        usize::try_from(val).map_err(ParseOffsetError::OffsetOverflow)
    }
}

macro_rules! impl_offset {
    ($($ty:ty),+) => {
        $(
        impl ToOffset<{ size_of::<$ty>() }> for $ty {
            fn from_bytes(input: [u8; size_of::<$ty>()], endian: Endianness) -> Self {
                match endian {
                    Endianness::Little => Self::from_le_bytes(input),
                    Endianness::Big => Self::from_be_bytes(input),
                }
            }
        }
        )+
    };
}

impl_offset!(u16, u32, u64);
