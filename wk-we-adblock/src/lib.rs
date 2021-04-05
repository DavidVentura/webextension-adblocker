use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn is_ad(page_uri: *const c_char) -> bool {
    let cstr = unsafe { CStr::from_ptr(page_uri) };
    let slice = cstr.to_str().unwrap();
    println!("Got a page uri in rust: {}!", slice);

    slice.contains(".js")
}
