//! # disposable-email-domains
//!
//! Fast, dependency-light checking of disposable / temporary email domains,
//! built from the community-maintained
//! [`disposable-email-domains/disposable-email-domains`](https://github.com/disposable-email-domains/disposable-email-domains)
//! dataset (CC0-licensed, ~8,000 domains and growing).
//!
//! The domain list is embedded at **compile time** as a perfect hash set
//! ([`phf`](https://docs.rs/phf)), so lookups are `O(1)`, allocation-light,
//! and require no filesystem or network access at runtime.
//!
//! ```
//! use dispocheck::is_disposable_email;
//!
//! assert!(is_disposable_email("someone@mailinator.com"));
//! assert!(!is_disposable_email("someone@gmail.com"));
//! ```
//!
//! ## Subdomain matching
//!
//! The upstream list stores second-level (or, for public-suffix domains,
//! third-level) domains. A blocked entry also blocks any subdomain of it,
//! e.g. if `mailinator.com` is listed, `foo.mailinator.com` is also treated
//! as disposable:
//!
//! ```
//! use dispocheck::is_disposable_domain;
//!
//! assert!(is_disposable_domain("sub.mailinator.com"));
//! ```
//!
//! ## Custom / runtime lists
//!
//! If you need to merge the embedded list with your own domains, or refresh
//! the list at runtime without recompiling, use `Blocklist` (requires the
//! default `std` feature).
//!
//! ## Freshness
//!
//! The upstream dataset changes daily. This crate's embedded list is
//! refreshed and republished automatically by CI — see the crate's
//! repository for the update cadence and the exact upstream commit each
//! release was built from (recorded in `data/SOURCE_COMMIT`).
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![warn(clippy::all)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
extern crate std;

include!(concat!(env!("OUT_DIR"), "/blocklist_data.rs"));

/// Returns `true` if `email`'s domain (or one of its parent domains) is a
/// known disposable/temporary email domain.
///
/// Matching is case-insensitive. Malformed input (no `@`, empty domain)
/// returns `false` rather than erroring, since callers typically validate
/// email *syntax* separately (e.g. with a dedicated email-parsing crate)
/// before checking disposability.
///
/// # Examples
///
/// ```
/// use dispocheck::is_disposable_email;
///
/// assert!(is_disposable_email("test@mailinator.com"));
/// assert!(is_disposable_email("Test@Mailinator.COM")); // case-insensitive
/// assert!(!is_disposable_email("test@company.com"));
/// assert!(!is_disposable_email("not-an-email"));
/// ```
pub fn is_disposable_email(email: &str) -> bool {
    match email.rsplit_once('@') {
        Some((_, domain)) if !domain.is_empty() => is_disposable_domain(domain),
        _ => false,
    }
}

/// Returns `true` if `domain`, or any of its parent domains, is a known
/// disposable/temporary email domain.
///
/// For example, if `mailinator.com` is in the blocklist, both
/// `mailinator.com` and `anything.mailinator.com` return `true`.
///
/// Matching is case-insensitive and tolerant of a single trailing dot
/// (as in a fully-qualified domain name).
///
/// # Examples
///
/// ```
/// use dispocheck::is_disposable_domain;
///
/// assert!(is_disposable_domain("mailinator.com"));
/// assert!(is_disposable_domain("deeply.nested.mailinator.com"));
/// assert!(!is_disposable_domain("example.com"));
/// ```
pub fn is_disposable_domain(domain: &str) -> bool {
    let normalized = normalize(domain);
    if normalized.is_empty() {
        return false;
    }
    domain_suffixes(&normalized).any(|suffix| BLOCKLIST.contains(suffix))
}

/// Returns the total number of domains in the embedded blocklist.
///
/// # Examples
///
/// ```
/// assert!(dispocheck::len() > 1000);
/// ```
pub fn len() -> usize {
    DOMAIN_COUNT
}

/// Returns `true` if the embedded blocklist is empty. Always `false` in
/// practice; the build script refuses to compile an implausibly small list.
pub fn is_empty() -> bool {
    DOMAIN_COUNT == 0
}

/// Returns an iterator over every domain in the embedded blocklist.
///
/// Iteration order is unspecified.
pub fn iter() -> impl Iterator<Item = &'static &'static str> {
    BLOCKLIST.into_iter()
}

/// Lowercases and trims a domain, stripping a single trailing dot (FQDN
/// notation).
fn normalize(domain: &str) -> String {
    let trimmed = domain.trim().trim_end_matches('.');
    trimmed.to_lowercase()
}

/// Iterator over `domain` and each of its parent domains, most-specific
/// first, stopping before the bare top-level domain — matching the
/// suffix-walk algorithm documented by the upstream dataset.
fn domain_suffixes(domain: &str) -> impl Iterator<Item = &str> {
    let parts: Vec<&str> = domain.split('.').collect();
    let len = parts.len();
    (0..len.saturating_sub(1)).map(move |i| {
        let start: usize = parts[..i].iter().map(|p| p.len() + 1).sum();
        &domain[start..]
    })
}

#[cfg(feature = "std")]
mod runtime;
#[cfg(feature = "std")]
pub use runtime::Blocklist;
