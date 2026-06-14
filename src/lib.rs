use std::env;
use std::ffi::{CString, c_char, c_void};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, Once};

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

static LOG_MUTEX: Mutex<()> = Mutex::new(());

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vtable {
    pub hachimi_instance: unsafe extern "C" fn() -> *const c_void,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const c_void) -> *const c_void,

    pub interceptor_hook: unsafe extern "C" fn(
        this: *const c_void,
        orig_addr: *mut c_void,
        hook_addr: *mut c_void,
    ) -> *mut c_void,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: *const c_void,
        vtable: *mut *mut c_void,
        vtable_index: usize,
        hook_addr: *mut c_void,
    ) -> *mut c_void,
    pub interceptor_get_trampoline_addr:
        unsafe extern "C" fn(this: *const c_void, hook_addr: *mut c_void) -> *mut c_void,
    pub interceptor_unhook:
        unsafe extern "C" fn(this: *const c_void, hook_addr: *mut c_void) -> *mut c_void,

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    pub il2cpp_get_assembly_image:
        unsafe extern "C" fn(assembly_name: *const c_char) -> *const c_void,
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const c_void,
        namespace: *const c_char,
        class_name: *const c_char,
    ) -> *mut c_void,
    pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *const c_void,
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        params: *const c_void,
        param_count: usize,
    ) -> *const c_void,
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *mut c_void,
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        params: *const c_void,
        param_count: usize,
    ) -> *mut c_void,
    pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *const c_void,
    pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: *mut c_void,
        name: *const c_char,
        args_count: i32,
    ) -> *mut c_void,
    pub il2cpp_find_nested_class:
        unsafe extern "C" fn(class: *mut c_void, name: *const c_char) -> *mut c_void,
    pub il2cpp_resolve_icall: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    pub il2cpp_class_get_methods:
        unsafe extern "C" fn(klass: *mut c_void, iter: *mut *mut c_void) -> *const c_void,
    pub il2cpp_get_field_from_name:
        unsafe extern "C" fn(class: *mut c_void, name: *const c_char) -> *mut c_void,
    pub il2cpp_get_field_value:
        unsafe extern "C" fn(obj: *mut c_void, field: *mut c_void, out_value: *mut c_void),
    pub il2cpp_set_field_value:
        unsafe extern "C" fn(obj: *mut c_void, field: *mut c_void, value: *const c_void),
    pub il2cpp_get_static_field_value:
        unsafe extern "C" fn(field: *mut c_void, out_value: *mut c_void),
    pub il2cpp_set_static_field_value:
        unsafe extern "C" fn(field: *mut c_void, value: *const c_void),
    pub il2cpp_object_new: unsafe extern "C" fn(klass: *const c_void) -> *mut c_void,
    pub il2cpp_unbox: unsafe extern "C" fn(obj: *mut c_void) -> *mut c_void,
    pub il2cpp_get_main_thread: unsafe extern "C" fn() -> *mut c_void,
    pub il2cpp_get_attached_threads: unsafe extern "C" fn(out_size: *mut usize) -> *mut *mut c_void,
    pub il2cpp_schedule_on_thread:
        unsafe extern "C" fn(thread: *mut c_void, callback: unsafe extern "C" fn()),
    pub il2cpp_create_array:
        unsafe extern "C" fn(element_type: *mut c_void, length: usize) -> *mut c_void,
    pub il2cpp_get_singleton_like_instance: unsafe extern "C" fn(class: *mut c_void) -> *mut c_void,

    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char),
    pub gui_register_menu_item: unsafe extern "C" fn(
        label: *const c_char,
        callback: Option<extern "C" fn(*mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_register_menu_section: unsafe extern "C" fn(
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_show_notification: unsafe extern "C" fn(message: *const c_char) -> bool,
    pub gui_ui_heading: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_label: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_separator: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_checkbox:
        unsafe extern "C" fn(ui: *mut c_void, text: *const c_char, value: *mut bool) -> bool,
    pub gui_ui_text_edit_singleline:
        unsafe extern "C" fn(ui: *mut c_void, buffer: *mut c_char, buffer_len: usize) -> bool,
    pub gui_ui_horizontal: unsafe extern "C" fn(
        ui: *mut c_void,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_grid: unsafe extern "C" fn(
        ui: *mut c_void,
        id: *const c_char,
        columns: usize,
        spacing_x: f32,
        spacing_y: f32,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_ui_end_row: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_colored_label: unsafe extern "C" fn(
        ui: *mut c_void,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        text: *const c_char,
    ) -> bool,
    pub gui_register_menu_item_icon: unsafe extern "C" fn(
        label: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
    ) -> bool,
    pub gui_register_menu_section_with_icon: unsafe extern "C" fn(
        title: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
        callback: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        userdata: *mut c_void,
    ) -> bool,

    pub android_dex_load:
        unsafe extern "C" fn(dex_ptr: *const u8, dex_len: usize, class_name: *const c_char) -> u64,
    pub android_dex_unload: unsafe extern "C" fn(handle: u64) -> bool,
    pub android_dex_call_static_noargs:
        unsafe extern "C" fn(handle: u64, method: *const c_char, sig: *const c_char) -> bool,
    pub android_dex_call_static_string: unsafe extern "C" fn(
        handle: u64,
        method: *const c_char,
        sig: *const c_char,
        arg: *const c_char,
    ) -> bool,
}

static INIT: Once = Once::new();
static mut VTABLE_PTR: *const Vtable = std::ptr::null();
static mut API_VERSION: i32 = 0;

extern "C" fn on_menu_click(_userdata: *mut c_void) {
    unsafe {
        let vtable = VTABLE_PTR.as_ref();
        if let Some(vtable) = vtable {
            let message = CString::new("Hello from the example plugin!").unwrap();
            (vtable.gui_show_notification)(message.as_ptr());
        }
    }
}

extern "C" fn on_menu_section(ui: *mut c_void, _userdata: *mut c_void) {
    unsafe {
        let vtable = VTABLE_PTR.as_ref();
        if let Some(vtable) = vtable {
            let heading = CString::new("Example Plugin").unwrap();
            (vtable.gui_ui_heading)(ui, heading.as_ptr());
            (vtable.gui_ui_separator)(ui);
            let label = CString::new("This section is rendered by the plugin.").unwrap();
            (vtable.gui_ui_label)(ui, label.as_ptr());
        }
    }
}

unsafe fn log(level: i32, tag: &str, message: &str) {
    let vtable = VTABLE.unwrap();
    let tag_cstr = CString::new(tag).unwrap();
    let msg_cstr = CString::new(message).unwrap();
    (vtable.log)(level, tag_cstr.as_ptr(), msg_cstr.as_ptr());
}

unsafe fn log_info(tag: &str, message: &str) {
    log(3, tag, message);
}

unsafe fn get_methods(klass: *mut c_void) -> impl Iterator<Item = *const c_void> {
    let mut iter: *mut c_void = std::ptr::null_mut();

    std::iter::from_fn(move || {
        let vtable = VTABLE.unwrap();
        let method = (vtable.il2cpp_class_get_methods)(klass, &mut iter);

        if method.is_null() { None } else { Some(method) }
    })
}

unsafe fn try_class(img: *const c_void, name: &str) {
    let klass = get_class(img, "Gallop", name);
    if klass.is_null() {
        log!("[-] {}", name);
    } else {
        log!("[+] FOUND CLASS: {}", name);
        //log!("Please {:?}", get_method_addr(klass, "get_HpGauge", 0));
        log!("Please {:?}", get_method_addr(klass, "get_Hp", 0));
    }
}

unsafe fn get_method_addr(klass: *mut c_void, method_name: &str, arg_count: i32) -> *mut c_void {
    let vtable = VTABLE.unwrap();
    let method_cstr = CString::new(method_name).unwrap();
    (vtable.il2cpp_get_method_addr)(klass, method_cstr.as_ptr(), arg_count)
}

unsafe fn get_class(image: *const c_void, namespace: &str, class_name: &str) -> *mut c_void {
    let vtable = VTABLE.unwrap();
    let ns_cstr = CString::new(namespace).unwrap();
    let class_cstr = CString::new(class_name).unwrap();
    (vtable.il2cpp_get_class)(image, ns_cstr.as_ptr(), class_cstr.as_ptr())
}

unsafe fn get_assembly_image(name: &str) -> *const c_void {
    let vtable = VTABLE.unwrap();
    let name_cstr = CString::new(name).unwrap();
    (vtable.il2cpp_get_assembly_image)(name_cstr.as_ptr())
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
        let title = CString::new("Example Plugin").unwrap();
        let image = get_assembly_image("umamusume");
        //let image = get_assembly_image("UnityEngine.CoreModule");
        log!("img ptr: {:?}", image);
        //try_class(image, "SingleModeMainHeaderAndFooterController");
        try_class(image, "SingleModeMainViewHpGauge");
        (vtable.gui_register_menu_item)(title.as_ptr(), Some(on_menu_click), std::ptr::null_mut());
        (vtable.gui_register_menu_section)(Some(on_menu_section), std::ptr::null_mut());
    });

    InitResult::Ok
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        debug_log_internal(&format!($($arg)*));
    };
}

pub fn debug_log_internal(msg: &str) {
    let _guard = LOG_MUTEX.lock().ok();

    let plugin_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let log_dir = plugin_dir.join("hachimi");
    let _ = create_dir_all(&log_dir);
    let log_path = log_dir.join("energy_spy.log");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(file, "{}", msg);
    }
}
