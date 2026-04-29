---
audience: design
type: spec
updated: 2026-04-19
---

# Integrator Internals — Data Casting Methods

<!-- @c:vision#Evolution -->
<!-- @c:glossary#Data Tree -->
<!-- @c:glossary#Cross-Language Integration -->
<!-- @c:spec/aljam3-sdk -->

This document describes how the `aljam3-interface` library casts data types across language boundaries. This is internal architecture — library developers need this; end users do not.

## Two Casting Methods

### Method 1: Serialized Data Trees (Primary — Phase 1)

<!-- @u:technical/spec/native-dispatch#Serialization Protocol -->

All Aljam3 data is represented as [[glossary#Data Tree|c:data trees]] — trees of serialized strings. Cross-language data flows follow the path:

```
Language A → Serialized data tree → Language B
```

Every programming language can parse string data, so the library provides functions that take serialized data and convert it to the target type in the native language. This method requires less setup and configuration but is less efficient than FFI.

This approach uses the same JSON wire protocol defined in `native-dispatch.md` — the `aljam3-interface` library is a consumer of that protocol.

### Method 2: FFI via -Run.Bridge

FFI between language pairs offers significantly better performance than serialized string exchange by avoiding JSON serialization overhead. However, FFI requires dynamic code generation to set up bindings between arbitrary language pairs — this violates the SDK's "no dynamic code generation" design principle. FFI is also not universally available across all supported languages.

For language pairs where FFI is feasible and performance-critical, Aljam3 provides `-Run.Bridge` — a pairwise, language-specific integration pipeline. Bridge pipelines require two `-W.Env` wrappers (one per language) and use [[pglib/pipelines/Variable/Convert|-Variable.Convert]] for direct type mapping between language type systems. See [[pglib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] for cross-language function calls and [[pglib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] for cross-language variable binding. The [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] documents the full conversion flow.

The `aljam3-interface` SDK uses Method 1 (serialized data trees) exclusively. Method 2 is a separate optimization path, not part of the SDK. See [[aljam3-sdk#Why No FFI in the SDK]] for the full rationale.

## Related

- [[vision#Evolution|c:Evolution]] — integration evolution roadmap
- [[aljam3-sdk]] — SDK public API, type mapping, and serialization algorithm
- `docs/technical/spec/native-dispatch.md` — JSON wire protocol and native function contract
- `lib/README.md` — SDK structure and language folders
- [[pglib/pipelines/Run/Bridge.Function]] — cross-language function call specification
- [[pglib/pipelines/Run/Bridge.Script]] — cross-language variable binding specification
- [[technical/algorithms/bridge-conversion]] — Bridge conversion algorithm
