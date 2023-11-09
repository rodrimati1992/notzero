//! Provides the [`nz`] macro for constructing a 
//! `std::num::NonZero*` from an integer constant.
//! 
//! # Example
//! 
//! ### Usage
//! 
//! ```rust
//! #![feature(inline_const)]
//! 
//! use notzero::nz;
//! use std::num::NonZeroU64;
//! 
//! let two = nz!(2u16); // returns a `NonZeroU16`
//! assert_eq!(two.get(), 2u16);
//! 
//! // infers the argument's type from the returned `NonZero`
//! const THREE: NonZeroU64 = nz!(3); 
//! assert_eq!(THREE.get(), 3u64);
//! 
//! const FOUR: i8 = -4;
//! let fourz = nz!(FOUR); // returns a `NonZeroI8`
//! assert_eq!(fourz.get(), -4i8);
//! ```
//! 
//! ### Zero argument
//! 
//! ```compile_fail
//! #![feature(inline_const)]
//! 
//! const ZERO: u8 = 0;
//! let _ = notzero::nz!(ZERO);
//! ```
//! the above code produces this compile-time error:
//! ```text
//! error[E0080]: evaluation of `main::_doctest_main_src_lib_rs_27_0::{constant#0}` failed
//!  --> src/lib.rs:32:9
//!   |
//! 8 | let _ = notzero::nz!(ZERO);
//!   |         ^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'passed in a `0` argument', src/lib.rs:8:9
//!   |
//!   = note: this error originates in the macro `notzero::nz` (in Nightly builds, run with -Z macro-backtrace for more info)
//! ```
//! 
//! # No-std support
//!
//! `notzero` is `#![no_std]`, it can be used anywhere Rust can be used.
//! 
//! # Minimum Supported Rust Version
//! 
//! This crate currently requires users to enable the [`inline_const`] unstable feature,
//! which means that it requires the nightly compiler.
//! 
//! This crate will support stable Rust once [`inline_const`] stabilizes,
//! requiring only the Rust version where the feature is stabilized.
//! 
//! [`inline_const`]: https://github.com/rust-lang/rust/issues/76001
//! [`nz`]: crate::nz
#![no_std]
#![cfg_attr(test, feature(inline_const))]

#![deny(unused_results)]
#![deny(clippy::missing_safety_doc)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use typewit::TypeEq;

#[cfg(test)]
mod tests;


/// Constructs a `std::num::NonZero*` from an integer constant.
/// 
/// # Type Inference
/// 
/// This macro can infer the argument's type from the return type and vice-versa,
/// so you can do either of:
/// - `let _ = nz!(1u8);`
/// - `let _: NonZeroU8 = nz!(1);`
/// 
/// # Compile-time errors
/// 
/// This macro checks that the `$int` argument is non-zero at compile-time,
/// producing a compile-time error if it is `0`.
/// 
/// # Example
/// 
/// ### Usage
/// 
/// ```rust
/// #![feature(inline_const)]
/// 
/// use notzero::nz;
/// use std::num::NonZeroU64;
/// 
/// let two = nz!(2u16); // returns a `NonZeroU16`
/// assert_eq!(two.get(), 2u16);
/// 
/// // infers the argument's type from the returned `NonZero`
/// let THREE: NonZeroU64 = nz!(3); 
/// assert_eq!(THREE.get(), 3u64);
/// ```
/// 
/// ### Zero argument
/// 
/// ```compile_fail
/// #![feature(inline_const)]
/// 
/// let _ = notzero::nz!(0);
/// ```
/// the above code produces this compile-time error:
/// ```text
/// error[E0080]: evaluation of `main::_doctest_main_src_lib_rs_77_0::{constant#0}` failed
///  --> src/lib.rs:81:9
///   |
/// 7 | let _ = notzero::nz!(0);
///   |         ^^^^^^^^^^^^^^^ the evaluated program panicked at 'passed in a `0` argument', src/lib.rs:7:9
///   |
///   = note: this error originates in the macro `notzero::nz` (in Nightly builds, run with -Z macro-backtrace for more info)
/// ```
#[macro_export]
macro_rules! nz {
    ($int:expr) => {
        const { $crate::__nonzero_new_unwrap($int) }
    };
}


macro_rules! call_back_with_integer_types {
    ($callback:ident) => {
        $callback!{
            (   NonZeroI8,    i8)
            (  NonZeroI16,   i16)
            (  NonZeroI32,   i32)
            (  NonZeroI64,   i64)
            ( NonZeroI128,  i128)
            (NonZeroIsize, isize)
            (   NonZeroU8,    u8)
            (  NonZeroU16,   u16)
            (  NonZeroU32,   u32)
            (  NonZeroU64,   u64)
            ( NonZeroU128,  u128)
            (NonZeroUsize, usize)
        }
    }
}



macro_rules! declare_withness_stuff {
    ( $(($nz_ty:ident, $int_ty:ident))* ) => {
        use core::num::{$($nz_ty),*};

        #[doc(hidden)]
        pub trait __Integer: Copy + 'static {
            type NonZero: __NonZeroInt<Zeroable = Self> + Copy + 'static;

            #[doc(hidden)]
            const __WITNESS: __IntegerWit<Self>;
        }

        #[doc(hidden)]
        pub trait __NonZeroInt: Copy + 'static {
            type Zeroable: __Integer<NonZero = Self> + Copy + 'static;
        }

        typewit::type_fn!{
            #[doc(hidden)]
            pub struct __ToNonZeroFn;

            impl<I: __Integer> I => I::NonZero;
        }


        #[allow(missing_debug_implementations)]
        #[doc(hidden)]
        #[non_exhaustive]
        #[allow(non_camel_case_types)]
        pub enum __IntegerWit<W> {
            $(
                $int_ty(TypeEq<W, $int_ty>),
            )*
        }

        $(
            impl __Integer for $int_ty {
                type NonZero = $nz_ty;

                #[doc(hidden)]
                const __WITNESS: __IntegerWit<Self> = __IntegerWit::$int_ty(TypeEq::NEW);
            }
            impl __NonZeroInt for $nz_ty {
                type Zeroable = $int_ty;
            }
        )*
    };
}

call_back_with_integer_types!{ declare_withness_stuff }



#[doc(hidden)]
#[track_caller]
pub const fn __nonzero_new_unwrap<I, NZ>(n: I) -> NZ
where
    I: __Integer<NonZero = NZ>,
    NZ: __NonZeroInt<Zeroable = I>
{
    macro_rules! __inner {
        ( $(($nz_ty:ident, $int_ty:ident))* ) => {
            match I::__WITNESS {
                $(
                    __IntegerWit::$int_ty(te) => {
                        let Some(ret) = $nz_ty::new(te.to_right(n)) 
                        else { panic!("passed in a `0` argument") };

                        te.map(__ToNonZeroFn).to_left(ret)
                    }
                )*
            }
        }
    }

    call_back_with_integer_types!{__inner}
}