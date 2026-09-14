use std::{fs, process::Command};

#[test]
fn missing_comparison_fails_without_writing_evidence() {
    let workspace = tempfile::tempdir().expect("temporary workspace");
    let skill = workspace.path().join("release-ready");
    fs::create_dir(&skill).expect("skill directory");
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: release-ready\ndescription: bounded\n---\nRequest human approval.",
    )
    .expect("skill fixture");
    let missing_comparison = workspace.path().join("missing-comparison");
    let evidence = workspace.path().join("evidence.json");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-fence"))
        .args([
            "--skill",
            skill.to_str().expect("UTF-8 skill path"),
            "--compare",
            missing_comparison.to_str().expect("UTF-8 comparison path"),
            "--out",
            evidence.to_str().expect("UTF-8 evidence path"),
        ])
        .output()
        .expect("run SkillFence");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains(missing_comparison.to_str().expect("UTF-8 comparison path")),
        "stderr should identify the unreadable comparison: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!evidence.exists());
}
