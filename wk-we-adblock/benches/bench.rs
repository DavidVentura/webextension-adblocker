use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn _git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use trie_rs::{Trie, TrieBuilder};
use wk_we_adblock::wk_adblock::is_domain_blocked;

fn get_lines() -> Vec<String> {
    let repo_root = env::var("REPO_ROOT").expect("REPO_ROOT environment variable must be set.");
    let ad_list = format!("{}/benches/hosts.txt", repo_root);
    println!("Reading dictionary file from: {}", ad_list);

    let file = File::open(ad_list);
    let lines: Result<Vec<String>, _> = BufReader::new(file.unwrap()).lines().collect();
    lines.unwrap()
}

fn str_trie() -> Trie<Vec<u8>> {
    let lines = get_lines();
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
    let lines = get_lines();
    let mut builder = TrieBuilder::new();
    for line in &lines {
        let mut s = line.clone().into_bytes();
        s.reverse();
        builder.push(s);
    }
    println!("Read {} lines", lines.len());

    builder.build()
}

pub fn bench_str_vec(c: &mut Criterion) {
    let trie = str_trie();

    let sentinel = vec![
        "com".to_string().into_bytes(),
        "yourdictionary".to_string().into_bytes(),
        "adjuggler".to_string().into_bytes(),
    ];

    let res = trie.common_prefix_match(&sentinel);
    assert_eq!(res, true);
    c.bench_function("vec of vecs match", |b| {
        b.iter(|| trie.common_prefix_match(&sentinel));
    });
}

pub fn bench_u8_vec(c: &mut Criterion) {
    let trie_u8 = bytes_trie();
    //let sentinel: &[u8] = "adjuggler.yourdictionary.com".as_bytes();
    let sentinel: &[u8] = "moc.yranoitcidruoy.relggujda".as_bytes();
    let res = trie_u8.common_prefix_match(&sentinel);
    assert_eq!(res, true);

    c.bench_function("direct match", |b| {
        b.iter(|| trie_u8.common_prefix_match(&sentinel));
    });
}

pub fn bench_u8_vec_vs_str(c: &mut Criterion) {
    // vec of bytes
    let trie_u8 = bytes_trie();
    let sentinel_u8: &[u8] = "moc.yranoitcidruoy.relggujda".as_bytes();
    let res = trie_u8.common_prefix_match(&sentinel_u8);
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

    let mut group = c.benchmark_group("bytes vs str");
    group.bench_function(BenchmarkId::new("Bytes", 1), |b| {
        b.iter(|| trie_u8.common_prefix_match(&sentinel_u8))
    });
    group.bench_function(BenchmarkId::new("String", 1), |b| {
        b.iter(|| trie.common_prefix_match(&sentinel_str))
    });
    group.finish();
}
criterion_group!(benches, bench_str_vec, bench_u8_vec, bench_u8_vec_vs_str);
criterion_main!(benches);
