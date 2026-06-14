mod api;
mod log;
mod vtable;

use std::ffi::{CString, c_void};
use std::sync::Once;

use crate::api::{get_assembly_image, get_class, get_hachimi_and_interceptor, get_method_addr};
use crate::vtable::Vtable;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

static INIT: Once = Once::new();
static mut VTABLE_PTR: *const Vtable = std::ptr::null();
static mut API_VERSION: i32 = 0;

static mut HEADER: *mut c_void = std::ptr::null_mut();

extern "C" fn on_menu_click(_userdata: *mut c_void) {
    unsafe {
        let vtable = VTABLE_PTR.as_ref();
        if let Some(vtable) = vtable {
            let image = get_assembly_image("umamusume");

            let gauge: *mut c_void;

            if !HEADER.is_null() {
                let addr = get_method_addr(
                    get_class(image, "Gallop", "SingleModeMainHeaderAndFooterController"),
                    "get_HpGauge",
                    0,
                );
                type GetHpGauge = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
                gauge = (std::mem::transmute::<_, GetHpGauge>(addr))(HEADER);
                log!("Gauge addr: {:?}", gauge);
            } else {
                let message = CString::new("Header not in scope.").unwrap();
                (vtable.gui_show_notification)(message.as_ptr());
                return;
            }
            if !gauge.is_null() {
                let addr = get_method_addr(
                    get_class(image, "Gallop", "SingleModeMainViewHpGauge"),
                    "get_Hp",
                    0,
                );
                type GetHp = unsafe extern "C" fn(*mut c_void) -> i32;
                let hp = (std::mem::transmute::<_, GetHp>(addr))(gauge);

                let message = CString::new(format!("Current energy: {}", hp)).unwrap();
                (vtable.gui_show_notification)(message.as_ptr());
            } else {
                return;
            }
        }
    }
}

static mut VTABLE: Option<&'static Vtable> = None;

#[unsafe(no_mangle)]
pub extern "C" fn hachimi_init(vtable: *const Vtable, version: i32) -> InitResult {
    if vtable.is_null() || version < 2 {
        return InitResult::Error;
    }

    unsafe {
        VTABLE = Some(&*vtable);
        VTABLE_PTR = vtable;
        API_VERSION = version;
    }

    INIT.call_once(|| unsafe {
        let vtable = &*vtable;
        let title = CString::new("Show Energy").unwrap();
        let (_, interceptor) = get_hachimi_and_interceptor();
        let image = get_assembly_image("umamusume");
        let header_init_addr = get_method_addr(
            get_class(image, "Gallop", "SingleModeMainHeaderAndFooterController"),
            "Initialize",
            0,
        );

        ((vtable.interceptor_hook)(interceptor, header_init_addr, header_hijack as *mut c_void));
        (vtable.gui_register_menu_item)(title.as_ptr(), Some(on_menu_click), std::ptr::null_mut());
    });

    InitResult::Ok
}

type UpdateFunc = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" fn header_hijack(this: *mut c_void) {
    if let Some(vtable) = unsafe { VTABLE } {
        let (_, interceptor) = unsafe { get_hachimi_and_interceptor() };

        let trampoline = unsafe {
            (vtable.interceptor_get_trampoline_addr)(interceptor, header_hijack as *mut c_void)
        };
        let original: UpdateFunc = unsafe { std::mem::transmute(trampoline) };

        unsafe {
            HEADER = this;
            original(this);
        };
    }
}
