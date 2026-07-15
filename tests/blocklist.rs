#![cfg(feature = "std")]

use dispocheck::Blocklist;

#[test]
fn embedded_contains_known_domain() {
    let list = Blocklist::from_embedded();
    assert!(list.is_disposable_domain("mailinator.com"));
    assert!(list.is_disposable_email("user@mailinator.com"));
}

#[test]
fn add_domain() {
    let mut list = Blocklist::from_embedded();

    assert!(!list.is_disposable_domain("example.test"));

    list.add_domain("example.test");

    assert!(list.is_disposable_domain("example.test"));
    assert!(list.is_disposable_email("user@example.test"));
}

#[test]
fn remove_embedded_domain() {
    let mut list = Blocklist::from_embedded();

    assert!(list.is_disposable_domain("mailinator.com"));

    list.remove_domain("mailinator.com");

    assert!(!list.is_disposable_domain("mailinator.com"));
}

#[test]
fn remove_runtime_domain() {
    let mut list = Blocklist::from_embedded();

    list.add_domain("runtime.test");
    assert!(list.is_disposable_domain("runtime.test"));

    list.remove_domain("runtime.test");
    assert!(!list.is_disposable_domain("runtime.test"));
}

#[test]
fn parent_domain_matching() {
    let mut list = Blocklist::from_embedded();

    list.add_domain("example.test");

    assert!(list.is_disposable_domain("sub.example.test"));
    assert!(list.is_disposable_domain("a.b.example.test"));
    assert!(list.is_disposable_email("user@sub.example.test"));
}

#[test]
fn from_domains_replaces_embedded() {
    let list = Blocklist::from_domains(["custom.test"]);

    assert!(list.is_disposable_domain("custom.test"));
    assert!(list.is_disposable_email("user@custom.test"));

    assert!(!list.is_disposable_domain("mailinator.com"));
}

#[test]
fn invalid_email_returns_false() {
    let list = Blocklist::from_embedded();

    assert!(!list.is_disposable_email("not-an-email"));
    assert!(!list.is_disposable_email("@"));
    assert!(!list.is_disposable_email(""));
    assert!(!list.is_disposable_email("user@"));
}

#[test]
fn empty_domain_returns_false() {
    let list = Blocklist::from_embedded();

    assert!(!list.is_disposable_domain(""));
    assert!(!list.is_disposable_domain(" "));
}

#[test]
fn len_and_is_empty() {
    let mut list = Blocklist::from_domains(std::iter::empty::<&str>());

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);

    list.add_domain("one.test");

    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
}

#[test]
fn normalization_works() {
    let mut list = Blocklist::from_domains(std::iter::empty::<&str>());

    list.add_domain("CUSTOM.TEST");

    assert!(list.is_disposable_domain("custom.test"));
    assert!(list.is_disposable_domain("CUSTOM.TEST"));
}

#[test]
fn adding_existing_embedded_domain_keeps_it_blocked() {
    let mut list = Blocklist::from_embedded();

    assert!(list.is_disposable_domain("mailinator.com"));

    list.add_domain("mailinator.com");

    assert!(list.is_disposable_domain("mailinator.com"));
}

#[test]
fn removing_then_adding_domain_restores_it() {
    let mut list = Blocklist::from_embedded();

    list.remove_domain("mailinator.com");
    assert!(!list.is_disposable_domain("mailinator.com"));

    list.add_domain("mailinator.com");
    assert!(list.is_disposable_domain("mailinator.com"));
}

#[test]
fn runtime_domains_are_case_normalized() {
    let mut list = Blocklist::from_domains(std::iter::empty::<&str>());

    list.add_domain("MyThrowAway.TEST");

    assert!(list.is_disposable_domain("mythrowaway.test"));
    assert!(list.is_disposable_domain("MYTHROWAWAY.TEST"));
}

#[test]
fn from_domains_accepts_multiple_domains() {
    let list = Blocklist::from_domains(["first.test","second.test","third.test"]);

    assert!(list.is_disposable_domain("first.test"));
    assert!(list.is_disposable_domain("second.test"));
    assert!(list.is_disposable_domain("third.test"));

    assert_eq!(list.len(), 3);
}