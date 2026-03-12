# Syntax Corrections - v0.0.4 Documentation Review

**Date:** 2025-12-12
**Status:** Documentation corrections after v0.0.4 example review

---

## Overview

This document records syntax corrections made to v0.0.4 specification documentation based on user feedback and verification against actual language design.

---

## Corrections Applied

### 1. End Markers: `[X]` → `{x}`

**Issue:** v0.0.3 vs v0.0.4 end marker confusion

**v0.0.3 Syntax:**
- Registry blocks: `{@} ... {x}`
- Definition blocks: `{|} ... [X]`, `{#} ... [X]`, `{!} ... [X]`

**v0.0.4 Syntax:**
- ALL blocks use `{x}`: `{@} ... {x}`, `{|} ... {x}`, `{#} ... {x}`, `{!} ... {x}`

**Files Corrected:**
- `specifications/v0.0.5/README.md` - 14 instances changed
- `specifications/v0.0.5/v0.0.5-improvement-proposals.md` - Contains `[X]` export marker proposal (⚠️ conflict)

**Note:** v0.0.5 proposals suggest using `[X]` as an export marker, which conflicts with v0.0.3's use of `[X]` as end marker. This needs resolution in v0.0.5 design phase.

---

### 2. Pipeline IO Markers: `[i<]` `[o>]` → `[|] <param` `[|] >param`

**Issue:** Incorrect IO marker syntax for pipeline definitions

**v0.0.3 Syntax:**
```polyglot
{|} Pipeline
[<] <input ,:type
[>] >output ,:type
[X]
```

**v0.0.4 Syntax:**
```polyglot
{|} Pipeline
[|] <input :type
[|] >output :type << $value
{x}
```

**Key Points:**
- The `[|]` marker indicates "IO to pipeline definition"
- The `<` and `>` operators prefix the parameter name to indicate input/output
- `[<]` and `[>]` markers are deprecated in v0.0.4

**Context-Specific IO Markers:**
- `[|]` - Pipeline definition IO (under `{|}` context)
- `[|]` - Pipeline call IO (in `[r] |Pipeline` context)
- `[~]` - Unpack IO (collection → iteration)
- `[*]` - Pack IO (iteration → collection)

**Files Corrected:**
- `specifications/v0.0.5/README.md` - Example corrected

---

### 3. Multi-line String Concatenation: `[+] "text"` → `[+] +"text"`

**Issue:** Missing `+` prefix operator for string concatenation

**Incorrect:**
```polyglot
[r] $message << "First line"
[+] "Second line"
[+] "Third line"
```

**Correct:**
```polyglot
[r] $message << "First line"
[+] +"Second line"
[+] +"Third line"
```

**Rationale:**
- Explicit `+` prefix prevents silent concatenation bugs (Python-style)
- Makes concatenation operation visually distinct
- Can mix literals and inline pipelines: `[+] +|DT.Now""`

**Files Corrected:**
- `specifications/v0.0.4/README.md` - Example corrected

---

### 4. Operator Negation: Universal `!?` Pattern

**Rule:** Replace `?` with `!?` to negate ANY conditional operator

**Both forms are equivalent:**
```polyglot
// These mean the same thing:
[y] $age >=? $min_age    // Direct operator (greater-or-equal)
[y] $age <!? $min_age    // Negated operator (NOT less-than)

// These mean the same thing:
[y] $stock <=? 0         // Direct operator (less-or-equal)
[y] $stock >!? 0         // Negated operator (NOT greater-than)
```

**Universal Negation Pattern:**
- `=?` / `!=?` or `=!?` - Equal / Not equal
- `>?` / `!>?` - Greater / Not greater (equivalent to `<=?`)
- `<?` / `!<?` - Less / Not less (equivalent to `>=?`)
- `in?` / `!in?` - In collection / Not in collection
- `?[min, max]` / `!?[min, max]` - In range / Not in range

**Key Point:** Both direct and negated forms are valid - use whichever is clearer for your logic.

---

