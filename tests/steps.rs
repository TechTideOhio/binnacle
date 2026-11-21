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

// ---------------------------------------------------------------------------
// Discovery and envelope tests, built on fake tool binaries in a temp dir.
// ---------------------------------------------------------------------------

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, process};

use binnacle::{doctor, find_binary, find_tool_dir, run_tool};

/// Serialises tests that write and then execute fake binaries. Without it, a
/// concurrent fork can inherit a still-open write handle and make `exec` fail
/// with "Text file busy" on Linux.
static SPAWN_LOCK: Mutex<()> = Mutex::new(());

/// A fresh, empty temp directory unique to this process and test.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("binnacle-test-{}-{name}", process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

/// Create `<root>/<tool>/target/release/<tool>` as a shell script that echoes
/// its arguments and exits with `code` (except `--help`, which exits 0 when
/// `help_ok` is true and 1 otherwise).
#[cfg(unix)]
fn fake_tool(root: &Path, tool: &str, code: i32, help_ok: bool) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let dir = root.join(tool).join("target").join("release");
    fs::create_dir_all(&dir).expect("create bin dir");
    let bin = dir.join(tool);
    let help_code = i32::from(!help_ok);
    let script = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--help\" ]; then exit {help_code}; fi\necho \"args:$*\"\necho oops >&2\nexit {code}\n"
    );
    fs::write(&bin, script).expect("write script");
    fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).expect("chmod");
    bin
}

#[test]
fn find_tool_dir_checks_all_four_layouts() {
    let root = scratch("tool-dir");

    // Nothing exists yet.
    assert!(find_tool_dir(&root.join("work"), "keel").is_none());

    // <root>/<tool>
    fs::create_dir_all(root.join("a").join("keel")).expect("mkdir");
    assert_eq!(
        find_tool_dir(&root.join("a"), "keel"),
        Some(root.join("a").join("keel"))
    );

    // <root>/Tools/<tool>
    fs::create_dir_all(root.join("b").join("Tools").join("keel")).expect("mkdir");
    assert_eq!(
        find_tool_dir(&root.join("b"), "keel"),
        Some(root.join("b").join("Tools").join("keel"))
    );

    // <root>/../<tool>
    fs::create_dir_all(root.join("c").join("keel")).expect("mkdir");
    fs::create_dir_all(root.join("c").join("work")).expect("mkdir");
    assert_eq!(
        find_tool_dir(&root.join("c").join("work"), "keel"),
        Some(root.join("c").join("work").join("..").join("keel"))
    );

    // <root>/../Tools/<tool>
    fs::create_dir_all(root.join("d").join("Tools").join("keel")).expect("mkdir");
    fs::create_dir_all(root.join("d").join("work")).expect("mkdir");
    assert_eq!(
        find_tool_dir(&root.join("d").join("work"), "keel"),
        Some(
            root.join("d")
                .join("work")
                .join("..")
                .join("Tools")
                .join("keel")
        )
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
#[cfg(unix)]
fn find_binary_prefers_release_build_under_root() {
    let root = scratch("find-binary");
    let bin = fake_tool(&root, "keel", 0, true);
    assert_eq!(find_binary(&root, "keel"), Some(bin));
    let _ = fs::remove_dir_all(&root);
}

#[test]
#[cfg(unix)]
fn run_tool_passes_on_exit_zero_and_forwards_args() {
    let _guard = SPAWN_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = scratch("pass");
    fake_tool(&root, "keel", 0, true);

    let args = vec!["check".to_owned(), "--suite".to_owned()];
    let env = run_tool(&root, "keel", &args).expect("tool should run");

    assert_eq!(env.step, 6);
    assert_eq!(env.tool, "keel");
    assert_eq!(env.exit_code, Some(0));
    assert!(env.passed);
    assert!(env.binary.is_some());
    assert!(env.stdout.contains("args:check --suite"));
    assert!(env.stderr.contains("oops"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
#[cfg(unix)]
fn run_tool_fails_on_nonzero_exit_but_still_returns_envelope() {
    let _guard = SPAWN_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = scratch("fail");
    fake_tool(&root, "keel", 3, true);

    let env = run_tool(&root, "keel", &[]).expect("a failing tool is still an envelope");

    assert_eq!(env.exit_code, Some(3));
    assert!(!env.passed);
    let _ = fs::remove_dir_all(&root);
}

#[test]
#[cfg(unix)]
fn run_tool_accepts_step_number() {
    let _guard = SPAWN_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = scratch("by-number");
    fake_tool(&root, "keel", 0, true);

    let env = run_tool(&root, "6", &[]).expect("step number should resolve");
    assert_eq!(env.tool, "keel");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn run_tool_rejects_unknown_tool() {
    let err = run_tool(Path::new("."), "definitely-not-a-tool", &[])
        .err()
        .expect("unknown tool must be an error");
    assert!(err.contains("unknown tool or step"));
}

#[test]
#[cfg(unix)]
fn doctor_reports_responds_based_on_help_exit_code() {
    let _guard = SPAWN_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = scratch("doctor");
    fake_tool(&root, "keel", 0, true);
    fake_tool(&root, "reval", 0, false);

    let rows = doctor(&root);
    assert_eq!(rows.len(), 15);

    let keel = rows.iter().find(|r| r.tool == "keel").expect("keel row");
    assert!(keel.binary.is_some());
    assert!(keel.responds);

    let reval = rows.iter().find(|r| r.tool == "reval").expect("reval row");
    assert!(reval.binary.is_some());
    assert!(
        !reval.responds,
        "--help exiting non-zero must not count as healthy"
    );
    let _ = fs::remove_dir_all(&root);
}
