# v0.0.5 Training Sessions Index

This directory contains records of interactive training sessions where Polly (Claude) learned v0.0.5 syntax through code generation and user corrections.

---

## Training Methodology

**Approach:** Iterative learning through mistakes
1. Generate code examples in v0.0.5 syntax
2. User provides corrections
3. Document corrections and patterns
4. Apply learnings to next example
5. Record session for future reference

**Benefits:**
- Discovers edge cases and ambiguities
- Documents real usage patterns
- Creates comprehensive examples
- Builds systematic understanding

---

## Sessions

### [Session 001 - 2026-01-02](./session-001-2026-01-02.md)

**Focus:** Core v0.0.5 syntax fundamentals

**Topics Covered:**
- Variable lifecycle and immutability
- Error handling with `[!]` markers
- Reserved vs user-defined enums
- Schema-based type system
- Trigger I/O wiring
- Field naming conventions

**Examples Created:**
1. Marketing Campaign Automation
2. Service Health Monitor
3. Log File Analyzer
4. Backup System
5. Document Processor
6. Ramadan Email Campaign (Hijri calendar)
7. Hello World Multi-Runtime (continuation 2026-01-04)

**Key Learnings:**
- Variables are immutable (Default → Final pattern)
- Inline pipelines replace single values only
- Pipeline structure order is strict
- Error handling must be exhaustive
- Schema-based inputs reduce verbosity

**Documentation Created:**
- Variable lifecycle specification
- Error handling guide
- 5 stdlib YAML files (enums, pipelines, wrappers, triggers, operators)
- Stdlib audit for serial outputs
- Enhancement proposals

**Stats:**
- Corrections: 48+ (40 initial + 8 continuation)
- Documentation: ~3800 lines
- Duration: ~3 hours (2 hours initial + 1 hour continuation)

---

## Cumulative Learnings

### Syntax Rules Discovered

1. ✅ Variable lifecycle: Declared → Pending → Default/Final → Released
2. ✅ Maximum 2 pushes per variable (default + final)
3. ✅ Pipeline order: `{|} → [t] → [<] → [>] → [w] → execution`
4. ✅ Error handling exhaustiveness required
5. ✅ Inline pipelines for single values only
6. ✅ Field names: underscores not dashes
7. ✅ DateTime type: `:dt` not `:datetime`
8. ✅ Reserved enums: `-` prefix (stdlib)
9. ✅ User enums: `#` prefix
10. ✅ Subfield marker `[.]` universal usage
11. ✅ Schema-based wrapper inputs
12. ✅ Trigger patterns: inline OR I/O wiring
13. ✅ `|U.Do.Nothing` for placeholder actions
14. ✅ AND conditions: `[&]` marker
15. ✅ Wrapper ordering: ALL `[w]` BEFORE ALL execution markers
16. ✅ Environment variables: `<env.vars.*` all `:string` type
17. ✅ Input shorthand: `<var#EnumType` implies value from enum
18. ✅ Boolean type: `:bool` ≡ `-Boolean` with `-True`/`-False`
19. ✅ Serial construction: dot notation recommended
20. ✅ Hard syntax rule: `<indent><Marker><One-expression>` per line
21. ✅ Runtime patterns: `|W.RT.{Language}` and `|RT.{Language}.Code`
22. ✅ Native types: `:py.str`, `:rust.i32`, `:js.number`, etc.

### Proposed Enhancements

1. 🔄 Code block marker `[c]` for multi-line content
2. 🔄 Reserved schema `-DB-Settings` for database configuration
3. 🔄 Exhaustive enum matching without `[f] *?`
4. 🔄 Additional reserved schemas (HTTP, Email, Cache)

### Common Mistakes

1. ❌ Treating variables as mutable/reassignable
2. ❌ Mixing inline pipeline with I/O wiring
3. ❌ Non-exhaustive error handling
4. ❌ Wrong pipeline structure order
5. ❌ Using dashes in field names
6. ❌ Using `:serial` instead of typed schemas
7. ❌ Missing `|U.Do.Nothing` placeholders
8. ❌ Using reserved enum prefix for user enums
9. ❌ Placing wrappers after execution markers
10. ❌ Using wrong types for `<env.vars.*` (must be `:string`)
11. ❌ Redundant enum value assignment in inputs
12. ❌ Using lowercase boolean literals instead of `-True`/`-False`
13. ❌ Initializing empty serial then adding fields (contradictory)

---

## Future Sessions

### Planned Topics

**Session 002: Advanced Patterns**
- Complex conditional logic
- Nested loops and pack operations
- Error recovery strategies
- State machine patterns

**Session 003: Performance & Optimization**
- Efficient collection handling
- Parallel processing patterns
- Resource management best practices
- Memory optimization

**Session 004: Integration Patterns**
- Multi-service orchestration
- Event-driven architectures
- Message queue integration
- API gateway patterns

**Session 005: Testing & Quality**
- Test pipeline patterns
- Mock data generation
- Validation strategies
- CI/CD integration

---

## Documentation Impact

### Created Specifications

| Document | Purpose | Lines | Status |
|----------|---------|-------|--------|
| variable-lifecycle.md | Core immutability model | 350+ | ✅ Complete |
| error-handling.md | Error handling patterns | 400+ | ✅ Complete |
| reserved-enums.yaml | Stdlib enum types | 200+ | 🔄 Growing |
| standard-pipelines.yaml | Core pipelines | 300+ | 🔄 Growing |
| standard-wrappers.yaml | Resource wrappers | 250+ | 🔄 Growing |
| standard-triggers.yaml | Activation triggers | 300+ | 🔄 Growing |
| standard-operators.yaml | Pack/unpack ops | 300+ | 🔄 Growing |
| stdlib-audit-serial-outputs.md | Migration tracker | 400+ | 🔄 Living doc |
| brainstorming-v0.0.5-enhancements.md | Proposals | 500+ | 🔄 Living doc |

**Total Documentation:** ~3000 lines and growing

### Audit Trail

All training sessions are:
- Timestamped
- Fully documented with examples
- Cross-referenced to specifications
- Tracked for cumulative learning

---

## Contributing

### For Future Training Sessions

1. Create session file: `session-XXX-YYYY-MM-DD.md`
2. Document all examples and corrections
3. Update cumulative learnings
4. Reference related specifications
5. Add statistics and metrics
6. Update this index

### Template Structure

```markdown
# Training Session XXX - Topic

**Date:** YYYY-MM-DD
**Focus:** Brief description
**Status:** ✅ Complete

## Examples Created
## Corrections & Learnings
## Documentation Created
## Statistics
## Key Takeaways
```

---

## Metrics

**Total Sessions:** 1 (with continuation)
**Total Examples:** 7
**Total Corrections:** 48+
**Documentation Created:** ~3800 lines
**Syntax Rules Discovered:** 22+
**Proposed Enhancements:** 4

**Effectiveness Rating:** ⭐⭐⭐⭐⭐

Training-based learning has proven highly effective for:
- Discovering edge cases
- Building systematic understanding
- Creating comprehensive documentation
- Establishing best practices

---

**Last Updated:** 2026-01-04
**Next Session:** TBD (Session 002 - Advanced Patterns)
