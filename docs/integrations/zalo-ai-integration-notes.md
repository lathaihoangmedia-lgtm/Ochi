# Zalo AI integration notes for Ochi (`FUNC-01`)

This note captures the currently available Zalo AI API onboarding details and translates them into Ochi integration constraints.

## What is confirmed from current docs

- API host pattern: `https://api.zalo.ai/{{target_api_url}}`
- Required auth header: `apikey: <your_api_key_here>`
- Basic call shape example:

```bash
curl -H "apikey: your_api_key_here" -X POST "https://api.zalo.ai/{{target_api_url}}"
```

## What this means for Ochi runtime

1. Zalo auth is **not** OAuth/Bearer in the provided docs.
2. The HTTP adapter for `zalo_ai` must support custom header name `apikey`.
3. `ZALO_AI_TOKEN` can still be used as env var, but injected into header key `apikey`.

## Still missing to complete NLP provider integration

To implement robust NLP fallback parity with Wit.ai, we still need:
- exact NLP endpoint path (`{{target_api_url}}`) for text understanding
- request contract (query/body fields, locale/context)
- response schema (intents/entities/traits/confidence)
- error schema and retry semantics (4xx/5xx/rate-limit)

Without these fields, only connectivity/auth shape can be prepared; semantic parsing/mapping cannot be finalized.
