# RustyPing Audit Tool Usage Documentation

## Overview
This document details the diagnostic toolset used to ensure the stability, security, and efficiency of RustyPing. These tools should be run before every major release.

---

## 1. cargo-deny (v0.19.0)
**Purpose**: Security, License, and Dependency Auditing.

### **Installation**
```powershell
cargo install --locked cargo-deny
```

### **Usage Guide**
1.  **Check Advisories**: `cargo deny check advisories`
    - Scans `Cargo.lock` against the [RustSec Advisory Database](https://rustsec.org/).
    - **Action**: If a vulnerability is found, run `cargo update <package>` to see if a fix is available.
2.  **Check Licenses**: `cargo deny check licenses`
    - Ensures all dependencies use licenses compatible with the project.
    - **Action**: If a license is rejected, verify it in `deny.toml` and add it to the `allow` list if safe.

---

## 2. cargo-bloat (v0.12.1)
**Purpose**: Binary Size Analysis.

### **Installation**
```powershell
cargo install cargo-bloat
```

### **Usage Guide**
1.  **Top Bloat**: `cargo bloat --release --n 20`
    - Shows the 20 largest functions in the final executable.
2.  **Crate Analysis**: `cargo bloat --release --crates`
    - Shows how much space each library (e.g., `tokio`, `ratatui`) contributes.

---

## 3. cargo-outdated (v0.16.0)
**Purpose**: Identifying Stale Dependencies.

### **Installation**
```powershell
cargo install cargo-outdated
```

### **Usage Guide**
1.  **Check Status**: `cargo outdated -d 1`
    - Shows dependencies that have newer versions available.
    - **Action**: Evaluate "Latest" vs "Compat" versions. Significant jumps (e.g., `0.12` to `0.13`) should be tested in a separate branch.

---

## 4. cargo-nextest (v0.9.114)
**Purpose**: High-Performance Testing.

### **Installation**
```powershell
cargo install cargo-nextest --version 0.9.114 --locked
```

### **Usage Guide**
1.  **Run All Tests**: `cargo nextest run`
    - Runs tests in parallel with a cleaner UI than `cargo test`.
    - **Interpretation**: If a test fails, `nextest` provides a detailed log of that specific failure without noise from other tests.

---

## 5. cargo-clippy (Native)
**Purpose**: Static Analysis / Linting.

### **Usage Guide**
1.  **Strict Audit**: `cargo clippy --all-targets --all-features -- -D warnings`
    - Treats all lints as errors. This is the gold standard for production code.
    - **Action**: Fix all "errors" reported by Clippy before committing.
