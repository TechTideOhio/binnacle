# Contributing to Binnacle

Welcome! We are thrilled you want to contribute to **Binnacle**, the unified, zero-FFI, pure-Rust evaluator agent orchestrating the 15-tool TideOS evaluation stack.

Evaluation infrastructure is what compounds in modern AI systems engineering. Our goal is to keep Binnacle lightning-fast, mathematically bounded, deterministic, and accessible to every engineer building autonomous coding agents.

---

## Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for everyone, regardless of background or experience level. Please treat all contributors with respect, professionalism, and constructive empathy.

---

## Architectural Principles & Invariants

When contributing code to Binnacle, please keep these non-negotiable principles in mind:

1. **Zero-FFI & Pure Rust**: Every crate and module compiles under `#![forbid(unsafe_code)]` with zero C-bindings or foreign library dependencies.
2. **Sub-Second Execution**: Deterministic evaluations must run in single-digit milliseconds. Do not add heavy runtime overhead or stochastic network calls to core evaluation loops.
3. **Decoupled Evaluator Roles**: Binnacle is a thin, robust orchestrator over dedicated Unix-philosophy CLI binaries (`reval`, `gild`, `tessel`, `assay`, etc.). Tool scoring logic belongs in each tool's dedicated engine; Binnacle handles discovery, invocation, envelope normalization, and doctor healthchecks.
4. **Standardized Envelope Contract**: Every execution emits a structured JSON envelope containing:
   ```json
   {
     "step": 6,
     "tool": "keel",
     "binary": "target/release/keel.exe",
     "exit_code": 0,
     "passed": true,
     "stdout": "...",
     "stderr": "..."
   }
   ```

---

## Where to Start

If you are new to Binnacle or looking for your first contribution:

- Browse open issues with the [**good first issue**](https://github.com/TechTideOhio/binnacle/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) or [**help wanted**](https://github.com/TechTideOhio/binnacle/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22) labels.
- Join the conversation on [**GitHub Discussions**](https://github.com/TechTideOhio/binnacle/discussions) to propose ideas, discuss evaluation metrics, or ask questions before opening a PR.

---

## Development Setup

### Prerequisites
- **Rust**: 1.82+ (2021 edition) via `rustup`
- **Cargo**: Standard package manager included with Rust

### Clone and Build
```bash
git clone https://github.com/TechTideOhio/binnacle.git
cd binnacle

# Build debug binary
cargo build

# Run unit and integration tests
cargo test

# Run strict linter gate
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt -- --check
```

### Local Testing & Healthchecks
```bash
# Print the 15-tool evaluation table
cargo run -- table

# Inspect tool health and binary resolution
cargo run -- doctor

# Run a specific tool with passthrough arguments
cargo run -- run keel --help
```

---

## How to Add or Modify Tool Discovery

Tool steps and discovery paths are declared in [`src/lib.rs`](src/lib.rs):
- `STEPS`: Table mapping step numbers (1–15), step names, and tool identifiers.
- `find_binary`: Resolves built binaries under local `target/{release,debug}`, workspace directories (`Tools/<tool>`), sibling directories (`../<tool>`), cargo install locations (`~/.cargo/bin`), and system `PATH`.
- `find_tool_dir`: Resolves the root directory of a tool to ensure relative fixture paths resolve seamlessly during execution.

If proposing an additional evaluator or extending the CLI:
1. Ensure the binary name and step identifier align with existing naming conventions.
2. Add comprehensive unit tests in [`tests/steps.rs`](tests/steps.rs).
3. Verify that `cargo test` and `cargo clippy --all-targets -- -D warnings` pass cleanly.

---

## Pull Request Workflow

1. **Fork and Branch**:
   Create a focused feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Commit Conventions**:
   We follow standard [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` New capability or CLI enhancement
   - `fix:` Bug fix or path resolution repair
   - `docs:` Documentation improvements
   - `chore:` Dependency maintenance or CI workflows
   - `test:` Additional tests or test fixtures

3. **Verify Locally Before Submitting**:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt -- --check
   ```

4. **Open a Pull Request**:
   - Provide a concise summary of the motivation, changes, and verification evidence.
   - Link any related issues (e.g. `Fixes #12`).
   - CI will automatically execute formatting checks, test suites, and linter gates across Windows, Linux, and macOS runners.

---

## Questions or Ideas?

Feel free to open an issue or start a thread in [**GitHub Discussions**](https://github.com/TechTideOhio/binnacle/discussions). We review submissions promptly and welcome collaboration from across the community!
