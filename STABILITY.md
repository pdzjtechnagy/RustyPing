# RustyPing Stability & Diagnostics Guide

This document outlines the procedures for diagnostic logging, stability maintenance, and the "Expert Plan" for ensuring high-quality code in the RustyPing project.

## 1. Log Retrieval Instructions

### **Application Logs (Runtime)**
RustyPing uses the `tracing` ecosystem for structured logging.
- **Local Development**: Run with `RUST_LOG=debug cargo run` to see real-time packet-level diagnostics in the console.
- **Persistent Logs**: By default, logs are written to `rustyping.log` in the application directory when the `--verbose` flag is used.
- **Retrieval**: 
  - Windows: `Get-Content .\rustyping.log -Wait`
  - Linux: `tail -f rustyping.log`

### **CI/CD Build Logs (GitHub Actions)**
- **Location**: [GitHub Actions Tab](https://github.com/pdzjtechnagy/RustyPing/actions)
- **Retrieval Method**: 
  1. Click on the latest workflow run.
  2. Scroll down to the **"Summary"** section.
  3. Look for **"Build Failure Analysis"**. This section extracts actual compiler/linker errors from the `build_full.log` file generated during the cross-compilation process.

---

## 2. Diagnostic Tools & Methodologies

### **Tools**
- **`cross`**: Essential for multi-platform stability. It runs builds inside Docker containers pre-configured with the correct cross-compilers for ARM, RISC-V, and PowerPC.
- **`cargo clippy`**: Our primary static analysis tool. Use it to catch "code smells" before they become bugs.
- **`cargo audit`**: Scans for security vulnerabilities in the dependency tree (`Cargo.lock`).
- **`tokio-console`**: (Optional) For debugging async task contention in the network stack.

### **Methodologies**
- **Structured Tracing**: Every network packet and state transition in [ping.rs](src/network/ping.rs) is instrumented. Use `tracing::instrument` to automatically track function execution context.
- **"Fail-Fast" CI**: Our pipeline is configured to test all architectures in parallel. If one architecture fails (e.g., RISC-V), it provides immediate feedback without stopping the others.
- **Integration Testing with Mocking**: Use the `#[ignore]` attribute for tests requiring raw socket permissions in restricted environments (like CI) but keep them available for local `sudo` testing.

---

## 3. Recommended IDE Extensions (VS Code/Trae)

To maintain stability during development, the following extensions are recommended:
1. **rust-analyzer**: The gold standard for Rust. Provides real-time type checking and error detection.
2. **Even Better TOML**: For managing `Cargo.toml` and ensuring dependency versions are valid.
3. **GitHub Actions**: Allows you to monitor CI build logs directly inside the IDE.
4. **Code Spell Checker**: Prevents typos in variable names and documentation which can lead to confusing logs.
5. **Error Lens**: Highlights errors and warnings directly in the line where they occur, making them impossible to miss.

---

## 4. The Stability Plan (Expert Protocol)

To ensure RustyPing reaches and maintains a "Production Grade" stable state, follow this 4-step protocol:

### **Phase 1: The Local Check-In (Pre-Push)**
Never push directly to `main`. Always run the following locally:
```powershell
# Run our local check-in suite
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

### **Phase 2: The "Multi-Arch" Gate (CI)**
Monitor the GitHub Actions. A push is only considered "stable" when:
- [x] Windows x86_64 compiles.
- [x] Linux (GNU & Musl) compiles.
- [x] ARM64 & RISC-V linkers successfully resolve.

### **Phase 3: Automated Versioning**
Use `automate_release.ps1` to handle version bumps. This script ensures that version numbers are synchronized across 10+ files, preventing "Version Mismatch" bugs during asset discovery.

### **Phase 4: Continuous Auditing**
Weekly runs of `cargo update` followed by `cargo audit` to ensure that network-critical libraries (like `tokio` or `reqwest`) are patched against security vulnerabilities.
