# Push/Pull Operators Clarification

**Date:** 2025-12-03
**Type:** Conceptual Clarification
**Status:** ✅ **COMPLETE**
**Scope:** Clarified that `<<`, `>>`, `<~`, and `~>` are bidirectional push/pull operators

---

## Summary

Corrected documentation to reflect that Polyglot's data flow operators are **bidirectional push/pull operators** - they simultaneously pull data from the source and push data to the destination. This is not just "push" or "pull" - it's **both operations** happening together.

---

## The Misconception

### ❌ INCORRECT (Previous Understanding):

**Operator Names:**
- `<<` - "PUSH Operator" (push only)
- `>>` - "PULL Operator" (pull only)
- `<~` - "Default PUSH" (push only)
- `~>` - "Default PULL" (pull only)

**Problem:** These names implied unidirectional operations, but the reality is bidirectional.

---

## The Correct Understanding

### ✅ CORRECT: Bidirectional Push/Pull Operators

**All four operators perform BOTH operations:**

1. **Pull** data from source
2. **Push** data to destination

**Operator Semantics:**

| Operator | Operation | Pull From | Push To | Direction |
|----------|-----------|-----------|---------|-----------|
| `<<` | Push/Pull Left | Right side (source) | Left side (destination) | `dest << src` |
| `>>` | Push/Pull Right | Left side (source) | Right side (destination) | `src >> dest` |
| `<~` | Default Push/Pull Left | Right side (default source) | Left side (destination) | `dest <~ src` |
| `~>` | Default Push/Pull Right | Left side (default source) | Right side (destination) | `src ~> dest` |

---

## Why This Matters

### Conceptual Clarity

**Every data flow operation involves:**
1. **Pull:** Evaluating/accessing the source
2. **Push:** Assigning/delivering to the destination

**Examples:**

```polyglot
// Pull from literal 42, push to .number
.number: pg\int << 42

// Pull from .source, push to .dest
.dest: pg\string << .source

// Pull from >output (pipeline), push to .result (caller)
[>] >output: pg\dict >> .result

// Pull default from 30, push to .timeout
[i] .timeout: pg\int <~ 30
```

---

## Updated Documentation Sections

### 1. `<<` - Push/Pull Left Operator

**Before:**
```markdown
## The PUSH Operator `<<`

The PUSH operator assigns a value to a variable...
```

**After:**
```markdown
## The Push/Pull Operator `<<`

The `<<` operator is a **bidirectional push/pull operator** that pulls data from the source (right) and pushes it to the destination (left).

**Operation:** `destination << source`
- **Pull:** Data is pulled from the source (right side)
- **Push:** Data is pushed to the destination (left side)
```

---

### 2. `>>` - Push/Pull Right Operator

**Before:**
```markdown
## The PULL Operator `>>`

The PULL operator is used in pipeline outputs to pull data from pipeline outputs into variables.
```

**After:**
```markdown
## The Push/Pull Operator `>>`

The `>>` operator is a **bidirectional push/pull operator** that pulls data from the source (left) and pushes it to the destination (right).

**Operation:** `source >> destination`
- **Pull:** Data is pulled from the source (left side)
- **Push:** Data is pushed to the destination (right side)
```

---

### 3. `<~` - Default Push/Pull Left Operator

**Before:**
```markdown
### Default PUSH `<~`

Creates variable in **Default** state with a default value...
```

**After:**
```markdown
### Default Push/Pull Left `<~`

**Operation:** `destination <~ source`
- **Pull:** Default value pulled from source (right side)
- **Push:** Default pushed to destination (left side)
- **Override:** Can be overridden once before becoming Final
```

---

### 4. `~>` - Default Push/Pull Right Operator

**Before:**
```markdown
### Default PULL `~>`

Creates variable in **Default** state, pulling default value from `.source`.
```

**After:**
```markdown
### Default Push/Pull Right `~>`

**Operation:** `source ~> destination`
- **Pull:** Default value pulled from source (left side)
- **Push:** Default pushed to destination (right side)
- **Override:** Can be overridden once before becoming Final
```

---

## Enhanced Examples

### Example 1: Variable Assignment

**Old Documentation:**
```polyglot
.result: pg\int << 42  // Push 42 to .result
```

**New Documentation:**
```polyglot
.result: pg\int << 42  // Pull 42, push to .result → Final
```

**Explanation:** Explicitly shows both pull (from literal) and push (to variable).

---

### Example 2: Variable Copy

**Old Documentation:**
```polyglot
.copy: pg\string << .source  // Push .source to .copy
```

**New Documentation:**
```polyglot
.copy: pg\string << .source  // Pull from .source, push to .copy
```

**Explanation:** Shows data flows FROM source TO destination.

---

### Example 3: Pipeline Output

**Old Documentation:**
```polyglot
[>] >response: pg\dict >> .http_response
// .http_response is Pending until pipeline completes
```

**New Documentation:**
```polyglot
[>] >response: pg\dict >> .http_response
// Pull from >response (pipeline output), push to .http_response (caller)
// .http_response is Pending until |HttpGet completes
```

