//! Use [memsec](https://github.com/quininer/memsec) protected secret memory.

#![cfg_attr(not(feature = "use_std"), no_std)]
#![cfg_attr(feature = "nightly", feature(i128_type))]

#[cfg(feature = "use_std")] extern crate core;
extern crate memsec;

mod tempkey;
#[cfg(feature = "use_std")] mod seckey;

use core::mem;
use memsec::memzero;

pub use zerosafe::ZeroSafe;
pub use tempkey::*;
#[cfg(feature = "use_std")] pub use seckey::*;


/// Zero a value
///
/// ```
/// use seckey::zero;
///
/// let mut v = [1, 2, 3];
/// zero(&mut v);
/// assert_eq!(v, [0, 0, 0]);
///
/// let mut v = &mut [1, 2, 3][..];
/// zero(v);
/// assert_eq!(v, [0, 0, 0]);
/// ```
pub fn zero<T: ?Sized + ZeroSafe>(t: &mut T) {
    unsafe { memzero(t as *mut T as *mut u8, mem::size_of_val(t)) };
}

mod zerosafe {
    pub unsafe trait ZeroSafe {}

    macro_rules! impl_zerosafe {
        ( Type : $( $t:ty ),* ) => {
            $(
                unsafe impl ZeroSafe for $t {}
            )*
        };
        ( Generic : $( $t:ty ),* ) => {
            $(
                unsafe impl<T: ?Sized> ZeroSafe for $t {}
            )*
        };
        ( Array : $( $n:expr ),* ) => {
            $(
                unsafe impl<T: ZeroSafe> ZeroSafe for [T; $n] {}
            )*
        }
    }

    impl_zerosafe!{ Type:
        usize, u8, u16, u32, u64,
        isize, i8, i16, i32, i64
    }

    #[cfg(feature = "nightly")]
    impl_zerosafe!{ Type: u128, i128 }

    impl_zerosafe!{ Generic: *const T, *mut T }

    impl_zerosafe!{ Array:
         0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
        64,

        128, 256, 384, 512, 1024, 2048
    }

    unsafe impl ZeroSafe for str {}
    unsafe impl<T: ZeroSafe> ZeroSafe for [T] {}
}
