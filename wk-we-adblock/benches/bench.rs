use aho_corasick::{AhoCorasickBuilder, MatchKind};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use trie_rs::{Trie, TrieBuilder};
use twoway;

fn _git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

fn get_b_lines<'a>(fname: &str) -> Vec<Vec<u8>> {
    let lines = get_lines(fname);
    lines.iter().map(|x| x.as_bytes().to_vec()).collect()
}
fn get_lines(fname: &str) -> Vec<String> {
    let repo_root = env::var("REPO_ROOT").expect("REPO_ROOT environment variable must be set.");
    let ad_list = format!("{}/benches/{}", repo_root, fname);
    println!("Reading dictionary file from: {}", ad_list);

    let file = File::open(ad_list);
    let lines: Result<Vec<String>, _> = BufReader::new(file.unwrap()).lines().collect();
    lines.unwrap()
}

fn str_trie() -> Trie<Vec<u8>> {
    let lines = get_lines("hosts.txt");
    let mut builder = TrieBuilder::new();
    for mline in lines {
        let parts = mline.split(".").map(|s| s.to_string());
        let mut revparts = Vec::new();
        for part in parts {
            revparts.insert(0, part.into_bytes());
        }
        builder.push(revparts);
    }
    builder.build()
}
fn bytes_trie() -> Trie<u8> {
    let lines = get_lines("hosts.txt");
    let mut builder = TrieBuilder::new();
    for line in &lines {
        let mut s = line.clone().into_bytes();
        s.reverse();
        builder.push(s);
    }
    println!("Read {} lines", lines.len());

    builder.build()
}

fn bench_hosts_trie_corasick(c: &mut Criterion) {
    // vec of bytes
    let trie_u8 = bytes_trie();
    let sentinel_u8: &[u8] = "adjuggler.yourdictionary.com".as_bytes();
    let sentinel_u8_rev: &[u8] = "moc.yranoitcidruoy.relggujda".as_bytes();
    let res = trie_u8.common_prefix_match(&sentinel_u8_rev);
    assert_eq!(res, true);

    // vec of str
    let trie = str_trie();
    let sentinel_str = vec![
        "com".to_string().into_bytes(),
        "yourdictionary".to_string().into_bytes(),
        "adjuggler".to_string().into_bytes(),
    ];

    let res = trie.common_prefix_match(&sentinel_str);
    assert_eq!(res, true);
    let b_lines = get_b_lines("hosts.txt");
    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(&b_lines);

    let mut group = c.benchmark_group("Trie u8 vs Trie str vs aho-corasick");
    group.warm_up_time(Duration::from_secs(5));
    group.bench_function(BenchmarkId::new("Bytes", 1), |b| {
        b.iter(|| trie_u8.common_prefix_match(&sentinel_u8_rev))
    });
    group.bench_function(BenchmarkId::new("String", 1), |b| {
        b.iter(|| trie.common_prefix_match(&sentinel_str))
    });
    group.bench_function(BenchmarkId::new("aho-corasick", 1), |b| {
        b.iter(|| {
            let a = ac.find(sentinel_u8);
            let found = a.is_some();
            assert!(found);
            return found;
        })
    });
    group.finish();
}

fn window_match(lines: &Vec<Vec<u8>>, validate: &[u8]) -> bool {
    for bad_substr in lines {
        let blen = bad_substr.len();
        if blen > validate.len() {
            continue;
        }
        let windows = validate.windows(blen);
        for part_to_validate in windows {
            // println!("{:?} vs {:?}", part_to_validate, bad_substr);
            if part_to_validate == &bad_substr[..] {
                return true;
            }
        }
    }
    return false;
}

fn bench_fragment_substr_corasick_twoway(c: &mut Criterion) {
    let lines = get_lines("url_fragments.txt");
    let b_lines = get_b_lines("url_fragments.txt");
    let sentinel_url = "https://ultra-dody.ru/xmr-monero.js";
    let b_sentinel_url = "https://ultra-dody.ru/xmr-monero.js".as_bytes();
    //let sentinel_url = "/xmr-monero.js";
    //let b_sentinel_url = "/xmr-monero.js".as_bytes();
    //let sentinel_url = "/somegarbage/iicons?selected=Material+Icons&icon.query=back";
    //let b_sentinel_url = "/somegarbage/iicons?selected=Material+Icons&icon.query=back".as_bytes();

    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(&b_lines);

    let mut group = c.benchmark_group("fragment comparison");
    group.warm_up_time(Duration::from_secs(5));

    assert!(window_match(
        &vec!["/xmr-monero.js".as_bytes().to_vec()],
        "https://ultra-dody.ru/xmr-monero.js".as_bytes()
    ));

    group.bench_with_input(BenchmarkId::new("Substr bytes", 1), &b_lines, |b, lines| {
        b.iter(|| window_match(lines, b_sentinel_url))
    });
    group.bench_function(BenchmarkId::new("Substr", 1), |b| {
        b.iter(|| {
            for bad_substr in &lines {
                if bad_substr.len() > sentinel_url.len() {
                    continue;
                }
                if sentinel_url.contains(bad_substr) {
                    return true;
                }
            }
            return false;
        })
    });
    group.bench_with_input(BenchmarkId::new("twoway bytes", 1), &b_lines, |b, lines| {
        b.iter(|| {
            for bad_substr in lines {
                if bad_substr.len() > b_sentinel_url.len() {
                    continue;
                }
                if let Some(_) = twoway::find_bytes(b_sentinel_url, bad_substr) {
                    return true;
                }
            }
            return false;
        })
    });
    group.bench_function(BenchmarkId::new("twoway u8", 1), |b| {
        b.iter(|| {
            for bad_substr in &lines {
                if bad_substr.len() > sentinel_url.len() {
                    continue;
                }
                if let Some(_) = twoway::find_str(sentinel_url, bad_substr) {
                    return true;
                }
            }
            return false;
        })
    });
    group.bench_function(BenchmarkId::new("aho-corasick", 1), |b| {
        b.iter(|| {
            let a = ac.find(sentinel_url);
            let found = a.is_some();
            assert!(found);
            return found;
        })
    });
    group.finish();
}
criterion_group!(
    benches,
    bench_fragment_substr_corasick_twoway,
    bench_hosts_trie_corasick
);
criterion_main!(benches);
