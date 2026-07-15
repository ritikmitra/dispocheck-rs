# Contributing

Thanks for considering a contribution!

## Adding or removing a disposable domain

Please don't edit `data/disposable_email_blocklist.conf` directly — it is
vendored automatically from the upstream
[disposable-email-domains/disposable-email-domains](https://github.com/disposable-email-domains/disposable-email-domains)
repository (see `.github/workflows/update-domains.yml`) and any manual
edits will be overwritten on the next sync.

To add, remove, or dispute a domain, please open a PR against the
**upstream** repository instead. Once merged there, it will flow into this
crate automatically (typically within a day).

## Code changes

1. Fork and clone the repo.
2. Make your change.
3. Run the full check suite locally before opening a PR:

   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   cargo test --no-default-features
   ```

4. If you touched public API, update the doc comments — doctests run as
   part of `cargo test` and are checked in CI.
5. Open a PR. CI (tests, clippy, fmt, cargo-deny, cargo-audit) must pass
   before merge.

## Reporting a bug

Please include:

- The crate version and Rust version (`rustc --version`).
- A minimal reproduction (a failing `is_disposable_email`/`is_disposable_domain`
  call and what you expected vs. got).

## Security issues

Please see [SECURITY.md](SECURITY.md) instead of filing a public issue.
