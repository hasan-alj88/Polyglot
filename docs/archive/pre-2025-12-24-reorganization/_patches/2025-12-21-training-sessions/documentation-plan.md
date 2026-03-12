# Documentation Update Plan
**Generated:** 2025-12-21
**Source:** Training Sessions 1 & 2 + Scan Reports (Dec 19)
**Agent:** Polly v1.0.0
**Status:** Ready for implementation

---

## Executive Summary

This plan addresses documentation for **6 major learnings** from training sessions plus **7 features** from scan reports, requiring creation of **8-10 new files** and updates to **60+ existing files**.

**Priority Levels:**
- 🔴 **CRITICAL** - New syntax/features that must be documented
- 🟡 **HIGH** - Important clarifications and corrections
- 🟢 **MEDIUM** - Enhancements and examples

---

## Phase 1: Critical New Features (Implement First)

### 1. Multi-Line String System 🔴 CRITICAL
**Learning:** L-2025-12-21-006
**Status:** Completely new feature, not documented anywhere

**Action:** Create new file
- **Path:** `docs/User/language/advanced/multi-line-strings.md`
- **Content:**
  - `[+]` marker syntax
  - `+""` (same line) vs `-""` (new line) semantics
  - Language-specific utilities: `|U.String.Python`, `|U.String.Rust`, `|U.String.JS`
  - Double-brace interpolation `{{$var}}`
  - Complete examples for each language

**Template:** See `_patches/2025-12-21-training-sessions/multi-line-strings.md`

---

### 2. Runtime Wrapper System 🔴 CRITICAL
**Learning:** L-2025-12-21-006
**Status:** New stdlib section

**Action:** Create new files
- **Path 1:** `docs/User/stdlib/wrappers/runtime-wrappers.md`
- **Path 2:** `docs/User/stdlib/wrappers/README.md` (update index)

**Content:**
  - `|W.Runtime.Python{VERSION}` pattern
  - `|W.Runtime.Rust{VERSION}` pattern
  - `|W.Runtime.JavaScript.Node.{VERSION}` pattern
  - Version-in-name convention explanation
  - Configuration file support (`<requirements:pg.path`)
  - Session management (`>session:serial`)
  - Complete multi-language example

**Related Pipelines:**
  - `|Python.Run.Code` vs `|Python.Run.File`
  - `|Rust.Run.Code` vs `|Rust.Run.File`
  - `|JavaScript.Run.Code` vs `|JavaScript.Run.File`

---

### 3. Default Output Values 🔴 CRITICAL
**Learning:** L-2025-12-21-006
**Status:** New operator syntax

**Action:** Update existing file
- **Path:** `docs/User/language/syntax/io-operators.md`
- **Section:** Add new section "Default Output Values"

**Content to add:**
```markdown
## Default Output Values

Output parameters can have default values using the `<~` operator.

### Syntax
\`\`\`polyglot
[>] >param:type <~ default_value
\`\`\`

### Example
\`\`\`polyglot
[>] >status:bool <~ #False  # Defaults to False if not explicitly set
[>] >count:int <~ 0          # Defaults to 0
\`\`\`

### Behavior
- Default value is assigned when output is declared
- If pipeline explicitly assigns to output, default is overwritten
- Useful for optional outputs or error handling paths
```

**Diff:**
```diff
--- a/docs/User/language/syntax/io-operators.md
+++ b/docs/User/language/syntax/io-operators.md
@@ +XX,20 @@
+## Default Output Values
+
+Output parameters can have default values using the `<~` operator.
...
```

---

### 4. CLI Trigger Pattern 🔴 CRITICAL
**Learning:** L-2025-12-21-006
**Status:** New trigger type, needs stdlib documentation

**Action:** Update existing file
- **Path:** `docs/User/stdlib/triggers/README.md`
- **Section:** Add CLI triggers

**Content to add:**
```markdown
## CLI Triggers

### |T.CLI.Run

Trigger pipeline from command-line execution.

**Command Format:**
\`\`\`bash
polyglot run <command> [arguments...]
\`\`\`

**Example:**
\`\`\`polyglot
[t] |T.CLI.Run
(|) <command:string << "hello_world"
(|) >param.file:pg.path >> <file_path
(|) >param.count:int >> <count
\`\`\`

**CLI Invocation:**
\`\`\`bash
polyglot run hello_world --file output.txt --count 5
\`\`\`

**Parameter Extraction:**
- CLI arguments become `>param.*` outputs
- Type coercion based on output type annotation
- Required vs optional based on trigger I/O definition
```

