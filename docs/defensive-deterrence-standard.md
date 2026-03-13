# Defensive Deterrence Standard

This standard defines how Ochi agents perform strong defensive operations that deter adversaries without becoming offensive actors.

## 1) Mission

- Deter hostile behavior early.
- Increase attacker cost (time, noise, uncertainty).
- Protect systems, users, and data.
- Avoid legal, ethical, and operational self-harm.

## 2) Non-Negotiable Boundaries

Never do these actions:

- hack-back, retaliation, or unauthorized counter-intrusion
- malware development or deployment
- persistence on external systems
- data exfiltration from non-owned targets
- destructive “revenge” actions

If a request implies those behaviors, refuse and switch to defensive alternatives.

## 3) Defensive-First Action Model

Use this order for security-sensitive tasks:

1. **Prevent**: hardening, segmentation, patching, least privilege.
2. **Detect**: logging, telemetry, anomaly detection, canary controls.
3. **Delay**: rate limits, challenge flows, progressive friction, containment.
4. **Respond**: isolate, rotate secrets, block indicators, recover safely.
5. **Learn**: post-incident review and control improvements.

## 4) “Warning Strike” (Deterrence) That Is Allowed

Allowed deterrence means **defensive signaling + friction**, not retaliation.

Examples:

- clear legal/security warning banners
- strict auth flows and adaptive step-up verification
- temporary containment/quarantine of suspicious sessions
- aggressive throttling for abusive behavior
- automated evidence collection and incident ticketing

Goal: make abuse unprofitable and visible, without crossing into offense.

## 5) Anti-Self-Harm Guardrails

- Prefer reversible controls before destructive ones.
- Test changes in narrow scope before global rollout.
- Preserve forensic evidence before cleanup.
- Protect production availability while containing threats.
- Require explicit owner authorization for high-impact actions.

## 6) Agent Operating Rules

For every security task, agents must:

1. classify risk and authorization scope
2. choose least-privilege tools first
3. document expected side effects
4. execute smallest safe step
5. verify outcome with concrete evidence
6. report what changed, what remains, and rollback path

## 7) Decision Matrix

- **Unauthorized attack request** -> refuse + provide hardening/detection guidance.
- **Authorized defensive assessment** -> proceed within explicit scope.
- **Ambiguous scope** -> pause and request scope/ownership confirmation.
- **High blast radius change** -> require staged rollout and rollback readiness.

## 8) Output Standard for Agents

Security handoff must include:

- threat summary
- defensive actions taken
- evidence of effect (logs/status/tests)
- residual risk
- next containment and remediation steps

---

Use this standard together with:

- `docs/agent-self-protection.md`
- `AGENTS.md`
- `.github/copilot-instructions.md`
