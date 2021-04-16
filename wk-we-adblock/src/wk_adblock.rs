use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use once_cell::sync::OnceCell;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::Instant;

static BAD_DOMAINS: OnceCell<AhoCorasick> = OnceCell::new();
static BAD_FRAGMENTS: OnceCell<AhoCorasick> = OnceCell::new();

pub fn parse_file(fname: &str) -> Vec<Vec<u8>> {
    let mut hosts: Vec<Vec<u8>> = Vec::new();
    let _home = env::var("HOME");
    if _home.is_err() {
        println!("$HOME is not set, not searching for adblock file");
        return hosts;
    }

    let home = PathBuf::from(_home.unwrap());
    let ad_list = home.join(fname);
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
            //let mut s = host.into_bytes();
            //s.reverse();
            hosts.push(host.into_bytes());
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
    let hosts = parse_file(".config/wk_adblock/hosts.txt");

    let host_len = hosts.len();
    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(hosts);
    let _res = BAD_DOMAINS.set(ac);

    let fragments = parse_file(".config/wk_adblock/fragments.txt");
    let frag_len = fragments.len();
    let fragment_matcher = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(fragments);
    BAD_FRAGMENTS.set(fragment_matcher).unwrap();

    let finish = Instant::now();
    println!(
        "Blocking {} hosts and {} fragments, startup took {:?}",
        host_len,
        frag_len,
        finish - start
    );
}

pub fn is_ad(page_uri: &[u8]) -> bool {
    let start = Instant::now();
    let domain_matcher = BAD_DOMAINS.get().unwrap();
    let matched;
    if domain_matcher.earliest_find(page_uri).is_some() {
        matched = true;
    } else {
        let fragment_matcher = BAD_FRAGMENTS.get().unwrap();
        let frag = fragment_matcher.earliest_find(page_uri);
        if frag.is_some() {
            //println!("fragment! {:?}", frag);
            matched = true;
        } else {
            matched = false;
        }
    }
    let finish = Instant::now();
    if matched {
        println!("BLOCKED uri {:?}", String::from_utf8_lossy(page_uri));
        println!("In {:?}", finish - start);
    }
    matched
}
