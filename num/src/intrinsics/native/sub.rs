/// Module implementing subtraction intrinsics.

use crate::u256;
use std::mem::MaybeUninit;

#[inline]
pub fn sub2(r: &mut u256, a: &u256) {
    let (lo, carry) = r.low().overflowing_sub(*a.low());
    *r.low_mut() = lo;
    *r.high_mut() = r.high().wrapping_sub(carry as _).wrapping_sub(*a.high());
}

#[inline]
pub fn sub3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let (lo, carry) = a.low().overflowing_sub(*b.low());
    let hi = a.high().wrapping_sub(carry as _).wrapping_sub(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}

#[inline]
pub fn subc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool {
    let (lo, carry_lo) = a.low().overflowing_sub(*b.low());
    let (hi, carry_c) = a.high().overflowing_sub(carry_lo as _);
    let (hi, carry_hi) = hi.overflowing_sub(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
    carry_c || carry_hi
}
