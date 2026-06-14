use std::ffi::{CString, c_void};

use crate::VTABLE;

pub unsafe fn get_method_addr(
    klass: *mut c_void,
    method_name: &str,
    arg_count: i32,
) -> *mut c_void {
    let vtable = unsafe { VTABLE.unwrap() };
    let method_cstr = CString::new(method_name).unwrap();
    unsafe { (vtable.il2cpp_get_method_addr)(klass, method_cstr.as_ptr(), arg_count) }
}

pub unsafe fn get_class(image: *const c_void, namespace: &str, class_name: &str) -> *mut c_void {
    let vtable = unsafe { VTABLE.unwrap() };
    let ns_cstr = CString::new(namespace).unwrap();
    let class_cstr = CString::new(class_name).unwrap();
    unsafe { (vtable.il2cpp_get_class)(image, ns_cstr.as_ptr(), class_cstr.as_ptr()) }
}

pub unsafe fn get_assembly_image(name: &str) -> *const c_void {
    let vtable = unsafe { VTABLE.unwrap() };
    let name_cstr = CString::new(name).unwrap();
    unsafe { (vtable.il2cpp_get_assembly_image)(name_cstr.as_ptr()) }
}

pub unsafe fn get_hachimi_and_interceptor() -> (*const c_void, *const c_void) {
    let vtable = unsafe { VTABLE.unwrap() };
    let hachimi = unsafe { (vtable.hachimi_instance)() };
    let interceptor = unsafe { (vtable.hachimi_get_interceptor)(hachimi) };
    (hachimi, interceptor)
}
