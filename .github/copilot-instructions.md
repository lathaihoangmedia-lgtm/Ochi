# Copilot Instructions for Ochi

This repository follows a high-consistency execution style inspired by prior Draft repositories.

## Scope

- Primary stack: Rust + Go with Rust2Go interoperability.
- Rust workspace centered on `crates/ochi-core` (current active member).
- Favor practical MVP progress with clear verification.

## Architecture Discipline (Rust + Go + Rust2Go)

- Keep boundaries clear: Rust core logic in crates, Go integration adapters thin and explicit.
- Split features by crate/module responsibility; avoid monolithic files.
- Prefer small modules that are easy to compile and test independently.
- For Rust2Go bridge code, keep FFI surface minimal and typed; avoid leaking transport details into domain logic.

## Size & Readability Constraints

- Prefer short, direct sentences in generated docs/messages (target <= 25 words per sentence).
- Keep code line length readable (target <= 100 characters; avoid long chained expressions).
- Keep module/file size lightweight (target <= 300 lines per file where practical).
- Keep functions focused (target <= 60 lines; extract helpers when growing).

## Default Working Style

1. Read before writing: inspect files and patterns first.
2. Plan for non-trivial tasks (short, ordered steps).
3. Implement with minimal surface area.
4. Validate changes with project-appropriate commands.
5. Summarize outcomes and blockers clearly.

## Execution Consent Mode

- Default at task start: **auto-allow** for read-only and diagnostic work.
- Auto-allow scope includes: listing/searching files, reading docs/code, dependency inspection, and non-mutating checks/tests.
- Before any state-changing action, require explicit user confirmation.
- State-changing actions include: file edits/creation/deletion, dependency install/upgrade, git commit/push/rebase/reset, environment/config changes, and destructive shell commands.
- If a command is mixed (read + write), treat it as state-changing and ask first.

## Coding Rules

- Match existing style and naming.
- Avoid speculative abstractions.
- Avoid one-letter variable names.
- Keep public API changes intentional and explicit.
- Do not add unrelated formatting or refactors.

## Validation Rules

For Rust updates, attempt:

```bash
cargo check --workspace
cargo test --workspace
```

If full workspace checks are blocked by local environment constraints (permissions, missing tools, etc.), run narrower checks and report the exact blocker.

For Go or Rust2Go-related updates, also attempt the narrowest relevant checks available (module/package-level first), then broader checks.

## Testing Precision Rules

- Start with the smallest impacted crate/module/package tests.
- Prefer deterministic tests over broad end-to-end runs when narrowing regressions.
- Only expand to wider test scope after local target checks pass.
- In reports, include exactly what was tested and what was intentionally not tested.

## Prompt & Skill Discipline

- Reuse existing prompt patterns before inventing new ones.
- Prefer explicit instruction blocks: role, task, constraints, output format.
- Keep skill files deterministic and reusable.
- Add new skills under `skills/<skill-name>/SKILL.md` with concise frontmatter and practical content.

## Multi-Agent Consistency

- Use the same definition of done across agents.
- Require verification evidence, not assumptions.
- Keep handoff format stable: what changed, where, validation, next step.

## Self-Protection Baseline

- Apply `docs/agent-self-protection.md` for all security-sensitive flows.
- Apply `docs/defensive-deterrence-standard.md` for active defensive deterrence flows.
- Prefer defensive outputs and remediation guidance over offensive instruction.
- Require explicit authorization and bounded scope for penetration testing tasks.
