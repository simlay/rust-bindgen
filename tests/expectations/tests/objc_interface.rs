/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub struct struct_Foo(id);
impl std::ops::Deref for struct_Foo {
    type Target = id;
    fn deref(&self) -> &Self::Target {
        unsafe { ::core::mem::transmute(self.0) }
    }
}
unsafe impl objc::Message for struct_Foo {}
impl interface_Foo for struct_Foo {}
pub trait interface_Foo: Sized + std::ops::Deref + objc::Message {}
pub trait protocol_bar: Sized + std::ops::Deref + objc::Message {}
