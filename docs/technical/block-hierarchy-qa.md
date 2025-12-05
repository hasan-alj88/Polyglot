# Block Hierarchy - Questions & Answers

**Date:** 2025-11-20
**Status:** ✅ RESOLVED - All questions answered
**Session:** hhj clarifications 2025-11-20

---

## 1. Complete Parent-Child Relationships ✅ RESOLVED

**Answer:** YES - Complete hierarchy documented

### File Scope Structure:

```
Polyglot File (.pg)
│
├─ [@] Package Declaration (MANDATORY, FIRST, ONE instance)
│  ├─ Header: [@] Registry@Package.Full.Name:Major.Minor.Patch
│  ├─ [#] FileNumber (REQUIRED - inside [@])
│  ├─ [<] alias << registry@import.package:Major.Minor.Patch (imports)
│  └─ [X] (ENDS with [X])
│
├─ [#] Enumeration.Full.Path (Optional, multiple, sibling to [@])
│  ├─ [<] .variant: type << value
│  ├─ [A] AliasName
│  └─ [X]
│
├─ [!] Error.Full.Path (Optional, multiple, sibling to [@])
│  ├─ [<] .field: type << value
│  └─ [X]
│
├─ [M] Macro.Full.Name (Optional, multiple, sibling to [@])
│  ├─ [<] Macro.include"<chars+" (REQUIRED)
│  ├─ [{] .input: type (Scope input - optional)
│  ├─ [}] .output: type (Scope output - optional)
│  ├─ [i], [i], [t], [\], [r], [p], [s], [b], [Y], [/], [o] (optional)
│  └─ [X]
│
└─ [|] Pipeline.Full.Name (Optional, multiple, sibling to [@])
   ├─ [i] Input (REQUIRED in canonical - or [i] #None)
   ├─ [i] Constants (optional)
   ├─ [t] Trigger (REQUIRED in canonical)
   │  ├─ [<] Inputs
   │  └─ Boolean Blocks: [+], [&], [^], [.], [~], [-]
   ├─ [Q] Queue (optional)
   ├─ [W] Wrapper/Unwrap (optional - UNIQUE to pipelines)
   │  ├─ [<] Wire [{] inputs
   │  └─ [>] Wire [}] outputs
   ├─ [\] Setup (REQUIRED in canonical)
   ├─ Execution (REQUIRED - at least ONE of):
   │  ├─ [r] Sequential
   │  ├─ [p] Parallel (must join via [Y])
   │  ├─ [s] Serial load
   │  └─ [b] Background
   ├─ [Y] Join (if [p] used)
   ├─ [?] Switch
   │  ├─ Boolean Blocks (same as [t])
   │  └─ [~][r] Nested operations
   ├─ [!] Error handling
   ├─ [/] Cleanup (REQUIRED in canonical)
   ├─ [o] Output (REQUIRED in canonical - or [o] !No.Output)
   └─ [X]
```

