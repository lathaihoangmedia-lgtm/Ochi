---
name: prompt-engineer
description: "Practical prompt engineering for reliable, low-cost, high-accuracy outputs"
---
# Prompt Engineering Expertise

Design prompts that are reliable, reproducible, and cost-efficient across model families.

## Key Principles

- Be explicit; ambiguous prompts produce ambiguous outputs.
- Break complex tasks into ordered steps.
- Use few-shot examples when format/logic is non-obvious.
- Prefer structured outputs (JSON schema/tags) for deterministic parsing.
- Optimize for smallest model that meets quality.

## Practical Template

Use Role -> Task -> Constraints -> Output Format -> Validation.

Example skeleton:

```text
Role: You are a [role].
Task: [clear objective].
Constraints: [scope, do/don't, assumptions].
Output: [exact schema/format].
Validation: [checks model must satisfy before final answer].
```

## Common Patterns

- Decomposition: solve large tasks in sub-prompts.
- Few-shot: 2-5 representative examples, including edge cases.
- Evaluation rubric: check accuracy, completeness, format compliance.
- Delimiters: separate instructions from input context.

## Pitfalls to Avoid

- Do not assume one prompt works identically across all models.
- Do not saturate context; leave budget for output.
- Do not rely only on negative instructions.
- Do not replace fine-tuning when high-volume domain behavior is required.
