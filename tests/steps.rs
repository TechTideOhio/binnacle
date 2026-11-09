use binnacle::{extract, find_step, Envelope, STEPS};

#[test]
fn pointer_extracts_envelope_fields() {
    let env = Envelope {
        step: 6,
        tool: "keel".into(),
        binary: None,
        exit_code: Some(0),
        passed: true,
        stdout: "hi".into(),
        stderr: String::new(),
    };
    let get = |p: &str| extract(&env, p).expect("envelope serializes");
    assert_eq!(get("/passed").as_deref(), Some("true"));
    assert_eq!(get("/exit_code").as_deref(), Some("0"));
    assert_eq!(get("/stdout").as_deref(), Some("hi"));
    assert_eq!(get("/binary").as_deref(), Some("null"));
    assert!(get("/nope").is_none());
    assert!(get("passed").is_none());
}

fn stub_root(name: &str) -> std::path::PathBuf {
    let root = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let bin_dir = root.join("keel").join("target").join("release");
    std::fs::create_dir_all(&bin_dir).expect("create stub dir");
    let stub = bin_dir.join(format!("keel{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(env!("CARGO_BIN_EXE_binnacle"), stub).expect("copy stub binary");
    root
}

fn binnacle(root: &std::path::Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_binnacle"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .expect("binnacle binary should launch")
}

#[test]
fn pointer_through_binary_prints_field_and_keeps_exit_code() {
    let root = stub_root("pointer_exit_code");

    let ok = binnacle(&root, &["run", "--pointer", "/passed", "keel", "table"]);
    assert_eq!(String::from_utf8_lossy(&ok.stdout).trim(), "true");
    assert_eq!(ok.status.code(), Some(0));

    let fail = binnacle(&root, &["run", "--pointer", "/exit_code", "keel"]);
    assert_eq!(String::from_utf8_lossy(&fail.stdout).trim(), "2");
    assert_eq!(fail.status.code(), Some(1));
}

#[test]
fn pointer_not_found_through_binary_exits_2() {
    let root = stub_root("pointer_not_found");
    let out = binnacle(&root, &["run", "--pointer", "/nope", "keel", "table"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr).contains("not found"));
}

#[test]
fn pointer_flag_without_value_is_usage_error() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_binnacle"))
        .args(["run", "--pointer"])
        .output()
        .expect("binnacle binary should launch");
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn fifteen_unique_steps_map_to_unique_tools() {
    assert_eq!(STEPS.len(), 15);
    for (i, s) in STEPS.iter().enumerate() {
        assert_eq!(usize::from(s.step), i + 1);
        assert_eq!(STEPS.iter().filter(|o| o.tool == s.tool).count(), 1);
    }
}

#[test]
fn resolves_by_name_or_step_number() {
    assert_eq!(find_step("keel").map(|s| s.step), Some(6));
    assert_eq!(find_step("5").map(|s| s.tool), Some("tide-rill"));
    assert!(find_step("nope").is_none());
}

#[test]
fn missing_binary_is_an_error_not_a_pass() {
    let r = binnacle::run_tool(std::path::Path::new("does-not-exist"), "keel", &[]);
    assert!(r.is_err());
}
