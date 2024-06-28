[![Rust](https://github.com/rodrimati1992/notzero/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/notzero/actions)
[![crates-io](https://img.shields.io/crates/v/notzero.svg)](https://crates.io/crates/notzero)
[![api-docs](https://docs.rs/notzero/badge.svg)](https://docs.rs/notzero/*)

Provides the [`nz`] macro for constructing a 
`std::num::NonZero*` from an integer constant.

The [`nz`] macro can infer the argument's type from 
the inferred `NonZero*` return type,
while some alternative macros always require that 
the argument's type is specified.

# Example

### Usage

```rust
use notzero::nz;
use std::num::NonZeroU64;

let two = nz!(2u16); // returns a `NonZeroU16`
assert_eq!(two.get(), 2u16);

// infers the argument's type from the returned `NonZero`
const THREE: NonZeroU64 = nz!(3); 
assert_eq!(THREE.get(), 3u64);

const FOUR: i8 = -4;
let fourz = nz!(FOUR); // returns a `NonZeroI8`
assert_eq!(fourz.get(), -4i8);
```

### Zero argument

```compile_fail
const ZERO: u8 = 0;
let _ = notzero::nz!(ZERO);
```
the above code produces this compile-time error:
```text
error[E0080]: evaluation of `main::_doctest_main_src_lib_rs_27_0::{constant#0}` failed
 --> src/lib.rs:32:9
  |
8 | let _ = notzero::nz!(ZERO);
  |         ^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'passed in a `0` argument', src/lib.rs:8:9
  |
  = note: this error originates in the macro `notzero::nz` (in Nightly builds, run with -Z macro-backtrace more info)
```

# No-std support

`notzero` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

This crate requires Rust 1.79.0 because it uses `const { ... }` expressions
(also known as "inline const").

[`inline_const`]: https://github.com/rust-lang/rust/issues/76001
[`nz`]: https://docs.rs/notzero/latest/notzero/macro.nz.html
