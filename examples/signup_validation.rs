//! Example: rejecting signups that use a disposable email address.
//!
//! Run with: `cargo run --example signup_validation`

use dispocheck::is_disposable_email;

fn validate_signup_email(email: &str) -> Result<(), &'static str> {
    if is_disposable_email(email) {
        return Err("Please use a permanent email address.");
    }
    Ok(())
}

fn main() {
    let candidates = [
        "alice@company.com",
        "bob@mailinator.com",
        "carol@10minutemail.com",
        "dave@gmail.com",
    ];

    for email in candidates {
        match validate_signup_email(email) {
            Ok(()) => println!("{email:32} -> accepted"),
            Err(reason) => println!("{email:32} -> rejected ({reason})"),
        }
    }
}
