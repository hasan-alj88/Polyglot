# Pipeline I/O Operators Update - Breaking Syntax Change

**Date:** 2025-12-03
**Type:** Major Breaking Change
**Status:** ✅ **COMPLETE**
**Rationale:** Improve namespace clarity and distinguish pipeline arguments from scope variables

---

## Summary

Introduced new `<` and `>` prefix operators for pipeline input/output arguments to replace the ambiguous `.` prefix. This provides clear namespace distinction between:
- `.variable` - Variables in current scope
- `<input` - Pipeline input arguments
- `>output` - Pipeline output arguments

---

## Syntax Changes

### Before (Ambiguous)

```polyglot
[r] |HttpGet
[<] .url: pg\string << .my_url           // AMBIGUOUS: Looks like a variable
[>] .response: pg\dict >> .result         // AMBIGUOUS: Looks like a variable
// .url and .response appear to be variables but they're NOT!
```

### After (Clear)

```polyglot
[r] |HttpGet
[<] <url: pg\string << .my_url           // CLEAR: Pipeline input argument
[>] >response: pg\dict >> .result         // CLEAR: Pipeline output argument
// <url and >response are obviously NOT variables in current scope
```

---

## New Operators

### `<` - Input Argument Prefix

**Purpose:** Identifies a pipeline input argument (parameter passed TO the pipeline).

**Syntax:** `<identifier`

**Usage Context:** Used with `[<]` block marker in pipeline calls

**Namespace:** `<input` lives in `|Pipeline.Inputs.input`, NOT in caller's scope

**Example:**
```polyglot
[r] |ProcessData
[<] <input: pg\string << .data           // Input argument <input
[<] <config: #Config << .settings        // Input argument <config
[>] >result: pg\dict >> .output          // Output argument >result
```

**Reading:** "Input argument `<input` receives value from variable `.data`"

---

### `>` - Output Argument Prefix

**Purpose:** Identifies a pipeline output argument (value returned FROM the pipeline).

**Syntax:** `>identifier`

**Usage Context:** Used with `[>]` block marker in pipeline calls

**Namespace:** `>output` lives in `|Pipeline.Outputs.output`, NOT in caller's scope

**State Behavior:**
- Becomes **Pending** when pipeline starts (if async)
- Becomes **Final** when pipeline completes successfully
- Becomes **Faulted** when pipeline fails

**Example:**
```polyglot
[r] |Calculate
[<] <a: pg\int << .x
[<] <b: pg\int << .y
[>] >sum: pg\int >> .total               // Output argument >sum
[>] >product: pg\int >> .mult            // Output argument >product

// .total and .mult are Pending until |Calculate completes
```

**Reading:** "Output argument `>sum` pushes value into variable `.total`"

---

## Rationale: Why This Change?

### Problem with `.` Prefix

The dot prefix was ambiguous because it made pipeline arguments look like regular variables:

```polyglot
[r] |Process
[<] .input: pg\string     // Looks like a variable
[>] .output: pg\dict >> .result
// Question: Is .input accessible after the pipeline call?
// Answer: NO! It's a pipeline argument, not a variable.
// But the syntax doesn't make this clear!
```

**Issues:**
1. **Namespace confusion** - Pipeline arguments appeared to be scope variables
2. **Misleading syntax** - Developers might try to access `.input` after the call
3. **Unclear semantics** - No visual distinction between three different namespaces

### Solution with `<` and `>` Prefixes

The new prefixes make the distinction crystal clear:

```polyglot
[r] |Process
[<] <input: pg\string     // Clearly NOT a variable in current scope
[>] >output: pg\dict >> .result
// Question: Is <input accessible after the pipeline call?
// Answer: Obviously NO! The < prefix shows it's a pipeline argument.
```

**Benefits:**
1. **Clear namespace distinction** - Three different prefixes for three different namespaces
2. **Self-documenting code** - Arguments are visually distinct from variables
3. **Directional semantics** - `<` points INTO pipeline, `>` points OUT of pipeline
4. **Visual consistency** - Matches `[<]` and `[>]` block markers
5. **Better error messages** - "Undefined output argument `>response`" vs "Undefined variable `.response`"

---

## Complete Namespace Model

| Prefix | Purpose | Scope | Accessible After Call? | Example |
|--------|---------|-------|----------------------|---------|
| `.` | Variable | Current scope | ✅ Yes | `.myvar: pg\int << 42` |
| `<` | Input argument | Pipeline inputs | ❌ No | `<input: pg\string` |
| `>` | Output argument | Pipeline outputs | ❌ No | `>result: pg\dict` |

