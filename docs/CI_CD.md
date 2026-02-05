# 🚀 RustyPing CI/CD Pipeline

RustyPing uses GitHub Actions for automated multi-platform builds and releases. This ensures that every release is consistently compiled and packaged for a wide variety of Linux distributions and processor architectures.

## 🛠️ Build Infrastructure

The pipeline is defined in `.github/workflows/build.yml` and utilizes the following tools:
- **GitHub Actions**: Orchestrates the build matrix and release process.
- **Cross**: A Docker-based tool for cross-compiling Rust projects to multiple targets without needing native hardware.
- **Cargo-Deb**: Generates `.deb` packages for Debian, Ubuntu, Kali, and other Debian-based distros.
- **Cargo-Generate-RPM**: Generates `.rpm` packages for Fedora, RHEL, CentOS, and OpenSUSE.

## 🌍 Supported Platforms & Architectures

RustyPing is precompiled for the following environments:

### Linux
| Architecture | Target Triple | Package Formats |
| :--- | :--- | :--- |
| **x86_64** (AMD64) | `x86_64-unknown-linux-gnu` | `.deb`, `.rpm`, `.tar.gz` |
| **x86_64** (Static) | `x86_64-unknown-linux-musl` | `.tar.gz` (Zero dependencies) |
| **ARM64** (aarch64) | `aarch64-unknown-linux-gnu` | `.deb`, `.rpm`, `.tar.gz` |
| **PowerPC 64 LE** | `powerpc64le-unknown-linux-gnu` | `.tar.gz` |
| **RISC-V 64** | `riscv64gc-unknown-linux-gnu` | `.tar.gz` |

### Windows
| Architecture | Target Triple | Package Formats |
| :--- | :--- | :--- |
| **x86_64** (AMD64) | `x86_64-pc-windows-msvc` | `.exe` |
| **x86** (32-bit) | `i686-pc-windows-msvc` | `.exe` |
| **ARM64** | `aarch64-pc-windows-msvc` | `.exe` |

## 📦 Installer Generation

The build process automatically generates several types of installers:

1.  **DEB Packages (`.deb`)**:
    - For: Ubuntu, Debian, Pop!_OS, Mint, Kali Linux.
    - Installation: `sudo apt install ./rustyping_amd64.deb`

2.  **RPM Packages (`.rpm`)**:
    - For: Fedora, Red Hat Enterprise Linux, CentOS, AlmaLinux, Rocky Linux.
    - Installation: `sudo dnf install ./rustyping.rpm`

3.  **Static Binaries (`.tar.gz`)**:
    - For: Any Linux distribution (including Arch Linux, Alpine, Slackware).
    - These are "portable" versions that include all necessary libraries.
    - Installation: Extract and move to `/usr/local/bin`.

## 🔄 Release Workflow

1.  **Trigger**: A new git tag starting with `v` (e.g., `v2.7.0`) is pushed.
2.  **Compile**: GitHub Actions spins up multiple runners to build all targets in parallel.
3.  **Package**: The binaries are wrapped into `.deb`, `.rpm`, and `.tar.gz` files.
4.  **Publish**: All generated artifacts are automatically uploaded to the GitHub Release page as assets.
5.  **Notify**: The release notes are automatically generated from the commit history.

## 🧪 Testing and Validation

Every build undergoes automated testing:
- **Unit Tests**: `cargo test` is run on all platforms.
- **Binary Integrity**: The pipeline verifies that the generated binaries can be executed and report their version correctly.
- **Package Verification**: Installer metadata is checked for consistency.
