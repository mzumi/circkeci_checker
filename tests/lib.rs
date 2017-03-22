extern crate circleci_checker;

use circleci_checker::*;

#[test]
fn success_status() {
    let status =  Status::get("success").unwrap();
    assert_eq!("green", status.color());
    assert_eq!("✔︎", status.symbol());
}

#[test]
fn failed_status() {
    let status = Status::get("failed").unwrap();
    assert_eq!("red", status.color());
    assert_eq!("✖︎", status.symbol());
}

#[test]
fn cancel_status() {
    let status = Status::get("cancel").unwrap();
    assert_eq!("yellow", status.color());
    assert_eq!("⚠", status.symbol());
}

#[test]
fn timeout_status() {
    let status = Status::get("timedout").unwrap();
    assert_eq!("gray", status.color());
    assert_eq!(" ⃠", status.symbol());
}

#[test]
fn not_build_status() {
    let status = Status::get("no_tests").unwrap();
    assert_eq!("gray", status.color());
    assert_eq!("•", status.symbol());
}
