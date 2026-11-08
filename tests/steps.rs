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
    assert_eq!(extract(&env, "/passed").as_deref(), Some("true"));
    assert_eq!(extract(&env, "/exit_code").as_deref(), Some("0"));
    assert_eq!(extract(&env, "/stdout").as_deref(), Some("hi"));
    assert_eq!(extract(&env, "/binary").as_deref(), Some("null"));
    assert!(extract(&env, "/nope").is_none());
    assert!(extract(&env, "passed").is_none());
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
