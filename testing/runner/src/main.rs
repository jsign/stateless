use clap::Parser;
use ef_tests::{Suite, cases::blockchain_test::BlockchainTests};
use std::path::PathBuf;
use tempfile::TempDir;

/// CLI for running Ethereum Foundation execution witness tests.
#[derive(Parser)]
struct TestRunnerCommand {
    /// Path to the test suite directory or URL to a tar.gz archive.
    suite: String,
}

/// Resolves the suite path from a local path or a URL.
///
/// If the input is an HTTP(S) URL, the archive is downloaded and extracted into a temporary
/// directory. The `TempDir` is returned so that it stays alive for the duration of the test run.
fn resolve_suite_path(suite: &str) -> (PathBuf, Option<TempDir>) {
    if suite.starts_with("http://") || suite.starts_with("https://") {
        let response = reqwest::blocking::get(suite).expect("failed to download test suite");
        let tmp = tempfile::tempdir().expect("failed to create temp dir");
        let decoder = flate2::read::GzDecoder::new(response);
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(tmp.path()).expect("failed to extract archive");
        (tmp.path().to_path_buf(), Some(tmp))
    } else {
        (PathBuf::from(suite), None)
    }
}

fn main() {
    let cmd = TestRunnerCommand::parse();
    let (path, _tmp) = resolve_suite_path(&cmd.suite);
    BlockchainTests::new(path).run();
}
