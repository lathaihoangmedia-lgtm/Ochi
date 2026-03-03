# SDK Migration Guide: OpenFang → Ochi

This guide explains how to migrate SDK usage from legacy OpenFang naming to the new Ochi naming while preserving backward compatibility.

> TL;DR: prefer Ochi package/module names in all new code. Legacy OpenFang names still work during the transition window.

---

## Scope

This migration guide covers:

- JavaScript package rename: `@openfang/sdk` → `@ochi/sdk`
- Python module rename guidance: prefer `ochi_client` and `ochi_sdk`
- Compatibility aliases still available for existing code
- Publish and release checklist for SDK maintainers

---

## JavaScript SDK

### Package rename

| Legacy | Current (preferred) |
|---|---|
| `@openfang/sdk` | `@ochi/sdk` |

### Install

```bash
npm install @ochi/sdk
```

### Import migration

Prefer `Ochi` in new code:

```js
const { Ochi } = require("@ochi/sdk");
```

Compatibility alias remains available:

```js
const { OpenFang } = require("@ochi/sdk");
```

### Quick codemod hints

- Replace `@openfang/sdk` with `@ochi/sdk`
- Replace type/class references from `OpenFang` to `Ochi` (optional but recommended)
- Keep `OpenFang` only if you need to minimize churn in a large legacy codebase

---

## Python SDK

Python distribution currently ships both new and legacy module names for compatibility.

### Preferred imports

```python
from ochi_client import Ochi
from ochi_sdk import Agent
```

### Legacy imports still supported

```python
from openfang_client import OpenFang
from openfang_sdk import Agent
```

### Migration notes

- For application code that calls Ochi over HTTP/SSE, migrate to `ochi_client`.
- For agent implementation code, migrate to `ochi_sdk`.
- Existing code using `openfang_client` / `openfang_sdk` can continue running during the transition window.

---

## Compatibility Matrix

| Surface | Preferred | Legacy alias support |
|---|---|---|
| JS package | `@ochi/sdk` | `OpenFang` export alias in package API |
| Python HTTP client module | `ochi_client` | `openfang_client` |
| Python agent SDK module | `ochi_sdk` | `openfang_sdk` |

---

## SDK Publish Checklist (Maintainers)

Use this checklist for each SDK release:

1. **Versioning**
   - Bump package version (npm / Python).
   - Ensure release notes call out compatibility status.
2. **Naming checks**
   - JS package remains `@ochi/sdk`.
   - Python package metadata includes both new and legacy module exports during deprecation period.
3. **Examples & docs**
   - Examples use Ochi-first imports by default.
   - Migration snippets for legacy names remain documented.
4. **Validation**
   - Smoke test JS basic + streaming examples.
   - Smoke test Python basic + streaming examples.
5. **Announcement**
   - Mention migration timeline and deprecation window for OpenFang aliases.

---

## Recommended Migration Rollout

1. Update internal codebases to Ochi-first imports.
2. Keep legacy aliases enabled for at least 2–3 stable releases.
3. Add deprecation warnings in release notes before alias removal.
4. Remove legacy alias support only after timeline is announced and documented.

