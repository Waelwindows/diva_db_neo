use super::error::ReadStringError;
use crate::util::read::offset::ToOffset;
use crate::util::Endianness;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct StringDatabase<'a> {
    data: &'a [u8],
    pub index: usize,
    done: bool,
    endian: Endianness,
}

impl<'a> StringDatabase<'a> {
    pub fn new(data: &[u8]) -> StringDatabase {
        let offset = u32::parse_offset(data, Endianness::Little);
        let endian = match offset {
            Ok(e) if e > data.len() => Endianness::Big,
            _ => Endianness::Little,
        };
        StringDatabase {
            data,
            endian,
            index: 0,
            done: false,
        }
    }

    pub const fn with_endian(data: &[u8], endian: Endianness) -> StringDatabase {
        StringDatabase {
            data,
            endian,
            index: 0,
            done: false,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.done = false;
    }

    pub fn read_at(&self, idx: usize) -> Option<Result<&'a str, ReadStringError>> {
        const PTR_SIZE: usize = core::mem::size_of::<u32>();
        let index = idx * PTR_SIZE;
        let slice = self.data.get(index..index + PTR_SIZE)?;
        if slice == [0; 4] {
            return None;
        }
        let res = || -> Result<_, _> {
            let offset = u32::parse_offset(slice, self.endian)?;
            let string_slice = self
                .data
                .get(offset..)
                .ok_or(ReadStringError::OffsetOutOfRange)?;
            let string_bytes = string_slice
                .iter()
                .position(|&x| x == 0)
                .and_then(|end| string_slice.get(..end))
                .ok_or(ReadStringError::MissingNullTerminator)?;
            let string = core::str::from_utf8(string_bytes)?;
            Ok(string)
        };
        Some(res())
    }
}

impl<'a> Iterator for StringDatabase<'a> {
    type Item = Result<&'a str, ReadStringError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let string = self.read_at(self.index);
        if string.is_some() {
            self.index += 1;
        } else {
            self.done = true;
        }
        string
    }
}

impl core::iter::FusedIterator for StringDatabase<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [u8] = b"\0\0\0\x0C\0\0\0\x12\0\0\0\0Hello\0Goodbye\0";

    #[test]
    fn read() {
        let mut strdb = StringDatabase::new(INPUT);

        assert_eq!(strdb.next(), Some(Ok("Hello")));
        assert_eq!(strdb.next(), Some(Ok("Goodbye")));
        assert_eq!(strdb.next(), None);

        strdb.reset();

        assert_eq!(strdb.next(), Some(Ok("Hello")));
        assert_eq!(strdb.next(), Some(Ok("Goodbye")));
        assert_eq!(strdb.next(), None);
    }
}
