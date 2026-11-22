//! Configuration loading tests, ported from
//! internal/asc/config/config_test.go.
//!
//! Every case mutates the process environment, so they run sequentially
//! inside one test function (this binary has no other tests).

use asc_mcp::config::Config;

const VARS: [&str; 3] = ["ASC_ISSUER_ID", "ASC_KEY_ID", "ASC_PRIVATE_KEY_PATH"];

fn with_env(vars: &[(&str, &str)]) -> Result<Config, String> {
    // SAFETY: this test binary is single-threaded with respect to the
    // environment: only `load_cases` touches it.
    unsafe {
        for k in VARS {
            std::env::remove_var(k);
        }
        for (k, v) in vars {
            std::env::set_var(k, v);
        }
    }
    let r = Config::load();
    unsafe {
        for k in VARS {
            std::env::remove_var(k);
        }
    }
    r
}

fn expect_err(r: Result<Config, String>, contains: &str, case: &str) {
    match r {
        Ok(c) => panic!("{case}: expected error, got {c:?}"),
        Err(e) => assert!(e.contains(contains), "{case}: error {e:?} does not contain {contains:?}"),
    }
}

// Go: TestLoad + TestLoad_EmptyValues
#[test]
fn load_cases() {
    let dir = tempfile::tempdir().unwrap();
    let key = dir.path().join("test_key.p8");
    std::fs::write(&key, "test key content").unwrap();
    let key = key.to_string_lossy().into_owned();

    // valid configuration
    let cfg = with_env(&[("ASC_ISSUER_ID", "test-issuer-id"), ("ASC_KEY_ID", "TESTKEY123"), ("ASC_PRIVATE_KEY_PATH", &key)])
        .expect("valid configuration");
    assert_eq!(cfg.issuer_id, "test-issuer-id");
    assert_eq!(cfg.key_id, "TESTKEY123");
    assert_eq!(cfg.private_key_path, key);

    // missing values
    expect_err(with_env(&[("ASC_KEY_ID", "TESTKEY123"), ("ASC_PRIVATE_KEY_PATH", &key)]), "ASC_ISSUER_ID", "missing issuer ID");
    expect_err(with_env(&[("ASC_ISSUER_ID", "test-issuer-id"), ("ASC_PRIVATE_KEY_PATH", &key)]), "ASC_KEY_ID", "missing key ID");
    expect_err(
        with_env(&[("ASC_ISSUER_ID", "test-issuer-id"), ("ASC_KEY_ID", "TESTKEY123")]),
        "ASC_PRIVATE_KEY_PATH",
        "missing private key path",
    );
    expect_err(
        with_env(&[("ASC_ISSUER_ID", "test-issuer-id"), ("ASC_KEY_ID", "TESTKEY123"), ("ASC_PRIVATE_KEY_PATH", "/nonexistent/path/key.p8")]),
        "private key file not found",
        "nonexistent key file",
    );

    // empty values are treated as missing
    expect_err(with_env(&[("ASC_ISSUER_ID", ""), ("ASC_KEY_ID", "KEY123"), ("ASC_PRIVATE_KEY_PATH", &key)]), "ASC_ISSUER_ID", "empty issuer ID");
    expect_err(with_env(&[("ASC_ISSUER_ID", "issuer123"), ("ASC_KEY_ID", ""), ("ASC_PRIVATE_KEY_PATH", &key)]), "ASC_KEY_ID", "empty key ID");
    expect_err(
        with_env(&[("ASC_ISSUER_ID", "issuer123"), ("ASC_KEY_ID", "KEY123"), ("ASC_PRIVATE_KEY_PATH", "")]),
        "ASC_PRIVATE_KEY_PATH",
        "empty key path",
    );
}
