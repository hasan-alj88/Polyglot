# Safety Mechanisms Documentation - Explicit Intent Requirements

**Date:** 2025-12-03
**Type:** Safety Mechanism Specification
**Status:** ✅ **COMPLETE**
**Scope:** Comprehensive documentation of Polyglot's three safety mechanisms

---

## Summary

Documented Polyglot's three critical safety mechanisms that enforce **explicit intent** to prevent accidental omissions. These mechanisms require developers to explicitly declare when certain blocks are intentionally omitted, ensuring code correctness and preventing common mistakes.

---

## The Three Safety Mechanisms

### 1. `[W] |W.Polyglot.Scope` - Wrapper Safety Mechanism

**Rule:** When omitting `[\]` and `[/]` step blocks, you MUST explicitly declare `[W] |W.Polyglot.Scope`.

**Purpose:** Prevents accidental omission of step blocks in manual trigger pipelines.

**How It Works:**
- `|W.Polyglot.Scope` is **always implicit** (always runs automatically)
- BUT when you omit `[\]` and `[/]`, you must declare it explicitly
- Forces developer to acknowledge intentional omission

**Examples:**

```polyglot
// ❌ COMPILE ERROR: Missing [W] or {[\], [/]}
[|] |MyPipeline
[t] |T.Manual
[r] .x: pg\int << 10
[X]

// ✅ CORRECT: Explicit intent
[|] |MyPipeline
[t] |T.Manual
[W] |W.Polyglot.Scope  // "I intentionally omitted [\] and [/]"
[r] .x: pg\int << 10
[X]

// ✅ CORRECT: Step blocks present
[|] |MyPipeline
[t] |T.Manual
[\]
[r] .x: pg\int << 10
[/]
[X]

// ✅ CORRECT: Runtime wrapper makes it implicit
[|] |MyPipeline
[t] |T.Manual
[W] |W.RT.Python3.12
[\]
[r] .x: pg\int << py\calculate()
[/]
[X]
```

---

### 2. `[o] !NoError` - Output Safety Mechanism

**Rule:** When a pipeline intentionally produces no outputs, you MUST declare `[o] !NoError`.

**Purpose:** Prevents accidental omission of output declarations.

**How It Works:**
- All successful pipelines yield `!NoError` automatically
- BUT when you have no output variables, you must declare it explicitly
- Forces developer to acknowledge intentional lack of outputs

**Examples:**

```polyglot
// ❌ COMPILE ERROR: No outputs declared
[|] |LogMessage
[i] .message: pg\string
[t] |T.Call
[r] U.Log.Info"{.message}"
[X]

// ✅ CORRECT: Explicit no-output intent
[|] |LogMessage
[i] .message: pg\string
[o] !NoError  // "I intentionally have no outputs"
[t] |T.Call
[r] U.Log.Info"{.message}"
[X]

// ✅ CORRECT: Has output
[|] |ProcessMessage
[i] .message: pg\string
[o] .result: pg\string
[t] |T.Call
[r] .result: pg\string << U.String.ToUpper"{.message}"
[X]
```

---

### 3. `[i]` Input Usage - Input Safety Mechanism

**Rule:** All `[i]` inputs must be declared before `[t]` trigger and used in pipeline body.

**Purpose:** Prevents unused input declarations and ensures all declared inputs serve a purpose.

**How It Works:**
- Compiler tracks all declared `[i]` inputs
- Verifies each input is used in pipeline body
- Reports error if any input is declared but never used

**Examples:**

```polyglot
// ❌ COMPILE ERROR: Input .count declared but never used
[|] |ProcessData
[i] .data: pg\string
[i] .count: pg\int     // Declared but unused!
[o] .result: pg\string
[t] |T.Call
[r] .result: pg\string << U.String.ToUpper"{.data}"
[X]

// ✅ CORRECT: All inputs used
[|] |ProcessData
[i] .data: pg\string
[i] .count: pg\int
[o] .result: pg\string
[t] |T.Call
[r] .repeated: pg\string << U.String.Repeat"{.data, .count}"
[o] .result: pg\string << .repeated
[X]

// ✅ CORRECT: No unused inputs
[|] |ProcessData
[i] .data: pg\string
[o] .result: pg\string
[t] |T.Call
[r] .result: pg\string << U.String.ToUpper"{.data}"
[X]
```

---

## Design Philosophy

### Why Safety Mechanisms?

**Problem:** Developers make common mistakes:
1. Forgetting step blocks in manual trigger pipelines
2. Forgetting to declare outputs
3. Declaring inputs they never use

**Solution:** Force explicit acknowledgment of intent:
1. Can't omit `[\]` and `[/]` without declaring `[W] |W.Polyglot.Scope`
2. Can't omit outputs without declaring `[o] !NoError`
3. Can't declare unused inputs without compiler error

**Benefits:**
- **Prevents accidental errors** - Catches mistakes at compile time
- **Enforces clarity** - Code explicitly shows intent
- **Self-documenting** - Safety declarations explain why things are omitted
- **Reduces bugs** - Forces developers to think about structure

---

## Safety Mechanism Comparison

| Mechanism | What It Protects | Error Prevented | Explicit Declaration |
|-----------|------------------|-----------------|---------------------|
| `[W] |W.Polyglot.Scope` | Step blocks | Forgetting `[\]` and `[/]` | Required when omitting blocks |
| `[o] !NoError` | Output declarations | Forgetting outputs | Required when no outputs |
| `[i]` usage check | Input usage | Unused inputs | Automatic validation |

