#[test]
fn test_minroot_fifth() {
    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--",
            "minroot-fifth",
            "--n",
            "100",
            "-x",
            "4",
            "-y",
            "5",
        ])
        .output()
        .expect("failed to execute process");

    assert!(
        output.status.success(),
        "Command failed with status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
