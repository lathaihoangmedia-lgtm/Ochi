# Copilot Instructions for Ochi

This repository follows a high-consistency execution style inspired by prior Draft repositories.

## Scope

- Rust workspace centered on `crates/ochi-core` (current active member).
- Favor practical MVP progress with clear verification.

## Default Working Style

1. Read before writing: inspect files and patterns first.
2. Plan for non-trivial tasks (short, ordered steps).
3. Implement with minimal surface area.
4. Validate changes with project-appropriate commands.
5. Summarize outcomes and blockers clearly.

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
- Prefer defensive outputs and remediation guidance over offensive instruction.
- Require explicit authorization and bounded scope for penetration testing tasks.
