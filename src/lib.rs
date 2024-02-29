#![feature(core_intrinsics)]
#![allow(internal_features)]
#![feature(non_exhaustive_omitted_patterns_lint)]

use core::intrinsics;

pub trait N: Copy + std::ops::Add {
    fn from(num: u128) -> Self;
    fn into(self) -> u128;
}

impl N for u8 {
    fn from(num: u128) -> Self {
        num as u8
    }

    fn into(self) -> u128 {
        self as u128
    }
}

pub fn add<T>(a: u128, b: u128)
where
    T: N,
{
    let result = intrinsics::add_with_overflow(T::from(a), T::from(b));
}