---

## Phase 2: High-Priority Corrections

### 5. I/O Binding Context Clarification 🟡 HIGH
**Learning:** L-2025-12-21-002
**Status:** Critical distinction often confused

**Action:** Update and add examples
- **Path 1:** `docs/User/language/syntax/io-operators.md`
- **Path 2:** `docs/quick-reference/common-patterns.md`

**Key Correction:**
```markdown
### Parentheses vs Square Brackets for I/O

**IMPORTANT DISTINCTION:**

- `(|)` = I/O **binding** context (used in pipeline calls)
- `[|]` = **Structural** marker (pipeline definitions, chain continuation, boolean OR)

#### When to use `(|)` - I/O Bindings
\`\`\`polyglot
[r] |SomePipeline
(|) <input:type << $value      # Binding inputs
(|) >output:type >> $result    # Binding outputs

(~) <array << $collection      # Unpack bindings
(*) >result >> $packed         # Pack bindings
\`\`\`

#### When to use `[|]` - Structural Markers
\`\`\`polyglot
{|} |PipelineDef               # Pipeline definition
[|] <input:type                # Define inputs
[|] >output:type               # Define outputs
{x}

[|] |> |NextStep               # Chain continuation

[|] |T.Daily OR [|] |T.Manual  # Boolean OR
\`\`\`

**Rule:** Parentheses for binding values, brackets for structure.
```

---

### 6. IO Variable Scoping 🟡 HIGH
**Learning:** L-2025-12-21-006
**Status:** Implicit behavior needs explicit documentation

**Action:** Add clarification
- **Path:** `docs/User/language/types/variables-lifecycle.md`
- **Section:** Add "Input Parameters as Variables"

**Content:**
```markdown
## Input Parameters as Variables

Input parameters declared with `[<]` automatically become variables in pipeline scope.

**Pattern:**
\`\`\`polyglot
[<] <param_name << *t

# Later in pipeline, use as $param_name:
[r] |SomePipeline
(|) <input:type << $param_name  # Accessible as variable
\`\`\`

**Comment Convention:**
\`\`\`polyglot
// {|} IO are variables in scope
\`\`\`

This comment documents that all input/output parameters are accessible as variables throughout the pipeline body.
```

---

### 7. Formatting Rules 🟡 HIGH
**Learning:** L-2025-12-21-004
**Status:** No formal style guide exists

**Action:** Create new file
- **Path:** `docs/style-guide/formatting.md`

**Content:** Complete formatting guide with:
- 3 blank lines before `{}` markers (except `{@}`)
- 1 blank line before `[]` markers with IO bindings
- 1 blank line after IO binding groups
- 3-space indentation rules
- Examples showing proper spacing

---

### 8. Type System Best Practices 🟡 HIGH
**Learning:** L-2025-12-21-005
**Status:** Scattered guidelines, need central reference

**Action:** Create new file
- **Path:** `docs/best-practices/type-system.md`

**Key Guidelines:**
1. **Prefer enums over strings** for fixed value sets
2. **Inline OR IO bindings, never both** - mutually exclusive
3. **Use :pg.path for file paths**, not :pg.string
4. **Use |Path pipeline** for platform-independent paths

---

## Phase 3: Marker Migration (Bulk Update)

### 9. Fork/Join Marker Migration 🟡 HIGH
**Learning:** L-2025-12-21-001
**Status:** Affects 60+ files, already done in lexer

**Action:** Global find-replace across v0.0.4 docs

**Replacements:**
```bash
find docs/User/specifications/v0.0.4 -name "*.md" -exec sed -i 's/\[y\]/[f]/g' {} \;
find docs/User/specifications/v0.0.4 -name "*.md" -exec sed -i 's/\[Y\]/[v]/g' {} \;
```

**Files Affected:** ~60-80 files
**Verification:** Run tests after migration to ensure no broken references

---

## Phase 4: Stdlib Pipeline Registry

### 10. Standard Library Documentation 🟢 MEDIUM
**Source:** stdlib_pipelines tracking (25 pipelines)
**Status:** New comprehensive stdlib reference

**Actions:** Create organized stdlib documentation