**Key Points:**
- File scope blocks ([#], [!], [M], [|]) are SIBLINGS after [@]
- [@] CONTAINS [#] FileNumber and [<] imports, ENDS with [X]
- Canonical requirements apply to FINAL RENDERED pipeline (after macro unwrapping)

---

## 2. Ambiguities Resolved ✅

### A. Block `[W]` Wrapper ✅ RESOLVED

**Answer:** `[W]` is a child of `[|]` pipeline (UNIQUE to pipelines)

**Details:**
- **Location:** Inside `[|]` Pipeline definition only
- **Purpose:** Unwrap/inline a macro at compile-time
- **Syntax:** `[W] |MacroName`
- **Contains:**
  - `[<] .input << source` - Wire [{] scope inputs from macro
  - `[>] .output >> target` - Wire [}] scope outputs from macro
- **Behavior:** Compile-time inline - macro blocks merge by TYPE with pipeline blocks
- **Not in macros:** `[W]` only appears in pipelines, not in `[M]` definitions

---

### B. Block `[m]` Macro-exported ✅ RESOLVED

**Answer:** `[m]` was REPLACED by `[{]` and `[}]` scope flow pair

**Details:**
- `[m]` was a transitional proposal (brainstorming-2025-11-16)
- **Superseded by:** `[}]` Scope Output in carson-2025-11-18 macro system spec
- **Current design:** Variables declared with `[}]` inside `[M]` become available to caller
- **No `[m]` block exists** in final design

**Macro scope flow:**
- `[{]` - Scope INPUT (variables FROM caller INTO macro)
- `[}]` - Scope OUTPUT (variables FROM macro TO caller)

---

### C. Block `[*]` Line Continuation ✅ RESOLVED

**Answer:** `[*]` is purely syntactic (not part of hierarchy)

**Details:**
- **Purpose:** Continue previous line
- **Context:** Can appear after any line that needs continuation
- **Usage:**
  - String concatenation: `[*] +"text"`
  - Type continuation: `[*]` on next line for long types
  - Value continuation: `[*]` for multi-line values
- **NOT for:** Adding new parameters (use `[<]` for each parameter)
- **Hierarchy impact:** None - treated as same line by parser

---

### D. Setup `[\]` and Cleanup `[/]` ✅ RESOLVED

**Answer:** Both REQUIRED in canonical form, specific placement

**Placement Rules:**
- **Setup `[\]`:** After `[Q]`, before ALL execution blocks
- **Cleanup `[/]`:** After ALL execution blocks, before `[o]`
- **Canonical order:** `[Q] → [\] → [r],[p],[s],[b],[Y] → [/] → [o]`

**Multiple Blocks:**
- Can have multiple `[\]` and `[/]` blocks from different macros
- **FIFO order:** Setup blocks execute in order unwrapped
- **LIFO order:** Cleanup blocks execute in REVERSE order (RAII pattern)

**Interleaving:**
- NOT interleaved with execution blocks
- All setup together, then execution, then all cleanup together

**Macros:**
- Pipeline can satisfy requirement via macro unwrapping
- Macro provides `[\]` and `[/]`, pipeline doesn't need explicit ones

---

### E. Error Handling `[!]` ✅ RESOLVED

**Answer:** YES - `[!]` has dual meaning by context

**Context 1: File Scope (Error Definition)**
```polyglot
[!] Error.Full.Path
[<] .field: type << value
[X]
```
- **Role:** Define custom error type
- **Type:** Type definition
- **Must not be:** Reserved or extendable error type

**Context 2: Pipeline Level (Error Handler)**
```polyglot
[!] .var.error =? !ErrorType     // Variable-level
[s][!] !ErrorType                // Scope-level (serial)
[~][!] !ErrorType                // Previous block
```
- **Role:** Catch and handle errors
- **Type:** Control flow
- **Precedence:** `[~][!]` (specific) > `[s][!]` (general)

**Parser Distinction:**
- File scope: Followed by error path name, ends with `[X]`
- Pipeline level: Followed by condition or error type, contains handler blocks

---

### F. Switch Block `[?]` Nesting ✅ RESOLVED

**Answer:** Can nest `[r]`, `[p]`, `[s]`, `[b]` with `[~]` prefix

**Nesting Rules:**
- **Operations:** `[~][r]`, `[~][p]`, `[~][s]`, `[~][b]`
- **`[~]` required:** YES - for explicit nesting under `[?]`
- **Boolean blocks:** Can use `[+]`, `[&]`, `[^]`, `[.]`, `[-]` (same as `[t]`)

**Example:**
```polyglot
[?] .status =? #Status.Success
[~][r] |HandleSuccess           // Nested operation
[~][p] |ParallelProcess         // Can nest parallel
```

---

### G. Parallel Block `[p]` Nesting ✅ RESOLVED

**Answer:** YES - need `[~]` for nested operations

**Nesting Rules:**
- **Nested operations:** `[~][r]`, `[~][p]`, `[~][s]`, `[~][b]`, `[~][?]`
- **`[~]` required:** YES - for explicit nesting
- **Nest `[p]` inside `[p]`:** YES - can nest parallel inside parallel
- **`[?]` inside `[p]`:** YES - can nest switch inside parallel

**Join requirement:**
- If `[p]` used, MUST join via `[Y]` before continuing

**Example:**
```polyglot
[p] |ParallelOp
[~][r] |NestedSequential        // Nested operation
[~][p] |NestedParallel          // Nested parallel
[~][?] .condition =? value      // Nested switch
[Y]                             // Required join
```

---

### H. Expansion Prefix `[~]` Rules ✅ RESOLVED

**Answer:** `[~]` required for explicit nesting, implicit at file scope

**REQUIRED `[~]` (Explicit Nesting):**
- Inside `[p]` Parallel: `[~][r]`, `[~][p]`, etc.
- Inside `[?]` Switch: `[~][r]`, `[~][p]`, etc.
- Inside `[.]` Boolean group: `[~][t]`, `[~][+]`, etc.
- Nested groups: `[~][~][t]` for double nesting

**IMPLICIT `[~]` (No prefix needed):**
- File scope: All top-level definitions (`[@]`, `[#]`, `[!]`, `[M]`, `[|]`) have implicit first `[~]`
- First-level operations in pipeline: Direct children don't need `[~]`

**Visual Nesting Depth:**
- Number of `[~]` prefixes shows nesting level
- `[~][t]` = level 1
- `[~][~][t]` = level 2
- `[~][~][~][t]` = level 3

---

## 3. Canonical Order Questions ✅ RESOLVED

**Canonical Order:**
```
[t] → [i],[i] → [Q] → [\] → [r],[p],[s],[b],[Y] → [/] → [o] → [X]
```

### Question 1: Is this order STRICT in canonical form? ✅

**Answer:** YES - Strict order in canonical form

**After macro unwrapping, blocks MUST appear in this order:**
1. `[t]` Trigger (REQUIRED)
2. `[i]` Input and/or `[i]` Constants (REQUIRED - or `[i] #None`)
3. `[Q]` Queue (optional)
4. `[\]` Setup (REQUIRED)
5. Execution blocks: `[r]`, `[p]`, `[s]`, `[b]`, `[Y]` (at least ONE required)
6. `[/]` Cleanup (REQUIRED)
7. `[o]` Output (REQUIRED - or `[o] !No.Output`)
8. `[X]` End

**Validation:** Compiler enforces canonical order after macro expansion

---

### Question 2: Can blocks appear multiple times? ✅

**Answer:** YES - Most blocks can appear multiple times

**Multiple instances allowed:**
- `[i]` - Multiple input parameters
- `[i]` - Multiple constants
- `[r]` - Multiple sequential operations
- `[p]` - Multiple parallel operations
- `[s]` - Multiple serial loads
- `[b]` - Multiple background operations
- `[?]` - Multiple switches
- `[!]` - Multiple error handlers

**Single instance only:**
- `[@]` - ONE package declaration per file
- `[t]` - ONE trigger section (but can have multiple conditions inside)
- `[Q]` - ONE queue assignment
- `[\]` - ONE setup section (but can contain multiple operations)
- `[/]` - ONE cleanup section (but can contain multiple operations)
- `[o]` - ONE output section (but can have multiple outputs)

**Note:** "ONE section" means blocks of that type are grouped together in canonical order, even if they appear multiple times in source.

---

### Question 3: Can execution blocks be interleaved? ✅

**Answer:** NO - Blocks merge by TYPE in canonical form

**Canonical Behavior:**
- All `[r]` blocks merge together
- All `[p]` blocks merge together
- All `[s]` blocks merge together
- All `[b]` blocks merge together

**Example:**
```polyglot
// Source order:
[r] |Op1
[p] |Op2
[r] |Op3

// Canonical form (after compilation):
[r] |Op1
[r] |Op3
[p] |Op2
[Y]  // Join required
```

**Execution order:** Within each type group, maintains source order

---

## 4. Context-Dependent Blocks ✅ RESOLVED

### `[#]` Dual Meaning:

**Context 1: Inside [@] Package Declaration**
```polyglot
[@] Registry@Package.Full.Name:1.0.0
[#] 001                         // File number
[X]
```

**Context 2: File Scope (sibling to [@])**
```polyglot
[#] Enumeration.Full.Path
[<] .variant: type << value
[X]
```

**Parser Detection:** Position determines context:
- Inside `[@]` block = File number
- At file scope = Enumeration definition

---

### `[!]` Dual Meaning:

**Context 1: File Scope**
```polyglot
[!] Error.Full.Path
[<] .field: type
[X]
```

**Context 2: Inside Pipeline**
```polyglot
[!] .var.error =? !ErrorType
[r] |HandleError
```

**Parser Detection:**
- File scope with `[X]` = Error definition
- Inside pipeline = Error handler

---

### `[<]` Five Contexts:

1. **Import (inside [@]):** `[<] alias << registry@import.package:1.0.0`
2. **Define variant (inside [#]):** `[<] .variant: type << value`
3. **Macro include (inside [M]):** `[<] Macro.include"{\\/"`
4. **Pass input (operations):** `[<] .param: type << value`
5. **Wire scope input (inside [W]):** `[<] .input << source`

---

### `[>]` Three Contexts:

1. **Capture output (operations):** `[>] .result >> variable`
2. **Wire scope output (inside [W]):** `[>] .output >> target`
3. **Extract error data (inside [!]):** `[>] .message >> err_msg`

---

### Other Context-Dependent Blocks:

**None identified** - All other blocks have single, consistent meaning across contexts.

---

## 5. Block Pairing/Closing ✅ RESOLVED

### Blocks Requiring Closing:

**File Scope Definitions (close with [X]):**
- `[@]` ... `[X]` - Package declaration
- `[#]` ... `[X]` - Enumeration definition
- `[!]` ... `[X]` - Error definition
- `[M]` ... `[X]` - Macro definition
- `[|]` ... `[X]` - Pipeline definition

**Boolean Logic Blocks (close by context - NO explicit closing markers):**
- `[+]` - OR modifier (closes when nesting level changes)
- `[&]` - AND modifier (closes when nesting level changes)
- `[^]` - XOR modifier (closes when nesting level changes)
- `[.]` - Group modifier (closes when nesting level changes)
- `[-]` - NOT modifier (closes when nesting level changes)

**Not Paired (single-line or scoped by parent):**
- Operation blocks: `[r]`, `[p]`, `[s]`, `[b]`
- Other markers: `[i]`, `[i]`, `[t]`, `[\]`, `[/]`, `[o]`, `[Y]`, `[?]`, `[!]`

---

### Nesting Depth:

**Answer:** No documented maximum nesting depth

**Practical limits:**
- Parser implementation may impose limits
- Readability suggests avoiding deep nesting (>3-4 levels)
- Compiler should validate balanced pairs

**Arbitrary nesting allowed:**
- Boolean blocks can nest arbitrarily: `[+]` inside `[&]` inside `[.]`
- `[~]` prefix depth shows nesting level

---

## 6. Missing Information ✅ RESOLVED

### 1. Can `[b]` Background blocks have `[>]` output? ✅

**Answer:** NO - Background is fire-and-forget

**Rationale:**
- `[b]` never joins back to main pipeline
- No synchronization point to receive output
- Use `[<]` for inputs, but NO `[>]` for outputs

**Example:**
```polyglot
[b] |U.ProcessInBackground
[<] .data: pg\string << .input   // ✅ Can receive input
// NO [>] output - never joins
```

---

### 2. Can boolean blocks appear anywhere other than `[t]` and `[?]`? ✅

**Answer:** NO - Only in `[t]` Trigger and `[?]` Switch

**Boolean blocks:** `[+]`, `[&]`, `[^]`, `[.]`, `[~]`, `[-]`

**Valid contexts:**
- Inside `[t]` - Trigger conditions
- Inside `[?]` - Switch conditions

**NOT valid elsewhere:**
- Cannot use in `[r]`, `[p]`, `[s]`, `[b]`
- Cannot use at file scope

---

### 3. Maximum nesting depth? ✅

**Answer:** No documented limit

**Recommendation:** Keep nesting shallow for readability (max 3-4 levels)

**Visual indicator:** Number of `[~]` prefixes shows depth

---

### 4. Can `[!]` error handlers appear at any level? ✅

**Answer:** YES - Three levels of error handling

**Levels:**
1. **Variable-level:** `[!] .var.error =? !ErrorType`
2. **Scope-level:** `[s][!] !ErrorType` (for serial blocks)
3. **Block-level:** `[~][!] !ErrorType` (previous block)

**Precedence:** Specific (`[~][!]`) > General (`[s][!]`)

**Implicit handling:** If no explicit handler, automatic notification/logging

---

## Summary of Resolved Questions

| Question | Status | Answer Summary |
|----------|--------|----------------|
| 1. Parent-child relationships | ✅ RESOLVED | Complete hierarchy documented |
| 2A. [W] Wrapper | ✅ RESOLVED | Inside [|] pipelines only |
| 2B. [m] Macro-exported | ✅ RESOLVED | Replaced by [{] and [}] |
| 2C. [*] Line continuation | ✅ RESOLVED | Syntactic only |
| 2D. [\] [/] Setup/Cleanup | ✅ RESOLVED | Both required, specific placement |
| 2E. [!] Error handling | ✅ RESOLVED | Dual meaning by context |
| 2F. [?] Switch nesting | ✅ RESOLVED | Can nest r/p/s/b with [~] |
| 2G. [p] Parallel nesting | ✅ RESOLVED | Need [~] for nested ops |
| 2H. [~] Expansion rules | ✅ RESOLVED | Required for explicit nesting |
| 3.1. Canonical order strict? | ✅ RESOLVED | YES - strict order |
| 3.2. Multiple instances? | ✅ RESOLVED | YES - most blocks |
| 3.3. Interleaving? | ✅ RESOLVED | NO - merge by TYPE |
| 4. Context-dependent | ✅ RESOLVED | [#], [!], [<], [>] |
| 5. Block pairing | ✅ RESOLVED | All pairs identified |
| 6.1. [b] output? | ✅ RESOLVED | NO - fire-and-forget |
| 6.2. Boolean blocks elsewhere? | ✅ RESOLVED | NO - only [t] and [?] |
| 6.3. Max nesting? | ✅ RESOLVED | No limit documented |
| 6.4. [!] at any level? | ✅ RESOLVED | YES - 3 levels |

---

**Document Status:** ✅ COMPLETE - All questions resolved
**Date:** 2025-11-20
**Source:** hhj clarifications + block-hierarchy-reference.md
**Cross-Reference:** See `docs/technical/block-hierarchy-reference.md` for complete hierarchy
