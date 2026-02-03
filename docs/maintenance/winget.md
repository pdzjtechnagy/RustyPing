# Windows Winget Packaging Guide

This guide describes how to prepare RustyPing for Windows Package Manager (Winget).

## Prerequisites

- `wingetcreate` tool (optional but recommended)
- A release ZIP file containing the Windows executable (`rustyping.exe`).

## Packaging Steps

1. **Prepare Release ZIP**:
   Zip the `rustyping.exe`.
   ```powershell
   Compress-Archive -Path rustyping.exe -DestinationPath rustyping-windows-v2.4.1.zip
   ```

2. **Calculate SHA256**:
   ```powershell
   Get-FileHash rustyping-windows-v2.1.2.zip -Algorithm SHA256
   ```

3. **Update Manifest**:
   Edit `packaging/windows/RustyPing.yaml`:
   - Update `InstallerUrl` to the new release ZIP URL.
   - Update `InstallerSha256` with the hash.
   - Update `PackageVersion`.

4. **Submission**:
   - The official way is to fork [winget-pkgs](https://github.com/microsoft/winget-pkgs), create a new directory for your package, and submit a PR.
   - Alternatively, you can use `wingetcreate update` to automate this if the package is already in the repo.

## One-Liner Installation (Manual/Portable)

Since Winget submission takes time, users can install manually via PowerShell one-liner:

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/install.ps1 | iex
```
(You would need to host an `install.ps1` script).

Or simply download the exe.
