/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


pub const foo: _bindgen_ty_1 = _bindgen_ty_1::foo;
pub const bar: _bindgen_ty_1 = _bindgen_ty_1::bar;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 { foo = 4, bar = 8, }
pub type EasyToOverflow = ::std::os::raw::c_ulonglong;
pub const k: EasyToOverflow = 2147483648;
extern "C" {
    #[link_name = "k_expr"]
    pub static mut k_expr: EasyToOverflow;
}
extern "C" {
    #[link_name = "BAZ"]
    pub static mut BAZ: ::std::os::raw::c_longlong;
}
extern "C" {
    #[link_name = "fuzz"]
    pub static mut fuzz: f64;
}
extern "C" {
    #[link_name = "BAZZ"]
    pub static mut BAZZ: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "WAT"]
    pub static mut WAT: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "bytestring"]
    pub static mut bytestring: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "NOT_UTF8"]
    pub static mut NOT_UTF8: *const ::std::os::raw::c_char;
}
