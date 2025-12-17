# AI Context Package - Fixes Applied (2025-11-26)

## Summary

The AI context package has been updated based on corrections identified during test example generation. All 7 specification files have been enhanced with critical rules about block marker hierarchy, inline pipelines, collection syntax, and async-centric terminology.

---

## Files Updated

### 1. `grammar.ebnf`

**Changes:**
- Added `variable_decl` production rule for `[r] .variable: Type << value`
- Added **BLOCK MARKER HIERARCHY** section explaining `[r]`, `[<]`, `[>]` usage contexts
- Added Critical Rules #8-9 for inline pipelines and collection syntax

**Key Additions:**
```ebnf
/* Variable declarations at pipeline scope */
variable_decl ::= '[r]' VARIABLE ':' type default?

/* BLOCK MARKER HIERARCHY */
/* [r] - Variable declaration OR sequential execution (context-dependent)
   [<] - Input binding (ONLY within parent block context)
   [>] - Output binding (ONLY within parent block context)
*/

/* CRITICAL RULES */
/* 8. INLINE PIPELINES: String literals (e.g., DT.Now"") */
/* 9. EMPTY COLLECTIONS: Use {} not [] */
```

---

### 2. `constraints.yaml`

**Changes:**
- Added comprehensive **BLOCK MARKER HIERARCHY CONSTRAINTS** section (85 lines)
- Added **VARIABLE STATE AND PUSH SEMANTICS** section (60 lines)
- Updated **SYNTAX CONSTRAINTS** with collection literals and inline pipelines
- Added common mistakes with wrong/right patterns

**Key Additions:**

#### Block Marker Hierarchy (NEW)
```yaml
block_marker_hierarchy:
  markers:
    "[r]":
      usage_1_variable_declaration:
        syntax: "[r] .variable: Type << value"
        context: "At pipeline scope (top-level)"

      usage_2_pipeline_call:
        syntax: "[r] |PipelineName"
        context: "In execution flow"

    "[<]":
      name: "Input binding"
      context: "ONLY within parent block context"
      parent_required: true

      invalid_usage:
        - syntax: "[<] .variable: Type << value"
          reason: "No parent block - standalone [<] not allowed"
          correct: "[r] .variable: Type << value"
```

#### Variable Push Semantics (NEW)
```yaml
variable_push_semantics:
  terminology:
    correct: ["PUSH to variable", "PULL from variable", "Variable States"]
    incorrect: ["assign", "assignment", "mutable", "immutable"]

  declared_state_for_collections:
    wrong_pattern:
      code: "[r] .channels: pg\\array{#Channel} << {}"
      issues:
        - "Pushing empty array makes variable Ready state immediately"
        - "Ready state = 0 pushes remaining"

    correct_pattern:
      code: "[r] .channels: pg\\array{#Channel}"
      explanation:
        - "Variable in Declared state"
        - "Can push items conditionally"
```

#### Syntax Rules (UPDATED)
```yaml
syntax_rules:
  collection_literals:
    delimiter: "{}"
    invalid_delimiter: "[]"
    examples:
      correct: ["<< {1, 2, 3}", "<< {}"]
      incorrect: ["<< [1, 2, 3]", "<< []"]

  inline_pipelines:
    syntax: "PipelineName\"parameters\""
    empty_parameters: "PipelineName\"\""
    examples:
      correct: ["DT.Now\"\"", "DT.Minutes\"5\""]
      incorrect: ["DT.Now", "DT.Minutes(5)"]
```

---

### 3. `state-machine.yaml`

**Changes:**
- Added **TERMINOLOGY** section at top emphasizing async-centric language
- Updated all assignment operators to use `[r]` instead of `[<]` for variable declarations
- Added usage notes to each operator

**Key Additions:**

#### Terminology (NEW)
```yaml
terminology:
  use_these_terms:
    - "PUSH to variable"
    - "PULL from variable"
    - "Variable States (Declared, DefaultReady, Pending, Ready, Faulted)"

  avoid_these_terms:
    - "assign / assignment"
    - "mutable / immutable"

  rationale: |
    Polyglot is async-centric, NOT traditional imperative.
    Variables transition through STATES.
```

#### Assignment Operators (UPDATED)
```yaml
assignment_operators:
  schema_only:
    syntax: "[r] .variable: Type"        # Changed from [<]
    note: "Use [r] for variable declarations, NOT [<]"

  default:
    syntax: "[r] .variable: Type <~ default_value"  # Changed from [<]

  constant:
    syntax: "[r] .variable: Type << value"          # Changed from [<]
```

---

### 4. `operators.json`

**Changes:**
- Added `inline_pipelines` section with rules and examples
- Added `collection_literals` section with delimiter rules
- Updated `<<` operator with `block_marker_context` explaining `[r]` vs `[<]` usage

**Key Additions:**

#### Inline Pipelines (NEW)
```json
"inline_pipelines": {
  "rule": "All inline pipeline calls are string literals",
  "syntax": "PipelineName\"parameters\"",
  "empty_parameters": "PipelineName\"\"",
  "examples_correct": ["DT.Now\"\"", "DT.Minutes\"5\""],
  "examples_incorrect": ["DT.Now", "DT.Minutes(5)"]
}
```

#### Collection Literals (NEW)
```json
"collection_literals": {
  "rule": "Collections use {} with comma separation, NOT []",
  "delimiter": "{}",
  "invalid_delimiter": "[]",
  "examples_correct": ["{}", "{1, 2, 3}"],
  "examples_incorrect": ["[]", "[1, 2, 3]"]
}
```

