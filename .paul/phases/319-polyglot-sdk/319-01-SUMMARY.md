---
phase: 319-polyglot-sdk
plan: 01
status: complete
completed: 2026-04-18
---

# Plan 319-01 Summary: Polyglot SDK Specification

## What Was Done

Created `docs/technical/spec/polyglot-sdk.md` — the complete SDK specification covering cross-language type conversion and call protocol.

## Files Created

| File | Action |
|------|--------|
| `docs/technical/spec/polyglot-sdk.md` | Created (new) |

## Key Content

1. **Universal String Algorithm** — serialization flow from Foreign Variable through JSON envelope to NoSQL store and back
2. **SDK Public Interface** — 4 functions with signatures in Python, Rust, Go, JavaScript:
   - `to_polyglot(value, type) → str`
   - `from_polyglot(str) → value`
   - `call(pipeline, bindings) → result` with NATS request-reply protocol
   - `pull(name) → value` / `push(name, value)` for Bind protocol
3. **Primitive Type Mapping Table** — 7 Polyglot types mapped to 4 languages
4. **Type Mapping Descriptors** — per-IO-port descriptor schema emitted by compiler in Behavior Contract
5. **Pluggable Wire Format** — serialize/deserialize interface (JSON now, CBOR/MessagePack future)
6. **SDK Distribution** — package names per language, version locked to compiler version
7. **Design Decisions** — including documented "Why No FFI" with trade-off acknowledgment (deferred to -Run.Bridge #321)

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| SDK uses universal string only, no FFI | FFI requires dynamic code gen, not universally available; deferred to -Run.Bridge (#321) |
| NATS protocol documented for `call()` | polyglot.call.* / polyglot.result.* topics with correlation ID |
| Type mapping descriptors derived from native registry | Registry IO schema serves as descriptor — no separate emission needed |

## Acceptance Criteria

- [x] AC-1: Universal string algorithm documented
- [x] AC-2: SDK interface fully specified (4 functions, 4 languages)
- [x] AC-3: Primitive type mapping table complete (7 × 4)
- [x] AC-4: Type mapping descriptor specified
- [x] AC-5: Pluggable wire format interface defined
- [x] AC-6: SDK distribution and versioning documented
- [x] AC-7: "Why no FFI" decision documented
