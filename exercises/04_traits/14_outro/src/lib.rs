// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaturatingU16(u16);


impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}


impl <'a> From<&'a u16> for SaturatingU16 {
    fn from(value: &'a u16) -> Self {
        SaturatingU16(value.clone())
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value.into())
    }
}

impl <'a> From<&'a u8> for SaturatingU16 {
    fn from(value: &'a u8) -> Self {
        SaturatingU16(value.clone().into())
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;

    fn add(self, rhs: u16) -> Self::Output {
        self.0.saturating_add(rhs)
    }
}

impl <'a> Add<&'a SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &'a SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.clone().0))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}