---

## Similar Patterns in Other Languages

### Rust: `#[allow(dead_code)]`
```rust
#[allow(dead_code)]  // Explicit: "I know this is unused"
fn helper() {}
```

### Go: `//nolint`
```go
//nolint:unused  // Explicit: "I know this looks unused"
func internal() {}
```

### Polyglot: Safety Mechanisms
```polyglot
[W] |W.Polyglot.Scope  // Explicit: "I know I omitted [\] and [/]"
[o] !NoError           // Explicit: "I know I have no outputs"
```

**Common Theme:** Require developers to explicitly acknowledge intentional deviations from standard patterns.

---

## Implementation Requirements

### Compiler Validation Rules:

**1. Wrapper Safety Check:**
```
IF pipeline has [t] |T.Manual
AND no [\] and [/] blocks present
AND no [W] declaration
THEN COMPILE ERROR: "Missing [W] or {[\], [/]} - was this intentional?"
```

**2. Output Safety Check:**
```
IF pipeline has no [o] declarations
AND no [o] !NoError declaration
THEN COMPILE ERROR: "No outputs declared - was this intentional?"
```

**3. Input Usage Check:**
```
FOR EACH [i] input declaration
  IF input not used in pipeline body
  THEN COMPILE ERROR: "Input declared but never used: {input_name}"
```

---

## Error Messages

### Wrapper Safety Error:
```
Error: Missing wrapper or step blocks
  → [|] |MyPipeline
    [t] |T.Manual
    ^
  Expected either:
    - [W] |W.Polyglot.Scope (explicit intent: no step blocks)
    - [\] and [/] (step block markers)
    - [W] |W.RT.* with [\] and [/] (runtime wrapper)
```

### Output Safety Error:
```
Error: No outputs declared
  → [|] |LogMessage
    [X]
    ^
  Expected either:
    - [o] .output_name: Type (output variable)
    - [o] !NoError (explicit intent: no outputs)
```

### Input Usage Error:
```
Error: Input declared but never used
  → [i] .count: pg\int
    ^
  Input '.count' is declared but never referenced in pipeline body.
  Remove the declaration or use the input.
```

---

## Files Updated

### 1. `docs/user/variable-state-system.md`
**Added:**
- New section: "Safety Mechanisms: Explicit Intent"
- Comprehensive documentation of all three mechanisms
- Examples for each mechanism (error and correct versions)
- Updated "Nested Block Scopes" example to use safety mechanism

**Location:** Lines 26-134

### 2. `docs/project/implicit-wrapper-correction-2025-12-03.md`
**Updated:**
- Title: "Wrapper Safety Mechanism - `[W] |W.Polyglot.Scope` Explicit Intent"
- Completely revised to reflect safety mechanism understanding
- Clarified that `|W.Polyglot.Scope` is always implicit BUT must be explicitly declared as safety mechanism
- Updated all examples and patterns
- Added safety mechanism rules summary

---

## Best Practices

### When Writing Pipelines:

**1. Always consider step blocks:**
- Do you need `[\]` and `[/]`? Add them
- Don't need them? Add `[W] |W.Polyglot.Scope` to show intent

**2. Always consider outputs:**
- Does pipeline produce outputs? Declare them with `[o]`
- No outputs? Add `[o] !NoError` to show intent

**3. Always use declared inputs:**
- Every `[i]` input must be used in pipeline body
- Don't declare inputs "just in case"
- If unsure, don't declare - add later when needed

---

## Teaching Points

### For New Polyglot Developers:

**1. Safety mechanisms are your friends:**
- They prevent mistakes you don't know you're making
- They make your code clearer and more explicit
- They catch errors at compile time, not runtime

**2. Don't fight the mechanisms:**
- If compiler asks for `[W] |W.Polyglot.Scope`, ask yourself: "Should I have step blocks?"
- If compiler asks for `[o] !NoError`, ask yourself: "Should I have outputs?"
- If compiler complains about unused input, ask yourself: "Why did I declare this?"

**3. Use mechanisms to document intent:**
- `[W] |W.Polyglot.Scope` says "I thought about step blocks and don't need them"
- `[o] !NoError` says "I thought about outputs and don't need them"
- Clean input list says "These are all the inputs I actually use"

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Safety mechanisms documented | 3/3 | ✅ Complete |
| Examples provided per mechanism | 3+ each | ✅ Complete |
| Error messages specified | 3/3 | ✅ Complete |
| Implementation rules defined | 3/3 | ✅ Complete |
| Files updated | 2 files | ✅ Complete |
| Best practices documented | Complete | ✅ Complete |

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully documented all three Polyglot safety mechanisms:

**Achievements:**
- ✅ Comprehensive safety mechanism documentation added to variable-state-system.md
- ✅ Updated implicit-wrapper correction document to reflect safety mechanism
- ✅ Provided clear examples for each mechanism (error and correct versions)
- ✅ Defined compiler validation rules
- ✅ Specified error messages
- ✅ Documented best practices

**Benefits:**
- **Prevents common mistakes** at compile time
- **Enforces explicit intent** in code
- **Self-documenting** safety declarations
- **Improves code quality** through compile-time checks

**Design Philosophy:**
Polyglot's safety mechanisms follow the principle: **"Make the right thing easy, make the wrong thing hard, and make accidental mistakes impossible."**

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Documentation Type:** Safety Mechanism Specification
