# Dispocheck

[![Crates.io](https://img.shields.io/crates/v/dispocheck.svg)](https://crates.io/crates/dispocheck)
[![Docs.rs](https://img.shields.io/docsrs/dispocheck)](https://docs.rs/dispocheck)
[![CI](https://github.com/ritikmitra/dispocheck-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ritikmitra/dispocheck-rs/actions/workflows/ci.yml)
[![Domain list updated](https://github.com/ritikmitra/dispocheck-rs/actions/workflows/update-domains.yml/badge.svg)](https://github.com/ritikmitra/dispocheck-rs/actions/workflows/update-domains.yml)
[![codecov](https://codecov.io/gh/ritikmitra/dispocheck-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ritikmitra/dispocheck-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.74-informational)](Cargo.toml)

Dispocheck is a fast, production-ready Rust library for detecting disposable and temporary email domains. It embeds the community-maintained [`disposable-email-domains/disposable-email-domains`](https://github.com/disposable-email-domains/disposable-email-domains) dataset directly into your binary using compile-time generated perfect hash tables (phf), providing constant-time lookups with zero runtime parsing. dispocheck provides an idiomatic, production-ready Rust implementation backed by the upstream dataset.


```rust
use dispocheck::is_disposable_email;

assert!(is_disposable_email("someone@mailinator.com"));
assert!(!is_disposable_email("someone@gmail.com"));
```

## Why this crate

- **Zero runtime cost to load the list.** The ~8,000-domain dataset is
  compiled into a [perfect hash set](https://docs.rs/phf) at build time via
  `build.rs`. There's no file I/O, no parsing, and no heap allocation for a
  lookup — it's an `O(1)` hash lookup against static data baked into your
  binary.
- **Correct subdomain matching.** `mailinator.com` in the list also blocks
  `anything.mailinator.com`, matching the algorithm the upstream project
  documents (and that every other language binding implements).
- **Stays current.** The upstream list changes roughly daily. This crate's
  vendored copy is refreshed and republished automatically — see
  [Freshness](#freshness) below.
- **Minimal dependencies.** Only `phf` at runtime (no regex, no HTTP client,
  no async runtime pulled in for a domain lookup).
- **`no_std` compatible** (without the default `std` feature) for embedded
  or constrained environments; the compile-time list still works fully.
- **Escape hatch for custom lists.** [`Blocklist`](https://docs.rs/disposable-email-domains/latest/disposable_email_domains/struct.Blocklist.html)
  lets you merge in your own domains or swap in a freshly-fetched list at
  runtime, without recompiling.

## Installation

```bash
cargo add dispocheck
```

or add manually to `Cargo.toml`:

```toml
[dependencies]
dispocheck = "0.1"
```

## Usage

### Basic check

```rust
use dispocheck::is_disposable_email;

fn validate_signup_email(email: &str) -> Result<(), &'static str> {
    if is_disposable_email(email) {
        return Err("Please use a permanent email address.");
    }
    Ok(())
}
```

### Checking a domain directly

```rust
use dispocheck::is_disposable_domain;

assert!(is_disposable_domain("mailinator.com"));
assert!(is_disposable_domain("sub.mailinator.com")); // subdomains match too
assert!(!is_disposable_domain("example.com"));
```

### Custom or runtime-extended lists

```rust
use dispocheck::Blocklist;

let mut list = Blocklist::from_embedded();
list.add_domain("my-internal-throwaway.example");
list.remove_domain("mailinator.com"); // e.g. an internal allowlist exception

assert!(list.is_disposable_domain("my-internal-throwaway.example"));
assert!(!list.is_disposable_domain("mailinator.com"));
```

See [`examples/signup_validation.rs`](examples/signup_validation.rs) for a
complete runnable example (`cargo run --example signup_validation`).

### `no_std`

```toml
[dependencies]
dispocheck = { version = "0.1", default-features = false }
```

`is_disposable_email` and `is_disposable_domain` work identically; only the
`Blocklist` runtime-customization type requires `std` (it uses `HashSet`).

## Freshness

The upstream dataset is community-maintained and changes frequently (often daily). A
[scheduled workflow](.github/workflows/update-domains.yml) in this
repository checks upstream every day, and when the list has actually
changed:

1. It vendors the new `disposable_email_blocklist.conf` into `data/`.
2. Bumps the crate's patch version.
3. Opens a pull request with the diff, which must pass the full CI suite
   (tests, clippy, `cargo-deny`, `cargo-audit`) before merge.

Merging that PR triggers the [release workflow](.github/workflows/release.yml),
which tags the commit and publishes the new version to crates.io
automatically. The exact upstream commit any given release was built from
is always recorded in [`data/SOURCE_COMMIT`](data/SOURCE_COMMIT).

If you need guaranteed same-day freshness rather than waiting for a release,
use [`Blocklist::from_domains`](https://docs.rs/disposable-email-domains/latest/disposable_email_domains/struct.Blocklist.html#method.from_domains)
to load a list you fetch yourself at startup.

## How matching works

Mirroring the [algorithm documented upstream](https://github.com/disposable-email-domains/disposable-email-domains#example-usage):
given an email's domain, every progressively-shorter parent domain is
checked against the list (skipping the bare top-level domain), and the
check is case-insensitive:

```text
foo.bar.mailinator.com
  -> check "foo.bar.mailinator.com"
  -> check "bar.mailinator.com"
  -> check "mailinator.com"   <- match
  (never checks "com" alone)
```

## Performance

Lookups are a single `O(1)` hash-set probe per domain level (typically 1–3
probes for realistic domains) against data embedded in the binary — no
allocation on the hot path for `is_disposable_domain`/`is_disposable_email`.
See [`benches/lookup.rs`](benches/lookup.rs) (`cargo bench`) for
[Criterion](https://github.com/bheisler/criterion.rs)-based benchmarks.

## Project layout

```
.
├── src/
│   ├── lib.rs        # Public API: is_disposable_email, is_disposable_domain, ...
│   └── runtime.rs     # Blocklist: custom/runtime lists (std feature)
├── build.rs            # Compiles data/*.conf into a phf::Set at build time
├── data/
│   ├── disposable_email_blocklist.conf  # Vendored from upstream (do not hand-edit)
│   └── SOURCE_COMMIT                    # Upstream commit this data came from
├── tests/               # Integration tests
├── benches/             # Criterion benchmarks
├── examples/            # Runnable usage examples
└── scripts/             # update_domains.sh, bump_patch_version.py (used by CI)
```

## Comparison with other approaches

| Approach | This crate | Bundling the `.conf` file yourself + `HashSet` at runtime |
|---|---|---|
| Lookup cost | `O(1)`, compile-time hash set, no allocation | `O(1)`, but pay parse + hash-set build cost at startup |
| Binary size | List baked in (~150 KB source data) | Same, plus you write the loader |
| Freshness | Automated daily upstream sync + release | Manual |
| `no_std` | Supported | Typically not (needs `std::fs`) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Note: domain additions/removals
should go to the [upstream repository](https://github.com/disposable-email-domains/disposable-email-domains),
not this one — see that file for why.

## Security

See [SECURITY.md](SECURITY.md) for how to report vulnerabilities.

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option, matching the conventional dual-licensing used across the
Rust ecosystem.

The underlying domain data (`data/disposable_email_blocklist.conf`) is
vendored from the upstream project, which dedicates it to the public domain
under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/). See
[NOTICE](NOTICE) for details.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you shall be dual-licensed as
above, without any additional terms or conditions.
