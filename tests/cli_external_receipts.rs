#![cfg(unix)]

use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Output},
};

fn executable(directory: &Path, name: &str, body: &str) {
    let path = directory.join(name);
    fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn run_with_path(path: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_skill-fence"))
        .args(arguments)
        .env("PATH", path)
        .output()
        .unwrap()
}

fn evidence(output: &Output, adapter: &str) -> Value {
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    json["external_evidence"]
        .as_array()
        .unwrap()
        .iter()
        .find(|receipt| receipt["adapter"] == adapter)
        .unwrap()
        .clone()
}

#[test]
fn records_success_and_rejection_as_replayable_process_receipts() {
    let executables = tempfile::tempdir().unwrap();
    executable(
        executables.path(),
        "ruvector",
        "test \"$1|$2|$3\" = 'embed|text|agent skill release evidence' || exit 91\nprintf 'vector fixture accepted\\n'",
    );
    executable(
        executables.path(),
        "metaharness",
        "test \"$1|$2|$3|$4\" = 'pro|redblue|run|--mock-judge' || exit 92\nprintf 'fixture rejected\\n' >&2\nexit 23",
    );

    let output = run_with_path(
        executables.path(),
        &[
            "--skill",
            "examples/release-ready-skill",
            "--ruvector-probe",
            "--redblue-probe",
        ],
    );

    assert!(output.status.success(), "{output:?}");
    let ruvector = evidence(&output, "ruvector");
    assert_eq!(ruvector["status"], "Passed");
    assert_eq!(
        ruvector["command"],
        serde_json::json!(["ruvector", "embed", "text", "agent skill release evidence"])
    );
    assert_eq!(ruvector["exit_code"], 0);
    assert_eq!(ruvector["stdout"], "vector fixture accepted\n");
    assert_eq!(ruvector["stderr"], "");
    assert!(ruvector["error"].is_null());

    let redblue = evidence(&output, "metaharness-redblue");
    assert_eq!(redblue["status"], "Failed");
    assert_eq!(
        redblue["command"],
        serde_json::json!(["metaharness", "pro", "redblue", "run", "--mock-judge"])
    );
    assert_eq!(redblue["exit_code"], 23);
    assert_eq!(redblue["stdout"], "");
    assert_eq!(redblue["stderr"], "fixture rejected\n");
    assert!(redblue["error"].is_null());
}

#[test]
fn records_an_unavailable_executable_without_claiming_it_ran() {
    let empty_path = tempfile::tempdir().unwrap();
    let output = run_with_path(
        empty_path.path(),
        &[
            "--skill",
            "examples/release-ready-skill",
            "--ruvector-probe",
        ],
    );

    assert!(output.status.success(), "{output:?}");
    let receipt = evidence(&output, "ruvector");
    assert_eq!(receipt["status"], "Unavailable");
    assert_eq!(
        receipt["command"],
        serde_json::json!(["ruvector", "embed", "text", "agent skill release evidence"])
    );
    assert!(receipt["exit_code"].is_null());
    assert_eq!(receipt["stdout"], "");
    assert_eq!(receipt["stderr"], "");
    assert!(receipt["error"].as_str().unwrap().contains("No such file"));
}
