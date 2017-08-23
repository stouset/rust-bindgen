/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]



/// <div rustbindgen opaque>
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_opaque() {
    assert_eq!(
        ::std::mem::size_of::<opaque>(),
        4usize,
        concat!("Size of: ", stringify!(opaque))
    );
    assert_eq!(
        ::std::mem::align_of::<opaque>(),
        4usize,
        concat!("Alignment of ", stringify!(opaque))
    );
}
impl Clone for opaque {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct container {
    pub contained: opaque,
}
#[test]
fn bindgen_test_layout_container() {
    assert_eq!(
        ::std::mem::size_of::<container>(),
        4usize,
        concat!("Size of: ", stringify!(container))
    );
    assert_eq!(
        ::std::mem::align_of::<container>(),
        4usize,
        concat!("Alignment of ", stringify!(container))
    );
    assert_eq!(
        unsafe { &(*(0 as *const container)).contained as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(container),
            "::",
            stringify!(contained)
        )
    );
}
impl Clone for container {
    fn clone(&self) -> Self {
        *self
    }
}
