//! Execution-spec fixture tests for the Reth guest on the host.

use stateless_validator_tests::{
    execution::{ExecutionFailures, run_host_execution},
    fixture::eest_fixtures,
};

#[test]
fn executes_eest_glamsterdam_fixtures() {
    let fixtures = eest_fixtures();
    assert!(!fixtures.is_empty(), "no stateless validator fixtures loaded");

    // Keep the EIP-8037 regressions that require bluealloy/revm#3893 in this suite.
    for test in [
        "test_cross_frame_refund_repays_spill_at_merge",
        "test_cross_frame_refund_repays_spill_in_inner_frame",
        "test_cross_frame_refund_with_reservoir_grant",
        "test_child_clear_repays_own_spill_first",
        "test_repaid_credit_funds_execution",
        "test_cross_frame_refund_repays_spill_at_a_call",
    ] {
        let prefix = format!(
            "tests/amsterdam/eip8037_state_creation_gas_cost_increase/\
             test_state_gas_cross_frame_refund.py::{test}["
        );
        assert!(
            fixtures.iter().any(|fixture| fixture.name.starts_with(&prefix)),
            "missing EIP-8037 fixture: {test}",
        );
    }

    println!("Executing {} stateless validator fixtures", fixtures.len());
    let failures = run_host_execution(fixtures);
    assert!(
        failures.is_empty(),
        "stateless validator fixture failures:\n{}",
        ExecutionFailures(&failures),
    );
}