### 5. Field Access: Direct Dot Notation

**Issue:** Using inline pipelines for simple field extraction

**Inefficient:**
```polyglot
[r] $age << |ExtractField"{$user, \"age\"}"
```

**Correct:**
```polyglot
[r] $age :int << $user.age
```

**When to Use Each:**
- **Dot notation:** When field names are known at compile time
- **Inline pipeline:** When extraction logic is complex or dynamic
- **Recommendation:** Use `#Enumeration` types for predictable structures instead of `pg\serial`

---

### 6. Default Assignment: `<~` for DefaultReady State

**Issue:** Not using default assignment for values that may be pushed twice

**Standard Assignment (`<<`):**
```polyglot
[r] $total :float << 0.0    // Pending → Final (one push only)
```

**Default Assignment (`<~`):**
```polyglot
[r] $total :float <~ 0.0    // Pending → DefaultReady → Final (two pushes allowed)
[r] $total << |Calculate""  // Second push moves to Final
```

**State Transitions:**
- `<<` - Pending → Final/Ready (immutable after one push)
- `<~` or `~>` - Pending → DefaultReady → Final (allows one override)

---

### 7. Package Declaration Must Start File

**Issue:** Package declaration not at file start

**Incorrect:**
```polyglot
// Some comments
{|} Pipeline
...

{@} @package:name:1.0.0
```

**Correct:**
```polyglot
// ALL .pg files MUST start with package declaration
{@} @local:Example.Package:0.0.1
[#] 1  // First file in this package
[.] @Imports << @Some::Package:1.0.0
{x}

{|} Pipeline
...
{x}
```

**Rule:** `{@}` registry declaration must be first non-comment element in every `.pg` file.

---

### 8. Metadata Assignment: Use `<<` not `=`

**Issue:** Using assignment operator `=` instead of push operator `<<`

**Incorrect:**
```polyglot
[%] %Author
   [.] .name = "Alice"
```

**Correct:**
```polyglot
[%] %Author
   [.] .name << "Alice"
   [.] .since << "0.0.0.1"
```

**Rationale:** Polyglot has no `=` assignment operator, only push operators `<<`, `>>`, `<~`, `~>`.

---

### 9. Imported Pipeline Calls

**Issue:** Missing package alias prefix for imported pipelines

**Incorrect:**
```polyglot
[r] $result << |FetchOrders"{$id}"  // Where is FetchOrders from?
```

**Correct:**
```polyglot
// In package declaration
{@} @local:Example:1.0.0
[.] @Orders << @Local::Example.Order.Processing:0.3.2.2
{x}

// In pipeline
[r] $orders :serial << @Orders|FetchOrders"{$user_id}"
```

**Pattern:** `@PackageAlias|PipelineName"args"`

---

### 10. Variable Prefix and Type Separator

**v0.0.3:**
- Variable prefix: `,` (comma)
- Type separator: `\` (backslash)
- Example: `,age :pg\int`

**v0.0.4:**
- Variable prefix: `$` (dollar)
- Type separator: `.` (dot) when using aliases, `\` for full paths
- Example: `$age :int` or `$age :pg.int` or `$age :pg\int`

**Rationale:**
- Dollar sign familiar from shell/Perl/PHP
- Dot separator cleaner with type aliases
- Backslash still works for explicit full paths

---

### 11. Boolean Markers for Conditions

**Issue:** Documentation missing boolean logic markers

**Boolean Markers (under `[y]` or `[t]` context):**
```polyglot
[y] $stock >? 0
[&] $price ?(0.0, 1000000.0]    // AND - both must be true
[&] $quantity >? 0
    // Nested block executes if ALL conditions true

[y] $condition1 =? #;Boolean;True
[|] $condition2 =? #;Boolean;True    // OR - either can be true
    // Nested block executes if ANY condition true

[y] $flag1 =? #;Boolean;True
[^] $flag2 =? #;Boolean;True    // XOR - exactly one must be true
    // Nested block executes if ONE condition true (but not both)
