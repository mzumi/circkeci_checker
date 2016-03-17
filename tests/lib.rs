extern crate circleci_checker;

use circleci_checker::status;

#[test]
fn success_status() {
    let status = "success";
    assert_eq!("green", status::get(status).color());
    assert_eq!("✔︎", status::get(status).symbol());
}

#[test]
fn failed_status() {
    let status = "failed";
    assert_eq!("red", status::get(status).color());
    assert_eq!("✖︎", status::get(status).symbol());
}

#[test]
fn cancel_status() {
    let status = "cancel";
    assert_eq!("yellow", status::get(status).color());
    assert_eq!("⚠", status::get(status).symbol());
}

#[test]
fn timeout_status() {
    let status = "timedout";
    assert_eq!("gray", status::get(status).color());
    assert_eq!(" ⃠", status::get(status).symbol());
}

#[test]
fn not_build_status() {
    let status = "";
    assert_eq!("gray", status::get(status).color());
    assert_eq!("•", status::get(status).symbol());
}
