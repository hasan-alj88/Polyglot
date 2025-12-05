# Operator Directionality Rule - Critical Syntax Correction

**Date:** 2025-12-03
**Type:** Critical Syntax Rule
**Status:** ✅ **COMPLETE**
**Scope:** Enforces operator direction matching data flow in pipeline I/O

---

## Summary

Established and enforced critical rule: **Operator direction MUST match data flow direction.**

Block markers `[i]`, `[<]`, and `[>]` dictate which operators are allowed based on whether data flows INTO or OUT OF the pipeline.

---

## The Rule

| Marker | Purpose | Data Flow | ✅ Allowed Operators | ❌ Forbidden Operators |
|--------|---------|-----------|---------------------|----------------------|
| `[i]` | Input declaration | INTO pipeline (←) | `<<`, `<~` | `>>`, `~>` |
| `[<]` | Pipeline input arg | INTO pipeline (←) | `<<`, `<~` | `>>`, `~>` |
| `[>]` | Pipeline output arg | OUT OF pipeline (→) | `>>`, `~>` | `<<`, `<~` |

**Mnemonic:** Arrows point to data destination:
- `<<` and `<~` point LEFT (← into variable/input)
- `>>` and `~>` point RIGHT (→ out of pipeline)

---

## Examples

### ✅ CORRECT: `[i]` Input Uses `<<` or `<~`

```polyglot
[i] .timeout: pg\int <~ 30               // Data flows INTO input
[i] .defaultConfig: #Config <~ .config  // Data flows INTO input
```

**Reading:** "Default value flows INTO the input variable"

---

### ✅ CORRECT: `[<]` Pipeline Input Uses `<<` or `<~`

```polyglot
[r] |ProcessData
[<] <input: pg\string << .data           // Data flows INTO pipeline
[<] <config: #Config <~ .default         // Data flows INTO pipeline with default
```

**Reading:** "Caller's data flows INTO pipeline input argument"

---

### ✅ CORRECT: `[>]` Pipeline Output Uses `>>` or `~>`

```polyglot
[r] |FetchData
[>] >output: pg\string >> .result        // Data flows OUT OF pipeline
[>] >config: #Config ~> .fallback        // Data flows OUT with default fallback
```

**Reading:** "Pipeline output flows OUT TO caller's variable"

---

## Violations Fixed

### Violation 1: `[i]` Using `~>` (Wrong Direction)

**❌ INCORRECT (was in docs):**
```polyglot
[i] .config: #Config ~> .defaultConfig  // WRONG: ~> points outward!
```

**Problem:** `~>` points RIGHT (→), but `[i]` expects data to flow INTO (←) the input.

**✅ CORRECTED:**
```polyglot
[i] .defaultConfig: #Config <~ .config  // CORRECT: <~ points inward
```

**Fixed in:**
- `docs/user/variable-state-system.md` (2 instances)
- `docs/user/advanced/variable-states.md` (1 instance)
- `docs/user/syntax/operators.md` (2 instances)

---

### Violation 2: `[<]` Using `>>` (Wrong Direction)

**❌ INCORRECT (was in docs):**
```polyglot
[~][<] <error: !NetworkTimeout >> .timeout_err  // WRONG: >> points outward!
```

**Problem:** `>>` points RIGHT (→), but `[<]` expects data to flow INTO (←) the pipeline.

**✅ CORRECTED:**
```polyglot
[~][<] <error: !NetworkTimeout << .timeout_err  // CORRECT: << points inward
```

**Fixed in:**
- `docs/user/syntax/error-handling.md` (1 instance)
- `docs/user/syntax/overview.md` (1 instance)

---

## Why This Rule Exists

### Conceptual Clarity

**Operators encode direction:**
- `<<` = "push left" or "pull from right" (data moves ←)
- `<~` = "default from right" (data moves ←)
- `>>` = "push right" or "pull from left" (data moves →)
- `~>` = "default to right" (data moves →)

**Block markers encode role:**
- `[i]` = input slot (receives data ←)
- `[<]` = pipeline input argument (receives data ←)
- `[>]` = pipeline output argument (provides data →)

**The rule ensures consistency:** Operator direction matches block marker's data flow.

---

### Example: Complete Pipeline Call

```polyglot
// Pipeline definition
[|] |Transform
[i] .input: pg\string          // Input slot awaits data ←
[o] .output: pg\string         // Output slot provides data →
[t] |T.Call

[r] .result: pg\string << U.String.ToUpper"{.input}"
[o] .output: pg\string << .result
[X]

// Pipeline call
[r] |Transform
[<] <input: pg\string << "hello"     // Caller sends data INTO pipeline ←
[>] >output: pg\string >> .result    // Pipeline sends data OUT TO caller →
```

**Data flow:**
1. Caller has `"hello"` in scope
2. `[<] <input << "hello"` - Data flows INTO pipeline (←)
3. Pipeline processes input
4. `[>] >output >> .result` - Data flows OUT OF pipeline (→)
5. Caller receives result in `.result`

---

## Configuration Best Practice

**Recommended:** Use enumerations for configuration with defaults:

