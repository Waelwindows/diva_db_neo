pub mod read;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}
