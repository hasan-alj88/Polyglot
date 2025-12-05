# Block Markers Documentation Updates - 2025-12-02

**File Updated:** `docs/user/syntax/block-markers.md`
**Updated By:** Mai (Secretary)
**Date:** 2025-12-02

---

## Summary of Changes

### 1. Added Range Interval Operators to Conditionals

**Section:** `[?]` Conditional (Switch)

**Added:**
- `?[a,b]` - Closed interval (inclusive both ends)
- `?[a,b)` - Half-open interval (inclusive start, exclusive end)
- `?(a,b]` - Half-open interval (exclusive start, inclusive end)
- `?(a,b)` - Open interval (exclusive both ends)

**Example:**
```polyglot
[?] .value ?[50,100)               // 50 <= value < 100
[~][r] .result: pg\string << "medium"
[~]
```

---

### 2. Clarified Push/Pull Direction

**Section:** `[<]` Input Binding and `[>]` Output Binding

**Added Explanation:**
> **Push and Pull direction:** When reading left-to-right, `<<` pushes data (from right to left), while `>>` pulls data (from left to right). The direction depends on reading perspective - one direction is always push, the other is pull.

**Added Link:** [Pipeline Definition](../pipeline-definition.md) for data flow patterns.

---

### 3. Variable Declaration in All Execution Blocks

**Section:** `[r]` Run (Sequential) OR Variable Declaration

**Updated:**
- Clarified that variable declaration can be done in **all execution blocks** (`[r]`, `[p]`, `[b]`, `[s]`, `[?]`, etc.), not just `[r]`
- Added example showing declaration in nested scopes and conditional blocks
- Added link to [Variable State System](../variable-state-system.md)

---

### 4. Documented Unpack and Pack Operators

**Section:** `[Y]` Join Point

**Added:**
- **`~*`** - Unpack operator (expands collection into individual items)
- **`~Y.*`** - Pack operator (collects individual items back into collection)

**Added Link:** [Handling Collections](../handling-collections.md) for collection operations.

---

### 5. Major Format Change: Error/Input/Output Markers

**Global Change Throughout Document:**

**Old Format → New Format:**
- `[i] !No.Input` → `[i] !No.Input`
- `[o] !No.Output` → `[o] !No.Output`

**Locations Updated:**
- Input Declaration section examples
- Output Declaration section examples
- Mandatory Pipeline Structure section
- Minimal pipeline example

---

### 6. Added Code Continuation Convention

**Section:** `[X]` Block Terminator

**Added Note:**
> `...` indicates additional code not relevant to this example.

This clarifies that `...` is used to show there is more code that's not relevant to the feature/concept being introduced.

---

### 7. Clarified Implicit Trigger Behavior

**Section:** `[i]` Input Declaration

**Updated:**
- Emphasized that `[i]` triggers when input variable reaches the `Ready` state (not just "becomes Ready")
- Added state references in code comments
- Enhanced link to Variable State System documentation

---

### 8. Added `<~` PUSH Default Operator Documentation

**New Section:** Special Data Flow Operators

**Added Complete Documentation:**

```polyglot
[i] .timeout: pg\int <~ 30         // DefaultReady state
[i] .retries: pg\int <~ 3          // Can override
...
[r] .timeout << 60                 // Override (1 push remaining)
[r] .timeout << 20                 // Compile Error: No more push allowed
```

**Key Point:** Default values can only be overridden **once**. A second push attempt results in a compile-time error.

---

## Documentation Cross-References Added

The following cross-reference links were added throughout the document:

1. **Variable State System** - Links to `../variable-state-system.md`
   - From: `[i]` Input Declaration
   - From: `[r]` Variable Declaration

2. **Pipeline Definition** - Links to `../pipeline-definition.md`
   - From: `[<]` Input Binding

3. **Handling Collections** - Links to `../handling-collections.md`
   - From: `[Y]` Join Point

These links connect introduced concepts to their detailed documentation:
- Variable Declaration → Variable states
- Pipeline Operator → Pipeline definition
- Unpack\Pack Operator → Handling collections

---

## Files That May Need Similar Updates

Based on these corrections, the following files should be audited for similar errors:

1. **Pipeline-related docs** - Check for old `!No.Input` format
2. **Error handling docs** - Check for old `!No.Output` format
3. **Operator docs** - Ensure `<~` default push is documented correctly
4. **Examples** - Verify all examples use new format consistently
5. **Type system docs** - Check variable declaration examples

**Recommendation:** Run a global search for:
- `!No.Input` (replace with `!No.Input`)
- `!No.Output` (replace with `!No.Output`)
- Missing range operators `?[a,b)` and `?(a,b]`

---

## Validation Checklist

- [x] All occurrences of `!No.Input` replaced with `!No.Input`
- [x] All occurrences of `!No.Output` replaced with `!No.Output`
- [x] Range operators `?[a,b)` and `?(a,b]` documented
- [x] Push/Pull direction clarified with reading perspective
- [x] Variable declaration in all blocks clarified
- [x] Unpack (`~*`) and Pack (`~Y.*`) operators documented
- [x] Documentation cross-references added
- [x] `<~` PUSH Default operator documented with compile error example
- [x] `...` code continuation convention explained
- [x] Implicit trigger `Ready` state clarified

---

## Next Actions

1. **Audit other documentation files** for similar format errors
2. **Update code examples** in docs/user/examples/ directory
3. **Review parser implementation** to ensure it supports new formats
4. **Update test suites** to use new format conventions
5. **Create migration guide** if breaking changes affect existing code

---

**Status:** ✅ Complete
**Reviewed By:** Pending
**Approved By:** Pending
