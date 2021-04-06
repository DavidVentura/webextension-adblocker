use once_cell::sync::OnceCell;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::os::raw::c_char;
use std::time::Instant;

static BAD_DOMAINS: OnceCell<Vec<String>> = OnceCell::new();

#[no_mangle]
pub extern "C" fn init_ad_list() {
    println!("Doing INIT on ad list");
    let mut data: Vec<String> = Vec::new();
    let filename = "/home/david/git/py-gtk-hn/serverlist.txt";
    let file = File::open(filename);
    if file.is_err() {
        return;
    }
    let lines = io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(host) = line {
            data.push(host)
        }
    }
    BAD_DOMAINS.set(data).unwrap();
}
#[no_mangle]
pub extern "C" fn is_ad(page_uri: *const c_char) -> bool {
    let now = Instant::now();
    let cstr = unsafe { CStr::from_ptr(page_uri) };
    let slice = cstr.to_str().unwrap();
    let slice = slice
        .strip_prefix("http://")
        .or_else(|| slice.strip_prefix("https://"));
    if slice.is_none() {
        return false;
    }
    let slice = slice.unwrap();
    let chunks: Vec<&str> = slice.splitn(2, "/").collect();
    let domain = chunks.first().unwrap();

    // this disgusting impl takes max 759.181µs
    let mut matched = false;
    let bbd = BAD_DOMAINS.get().unwrap();
    for dom in bbd {
        if domain.ends_with(dom) {
            matched = true;
            break;
        }
    }
    if matched {
        println!(
            "Got a BLOCKED page uri in rust: {}, domain is {:?}!",
            slice, domain
        );
    }
    let done = Instant::now();
    println!("Took {:?}", done - now);
    matched
}