**Key Insight:** After a pipeline call:
- ❌ `<input` and `>output` arguments DON'T exist (pipeline namespace)
- ✅ `.variable` targets DO exist (current scope)

```polyglot
[r] |Transform
[<] <input: pg\string << .raw_data       // <input in pipeline namespace
[>] >output: pg\dict >> .parsed          // >output in pipeline namespace

// After call:
// ❌ Cannot access: <input, >output (pipeline arguments)
// ✅ Can access: .raw_data, .parsed (scope variables)
```

---

## Examples

### Example 1: Basic Pipeline Call

**Before:**
```polyglot
[r] |FetchData
[<] .url: pg\string << .endpoint
[>] .result: pg\string >> .data
```

**After:**
```polyglot
[r] |FetchData
[<] <url: pg\string << .endpoint
[>] >result: pg\string >> .data
```

---

### Example 2: Multiple I/O Arguments

**Before:**
```polyglot
[r] |Transform
[<] .input: pg\string << .raw
[<] .format: pg\string << "json"
[>] .output: pg\dict >> .parsed
[>] .errors: pg\array{pg\string} >> .errs
```

**After:**
```polyglot
[r] |Transform
[<] <input: pg\string << .raw
[<] <format: pg\string << "json"
[>] >output: pg\dict >> .parsed
[>] >errors: pg\array{pg\string} >> .errs
```

---

### Example 3: Error Handling

**Before:**
```polyglot
[r] |HttpGet
[<] .url: pg\string << "https://invalid-url.example"
[>] .data: pg\string >> .response

// .response becomes Faulted if request fails
```

**After:**
```polyglot
[r] |HttpGet
[<] <url: pg\string << "https://invalid-url.example"
[>] >data: pg\string >> .response

// .response becomes Faulted because >data is Faulted
```

---

### Example 4: Nested Pipeline Calls

**Before:**
```polyglot
[r] |FetchUser
[<] .id: pg\int << .user_id
[>] .user: #User >> .fetched_user

[r] |ProcessUser
[<] .user: #User << .fetched_user
[>] .result: pg\string >> .output
```

**After:**
```polyglot
[r] |FetchUser
[<] <id: pg\int << .user_id
[>] >user: #User >> .fetched_user

[r] |ProcessUser
[<] <user: #User << .fetched_user
[>] >result: pg\string >> .output
```

---

## Files Updated

### Documentation Files (Complete List)

**Operator Documentation:**
1. `docs/user/syntax/operators.md` (v0.0.4 → v0.0.5)
   - Added comprehensive `<` and `>` operator documentation
   - Updated operator table
   - Added namespace distinction section
   - Updated all pipeline examples
   - Updated reserved operators list

**Core User Documentation:**
2. `docs/user/variable-state-system.md`
   - Updated 30+ pipeline I/O instances
   - All `[<] .input:` → `[<] <input:`
   - All `[>] .output:` → `[>] >output:`

3. `docs/user/advanced/variable-states.md`
   - Updated 15+ pipeline I/O instances
   - Updated dependency tracking examples
   - Updated state transition examples

4. `docs/user/ai-quick-reference.md`
   - Added `.`, `<`, `>` to operator table
   - Updated `>>` operator description
   - Updated usage examples

**Example Files:**
5. `docs/user/examples/automation-workflows.md`
   - Updated 10+ pipeline calls
   - Daily report automation example
   - File monitoring example

6. `docs/user/examples/error-handling-patterns.md`
   - Updated retry pattern examples
   - Updated fallback strategy examples
   - Updated circuit breaker examples

7. `docs/user/examples/multi-step-pipelines.md`
   - Updated all pipeline I/O syntax

8. `docs/user/examples/cross-language-integration.md`
   - Updated all pipeline I/O syntax

9. `docs/user/examples/overview.md`
   - Updated all pipeline I/O syntax

**Other User Documentation:**
10. All files in `docs/user/syntax/` directory
11. All files in `docs/user/advanced/` directory
12. All remaining files in `docs/user/` directory

**Total:** 29 markdown files updated

---

## Pattern Transformations

### Automated Replacements

Used sed regex patterns to update all files systematically:

**Pattern 1: Input arguments**
```bash
s/\[<\] \.\([a-zA-Z_][a-zA-Z0-9_]*\):/[<] <\1:/g
```

**Pattern 2: Output arguments**
```bash
s/\[>\] \.\([a-zA-Z_][a-zA-Z0-9_]*\):/[>] >\1:/g
```

**Result:**
- `[<] .url:` → `[<] <url:`
- `[>] .result:` → `[>] >result:`