**Structure:**
```
docs/User/stdlib/
├── triggers/
│   ├── cli-triggers.md          # |T.CLI.Run
│   └── http-triggers.md         # |T.HTTP.GET, POST, etc.
├── utilities/
│   ├── datetime.md              # |DT.Now, |U.DateTime.*
│   ├── string-utilities.md      # |U.String.Python/Rust/JS, |F
│   └── path-utilities.md        # |Path
├── wrappers/
│   ├── runtime-wrappers.md      # |W.Runtime.*
│   └── database-wrappers.md     # |W.DB.*
├── file-operations/
│   └── file-io.md               # |File.Text.Append, etc.
├── runtime-execution/
│   └── code-execution.md        # |Python/Rust/JavaScript.Run.Code/File
├── http/
│   └── responses.md             # |HTTP.Response.*
├── queue/
│   └── execution-queues.md      # |Q.Sequential, |Q.Parallel
└── data-loading/
    └── yaml-loading.md          # |YAML.Load
```

**Per-Pipeline Template:**
```markdown
### |Pipeline.Name

**Purpose:** {description}

**Inputs:**
- `<input1:type` - {description}
- `<input2:type` - {description}

**Outputs:**
- `>output1:type` - {description}

**Example:**
\`\`\`polyglot
[r] |Pipeline.Name
(|) <input1 << $value
(|) >output1 >> $result
\`\`\`

**Variants:**
- `|Pipeline.Name.Variant` - {description}

**See Also:**
- Related pipelines
```

---

## Phase 5: Examples and Enrichments

### 11. Complete Examples 🟢 MEDIUM
**Source:** Training session verified code

**New Example Files:**

#### A. API Endpoint Example
**Path:** `docs/User/examples/api-endpoint-complete.md`
**Source:** L-2025-12-21-003
**Content:**
- Registry definitions
- Serial load blocks
- Trigger I/O wiring
- Database wrapper
- Fork AND conditions
- HTTP responses

#### B. Multi-Language Integration
**Path:** `docs/User/examples/multi-language-cli.md`
**Source:** L-2025-12-21-006
**Content:**
- CLI trigger
- Runtime wrappers for Python, Rust, JavaScript
- Multi-line code generation
- Sequential execution
- File operations

---

## Implementation Checklist

### Immediate Actions (Do First)
- [x] Create documentation plan (this file)
- [ ] Create `multi-line-strings.md` (Phase 1.1)
- [ ] Create `runtime-wrappers.md` (Phase 1.2)
- [ ] Update `io-operators.md` with default outputs (Phase 1.3)
- [ ] Update `triggers/README.md` with CLI triggers (Phase 1.4)

### High Priority (Do Next)
- [ ] Add I/O binding clarification (Phase 2.5)
- [ ] Add IO variable scoping docs (Phase 2.6)
- [ ] Create `formatting.md` style guide (Phase 2.7)
- [ ] Create `type-system.md` best practices (Phase 2.8)

### Bulk Updates (Scripted)
- [ ] Run marker migration script (Phase 3.9)
- [ ] Verify all tests pass after migration
- [ ] Update any broken cross-references

### Documentation Expansion (Lower Priority)
- [ ] Create stdlib pipeline docs (Phase 4.10)
- [ ] Add complete examples (Phase 5.11)
- [ ] Generate stdlib index pages
- [ ] Update main README with new features

---

## Verification Steps

After each phase:
1. ✅ Markdown linting passes
2. ✅ Cross-references valid
3. ✅ Code examples syntax-highlighted
4. ✅ No broken internal links
5. ✅ Version tags updated
6. ✅ Git diff reviewed

---

## Estimated Impact

- **New Files:** 10-12 files
- **Modified Files:** 65-80 files
- **Lines Added:** ~2,000-3,000 lines
- **Examples Created:** 2 complete examples, 20+ snippets
- **Stdlib Entries:** 25 pipeline documentation entries

---

## Next Steps

1. **Review this plan** - Confirm scope and priorities
2. **Phase 1 Implementation** - Create critical new feature docs (4 files)
3. **Phase 2 Implementation** - High-priority corrections (4 files)
4. **Phase 3 Bulk Migration** - Marker updates (scripted)
5. **Phase 4-5 Expansion** - Stdlib and examples (as time allows)

**Recommendation:** Start with Phase 1 (critical features), then Phase 2 (corrections), then evaluate before proceeding to bulk updates.

---

**Generated by:** Polly Training Documentation Workflow
**Session Reports Processed:** 4 (all)
**Learnings Incorporated:** 13 total
**Ready for:** Implementation
