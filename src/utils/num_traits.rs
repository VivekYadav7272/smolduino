use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};
pub trait UnsignedNumber:
    Add<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Not<Output = Self>
    + From<u8>
    + Sized
    + Clone
    + Copy
    + TryInto<u8>
{
}

impl UnsignedNumber for u64 {}
impl UnsignedNumber for u32 {}
impl UnsignedNumber for u16 {}
impl UnsignedNumber for u8 {}
