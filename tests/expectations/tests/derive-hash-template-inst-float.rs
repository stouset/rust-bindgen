/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]



/// Template definition that doesn't contain float can derive hash/partialeq/eq
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo<T> {
    pub data: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for foo<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
/// Can derive hash/partialeq/eq when instantiated with int
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct IntStr {
    pub a: foo<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_IntStr() {
    assert_eq!(
        ::std::mem::size_of::<IntStr>(),
        4usize,
        concat!("Size of: ", stringify!(IntStr))
    );
    assert_eq!(
        ::std::mem::align_of::<IntStr>(),
        4usize,
        concat!("Alignment of ", stringify!(IntStr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const IntStr)).a as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(IntStr),
            "::",
            stringify!(a)
        )
    );
}
impl Clone for IntStr {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for IntStr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
/// Cannot derive hash/eq when instantiated with float but can derive partialeq
#[repr(C)]
#[derive(Debug, Copy, PartialEq)]
pub struct FloatStr {
    pub a: foo<f32>,
}
#[test]
fn bindgen_test_layout_FloatStr() {
    assert_eq!(
        ::std::mem::size_of::<FloatStr>(),
        4usize,
        concat!("Size of: ", stringify!(FloatStr))
    );
    assert_eq!(
        ::std::mem::align_of::<FloatStr>(),
        4usize,
        concat!("Alignment of ", stringify!(FloatStr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const FloatStr)).a as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(FloatStr),
            "::",
            stringify!(a)
        )
    );
}
impl Clone for FloatStr {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for FloatStr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn __bindgen_test_layout_foo_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<foo<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify ! ( foo < :: std :: os :: raw :: c_int > )
        )
    );
    assert_eq ! ( :: std :: mem :: align_of :: < foo < :: std :: os :: raw :: c_int > > ( ) , 4usize , concat ! ( "Alignment of template specialization: " , stringify ! ( foo < :: std :: os :: raw :: c_int > ) ) );
}
#[test]
fn __bindgen_test_layout_foo_open0_float_close0_instantiation() {
    assert_eq ! ( :: std :: mem :: size_of :: < foo < f32 > > ( ) , 4usize , concat ! ( "Size of template specialization: " , stringify ! ( foo < f32 > ) ) );
    assert_eq ! ( :: std :: mem :: align_of :: < foo < f32 > > ( ) , 4usize , concat ! ( "Alignment of template specialization: " , stringify ! ( foo < f32 > ) ) );
}
