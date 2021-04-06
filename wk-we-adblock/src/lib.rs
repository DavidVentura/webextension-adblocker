use once_cell::sync::OnceCell;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::os::raw::c_char;
use trie_rs::{Trie, TrieBuilder};

static BAD_DOMAINS: OnceCell<Trie<u8>> = OnceCell::new();
const ASCII_SLASH: u8 = 47;
const AD_LIST: &str = "/home/david/git/py-gtk-hn/serverlist.txt";

#[no_mangle]
pub extern "C" fn init_ad_list() {
    let mut data = TrieBuilder::new();

    println!("Doing INIT on ad list");
    let file = File::open(AD_LIST);
    if file.is_err() {
        return;
    }
    let lines = io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(host) = line {
            let mut s = host.into_bytes();
            s.reverse();
            data.push(s)
        }
    }
    let trie = data.build();
    let _res = BAD_DOMAINS.set(trie);
}

/// Given an uri like http://example.com/something, this will return Some(example.com)
/// If it can't find two slashes (//) within the first 7 chars (length of `https://`)
/// it will return None
fn get_domain<'a>(uri: &'a [u8]) -> Option<&'a [u8]> {
    let mut cnt = 0;
    let mut slash_cnt = 0;
    let mut start_pointer = 0;
    let end_pointer;

    for elem in uri.iter() {
        if *elem == ASCII_SLASH {
            slash_cnt += 1;
            if slash_cnt == 2 {
                start_pointer = cnt;
            }
            if slash_cnt == 3 {
                end_pointer = cnt;
                return Some(&uri[start_pointer + 1..end_pointer]);
            }
        }
        cnt += 1;
        if cnt > 7 && start_pointer == 0 {
            // if we haven't found 2 / in 8 chars, no point to keep going
            // as we only care for the `http://` and `https://` prefixes
            return None;
        }
    }
    return None;
}
fn is_domain_blocked(domain: &[u8]) -> bool {
    let rd: Vec<u8> = domain.iter().rev().copied().collect();

    let trie = BAD_DOMAINS.get().unwrap();
    trie.common_prefix_match(rd)
}

#[no_mangle]
pub extern "C" fn is_ad(page_uri: *const c_char) -> bool {
    let uri_str = unsafe { CStr::from_ptr(page_uri) };
    let uri_bytes = uri_str.to_bytes();
    let domain = get_domain(uri_bytes);
    if domain.is_none() {
        return false;
    }
    let domain = domain.unwrap();
    let matched = is_domain_blocked(domain);
    if matched {
        println!("BLOCKED uri {:?}", uri_str);
    }
    matched
}
