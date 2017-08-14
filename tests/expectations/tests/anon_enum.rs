/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, PartialEq)]
pub struct Test {
    pub foo: ::std::os::raw::c_int,
    pub bar: f32,
}
pub const Test_T_NONE: Test__bindgen_ty_1 = Test__bindgen_ty_1::T_NONE;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Test__bindgen_ty_1 { T_NONE = 0, }
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(::std::mem::size_of::<Test>() , 8usize , concat ! (
               "Size of: " , stringify ! ( Test ) ));
    assert_eq! (::std::mem::align_of::<Test>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( Test ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Test ) ) . foo as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( Test ) , "::" ,
                stringify ! ( foo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Test ) ) . bar as * const _ as usize } ,
                4usize , concat ! (
                "Alignment of field: " , stringify ! ( Test ) , "::" ,
                stringify ! ( bar ) ));
}
impl Clone for Test {
    fn clone(&self) -> Self { *self }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Baz { Foo = 0, Bar = 1, }
