//! Binnacle: the unified evaluator-agent orchestrator for the 15-tool `TideOS` evaluation stack.
//!
//! # Architecture
//! Binnacle follows the Unix philosophy of small, decoupled tools. It serves as the conductor
//! and CLI supervisor over 15 standalone, zero-FFI Rust evaluation engines (`reval`, `gild`,
//! `tessel`, `assay`, `keel`, etc.).
//!
//! Each engine is maintained as an independent repository under the [`TechTideOhio`](https://github.com/TechTideOhio)
//! organization. Engines can be executed standalone in CI or pre-commit hooks, or discovered and
//! orchestrated via `binnacle` to emit normalized JSON execution envelopes.

#![forbid(unsafe_code)]

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Step {
    pub step: u8,
    pub name: &'static str,
    pub tool: &'static str,
}

pub const STEPS: [Step; 15] = [
    Step {
        step: 1,
        name: "error analysis",
        tool: "reval",
    },
    Step {
        step: 2,
        name: "golden dataset",
        tool: "gild",
    },
    Step {
        step: 3,
        name: "synthetic dimensions",
        tool: "tessel",
    },
    Step {
        step: 4,
        name: "deterministic assertions",
        tool: "assay",
    },
    Step {
        step: 5,
        name: "retrieval vs generation",
        tool: "tide-rill",
    },
    Step {
        step: 6,
        name: "trajectory evals",
        tool: "keel",
    },
    Step {
        step: 7,
        name: "pass@k vs pass^k",
        tool: "temper",
    },
    Step {
        step: 8,
        name: "simulated users",
        tool: "parley",
    },
    Step {
        step: 9,
        name: "judge alignment",
        tool: "gavel",
    },
    Step {
        step: 10,
        name: "pairwise judge bias",
        tool: "poise",
    },
    Step {
        step: 11,
        name: "statistics",
        tool: "plumb",
    },
    Step {
        step: 12,
        name: "CI gate",
        tool: "weir",
    },
    Step {
        step: 13,
        name: "online evals",
        tool: "eddy",
    },
    Step {
        step: 14,
        name: "cost and latency",
        tool: "tare",
    },
    Step {
        step: 15,
        name: "close the loop",
        tool: "tether",
    },
];

#[derive(Serialize)]
pub struct Envelope {
    pub step: u8,
    pub tool: String,
    pub binary: Option<String>,
    pub exit_code: Option<i32>,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize)]
pub struct DoctorRow {
    pub step: u8,
    pub tool: &'static str,
    pub binary: Option<String>,
    pub responds: bool,
}

#[must_use]
pub fn find_step(tool: &str) -> Option<&'static Step> {
    STEPS
        .iter()
        .find(|s| s.tool == tool || s.step.to_string() == tool)
}

/// Locate tool root directory under `<root>/<tool>`, sibling `<root>/../<tool>`, or `<root>/Tools/<tool>`.
#[must_use]
pub fn find_tool_dir(root: &Path, tool: &str) -> Option<PathBuf> {
    let direct = root.join(tool);
    if direct.is_dir() {
        return Some(direct);
    }
    let in_tools = root.join("Tools").join(tool);
    if in_tools.is_dir() {
        return Some(in_tools);
    }
    let parent = root.join("..").join(tool);
    if parent.is_dir() {
        return Some(parent);
    }
    let parent_tools = root.join("..").join("Tools").join(tool);
    if parent_tools.is_dir() {
        return Some(parent_tools);
    }
    None
}

/// Locate a built binary under `<root>/<tool>/target/{release,debug}`, sibling paths,
/// current executable's directory, ~/.cargo/bin, or system PATH.
#[must_use]
pub fn find_binary(root: &Path, tool: &str) -> Option<PathBuf> {
    let exe = format!("{tool}{}", std::env::consts::EXE_SUFFIX);
    let mut candidates = vec![
        root.join(tool).join("target").join("release").join(&exe),
        root.join(tool).join("target").join("debug").join(&exe),
        root.join("Tools")
            .join(tool)
            .join("target")
            .join("release")
            .join(&exe),
        root.join("Tools")
            .join(tool)
            .join("target")
            .join("debug")
            .join(&exe),
        root.join("..")
            .join(tool)
            .join("target")
            .join("release")
            .join(&exe),
        root.join("..")
            .join(tool)
            .join("target")
            .join("debug")
            .join(&exe),
        root.join("..")
            .join("Tools")
            .join(tool)
            .join("target")
            .join("release")
            .join(&exe),
        root.join("..")
            .join("Tools")
            .join(tool)
            .join("target")
            .join("debug")
            .join(&exe),
    ];

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(bin_dir) = current_exe.parent() {
            candidates.push(bin_dir.join(&exe));
        }
    }

    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        candidates.push(PathBuf::from(home).join(".cargo").join("bin").join(&exe));
    }
    if let Some(cargo_home) = std::env::var_os("CARGO_HOME") {
        candidates.push(PathBuf::from(cargo_home).join("bin").join(&exe));
    }

    if let Some(found) = candidates.into_iter().find(|p| p.is_file()) {
        return Some(found);
    }

    if let Some(path_var) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join(&exe);
            if p.is_file() {
                return Some(p);
            }
        }
    }

    None
}

/// Run a tool's own CLI and wrap the result. `passed` is true only for exit code 0.
/// Automatically executes within the tool's root directory if discovered, ensuring
/// relative fixture paths resolve seamlessly.
pub fn run_tool(root: &Path, tool: &str, args: &[String]) -> Result<Envelope, String> {
    let step = find_step(tool).ok_or_else(|| format!("unknown tool or step: {tool}"))?;
    let bin = find_binary(root, step.tool)
        .ok_or_else(|| format!("{} is not built under {}", step.tool, root.display()))?;

    let mut cmd = Command::new(&bin);
    if let Some(tool_dir) = find_tool_dir(root, step.tool) {
        cmd.current_dir(tool_dir);
    }
    cmd.args(args);

    let out = cmd
        .output()
        .map_err(|e| format!("could not launch {}: {e}", step.tool))?;
    Ok(Envelope {
        step: step.step,
        tool: step.tool.to_owned(),
        binary: Some(bin.display().to_string()),
        exit_code: out.status.code(),
        passed: out.status.success(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    })
}

/// Look up a JSON Pointer in the envelope.
/// Strings are returned without quotes; other values as JSON text.
/// Returns `Ok(None)` when the pointer matches nothing.
pub fn extract(env: &Envelope, pointer: &str) -> Result<Option<String>, String> {
    let v = serde_json::to_value(env).map_err(|e| format!("could not serialize envelope: {e}"))?;
    Ok(v.pointer(pointer).map(|found| {
        found
            .as_str()
            .map_or_else(|| found.to_string(), str::to_owned)
    }))
}

/// Check every step has a built binary that launches with `--help`.
#[must_use]
pub fn doctor(root: &Path) -> Vec<DoctorRow> {
    STEPS
        .iter()
        .map(|s| {
            let binary = find_binary(root, s.tool);
            let responds = binary
                .as_ref()
                .and_then(|b| Command::new(b).arg("--help").output().ok())
                .is_some_and(|o| o.status.success());
            DoctorRow {
                step: s.step,
                tool: s.tool,
                binary: binary.map(|b| b.display().to_string()),
                responds,
            }
        })
        .collect()
}
