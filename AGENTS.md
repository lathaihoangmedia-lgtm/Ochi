# Ochi Agent Standard

This file defines a shared execution standard for all AI agents working in this repository (Copilot, Claude Code, AI Builder style agents, and compatible subagents).

## 1) Operating Principles

- Solve tasks end-to-end; do not stop at analysis when implementation is expected.
- Prefer minimal, focused changes over broad refactors.
- Fix root causes instead of masking symptoms.
- Never assume success: always verify with concrete checks.
- Keep outputs concise, actionable, and professional.
- Keep implementation modular and test-friendly across Rust + Go + Rust2Go boundaries.

## 1.1) Size & Modularity Targets

- Prefer files <= 300 lines where practical.
- Prefer functions <= 60 lines.
- Prefer readable line length <= 100 characters.
- Prefer short, direct prose sentences in docs and handoff text.

## 2) Execution Workflow

1. Understand the task and constraints.
2. Inspect relevant files and existing patterns first.
3. Create a short implementation plan for non-trivial changes.
4. Apply code changes in small, reviewable steps.
5. Validate with the narrowest useful checks, then broader checks.
6. Report outcome, risks, and next step.

## 3) Quality Gates

For Rust changes, target this sequence:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If workspace constraints prevent full validation, run the most specific check possible and clearly report blockers.

## 4) Tooling and Safety

- Prefer existing scripts and tools before introducing new ones.
- Do not expose secrets, tokens, or private keys in output.
- Do not modify unrelated files.
- Keep commit messages scoped and meaningful.
- For security-sensitive work, apply `docs/agent-self-protection.md` and `docs/defensive-deterrence-standard.md`.

## 5) Delegation Rules (for multi-agent flows)

- Delegate only specialized sub-problems, not total responsibility.
- Give clear role + scope + acceptance criteria to each subagent.
- Merge sub-results into one coherent final outcome.

## 6) Definition of Done

A task is done only when:

- Requested behavior is implemented.
- Relevant checks pass (or blockers are explicitly documented).
- Changes are easy to review.
- Final handoff includes what changed, validation status, and next action.
