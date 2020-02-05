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
#[repr(transparent)]
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
    unsafe fn method()
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Foo), method)
    }
    unsafe fn methodWithInt_(foo: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Foo), methodWithInt: foo)
    }
    unsafe fn methodWithFoo_(foo: id)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Foo), methodWithFoo: foo)
    }
    unsafe fn methodReturningInt() -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Foo), methodReturningInt)
    }
    unsafe fn methodReturningFoo() -> *mut id
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Foo), methodReturningFoo)
    }
    unsafe fn methodWithArg1_andArg2_andArg3_(
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send ! ( class ! ( Foo ) , methodWithArg1 : intvalue andArg2 : ptr andArg3 : floatvalue )
    }
}
