# Audit Process Standard Operating Procedure (SOP)

## Purpose
To establish a repeatable, high-precision methodology for auditing RustyPing codebases, ensuring zero critical vulnerabilities reach production and maintaining a high standard of code quality.

---

## 1. Pre-Audit Preparation
1.  **Environment Isolation**: Ensure the audit is performed on a clean branch (e.g., `audit/vX.Y.Z`).
2.  **Tool Verification**: Run `cargo deny --version` and `cargo clippy --version` to ensure tools are present.
3.  **Dependency Refresh**:
    ```powershell
    cargo fetch
    cargo update
    ```

## 2. Execution Sequence
The audit must follow this specific order to minimize wasted effort:

| Step | Action | Tool | Success Criteria |
| :--- | :--- | :--- | :--- |
| 1 | **Security Gate** | `cargo deny check advisories` | Zero vulnerabilities. |
| 2 | **Static Analysis** | `cargo clippy` | Zero warnings/errors. |
| 3 | **License Check** | `cargo deny check licenses` | All licenses approved. |
| 4 | **Test Suite** | `cargo nextest run` | 100% pass rate. |
| 5 | **Bloat Audit** | `cargo bloat` | Size < 3.0MB for release build. |

## 3. Manual Review Guidelines
Automated tools cannot catch logic errors. Every audit must include a manual review of:
1.  **Error Handling**: Ensure `anyhow::Result` is used consistently and no `unwrap()` calls exist in the network stack.
2.  **Platform Gates**: Verify `#[cfg(windows)]` and `#[cfg(unix)]` blocks correctly isolate platform-specific APIs (e.g., raw sockets).
3.  **Resource Leaks**: Check that background tasks (tokio spawns) are properly managed and terminated.

## 4. Issue Management
1.  **Severity Levels**:
    - **Critical**: Remote crash potential, security leak, or data corruption.
    - **High**: Functional bug in core feature (Ping/Speedtest).
    - **Medium**: Dependency risk, UI glitch, or minor performance issue.
    - **Low**: Code style, documentation, or non-critical update available.
2.  **Tracking**: All issues found during the audit must be documented in `AUDIT_REPORT.md` before being moved to the GitHub Issue tracker.

## 5. Post-Audit Verification
After fixes are implemented:
1.  Re-run Step 1 and 2 of the Execution Sequence.
2.  Verify the fix on both **Windows** and **Linux** (via `cross` or local WSL).
3.  Sign off on the `AUDIT_REPORT.md`.
