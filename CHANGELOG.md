# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

Patch releases driven by upstream data updates are tagged automatically by
CI (see `.github/workflows/update-domains.yml` and `release.yml`) and are
not individually itemized here in detail — see each release's GitHub
Release page (auto-generated notes) and `data/SOURCE_COMMIT` for the exact
upstream commit a given version was built from.

## [Unreleased]

## [0.1.0] - 2026-07-15

### Added

- Initial release.
- `is_disposable_email` / `is_disposable_domain` with compile-time
  perfect-hash-set lookups over the embedded, daily-synced upstream list.
- `Blocklist` for runtime-customizable/merged lists (`std` feature, on by
  default).
- `no_std` support (without the `std` feature).