**Explanation:** Clarifies that data flows FROM pipeline output TO caller variable.

---

### Example 4: Default Values

**Old Documentation:**
```polyglot
[i] .timeout: pg\int <~ 30  // Default with value 30
```

**New Documentation:**
```polyglot
[i] .timeout: pg\int <~ 30
// Pull 30 (default), push to .timeout → Default state
```

**Explanation:** Shows both pull (default value) and push (to input variable).

---

## Pipeline Chaining Clarity

**Enhanced documentation showing push/pull flow:**

```polyglot
[r] |HttpGet
[<] <url: pg\string
[>] >response: pg\dict >> .http_response
// Pull from >response, push to .http_response

[r] |ParseJson
[<] <json_string: pg\string << .http_response
// Pull from .http_response, push to <json_string
[>] >parsed: pg\dict >> .json_data
// Pull from >parsed, push to .json_data

[r] |ExtractUser
[<] <data: pg\dict << .json_data
// Pull from .json_data, push to <data
[>] >user: pg\dict >> .user_data
// Pull from >user, push to .user_data
```

**Benefits:**
- Clear data flow visualization
- Explicit source and destination for each operation
- Shows bidirectional nature of every operator

---

## Conceptual Benefits

### 1. Consistent Mental Model

**Every operator follows same pattern:**
- Identify source (where data comes from)
- Identify destination (where data goes)
- Pull from source → Push to destination

### 2. Symmetric Understanding

**Operators are symmetric pairs:**
- `<<` and `>>` are mirrors (left vs right)
- `<~` and `~>` are mirrors (left vs right)
- Both pairs have same push/pull semantics, just different directions

### 3. Explicit Data Flow

**Comments now show complete flow:**
```polyglot
.a: pg\int << 10                // Pull 10, push to .a
.b: pg\int << 20                // Pull 20, push to .b
.sum: pg\int << U.Int.Add"{.a, .b}"  // Pull sum result, push to .sum
```

Every line explicitly shows where data comes from and where it goes.

---

## Terminology Update

### Operator Naming Convention

**Official Names:**
- `<<` - Push/Pull Left Operator
- `>>` - Push/Pull Right Operator
- `<~` - Default Push/Pull Left Operator
- `~>` - Default Push/Pull Right Operator

**Shorthand (Acceptable):**
- `<<` - Left operator
- `>>` - Right operator
- `<~` - Default left operator
- `~>` - Default right operator

**Avoid (Misleading):**
- ❌ "PUSH operator" (implies only pushing)
- ❌ "PULL operator" (implies only pulling)

---

## Teaching Points

### For New Polyglot Developers

**Key Concept:** Every data flow involves TWO operations:
1. **Pull** - Getting data from source
2. **Push** - Putting data into destination

**Mnemonic:**
- Arrow points to destination
- Data flows FROM source TO destination
- Both pull and push happen

**Example Walkthrough:**
```polyglot
.result: pg\int << U.Int.Add"{.a, .b}"
```

**What happens:**
1. **Pull:** Evaluate `U.Int.Add"{.a, .b}"` (pull from .a and .b)
2. **Push:** Assign result to `.result` (push to destination)
3. **Transition:** `.result` → Final

---

## Files Updated

**Main Documentation:**
1. `docs/user/variable-state-system.md`
   - Section: "The Push/Pull Operator `<<`" (lines 575-638)
   - Section: "The Push/Pull Operator `>>`" (lines 640-726)
   - Section: "The DEFAULT Push/Pull Operators `<~` and `~>`" (lines 727-794)

**Changes:**
- Updated section titles
- Added bidirectional push/pull explanations
- Enhanced all examples with pull/push comments
- Clarified operator semantics

---

## Impact on Other Documentation

**Files to Review/Update:**
- `docs/user/syntax/operators.md` - Operator reference
- `docs/user/ai-quick-reference.md` - Quick reference guide
- All example files - Update inline comments

**Search Pattern:**
```bash
# Find all operator usage comments
grep -rn "Push\|pull" docs/user --include="*.md"
```

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Sections updated | 4 sections | ✅ Complete |
| Examples enhanced | 10+ examples | ✅ Complete |
| Terminology standardized | All operators | ✅ Complete |
| Conceptual clarity | Bidirectional | ✅ Complete |

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully clarified that Polyglot's data flow operators are bidirectional push/pull operators:

**Key Clarification:**
- All operators (`<<`, `>>`, `<~`, `~>`) perform BOTH push and pull
- Source is pulled, destination is pushed
- Direction arrow indicates which side is destination

**Benefits:**
- **Conceptual Clarity:** Explicit understanding of data flow
- **Symmetric Understanding:** Operators are directional pairs
- **Better Documentation:** Comments show complete data flow
- **Easier Teaching:** Clear mental model for new developers

**Core Principle:**
> "Regardless of direction, the source is pulled and the destination is pushed."

This bidirectional understanding makes Polyglot's data flow semantics clear and consistent.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Clarification Type:** Operator Semantics - Bidirectional Push/Pull
