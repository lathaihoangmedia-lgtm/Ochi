# Git Auto-Sync Setup Guide

## Remote Template

- Remote SSH: `git@github.com:<owner>/<repo>.git`
- Branch: `main`
- Pull strategy: `rebase`

## Quick Commands

```batch
git fetch origin
git pull --rebase origin main
git add -A
git commit -m "your message"
git push origin main
```

## SSH Setup

1. Generate key:
   ```batch
   .\scripts\setup-ssh.bat
   ```
2. Add `%USERPROFILE%\\.ssh\\id_ed25519.pub` to https://github.com/settings/keys
3. Verify:
   ```batch
   ssh -T git@github.com
   ```

## HTTPS Alternative

```batch
git remote set-url origin https://github.com/<owner>/<repo>.git
```

## Git Identity

```batch
git config --global user.name "<your-name>"
git config --global user.email "<your-email@domain.com>"
```
