//! Module implementing addition intrinsics.

use crate::u256;
use std::mem::MaybeUninit;

#[inline]
pub fn add2(r: &mut u256, a: &u256) {
    let (lo, carry) = r.low().overflowing_add(*a.low());
    *r.low_mut() = lo;
    *r.high_mut() = r.high().wrapping_add(carry as _).wrapping_add(*a.high());
}

#[inline]
pub fn add3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let (lo, carry) = a.low().overflowing_add(*b.low());
    let hi = a.high().wrapping_add(carry as _).wrapping_add(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}

#[inline]
pub fn addc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool {
    let (lo, carry_lo) = a.low().overflowing_add(*b.low());
    let (hi, carry_c) = a.high().overflowing_add(carry_lo as _);
    let (hi, carry_hi) = hi.overflowing_add(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
    carry_c || carry_hi
}
