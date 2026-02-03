# Windows Winget Packaging Guide

This guide describes how to prepare RustyPing for Windows Package Manager (Winget).

## Packaging Steps

1. **Calculate SHA256**:
   ```powershell
   Get-FileHash rustyping-windows-v2.4.4.zip -Algorithm SHA256
   ```

2. **Update Manifest**:
   Edit [RustyPing.yaml](file:///c:/RustyPing/packaging/windows/RustyPing.yaml):
   - Update `PackageVersion: 2.4.4`
   - Update `InstallerUrl` to the new release ZIP.
   - Update `InstallerSha256` with the new hash.

3. **Validation**:
   ```powershell
   winget validate packaging/windows/RustyPing.yaml
   ```
