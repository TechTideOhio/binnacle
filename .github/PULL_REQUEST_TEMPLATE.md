## Description

Provide a clear and concise description of the changes introduced in this pull request.

Fixes #(issue)

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Documentation update
- [ ] Performance improvement / refactor
- [ ] CI / tooling update

## Invariants & Quality Checklist

- [ ] Zero-FFI & Pure Rust: Code contains no unsafe blocks (`#![forbid(unsafe_code)]` enforced).
- [ ] Deterministic & Fast: Sub-second evaluation times preserved.
- [ ] Unit & Integration Tests: New or existing tests cover the change (`cargo test`).
- [ ] Lint & Format Gate: Clean pass with zero warnings (`cargo clippy --all-targets -- -D warnings` and `cargo fmt -- --check`).
- [ ] Healthcheck: Verified with `cargo run -- doctor` and `cargo run -- table`.