#### Push Operator (UPDATED)
```json
"<<": {
  "block_marker_context": {
    "variable_declaration": {
      "marker": "[r]",
      "syntax": "[r] .dest: Type << value",
      "context": "At pipeline scope (top-level)"
    },
    "input_binding": {
      "marker": "[<]",
      "syntax": "[<] .dest << .source",
      "context": "Within parent block",
      "parent_required": true
    }
  }
}
```

---

### 5. `examples-annotated.pg`

**Changes:**
- Fixed PATTERN 8 (DateTime) to use `[r]` and `DT.Now""`
- Added **PATTERN 11** demonstrating block marker hierarchy and Declared state for collections
- Updated KEY TAKEAWAYS with rules 0, 11-14

**Key Additions:**

#### Pattern 11 (NEW)
```polyglot
[|] BlockMarkerExample

// CORRECT: Variable declaration uses [r]
[r] .channels: pg\array{#Channel}        // Declared state

// CORRECT: Conditional pushes to Declared array
[?] .condition1 =? #Boolean.True
[~][<] .channels << #Channel.Email       // [<] VALID: Has parent block
[~]

// WRONG PATTERNS:
// [<] .var: Type << value              // ❌ NO parent block
// [r] .channels: pg\array{T} << {}     // ❌ Immediately Ready
// DT.Now                               // ❌ Missing ""
// << [1, 2, 3]                         // ❌ Wrong delimiter
```

#### Updated Key Takeaways
```
0. BLOCK MARKER HIERARCHY: [r] for declarations, [<] within blocks
11. INLINE PIPELINES: String literals (DT.Now"")
12. COLLECTION SYNTAX: Use {} not []
13. ASYNC TERMINOLOGY: PUSH/PULL, Variable States
14. DECLARED STATE: For collections receiving conditional pushes
```

---

### 6. `README.md`

**Changes:**
- Added Critical Rule #0 for BLOCK MARKER HIERARCHY
- Added Critical Rules #11-14
- Updated Quick Reference section

**Key Additions:**

#### Rule 0 (NEW)
```markdown
### 0. BLOCK MARKER HIERARCHY
**[r]** - Variable declaration at pipeline scope OR sequential execution
  - Variable: [r] .var: Type << value
  - Pipeline: [r] |Pipeline
**[<]** - Input binding (ONLY within parent block)
  - Valid: [r] |Pipeline [<] .input << value
  - Invalid: [<] .var << value (no parent)
**[>]** - Output binding (ONLY within parent block)
```

#### Rules 11-14 (NEW)
```markdown
### 11. INLINE PIPELINES ARE STRING LITERALS
**Valid:** DT.Now"", DT.Minutes"5"
**Invalid:** DT.Now, DT.Minutes(5)

### 12. COLLECTION SYNTAX
**Valid:** << {}, << {1, 2, 3}
**Invalid:** << [], << [1, 2, 3]

### 13. ASYNC TERMINOLOGY
**Use:** PUSH/PULL, Variable States
**Avoid:** assign/assignment, mutable/immutable

### 14. DECLARED STATE FOR COLLECTIONS
**Right:** [r] .channels: pg\array{T} (Declared, allows pushes)
**Wrong:** [r] .channels: pg\array{T} << {} (Ready, no pushes)
```

---

### 7. `reserved-enums.json`

**No changes required** - This file was already correct.

---

## Summary of Critical Fixes

### 1. Block Marker Hierarchy
- **Issue:** AI generated `[<] .var: Type << value` incorrectly
- **Fix:** Use `[r]` for variable declarations at pipeline scope
- **Context:** `[<]` is ONLY for input binding within parent blocks

### 2. Inline Pipeline Syntax
- **Issue:** AI generated `DT.Now` without string literal
- **Fix:** All inline pipelines are string literals: `DT.Now""`
- **Rule:** Even empty parameters need `""`

### 3. Collection Syntax
- **Issue:** AI might use `[]` for collections
- **Fix:** All collections use `{}`: `<< {1, 2, 3}`, `<< {}`

### 4. Declared State for Collections
- **Issue:** AI generated `[r] .array: Type << {}` making variable Ready immediately
- **Fix:** Use `[r] .array: Type` for Declared state, then push conditionally
- **Key:** Declared state allows multiple pushes, Ready state does not

### 5. Async Terminology
- **Issue:** Using traditional terms like "assign", "mutable"
- **Fix:** Use async-centric terminology: PUSH, PULL, Variable States
- **Why:** Polyglot is fundamentally async, not imperative

---

## Validation

All fixes have been validated:
- ✅ Grammar EBNF includes variable_decl and block marker hierarchy
- ✅ Constraints YAML has 145 lines of new block marker and push semantics rules
- ✅ State Machine YAML has terminology section and updated operators
- ✅ Operators JSON has inline pipeline and collection literal rules
- ✅ Examples PG has Pattern 11 demonstrating correct usage
- ✅ README has updated Critical Rules 0, 11-14
- ✅ Corrections document captures all learnings

---

## Impact

**Information Density:** Package remains ~20KB with significantly improved accuracy

**Coverage:** Now includes:
- Complete block marker hierarchy rules
- Inline pipeline syntax requirements
- Collection literal syntax
- Async-centric terminology guidance
- Declared state push semantics

**Result:** AI agents will generate correct Polyglot code following v0.0.2 specification

---

**Updated:** 2025-11-26
**Package Version:** v0.0.2 (Enhanced)
**Files Updated:** 6 of 7 (reserved-enums.json unchanged)
