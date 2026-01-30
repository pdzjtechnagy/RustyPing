# Moving RustyPing to GitHub

Follow these steps to upload your local project to your GitHub account (`pdzjtechnagy`).

## 1. Create the Repository on GitHub

1.  Log in to [GitHub.com](https://github.com).
2.  Click the **+** icon in the top-right corner and select **New repository**.
3.  Name the repository: `RustyPing`.
4.  **Important**: Do **NOT** check "Add a README", "Add .gitignore", or "Choose a license". We want an empty repository.
5.  Click **Create repository**.

## 2. Prepare your Local Project

Open your terminal (PowerShell) in the `c:\RustyPing` folder and run the following commands:

```powershell
# Initialize Git
git init

# Configure your Git identity (if you haven't already)
git config --global user.name "pdzjtechnagy"
git config --global user.email "your_email@example.com" 

# Create a .gitignore file (to exclude build artifacts)
# (Copy-paste this entire block)
@"
/target
**/*.rs.bk
*.pdb
*.exe
.idea
.vscode
"@ | Out-File -Encoding UTF8 .gitignore

# Add files to staging
git add .

# Commit the files
git commit -m "Initial commit: RustyPing 2.0 with Speed Test and Port Scanner"
```

## 3. Push to GitHub

Replace `pdzjtechnagy` with your actual username if it's different, but you provided `pdzjtechnagy`.

```powershell
# Rename the default branch to main
git branch -M main

# Add the remote repository
git remote add origin https://github.com/pdzjtechnagy/RustyPing.git

# Push the code
git push -u origin main
```

## 4. Verify

Go to `https://github.com/pdzjtechnagy/RustyPing` in your browser. You should see your code and the README.

## 5. Enable the "One-Liner" Install

Once the code is on GitHub, anyone can install your tool using the following PowerShell command:

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```