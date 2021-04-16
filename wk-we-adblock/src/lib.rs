#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod wk_adblock;

use gobject_sys::{g_signal_connect_data, GObject};
use std::ffi::{c_void, CStr};
use std::mem;

#[no_mangle]
extern "C" fn webkit_web_extension_initialize(extension: *mut WebKitWebExtension) {
    println!("Hey from rust!");
    unsafe {
        g_signal_connect(
            extension as *mut c_void,
            CStr::from_bytes_with_nul_unchecked(b"page-created\0").as_ptr(),
            Some(mem::transmute(web_page_created_callback as *const ())),
            0 as *mut c_void,
        );
    };
    wk_adblock::init_ad_list();
}

#[no_mangle]
extern "C" fn web_page_created_callback(
    _extension: *const WebKitWebExtension,
    web_page: *mut WebKitWebPage,
    _user_data: *const gpointer,
) {
    println!("From webpage created cb");
    unsafe {
        g_signal_connect_object(
            web_page as *mut c_void,
            CStr::from_bytes_with_nul_unchecked(b"send-request\0").as_ptr(),
            Some(mem::transmute(web_page_send_request as *const ())),
            0 as *mut c_void, // NULL
            0,
        );
    }
}

#[no_mangle]
extern "C" fn web_page_send_request(
    _web_page: *mut WebKitWebPage,
    request: *mut WebKitURIRequest,
    _redirected_response: *mut WebKitURIResponse,
    _user_data: *mut gpointer,
) -> bool {
    let page_uri = unsafe { webkit_uri_request_get_uri(request) };
    // This could do 3rd party vs 1st party and URL matching
    /*
    let request_uri = unsafe { CStr::from_ptr(webkit_uri_request_get_uri(request)) };
    let page_uri = unsafe { CStr::from_ptr(webkit_web_page_get_uri(web_page)) };
    */

    let uri_str = unsafe { CStr::from_ptr(page_uri) };
    let uri_bytes = uri_str.to_bytes();
    wk_adblock::is_ad(uri_bytes)
}

unsafe fn g_signal_connect(
    instance: gpointer,
    detailed_signal: *const gchar,
    c_handler: GCallback,
    data: gpointer,
) -> gulong {
    g_signal_connect_data(
        instance as *mut GObject,
        detailed_signal,
        c_handler,
        data,
        None,
        std::mem::transmute(0),
    )
}
