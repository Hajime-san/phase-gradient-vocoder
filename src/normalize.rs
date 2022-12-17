use num_traits::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Mul, Sub};

pub trait MinMax {
    const MIN: Self;
    const MAX: Self;
}

impl MinMax for u8 {
    const MIN: u8 = u8::MIN;
    const MAX: u8 = u8::MAX;
}

impl MinMax for i16 {
    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;
}
// like trait alias
// it will be stable in the future https://rust-lang.github.io/rfcs/1733-trait-alias.html
pub trait GenericNormalize: MinMax + Add + Sub + Mul + FromPrimitive + ToPrimitive {}
impl GenericNormalize for u8 {}
impl GenericNormalize for i16 {}
