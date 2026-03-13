# Defensive Deterrence Standard

Purpose: deter attackers by raising cost while staying strictly defensive.

## Non-Negotiable Boundaries

Never do: hack-back, retaliation, malware, external persistence, unauthorized exfiltration, destructive revenge.

If a request implies those actions, refuse and switch to defensive guidance.

## Defensive Action Order

1. Prevent: hardening, patching, segmentation, least privilege.
2. Detect: telemetry, logging, anomaly/canary controls.
3. Delay: throttling, challenge flows, containment friction.
4. Respond: isolate, rotate secrets, block indicators, recover.
5. Learn: post-incident improvements.

## Allowed Deterrence (No Offense)

- legal/security warning banners
- step-up authentication and stricter session controls
- temporary quarantine of suspicious activity
- aggressive abuse throttling
- automated evidence collection and ticketing

## Anti-Self-Harm Guardrails

- prefer reversible controls first
- stage high-impact changes with rollback readiness
- preserve evidence before cleanup
- keep production availability as a constraint
- require explicit owner authorization for high blast-radius actions

## Decision Matrix

- unauthorized attack request -> refuse + provide hardening/detection steps
- authorized defensive assessment -> proceed in explicit scope
- ambiguous scope -> pause and ask for scope/ownership confirmation

## Required Security Handoff

- threat summary
- defensive actions taken
- evidence of effect
- residual risk
- next containment/remediation step

---

Use this standard together with:

- `docs/agent-self-protection.md`
- `AGENTS.md`
- `.github/copilot-instructions.md`
