//! Module contains iterator specific trait implementations.

use crate::U256;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

impl Sum for U256 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(U256::ZERO, Add::add)
    }
}

impl Product for U256 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(U256::ONE, Mul::mul)
    }
}

impl<'a> Sum<&'a U256> for U256 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(U256::ZERO, Add::add)
    }
}

impl<'a> Product<&'a U256> for U256 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(U256::ONE, Mul::mul)
    }
}

// TODO(nlordell): Implement `std::iter::Step` once it stabilizes.
