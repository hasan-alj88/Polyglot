---
phase: 319-polyglot-sdk
plan: 02
status: complete
completed: 2026-04-18
---

# Plan 319-02 Summary: Cross-Reference Updates

## What Was Done

Updated 4 existing documentation files to align with the new SDK spec, add type mapping descriptor references, fix FFI framing, and add bidirectional cross-references.

## Files Modified

| File | Changes |
|------|---------|
| `docs/technical/spec/behavior-contract.md` | Added item 6 (Type Mapping Descriptors) to Contract Structure; added Type Mapping Descriptors section; added @c: and [[wikilink]] to polyglot-sdk |
| `docs/technical/spec/native-dispatch.md` | Added type mapping descriptor note after registry entry schema; added SDK cross-ref in Integrator SDK section; added polyglot-sdk to Related table |
| `docs/technical/integrator-internals.md` | Rewrote FFI section: "Method 2: FFI via -Run.Bridge (Deferred)" — no longer says "generates a pipeline"; references #321; explains why no FFI in SDK |
| `docs/user/integrator/polyglot-interface.md` | Replaced FFI framing with universal string serialization; references -Run.Bridge for FFI optimization; added polyglot-sdk to Related |

## Acceptance Criteria

- [x] AC-1: Type mapping descriptor in Behavior Contract
- [x] AC-2: Type mapping descriptor linked in Native Dispatch
- [x] AC-3: Integrator internals FFI section updated (no "generates a pipeline", references #321)
- [x] AC-4: Polyglot interface FFI mention aligned (universal string, not FFI abstraction)
- [x] AC-5: Cross-references bidirectional (all 4 files reference polyglot-sdk.md)
