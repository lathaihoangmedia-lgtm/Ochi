# Claude Code Instructions for Ochi

This file aligns Claude Code behavior with the shared repository standard.

## Source of Truth

- Follow repository-wide standards in AGENTS.md.
- Follow Copilot/AI Builder workflow constraints in .github/copilot-instructions.md.

## Required Execution Style

- Read relevant code first, then implement.
- Keep changes minimal and scoped.
- Validate before handoff.
- Report blockers with exact command output.

## Validation Baseline

For Rust changes, attempt:

```bash
cargo check --workspace
cargo test --workspace
```

If environment restrictions block full checks, run narrower checks and clearly explain why.

## Handoff Format

Always include:

1. What changed
2. Where changed
3. Validation run and result
4. Remaining risk or next step
