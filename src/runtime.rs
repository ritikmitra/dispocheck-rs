//! Runtime, mutable blocklist support for callers who want to merge the
//! embedded dataset with their own domains, or swap in a freshly-fetched
//! list without recompiling.

use crate::{domain_suffixes, normalize};
use std::collections::HashSet;

/// A mutable, in-memory blocklist that starts from the crate's embedded
/// dataset and can be extended (or fully replaced) at runtime.
///
/// Prefer the free functions ([`crate::is_disposable_email`],
/// [`crate::is_disposable_domain`]) when the embedded list is sufficient —
/// they avoid the `HashSet` allocation this type carries.
///
/// # Examples
///
/// ```
/// use dispocheck::Blocklist;
///
/// let mut list = Blocklist::from_embedded();
/// assert!(list.is_disposable_domain("mailinator.com"));
///
/// list.add_domain("my-internal-throwaway.example");
/// assert!(list.is_disposable_domain("my-internal-throwaway.example"));
///
/// list.remove_domain("mailinator.com");
/// assert!(!list.is_disposable_domain("mailinator.com"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct Blocklist {
    extra: HashSet<String>,
    removed: HashSet<String>,
}

impl Blocklist {
    /// Creates a blocklist backed by the embedded compile-time dataset,
    /// with no extra domains and nothing removed.
    pub fn from_embedded() -> Self {
        Self {
            extra: HashSet::new(),
            removed: HashSet::new(),
        }
    }

    /// Creates a blocklist from an arbitrary set of domains, ignoring the
    /// embedded dataset entirely. Useful if you want to source the list
    /// yourself (e.g. fetched fresh over the network at startup).
    pub fn from_domains<I, S>(domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut list = Self {
            extra: HashSet::new(),
            removed: HashSet::new(),
        };
        // Remove everything embedded, then add back only what was given.
        for domain in crate::iter() {
            list.removed.insert((*domain).to_string());
        }
        for domain in domains {
            list.add_domain(domain.as_ref());
        }
        list
    }

    /// Adds a domain to the blocklist. No-op if already present (whether
    /// from the embedded set or previously added).
    pub fn add_domain(&mut self, domain: &str) {
        let normalized = normalize(domain);
        self.removed.remove(&normalized);
        self.extra.insert(normalized);
    }

    /// Removes a domain from the blocklist, whether it came from the
    /// embedded dataset or was added at runtime.
    pub fn remove_domain(&mut self, domain: &str) {
        let normalized = normalize(domain);
        self.extra.remove(&normalized);
        self.removed.insert(normalized);
    }

    /// Returns `true` if `email`'s domain (or a parent domain) is blocked.
    pub fn is_disposable_email(&self, email: &str) -> bool {
        match email.rsplit_once('@') {
            Some((_, domain)) if !domain.is_empty() => self.is_disposable_domain(domain),
            _ => false,
        }
    }

    /// Returns `true` if `domain`, or any of its parent domains, is
    /// blocked.
    pub fn is_disposable_domain(&self, domain: &str) -> bool {
        let normalized = normalize(domain);
        if normalized.is_empty() {
            return false;
        }
        let hit = domain_suffixes(&normalized).any(|suffix| {
            if self.removed.contains(suffix) {
                false
            } else {
                self.extra.contains(suffix) || crate::BLOCKLIST.contains(suffix)
            }
        });
        hit
    }

    /// Total number of domains currently in effect (embedded set minus
    /// removals, plus runtime additions). `O(embedded size)`; intended for
    /// diagnostics, not hot paths.
    pub fn len(&self) -> usize {
        let embedded_remaining = crate::iter()
            .filter(|d| !self.removed.contains(**d))
            .count();
        embedded_remaining + self.extra.len()
    }

    /// Returns `true` if this blocklist currently has no domains at all.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
