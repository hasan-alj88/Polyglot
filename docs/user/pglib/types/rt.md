---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# Runtime Types

<!-- @c:types -->

Types used by `-RT.*` runtime execution pipelines and `-W.RT` wrappers. No `[@]` import needed.

| Type | Description | File |
|------|-------------|------|
| `#Code` | Runtime execution output (per-language .Output) | [[Code]] |
| `#PyEnv` | Python environment handle | [[PyEnv]] |
| `#RsEnv` | Rust environment handle | [[RsEnv]] |

## Related

- [[pglib/pipelines/RT/INDEX|-RT]] -- runtime execution pipelines that consume these types
- [[pglib/pipelines/W/INDEX|-W]] -- `-W.RT` wrapper that produces `#PyEnv`/`#RsEnv`
- [[pglib/errors/errors|errors]] -- `!RT` error namespace for runtime failures