```polyglot
[#] #Config
[<] .port: pg\uint <~ 8080
[<] .host: pg\url <~ \\http\\localhost
[<] .timeout: pg\int <~ 30
[X]
```

**Benefits:**
- Centralized configuration
- Type-safe defaults
- Enumeration structure
- Clear field access

---

## Visual Diagram

```
DATA FLOW DIRECTION AND OPERATORS:

[i] Input                [<] Pipeline Input        [>] Pipeline Output
    ↓                         ↓                         ↓
  Receives                 Receives                  Provides
    data                     data                      data
    ←                        ←                         →

  Use << or <~            Use << or <~            Use >> or ~>


Example Flow:

  CALLER SCOPE          |  PIPELINE  |          CALLER SCOPE
                        |            |
  .data                 |  [i] .in   |
    └─────<<─────────>  |    ←       |
                        |            |
                        | [process]  |
                        |            |
                        |  [o] .out  |
    .result  <──────>>──┘    →       |
                        |            |
```

---

## Operator Compatibility Matrix

| Block | `<<` | `<~` | `>>` | `~>` |
|-------|------|------|------|------|
| `[i]` | ✅ | ✅ | ❌ | ❌ |
| `[<]` | ✅ | ✅ | ❌ | ❌ |
| `[>]` | ❌ | ❌ | ✅ | ✅ |
| `[o]` | ✅ | ✅ | ❌ | ❌ |

**Note:** `[o]` (output declaration) behaves like `[i]` - it receives/declares what to output.

---

## Common Mistakes

### ❌ Mistake 1: Using `~>` with `[i]`
```polyglot
[i] .config: #Config ~> .source  // WRONG: ~> is outward
```

✅ **Fix:** Reverse and use `<~`
```polyglot
[i] .source: #Config <~ .config  // CORRECT: <~ is inward
```

---

### ❌ Mistake 2: Using `>>` with `[<]`
```polyglot
[<] <input: pg\string >> .data  // WRONG: >> is outward
```

✅ **Fix:** Use `<<`
```polyglot
[<] <input: pg\string << .data  // CORRECT: << is inward
```

---

### ❌ Mistake 3: Using `<<` with `[>]`
```polyglot
[>] >output: pg\string << .result  // WRONG: << is inward
```

✅ **Fix:** Use `>>`
```polyglot
[>] >output: pg\string >> .result  // CORRECT: >> is outward
```

---

## Files Updated

1. **`docs/user/variable-state-system.md`**
   - Fixed 2 instances of `[i] .config ~> .defaultConfig`
   - Changed to `[i] .defaultConfig <~ .config`

2. **`docs/user/advanced/variable-states.md`**
   - Fixed 1 instance of `[i] .config ~> .defaultConfig`
   - Changed to `[i] .defaultConfig <~ .config`

3. **`docs/user/syntax/operators.md`** (v0.0.5 → v0.0.6)
   - Fixed 2 instances in examples
   - Added "Critical Rule: Operator Direction MUST Match Data Flow" section
   - Added compatibility matrix
   - Added visual examples

4. **`docs/user/syntax/error-handling.md`**
   - Fixed 1 instance of `[<] <error >> .timeout_err`
   - Changed to `[<] <error << .timeout_err`

5. **`docs/user/syntax/overview.md`**
   - Fixed 1 instance of `[<] <error >> .timeout_error`
   - Changed to `[<] <error << .timeout_error`

**Total:** 7 violations fixed across 5 files

---

## Success Metrics

| Metric | Target | Achieved | Status |
|--------|--------|----------|--------|
| Violations found | All | 7 instances | ✅ Met |
| Violations fixed | 100% | 7/7 | ✅ Met |
| Rule documented | Comprehensive | Complete | ✅ Met |
| Examples provided | Clear | 10+ examples | ✅ Met |
| Files updated | All affected | 5 files | ✅ Met |

---

## Enforcement

### Parser Level

**Parser MUST enforce:**
1. `[i]` followed by `<<` or `<~` only
2. `[<]` followed by `<<` or `<~` only
3. `[>]` followed by `>>` or `~>` only

**Error messages:**
```
Error: Invalid operator '>>' with input marker '[i]'
  → [i] .config: #Config ~> .source
                          ^^
  Input markers [i] require '<<' or '<~' (data flows INTO input)

  Did you mean: [i] .source: #Config <~ .config
```

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully established and enforced operator directionality rule:

**Rule:** Operator direction MUST match block marker's data flow direction.

**Violations Fixed:**
- 5 instances of `[i]` using `~>` (wrong direction)
- 2 instances of `[<]` using `>>` (wrong direction)

**Documentation Updated:**
- Comprehensive rule section added to operators.md
- All violations corrected
- Examples and diagrams provided

**Benefits:**
- **Consistency:** Operators always match intended data flow
- **Clarity:** Direction is visually obvious from operators
- **Error Prevention:** Parser can catch direction mismatches
- **Self-Documenting:** Code clearly shows data flow direction

This rule eliminates a common source of confusion by ensuring operators always align with their semantic meaning and the block marker's purpose.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Rule Type:** Operator Directionality Enforcement
