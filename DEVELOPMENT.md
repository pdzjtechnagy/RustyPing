# RustyPing Development Guide 🛠️

This guide explains how to continue developing RustyPing and how to push your changes to GitHub.

## 💻 Setting Up on a New Machine

To start coding on a different computer, follow these steps:

### 1. Install Tools
You need **Git**, **Rust**, and a **C++ Linker**.

**Windows (PowerShell):**
```powershell
# Install Git
winget install Git.Git

# Install Rust
winget install Rustlang.Rustup

# Install C++ Build Tools (Required for linking)
# This command automatically selects the required C++ workload so you don't have to click anything!
winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart"
```
*Note: This command runs silently in the background. Give it a few minutes to finish.*

### 2. Clone Your Repo
Download your code from GitHub:

```powershell
# Clone the repository
git clone https://github.com/pdzjtechnagy/RustyPing.git

# Enter the directory
cd RustyPing
```

### 3. Build & Run
Rust handles all dependencies automatically.

```powershell
# This downloads all libraries and builds the app
cargo run
```

---

## 🔄 The Development Cycle

1.  **Make Changes**: Edit your code in `src/`.
2.  **Test Locally**:
    ```powershell
    # Quick syntax check
    cargo check

    # Run the app
    cargo run
    ```
3.  **Commit Changes**:
    ```powershell
    # Stage all changes
    git add .

    # Commit with a message
    git commit -m "Added a cool new feature"
    ```
4.  **Push to GitHub**:
    ```powershell
    git push
    ```
    *Once you push, your changes are live on GitHub.*

## 📦 How to Version

When you make significant changes, you should update the version number.

1.  Open `Cargo.toml`.
2.  Find `version = "2.0.0"`.
3.  Change it (e.g., to `"2.0.1"` or `"2.1.0"`).
4.  Commit and push the change.

## 🐛 Debugging Tips

-   **Linter**: Run `cargo clippy` to catch common mistakes.
-   **Formatting**: Run `cargo fmt` to automatically format your code.

## 🚀 Releasing to Users

Because we use the "One-Liner" installer, **users get the latest code that is on the `main` branch**.

1.  You push code to `main`.
2.  The user runs the installer script again.
3.  The script downloads your latest code, builds it, and replaces their old version.