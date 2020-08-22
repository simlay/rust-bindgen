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
pub struct Foo(id);
impl std::ops::Deref for Foo {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        Self(unsafe { msg_send!(self, retain) })
    }
}
impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { msg_send!(self, release) }
    }
}
unsafe impl objc::Message for Foo {}
impl Foo {
    /// Get the internal `id` of this class. This is unsafe because the user of
    /// this function must release the ID after is't used.
    pub unsafe fn id(&self) -> id {
        msg_send!(self, retain)
    }
    pub fn from_id(id: id) -> Self {
        Self(id)
    }
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(Foo), alloc) })
    }
}
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FooStruct {
    pub foo: Foo,
}
#[test]
fn bindgen_test_layout_FooStruct() {
    assert_eq!(
        ::std::mem::size_of::<FooStruct>(),
        8usize,
        concat!("Size of: ", stringify!(FooStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<FooStruct>(),
        8usize,
        concat!("Alignment of ", stringify!(FooStruct))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FooStruct>())).foo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FooStruct),
            "::",
            stringify!(foo)
        )
    );
}
impl Default for FooStruct {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn fooFunc(foo: Foo);
}
extern "C" {
    pub static mut kFoo: Foo;
}