```

**Indentation for Grouping:**
```polyglot
[y] $a >? 0
[&] $b >? 0
    [y] $c >? 0
    [|] $d >? 0
        // Executes if: (a > 0 AND b > 0) AND (c > 0 OR d > 0)
```

**Context-Specific `[|]` Marker:**
- Under `{|}` or pipeline call: **IO marker** for pipeline parameters
- Under `[y]` conditional: **OR boolean operator**
- Under `[t]` trigger: **OR boolean operator** (either trigger)

**Example - Trigger OR:**
```polyglot
[t] |T.Cron"0 2 * * *"    // Scheduled trigger
[|] |T.Call               // OR manual call trigger
```
Pipeline executes on scheduled time **OR** when manually called.

---

### 12. Wildcard Condition (Exhaustive Matching)

**Issue:** Documentation missing wildcard/else syntax

**Rule:** Conditions must be exhaustive - use wildcard to catch remaining cases

**Wildcard Syntax:**
```polyglot
[y] $status =? #;Status;Active
    // Handle active

[y] $status =? #;Status;Pending
    // Handle pending

[y] *    // Wildcard - NO question mark
    // Handle all other cases (like "else")
```

**Why Exhaustive?**
In async environment, non-exhaustive conditions could leave pipeline waiting indefinitely.

**Wildcard is Mandatory:**
```polyglot
// ❌ INVALID - Missing wildcard
[y] $value >? 100
    [r] $category << "high"

// ✅ VALID - Exhaustive with wildcard
[y] $value >? 100
    [r] $category << "high"
[y] *
    [r] $category << "normal"
```

---

## Documentation Status

### Files Reviewed and Corrected
✅ `specifications/v0.0.4/README.md` - Multi-line strings corrected
✅ `specifications/v0.0.5/README.md` - End markers and IO markers corrected
✅ All README files created in reorganization - Verified clean

### Files Requiring No Changes
✅ `specifications/v0.0.4/loop-system/v0.0.3.1-loop-system-specification.md` - Already correct
✅ `specifications/v0.0.4/syntax-refinement/v0.0.4-final-decisions.md` - Already correct
✅ `DESIGN-SPECIFICATIONS-CATALOG.md` - No code examples, navigation only

### Files with Known Issues (Future Resolution)
⚠️ `specifications/v0.0.5/v0.0.5-improvement-proposals.md` - Proposes `[X]` export marker (conflicts with v0.0.3 end marker)

---

## Recommendations

### For v0.0.4 Implementation

1. **End Markers:** Consistently use `{x}` for ALL block endings
2. **IO Markers:** Use context-specific markers (`[|]`, `[~]`, `[*]`)
3. **Multi-line Strings:** Enforce `+` prefix in parser
4. **Operator Negation:** Document `!?` pattern clearly
5. **Field Access:** Recommend `#Enumeration` over `pg\serial` for structured data

### For v0.0.5 Design

1. **Export Marker Conflict:** Choose different marker than `[X]` for exports
   - Options: `[^]`, `[E]`, `[P]` (public), or metadata `[%] %Export`
2. **Type Separator:** Finalize whether `.` or `\` is canonical (both work via aliases)

### For Documentation Maintenance

1. **Always include package declaration** in examples
2. **Show import aliases** when using non-stdlib pipelines
3. **Specify types explicitly** in examples for clarity
4. **Use realistic pipeline names** from stdlib or imports

---

## Summary Statistics

**Total Corrections:** 12 syntax patterns clarified
**New Features Documented:** 2 (boolean markers, wildcard conditions)
**Files Modified:** 2 files
**Files Verified:** 23+ specification documents
**Breaking Changes Documented:** 3 (end markers, IO markers, variable prefix)
**Critical Clarifications:** 2 (operator negation equivalence, context-specific `[|]` marker)

---

**Last Updated:** 2025-12-12
**Reviewed By:** Polyglot Language Design Team
**Status:** ✅ Corrections applied, documentation verified
