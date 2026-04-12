---
audience: architect
type: spec
updated: 2026-04-12
---

# Integrator Internals — Data Casting Methods

<!-- @c:vision#Evolution -->
<!-- @c:glossary#Data Tree -->
<!-- @c:glossary#Cross-Language Integration -->

This document describes how the `polyglot-interface` library casts data types across language boundaries. This is internal architecture — library developers need this; end users do not.

## Two Casting Methods

### Method 1: Serialized Data Trees (Primary — Phase 1)

<!-- @u:technical/spec/native-dispatch#Serialization Protocol -->

All Polyglot data is represented as [[glossary#Data Tree|c:data trees]] — trees of serialized strings. Cross-language data flows follow the path:

```
Language A → Serialized data tree → Language B
```

Every programming language can parse string data, so the library provides functions that take serialized data and convert it to the target type in the native language. This method requires less setup and configuration but is less efficient than FFI.

This approach uses the same JSON wire protocol defined in `native-dispatch.md` — the `polyglot-interface` library is a consumer of that protocol.

### Method 2: FFI (Optimization — Phase 1 stretch / Phase 2)

The library generates a pipeline that sets up the necessary files and bindings for direct FFI between languages, then runs the code and casts the output to the target type. This method is more efficient but requires more setup and configuration.

Both methods are valid. Phase 1 prioritizes the serialized data tree method (simpler, universal). If time permits, FFI integration is attempted in Phase 1; otherwise it is deferred to Phase 2 as an optimization path.

## Related

- [[vision#Evolution|c:Evolution]] — integration evolution roadmap
- `docs/technical/spec/native-dispatch.md` — JSON wire protocol and native function contract
- `lib/README.md` — SDK structure and language folders
