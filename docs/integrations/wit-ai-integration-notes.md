# Wit.ai integration notes for Ochi (`FUNC-01`)

This note converts official Wit.ai HTTP API behavior into an implementation checklist for Ochi runtime integration.

## 1) Minimum viable NLP path (text understanding)

Target endpoint:
- `GET https://api.wit.ai/message`

Required request parts:
- Header: `Authorization: Bearer $WIT_AI_TOKEN`
- Query: `v=YYYYMMDD` (required API version pin)
- Query: `q=<url-encoded user input>`

Optional request parts to support:
- `n` (top intents/traits, max 8)
- `context` (timezone, locale, coords, reference_time)
- `entities` (dynamic entities)

Canonical success response fields to map:
- `text` (debug transcript)
- `intents[]` (sorted by confidence)
- `entities{}`
- `traits{}`

Suggested Ochi normalization:
- `intent` => `intents[0].name` if present
- `confidence` => `intents[0].confidence` if present
- `entities` => pass-through JSON map
- `traits` => pass-through JSON map

## 2) Error + rate-limit handling contract

Documented error HTTP status:
- `400` invalid params / request formatting
- `401` auth invalid or missing
- `408` timeout / slow stream (speech)
- `500` provider internal error
- `503` provider unavailable

Rate limits called out by Wit docs:
- Text endpoints (`/message`, `/event`): up to 2400 req/min/user, 600 req/min/app
- Audio endpoints (`/speech`, `/dictation`, `/converse`): up to 240 req/min/user, 60 req/min/app

Implementation guidance:
- mark `401` as auth error (trigger provider cooldown / key check)
- mark `408/429/5xx` as retriable provider failure
- return body details if provider sends JSON `{ code, error }`

## 3) Future Ochi support (phase 2)

- `POST /speech` for STT + NLU combined (streaming partial/final understanding)
- `POST /dictation` for STT-only transcription
- `POST /converse` and `POST /event` for Composer flows
- `POST /synthesize` for TTS (note: docs indicate English-only currently)

## 4) Open items still needed from Zalo AI

Current known onboarding details from Zalo docs:
- host pattern: `https://api.zalo.ai/{{target_api_url}}`
- auth header: `apikey: <token>`

To implement parity and fallback chain `Wit.ai -> Zalo AI -> Manus Provider -> OpenAI`, we still need:
- exact NLP endpoint(s) and method
- request body/query schema
- response schema for intents/entities/confidence
- rate-limit + error contract

Without the above, Ochi can integrate Wit.ai first but cannot complete robust Zalo fallback semantics.
