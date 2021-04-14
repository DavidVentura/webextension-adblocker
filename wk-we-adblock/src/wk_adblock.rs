use once_cell::sync::OnceCell;
use std::env;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::time::Instant;
use trie_rs::{Trie, TrieBuilder};

static BAD_DOMAINS: OnceCell<Trie<u8>> = OnceCell::new();
const ASCII_SLASH: u8 = 47;

pub fn hosts_from_file() -> Vec<Vec<u8>> {
    let mut hosts: Vec<Vec<u8>> = Vec::new();
    let _home = env::var("HOME");
    if _home.is_err() {
        println!("$HOME is not set, not searching for adblock file");
        return hosts;
    }

    let home = PathBuf::from(_home.unwrap());
    let ad_list = home.join(".config/wk_adblock/hosts.txt");
    if !ad_list.exists() {
        println!("Did not find {:?}, not enabling ad block", ad_list);
        return hosts;
    }

    let file = File::open(ad_list);
    if file.is_err() {
        println!("Error opening adblock file");
        return hosts;
    }
    let lines = io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(host) = line {
            let mut s = host.into_bytes();
            s.reverse();
            hosts.push(s);
        }
    }
    return hosts;
}
pub fn init_ad_list() {
    if let Some(_) = BAD_DOMAINS.get() {
        println!("Tried to re-init?");
        return;
    }
    let start = Instant::now();
    let mut data = TrieBuilder::new();
    let hosts = hosts_from_file();
    hosts.iter().for_each(|x| data.push(x));
    let trie = data.build();
    let _res = BAD_DOMAINS.set(trie);
    let finish = Instant::now();
    println!(
        "Blocking {} hosts, startup took {:?}",
        hosts.len(),
        finish - start
    );
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
pub fn is_domain_blocked<T: Ord + Copy>(domain: &[T], trie: &Trie<T>) -> bool {
    let rd: Vec<T> = domain.iter().rev().copied().collect();
    trie.common_prefix_match(rd)
}

pub fn is_ad(page_uri: *const c_char) -> bool {
    let uri_str = unsafe { CStr::from_ptr(page_uri) };
    let uri_bytes = uri_str.to_bytes();
    let domain = get_domain(uri_bytes);
    if domain.is_none() {
        return false;
    }
    let domain = domain.unwrap();
    let _start = Instant::now();

    let trie = BAD_DOMAINS.get().unwrap();
    let matched = is_domain_blocked(domain, trie);
    if matched {
        println!("BLOCKED uri {:?}", uri_str);
    }
    let _done = Instant::now();
    // println!("{:?}", done - start);
    matched
}
