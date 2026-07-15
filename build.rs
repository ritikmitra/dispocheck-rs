//! Build script: turns `data/disposable_email_blocklist.conf` into a
//! compile-time perfect hash set (`phf::Set<&'static str>`) so that lookups
//! at runtime are O(1) with no heap allocation and no parsing cost.
//!
//! If the data file is malformed (blank required, duplicate handling, etc.)
//! this script fails the build loudly rather than silently shipping a bad
//! or empty list.

use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

const DATA_FILE: &str = "data/disposable_email_blocklist.conf";

fn main() {
    println!("cargo:rerun-if-changed={DATA_FILE}");

    let contents =
        fs::read_to_string(DATA_FILE).unwrap_or_else(|e| panic!("failed to read {DATA_FILE}: {e}"));

    let mut domains: HashSet<String> = HashSet::new();
    for (line_no, raw_line) in contents.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let domain = line.to_lowercase();
        if !is_plausible_domain(&domain) {
            panic!(
                "invalid domain entry at {DATA_FILE}:{}: {:?}",
                line_no + 1,
                raw_line
            );
        }
        domains.insert(domain);
    }

    if domains.len() < 1000 {
        // Sanity floor: the upstream list has thousands of entries. If a bad
        // fetch ever truncated the file, fail the build instead of shipping
        // a near-empty blocklist.
        panic!(
            "refusing to build: only {} domains parsed from {DATA_FILE}, expected at least 1000",
            domains.len()
        );
    }

    let mut sorted: Vec<&str> = domains.iter().map(String::as_str).collect();
    sorted.sort_unstable();

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("blocklist_data.rs");
    let mut writer = BufWriter::new(fs::File::create(&dest_path).unwrap());

    // Not part of the public API surface (see `len()` / `is_empty()` in
    // lib.rs), so these stay `pub(crate)` and are exempt from `#[deny(missing_docs)]`.
    writeln!(
        writer,
        "pub(crate) const DOMAIN_COUNT: usize = {};",
        sorted.len()
    )
    .unwrap();
    writeln!(writer).unwrap();

    let mut set_builder: phf_codegen::Set<&str> = phf_codegen::Set::new();
    for domain in &sorted {
        set_builder.entry(*domain);
    }

    writeln!(
        writer,
        "pub(crate) static BLOCKLIST: phf::Set<&'static str> = {};",
        set_builder.build()
    )
    .unwrap();
}

/// Cheap structural sanity check for a blocklist entry: lowercase-able
/// ASCII/host characters, at least one dot, no `@`, no whitespace.
fn is_plausible_domain(domain: &str) -> bool {
    if domain.contains('@') || domain.contains(char::is_whitespace) {
        return false;
    }
    if !domain.contains('.') {
        return false;
    }
    domain
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}
