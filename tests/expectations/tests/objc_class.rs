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
extern "C" {
    pub static mut fooVar: *mut id;
}
pub struct struct_Foo(pub id);
impl std::ops::Deref for struct_Foo {
    type Target = id;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
unsafe impl objc::Message for struct_Foo {}
impl struct_Foo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(Foo), alloc) })
    }
}
impl interface_Foo for struct_Foo {}
pub trait interface_Foo: Sized + std::ops::Deref + objc::Message {
    unsafe fn method(self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(self, method)
    }
}
