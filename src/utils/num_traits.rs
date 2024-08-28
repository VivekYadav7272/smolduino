use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};
pub trait UnsignedNumber:
    Add<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Not<Output = Self>
    + Sized
    + Clone
    + Copy
    + TryInto<u8>
    + TryInto<u16>
    + TryInto<u32>
    + TryInto<u64>
    + TryInto<usize>
    + From<u8>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<usize>
{
}

impl UnsignedNumber for u64 {}
impl UnsignedNumber for u32 {}
impl UnsignedNumber for u16 {}
impl UnsignedNumber for u8 {}
