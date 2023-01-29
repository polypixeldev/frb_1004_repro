use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_add(port_: i64, a: usize, b: usize) {
    wire_add_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_foo(port_: i64, test: wire_TestStruct) {
    wire_foo_impl(port_, test)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_TestStruct() -> wire_TestStruct {
    wire_TestStruct::new_with_null_ptr()
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<TestStruct>> for wire_TestStruct {
    fn wire2api(self) -> RustOpaque<TestStruct> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_TestStruct {
    ptr: *const core::ffi::c_void,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_TestStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
