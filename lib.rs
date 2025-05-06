#[rustfmt::skip]
#[cfg(not(feature = "binary"))]
mod code;
#[cfg(not(feature = "binary"))]
pub(crate) mod workbench;

#[cfg(feature = "binary")]
use crate::code;
#[cfg(feature = "binary")]
use crate::workbench;

use std::ffi::{c_char, CStr};

use glib::translate::FromGlibPtrFull;
use gtk::glib;
use libc::{c_int, EXIT_FAILURE, EXIT_SUCCESS};

pub(crate) static mut BUILDER: Option<gtk::Builder> = None;
pub(crate) static mut WINDOW: Option<adw::Window> = None;
pub(crate) static mut URI: Option<String> = None;

#[no_mangle]
extern "C" fn _main() -> c_int {
    code::main();
    EXIT_SUCCESS
}

#[no_mangle]
pub(crate) extern "C" fn set_builder(builder_ptr: *mut gtk::ffi::GtkBuilder) -> c_int {
    unsafe {
        let builder = gtk::Builder::from_glib_full(builder_ptr);
        BUILDER = Some(builder.clone());
    }
    EXIT_SUCCESS
}

#[no_mangle]
pub(crate) extern "C" fn set_window(window_ptr: *mut adw::ffi::AdwWindow) -> c_int {
    unsafe {
        let window = adw::Window::from_glib_full(window_ptr);
        WINDOW = Some(window.clone());
    }
    EXIT_SUCCESS
}

#[no_mangle]
pub(crate) extern "C" fn set_base_uri(c_string: *const c_char) -> c_int {
    unsafe {
        if c_string.is_null() {
            return EXIT_FAILURE;
        }

        let c_str = CStr::from_ptr(c_string);
        if let Ok(str_slice) = c_str.to_str() {
            URI = Some(str_slice.to_string());
        }
    }
    EXIT_SUCCESS
}
