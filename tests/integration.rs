use dispocheck::{is_disposable_domain, is_disposable_email, is_empty, len};

#[test]
fn embedded_list_is_populated() {
    assert!(!is_empty());
    assert!(
        len() > 1000,
        "expected a substantial embedded list, got {}",
        len()
    );
}

#[test]
fn detects_known_disposable_domains() {
    // A handful of long-standing, well-known disposable providers that are
    // extremely unlikely to ever be removed from the upstream list.
    for domain in [
        "mailinator.com",
        "guerrillamail.com",
        "10minutemail.com",
        "yopmail.com",
    ] {
        assert!(
            is_disposable_domain(domain),
            "{domain} expected to be disposable"
        );
    }
}

#[test]
fn rejects_common_legitimate_domains() {
    for domain in [
        "gmail.com",
        "outlook.com",
        "example.com",
        "company.co.uk",
        "",
        " ",
        ".",
        "...",
    ] {
        assert!(
            !is_disposable_domain(domain),
            "{domain:?} unexpectedly flagged as disposable"
        );
    }
}

#[test]
fn matches_subdomains_of_disposable_domains() {
    assert!(is_disposable_domain("sub.mailinator.com"));
    assert!(is_disposable_domain("a.b.c.mailinator.com"));
}

#[test]
fn is_case_insensitive() {
    assert!(is_disposable_domain("MAILINATOR.COM"));
    assert!(is_disposable_domain("Mailinator.Com"));
    assert!(is_disposable_email("User@MAILINATOR.COM"));
}

#[test]
fn tolerates_trailing_dot_fqdn() {
    assert!(is_disposable_domain("mailinator.com."));
}

#[test]
fn email_extraction_handles_edge_cases() {
    assert!(!is_disposable_email("no-at-sign"));
    assert!(!is_disposable_email("trailing-at@"));
    assert!(!is_disposable_email(""));
    assert!(is_disposable_email("first.last+tag@mailinator.com"));
    // Only the final '@' should count as the domain separator.
    assert!(is_disposable_email("weird@name@mailinator.com"));
}

#[test]
fn does_not_falsely_match_domains_containing_a_blocked_domain_as_substring() {
    // "reallymailinator.com" is a distinct, made-up domain that merely
    // contains "mailinator.com" as a substring; matching is component-based
    // (split on '.'), so it must not be flagged just because of that.
    assert!(!is_disposable_domain("reallymailinator.com"));
}

#[test]
fn iterator_matches_len() {
    assert_eq!(dispocheck::iter().count(), len());
}
