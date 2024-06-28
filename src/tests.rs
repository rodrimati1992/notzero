use core::{
    any::{type_name, TypeId},
    num::{
        NonZeroI8,
        NonZeroI16,
        NonZeroI32,
        NonZeroI64,
        NonZeroI128,
        NonZeroIsize,
        NonZeroU8,
        NonZeroU16,
        NonZeroU32,
        NonZeroU64,
        NonZeroU128,
        NonZeroUsize,
    },
};

fn assert_type<A: 'static, B: 'static>(arg: A) -> A {
    assert_eq!(
        TypeId::of::<A>(), 
        TypeId::of::<B>(), 
        "\nA: {}\nB:{}\n",
        type_name::<A>(),
        type_name::<B>(),
    );

    arg
}



macro_rules! constructor_test_case {
    ($nz_ty:ident $int_ty:ident) => ({
        {
            let _ = assert_type::<_, $nz_ty>(crate::nz!(1 as $int_ty));

            assert_eq!(crate::nz!(1 as $int_ty).get(), 1 as $int_ty);
            assert_eq!(crate::nz!($int_ty::MAX).get(), $int_ty::MAX);
        }
        {
            let nz: $nz_ty = crate::nz!(1);
            assert_eq!(nz.get(), 1);
        }
        {
            let nz: $nz_ty = crate::nz!(44);
            assert_eq!(nz.get(), 44);
        }
    })
}

#[test]
fn constructor_i8() {
    constructor_test_case!{NonZeroI8 i8}
}
#[test]
fn constructor_i16() {
    constructor_test_case!{NonZeroI16 i16}
}
#[test]
fn constructor_i32() {
    constructor_test_case!{NonZeroI32 i32}
}
#[test]
fn constructor_i64() {
    constructor_test_case!{NonZeroI64 i64}
}
#[test]
fn constructor_i128() {
    constructor_test_case!{NonZeroI128 i128}
}
#[test]
fn constructor_isize() {
    constructor_test_case!{NonZeroIsize isize}
}
#[test]
fn constructor_u8() {
    constructor_test_case!{NonZeroU8 u8}
}
#[test]
fn constructor_u16() {
    constructor_test_case!{NonZeroU16 u16}
}
#[test]
fn constructor_u32() {
    constructor_test_case!{NonZeroU32 u32}
}
#[test]
fn constructor_u64() {
    constructor_test_case!{NonZeroU64 u64}
}
#[test]
fn constructor_u128() {
    constructor_test_case!{NonZeroU128 u128}
}
#[test]
fn constructor_usize() {
    constructor_test_case!{NonZeroUsize usize}
}

#[cfg(feature = "__ui_tests")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("src/ui_tests/*fine.rs");
    t.compile_fail("src/ui_tests/*err.rs");
}