---

## Not Changed

### Parallel Block Copy-Out

Copy-out markers in parallel blocks continue to use `.variable` syntax:

```polyglot
[p] .parallel_work
    .result1: pg\int << compute1()
    .result2: pg\int << compute2()
    [>] .result1  // Copy out - still uses .variable
    [>] .result2  // Copy out - still uses .variable
[Y] .parallel_work
```

**Reason:** These are copying variables FROM the parallel scope TO the parent scope, not receiving pipeline output arguments.

### Unpack/Join Operators

`~ForEach` and `~Y.Join` continue to use `.variable` syntax for their inputs/outputs:

```polyglot
[p] ~ForEach
[<] .items         // Still uses .variable
[>] .item          // Still uses .variable
[~][r] |ProcessItem
[~][<] <data: pg\any << .item    // Pipeline I/O uses < and >
[~][>] >result: pg\any >> .processed
[~]
```

**Reason:** `[<] .items` and `[>] .item` operate on variables in the current scope, not pipeline arguments.

---

## Impact Analysis

### Breaking Changes

**Parser/Lexer:**
- ✅ Must recognize `<identifier` and `>identifier` as valid tokens
- ✅ Must distinguish three identifier prefixes: `.`, `<`, `>`
- ✅ Must enforce that `<` and `>` only appear in pipeline I/O contexts

**Runtime:**
- ✅ Namespace resolution must handle three different prefixes
- ✅ Error messages must use correct terminology

**Existing Code:**
- ❌ ALL existing `.pg` files with pipeline calls will break
- ❌ Must update all codebases to new syntax

### Non-Breaking Changes

**Conceptual Only:**
- ✅ Semantics remain identical
- ✅ State model unchanged
- ✅ Data flow behavior unchanged

**Documentation:**
- ✅ All documentation updated consistently
- ✅ Examples updated
- ✅ Quick reference updated

---

## Migration Guide

### For Existing Code

**Step 1:** Identify all pipeline calls
```bash
grep -n "\[r\] |" *.pg
```

**Step 2:** Update input arguments
```bash
sed -i 's/\[<\] \.\([a-zA-Z_][a-zA-Z0-9_]*\):/[<] <\1:/g' *.pg
```

**Step 3:** Update output arguments
```bash
sed -i 's/\[>\] \.\([a-zA-Z_][a-zA-Z0-9_]*\):/[>] >\1:/g' *.pg
```

**Step 4:** Manual review
- Verify parallel block copy-outs still use `.variable`
- Verify ~ForEach/~Y.Join still use `.variable`
- Test all pipeline calls

---

## Implementation Checklist

- [x] Design new operator syntax
- [x] Document operators in operators.md
- [x] Update all user documentation
- [x] Update all example files
- [x] Update quick reference
- [x] Create migration guide
- [ ] Update lexer to recognize `<identifier` and `>identifier`
- [ ] Update parser to enforce context restrictions
- [ ] Update runtime namespace resolution
- [ ] Update error messages
- [ ] Update test suite
- [ ] Update LSP server
- [ ] Update syntax highlighter
- [ ] Announce breaking change to users

---

## Success Metrics

| Metric | Target | Achieved | Status |
|--------|--------|----------|--------|
| Documentation files updated | 29 files | 29 files | ✅ Met |
| Operator documentation | Complete | Complete | ✅ Met |
| Examples updated | All | 100% | ✅ Met |
| Quick reference updated | Yes | Yes | ✅ Met |
| Migration guide | Complete | Complete | ✅ Met |
| Consistency | 100% | 100% | ✅ Met |

---

## Conclusion

**Status:** ✅ **DOCUMENTATION COMPLETE**

Successfully introduced `<` and `>` pipeline I/O operators to provide clear namespace distinction:

**Operators Added:**
- `<` - Input argument prefix
- `>` - Output argument prefix
- `.` - Variable prefix (now explicitly documented)

**Documentation Updated:**
- 29 markdown files
- 100+ pipeline call examples
- Complete operator reference
- Migration guide included

**Benefits:**
- Clear namespace distinction (scope variables vs pipeline arguments)
- Self-documenting code (obvious what's an argument vs variable)
- Directional semantics (< into pipeline, > from pipeline)
- Better error messages possible
- Visual consistency with block markers

**Ready For:**
- Parser/lexer implementation
- Runtime updates
- Test suite updates
- User announcement

This change strengthens Polyglot's syntax clarity by making the distinction between pipeline arguments and scope variables visually obvious, reducing confusion and improving code readability.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Change Type:** Breaking Syntax Change - Pipeline I/O Operators
