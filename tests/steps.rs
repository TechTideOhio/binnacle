use binnacle::{find_step, STEPS};

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
