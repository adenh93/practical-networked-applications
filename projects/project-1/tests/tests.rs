use assert_cmd::prelude::*;
use predicates::str::contains;
use project_1::KvStore;
use std::process::Command;

// `kvs` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("project-1").unwrap().assert().failure();
}

// `kvs -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `kvs get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_get() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["get", "key1"])
        .assert()
        .failure()
        .stderr(contains("not implemented"));
}

// `kvs set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["set", "key1", "value1"])
        .assert()
        .failure()
        .stderr(contains("not implemented"));
}

// `kvs rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_rm() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["rm", "key1"])
        .assert()
        .failure()
        .stderr(contains("not implemented"));
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["get"])
        .assert()
        .failure();

    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["set"])
        .assert()
        .failure();

    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["rm"])
        .assert()
        .failure();

    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("project-1")
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}

// Should get previously stored value
#[test]
fn get_stored_value() {
    let mut store = KvStore::new();

    store.set("key1", "value1");
    store.set("key2", "value2");

    assert_eq!(store.get("key1"), Some("value1"));
    assert_eq!(store.get("key2"), Some("value2"));
}

// Should overwrite existent value
#[test]
fn overwrite_value() {
    let mut store = KvStore::new();

    store.set("key1", "value1");
    assert_eq!(store.get("key1"), Some("value1"));

    store.set("key1", "value2");
    assert_eq!(store.get("key1"), Some("value2"));
}

// Should get `None` when getting a non-existent key
#[test]
fn get_non_existent_value() {
    let mut store = KvStore::new();

    store.set("key1", "value1");
    assert_eq!(store.get("key2"), None);
}

#[test]
fn remove_key() {
    let mut store = KvStore::new();

    store.set("key1", "value1");
    store.remove("key1");
    assert_eq!(store.get("key1"), None);
}
