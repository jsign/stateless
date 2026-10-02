//! Execution-spec fixture tests for the Reth guest on the host.

use stateless_validator_tests::{
    execution::{ExecutionFailures, run_host_execution},
    fixture::{FixtureFormat, eest_fixtures},
};

/// Tests whose fixtures fail on upstream EIP-7928 block access list bugs, matched by test name.
///
/// alloy-evm runs the post-execution system calls before crediting withdrawals, the reverse of
/// EELS `apply_body`, and revm's BAL builder drops a write when a later commit at the same block
/// access index only reads it (<https://github.com/bluealloy/revm/pull/3954>).
const KNOWN_FAILURES: &[&str] = &[
    "test_bal_post_execution_calls_net_storage_at_last_index[",
    "test_bal_withdrawals_and_dequeues_net_balance_at_last_index[",
];

fn is_known_failure(name: &str) -> bool {
    KNOWN_FAILURES.iter().any(|test| name.contains(test))
}

fn executes_eest_fixtures(format: FixtureFormat) {
    let fixtures = eest_fixtures(format);
    assert!(!fixtures.is_empty(), "no {format:?} stateless validator fixtures loaded");
    let known_failures = fixtures.iter().filter(|fixture| is_known_failure(&fixture.name)).count();

    println!("Executing {} {format:?} stateless validator fixtures", fixtures.len());
    let (known, unexpected): (Vec<_>, Vec<_>) = run_host_execution(fixtures)
        .into_iter()
        .partition(|failure| is_known_failure(&failure.name));
    assert!(
        unexpected.is_empty(),
        "stateless validator fixture failures:\n{}",
        ExecutionFailures(&unexpected),
    );
    assert_eq!(
        known.len(),
        known_failures,
        "known failures now pass, remove them from KNOWN_FAILURES"
    );
}

#[test]
fn executes_eest_blockchain_test_fixtures() {
    executes_eest_fixtures(FixtureFormat::BlockchainTest);
}

#[test]
fn executes_eest_blockchain_test_engine_fixtures() {
    executes_eest_fixtures(FixtureFormat::BlockchainTestEngine);
}
