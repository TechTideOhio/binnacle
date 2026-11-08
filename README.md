<div align="center">

# binnacle: TideOS Eval Kit (The Evaluator Agent)

**The unified, zero-FFI, pure-Rust evaluator agent orchestrating the complete 15-tool TideOS evaluation stack across autonomous coding agents.**

[![CI](https://github.com/TechTideOhio/binnacle/actions/workflows/ci.yml/badge.svg)](https://github.com/TechTideOhio/binnacle/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/TechTideOhio/binnacle?color=blue&label=release)](https://github.com/TechTideOhio/binnacle/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
[![Rust: 2021 / 1.82+](https://img.shields.io/badge/Rust-2021%20%2F%201.82%2B-orange.svg)](Cargo.toml)
[![Zero-FFI](https://img.shields.io/badge/Dependencies-Zero--FFI%20Pure%20Rust-purple.svg)](Cargo.toml)

[Quickstart](#60-second-quickstart) | [Why binnacle?](#why-binnacle) | [The 15 Checklist Steps](#the-15-evaluation-steps) | [Tool Inventory](#the-complete-tideos-eval-kit) | [Architecture](#architectural-foundations) | [Contributing](CONTRIBUTING.md)

</div>

---

## Why binnacle?

Evaluation is the single most critical and compounding capability in modern AI systems engineering. Prompts are disposable; models are replaced on 90-day cycles; your evaluation harness and ground-truth regression suite are what compound.

In traditional stacks, agent evaluations are fragmented across brittle Python scripts, ad-hoc LangSmith callbacks, non-deterministic model judges, and unverified CI scripts. `binnacle` serves as the compass and helm for agentic evaluation:
- **Zero-FFI, Pure Rust**: Fast sub-second runs, compile-time safety, zero Python runtime dependencies or C library bindings.
- **Fail-Closed Gatekeeper**: Dispatches to dedicated, mathematically bounded evaluation engines (LCS sequence alignment, SimHash deduplication, exact McNemar tests, SplitMix64 bootstrap CIs).
- **Uniform CLI & Envelope Contract**: Standardized JSON envelopes across all 15 stages of evaluation.

## Architecture & Ecosystem

`binnacle` is designed around the Unix philosophy of small, single-purpose, decoupled tools:

- **The Orchestrator (`binnacle`)**: A lightweight supervisor CLI (`binnacle-eval`) that discovers installed engines, conducts healthchecks (`binnacle doctor`), executes evaluation steps (`binnacle run <tool>`), and normalizes results into standardized JSON envelopes.
- **The 15 Evaluation Engines**: Each tool (`reval`, `gild`, `tessel`, `assay`, `keel`, etc.) is an independent, zero-FFI, pure-Rust crate maintained in its own dedicated repository under the [`TechTideOhio`](https://github.com/TechTideOhio) organization.
- **Zero-Coupling Discovery**: `binnacle` does not vendor or statically link the engines. It dynamically resolves their binaries from your system `PATH`, Cargo bin (`~/.cargo/bin`), or local workspace directories, allowing you to use any tool standalone in CI or pre-commit hooks.

```mermaid
flowchart TD
    AGENT["Autonomous Agent Run\n(Claude, Codex, OpenCode, Cursor, Muse)"] --> BINNACLE["binnacle CLI (Orchestrator)\ngithub.com/TechTideOhio/binnacle"]
    
    BINNACLE -->|"Discovers & Invokes"| ENGINES
    
    subgraph ENGINES ["15 Standalone Rust Evaluation Engines (Independent Repositories)"]
        direction TB
        E1["reval ➔ github.com/TechTideOhio/reval"]
        E2["gild ➔ github.com/TechTideOhio/gild"]
        E3["tessel ➔ github.com/TechTideOhio/tessel"]
        E4["assay ➔ github.com/TechTideOhio/assay"]
        E5["tide-rill ➔ github.com/TechTideOhio/tide-rill"]
        E6["keel ➔ github.com/TechTideOhio/keel"]
        E7["temper ➔ github.com/TechTideOhio/temper"]
        E8["parley ➔ github.com/TechTideOhio/parley"]
        E9["gavel ➔ github.com/TechTideOhio/gavel"]
        E10["poise ➔ github.com/TechTideOhio/poise"]
        E11["plumb ➔ github.com/TechTideOhio/plumb"]
        E12["weir ➔ github.com/TechTideOhio/weir"]
        E13["eddy ➔ github.com/TechTideOhio/eddy"]
        E14["tare ➔ github.com/TechTideOhio/tare"]
        E15["tether ➔ github.com/TechTideOhio/tether"]
    end

    ENGINES --> VERDICT{"Deterministic Gate"}
    VERDICT -->|"Pass (exit 0)"| DEPLOY["Ship to Production / CI Green"]
    VERDICT -->|"Fail (exit 1/2)"| HARVEST["tether capture\n(Harvest Failure to Gold Set)"]
```

---

## The 15 Evaluation Steps

> [!NOTE]
> **Architecture: Orchestrator vs Engines**  
> `binnacle` follows the Unix philosophy of small, sharp tools. It is the **conductor and CLI supervisor**, not a monolithic codebase. Each of the 15 evaluation engines is a completely independent, zero-FFI, pure-Rust repository under the [`TechTideOhio`](https://github.com/TechTideOhio) organization. You can run any tool standalone in your pre-commit hooks, or use `binnacle` to discover, healthcheck, and orchestrate them together.

`binnacle` routes directly to the specialized TideOS evaluation tool responsible for each step of production AI engineering:

| # | Step | Tool | Dedicated Repository | Core Engine & Invariants |
|---|---|---|---|---|
| **1** | **Error Analysis** | [`reval`](https://github.com/TechTideOhio/reval) | [TechTideOhio/reval](https://github.com/TechTideOhio/reval) | Open/axial failure coding, causal Bayesian trace DAGs, OTel export. |
| **2** | **Golden Dataset** | [`gild`](https://github.com/TechTideOhio/gild) | [TechTideOhio/gild](https://github.com/TechTideOhio/gild) | Stratified ground-truth authoring, schema validation, cryptographic versioning. |
| **3** | **Synthetic Dimensions** | [`tessel`](https://github.com/TechTideOhio/tessel) | [TechTideOhio/tessel](https://github.com/TechTideOhio/tessel) | Covering arrays ($P \times I \times C \times E$) with pairwise mathematical coverage guarantees. |
| **4** | **Deterministic Assertions** | [`assay`](https://github.com/TechTideOhio/assay) | [TechTideOhio/assay](https://github.com/TechTideOhio/assay) | JSON Schema, JSON Pointer, zero-variance regex, pure-Rust in-memory DB/RLS. |
| **5** | **Retrieval vs Generation** | [`tide-rill`](https://github.com/TechTideOhio/tide-rill) | [TechTideOhio/tide-rill](https://github.com/TechTideOhio/tide-rill) | Separate recall@k, MRR, nDCG from context claim lexical support rates and conflict gates. |
| **6** | **Trajectory Evals** | [`keel`](https://github.com/TechTideOhio/keel) | [TechTideOhio/keel](https://github.com/TechTideOhio/keel) | Tool-call precision/recall, LCS order alignment, step efficiency, 14 vs 3 step cost bug detection. |
| **7** | **pass@k vs pass^k** | [`temper`](https://github.com/TechTideOhio/temper) | [TechTideOhio/temper](https://github.com/TechTideOhio/temper) | Multi-trial capability vs production reliability, real sampling temperature checks. |
| **8** | **Simulated Users** | [`parley`](https://github.com/TechTideOhio/parley) | [TechTideOhio/parley](https://github.com/TechTideOhio/parley) | Multi-turn stateful conversational eval, hidden goals, sycophancy resistance. |
| **9** | **Judge Alignment** | [`gavel`](https://github.com/TechTideOhio/gavel) | [TechTideOhio/gavel](https://github.com/TechTideOhio/gavel) | Binary classifier calibration, TPR/TNR reporting, Rogan-Gladen prevalence debiasing. |
| **10** | **Pairwise Judge Bias** | [`poise`](https://github.com/TechTideOhio/poise) | [TechTideOhio/poise](https://github.com/TechTideOhio/poise) | Order-swapped double-blind adjudication, length-skew normalization. |
| **11** | **Paired Statistics** | [`plumb`](https://github.com/TechTideOhio/plumb) | [TechTideOhio/plumb](https://github.com/TechTideOhio/plumb) | SplitMix64 BCa bootstrap CIs, exact McNemar test, TOST equivalence gating. |
| **12** | **CI Gating** | [`weir`](https://github.com/TechTideOhio/weir) | [TechTideOhio/weir](https://github.com/TechTideOhio/weir) | Per-category non-compensatory regression prevention in pull requests. |
| **13** | **Online Production** | [`eddy`](https://github.com/TechTideOhio/eddy) | [TechTideOhio/eddy](https://github.com/TechTideOhio/eddy) | Deterministic basis-point hash sampling, async judge pools, CUSUM drift detection. |
| **14** | **Cost & Latency** | [`tare`](https://github.com/TechTideOhio/tare) | [TechTideOhio/tare](https://github.com/TechTideOhio/tare) | Tokens per request, Cost Per Successful Task (CPST), tail latency (p50/p95/p99) Pareto gates. |
| **15** | **Close the Loop** | [`tether`](https://github.com/TechTideOhio/tether) | [TechTideOhio/tether](https://github.com/TechTideOhio/tether) | Secret-scrubbed failure harvesting, SimHash deduplication, fail-closed model release replay. |

---

## Installation

### Option 1: Cargo Install (Recommended for Rust Developers)
```bash
cargo install --git https://github.com/TechTideOhio/binnacle
```

### Option 2: Pre-Built Release Binaries
Pre-compiled standalone binaries for Windows (`x86_64`), macOS (`x86_64`), and Linux (`x86_64`) are available on the [Latest Releases](https://github.com/TechTideOhio/binnacle/releases/latest) page.

#### Windows (PowerShell)
```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.ps1 -OutFile install.ps1
powershell -ExecutionPolicy Bypass -File install.ps1
```

#### macOS / Linux (Bash)
```bash
curl -fsSL https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.sh -o install.sh
bash install.sh
```

### Option 3: Build from Source
```bash
git clone https://github.com/TechTideOhio/binnacle.git
cd binnacle
cargo build --release
# Binary is available at target/release/binnacle
```

---

## 60-Second Quickstart

### Inspect the Evaluation Table
```bash
binnacle table
```

### Healthcheck All 15 Tools
```bash
binnacle doctor
```
> **Tip**: `binnacle doctor` automatically scans local directories, `Tools/<tool>`, `../<tool>`, `~/.cargo/bin`, and your system `PATH`. You can also supply a custom root directory with `--root <path>`.

### Execute a Step or Tool
```bash
# Evaluate an agent trajectory (Step 6 / keel)
binnacle run keel check --suite packs/softfactory.toml --root .

# Evaluate retrieval and faithfulness floors (Step 5 / tide-rill)
binnacle run tide-rill gate examples/synthetic.json examples/thresholds-pass.json

# Check cost-latency Pareto trade-offs (Step 14 / tare)
binnacle run tare table

# Print a single envelope field instead of the full JSON (no jq needed)
binnacle run --pointer /passed keel check --suite packs/softfactory.toml --root .
```
> binnacle's own flags go between `run` and the tool name; everything after the tool name is passed to the tool unchanged.

---

## Architectural Foundations

1. **Zero External FFI & Zero Unsafe Code**: Every tool in the evaluation suite compiles under `#![forbid(unsafe_code)]` with zero C-bindings or foreign library dependencies.
2. **Sub-Second Execution**: Deterministic evaluations execute in single-digit milliseconds, allowing entire suites to run locally on pre-commit hooks and in CI.
3. **Decoupled Evaluator Roles**:
   - **Deterministic Systems**: Assertion checkers (`assay`), trajectory alignment (`keel`), and statistical significance (`plumb`) run without LLM API latency or stochastic variance.
   - **Orchestration Plane**: Clean separation of concern where `binnacle` executes as a thin supervisor over dedicated Unix-philosophy CLI binaries.

---

## License

MIT License. Copyright (c) 2026 Alex & TechTide contributors.
