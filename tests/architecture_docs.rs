use std::process::Command;

#[test]
fn repository_documentation_has_machine_checked_schema_and_traceability() {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-fence-docs"))
        .args(["--root", env!("CARGO_MANIFEST_DIR")])
        .output()
        .expect("run the documentation validator");

    assert!(
        output.status.success(),
        "validator should accept substantive linked docs: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("accepted ADRs: 3"), "{stdout}");
    assert!(stdout.contains("DDD artifacts: 3"), "{stdout}");
}

#[test]
fn completion_mode_fails_closed_with_exact_document_shortfall() {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-fence-docs"))
        .args(["--root", env!("CARGO_MANIFEST_DIR"), "--completion"])
        .output()
        .expect("run the documentation validator");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("accepted ADRs: 3/24"), "{stderr}");
    assert!(stderr.contains("DDD artifacts: 3/12"), "{stderr}");
}
