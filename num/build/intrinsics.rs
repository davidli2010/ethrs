//! This module is used as atemplate to generate LLVM IR for `u128` intrinsics
//! which is used as a template `u256` intrinsics.
//!
//! This is done instead of hand-writting LLVM IR to ensure that the attributes
//! on functions and parameters are as accurate as possible for the resulting
//! IR, allowing for better optimizations when using plugin LTO optimizations.

use std::mem::MaybeUninit;

macro_rules! def {
    ($(
        pub unsafe fn $name:ident(
            $($p:ident : $t:ty),*
        ) $(-> $ret:ty)? $impl:block
    )*) => {$(
        export! {
            name = concat!("__ethnum_", stringify!($name));
            pub unsafe extern "C" fn $name(
                $($p: $t,)*
            ) $(-> $ret)? {
                $impl
            }
        }
    )*};
}

macro_rules! export {
    (name = $sym:expr; $fn:item) => {
        #[export_name = $sym]
        $fn
    };
}

def! {
    pub unsafe fn add2(r: &mut u128, a: &u128) {
        *r += *a;
    }
    pub unsafe fn add3(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) {
        let res = a.wrapping_add(*b);
        r.as_mut_ptr().write(res);
    }
    pub unsafe fn addc(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) -> bool {
        let (res, carry) = a.overflowing_add(*b);
        r.as_mut_ptr().write(res);
        carry
    }

    pub unsafe fn sub2(r: &mut u128, a: &u128) {
        *r -= *a;
    }
    pub unsafe fn sub3(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) {
        let res = a.wrapping_sub(*b);
        r.as_mut_ptr().write(res);
    }
    pub unsafe fn subc(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) -> bool {
        let (res, carry) = a.overflowing_sub(*b);
        r.as_mut_ptr().write(res);
        carry
    }

    pub unsafe fn mul2(r: &mut u128, a: &u128) {
        *r *= *a;
    }
    pub unsafe fn mul3(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) {
        let res = a.wrapping_mul(*b);
        r.as_mut_ptr().write(res);
    }
    pub unsafe fn mulc(r: &mut MaybeUninit<u128>, a: &u128, b: &u128) -> bool {
        let (res, carry) = a.overflowing_mul(*b);
        r.as_mut_ptr().write(res);
        carry
    }

    pub unsafe fn shl2(r: &mut u128, a: u32) {
        *r <<= a;
    }
    pub unsafe fn shl3(r: &mut MaybeUninit<u128>, a: &u128, b: u32) {
        let res = a.wrapping_shl(b);
        r.as_mut_ptr().write(res);
    }

    pub unsafe fn shr2(r: &mut u128, a: u32) {
        *r >>= a;
    }
    pub unsafe fn shr3(r: &mut MaybeUninit<u128>, a: &u128, b: u32) {
        let res = a.wrapping_shr(b);
        r.as_mut_ptr().write(res);
    }

    pub unsafe fn rotate_left(r: &mut MaybeUninit<u128>, a: &u128, b: u32) {
        r.as_mut_ptr().write(a.rotate_left(b))
    }
    pub unsafe fn rotate_right(r: &mut MaybeUninit<u128>, a: &u128, b: u32) {
        r.as_mut_ptr().write(a.rotate_right(b))
    }

    pub unsafe fn ctlz(a: &u128) -> u32 {
        a.leading_zeros()
    }
    pub unsafe fn cttz(a: &u128) -> u32 {
        a.trailing_zeros()
    }
}
