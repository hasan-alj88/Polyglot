# Polyglot v0.0.2 Decision Log

**Purpose:** Document decisions made to resolve inconsistencies found in v0.0.1 documentation
**Created:** 2025-11-11
**Status:** In Progress
**Reference:** See `inconsistencies-log.md` for detailed analysis of all 30 issues

---

## How to Use This Document

For each issue:
1. Review the issue details in `inconsistencies-log.md`
2. Make a decision on how to resolve it
3. Document the decision below with rationale
4. Mark status as: `PENDING` → `DECIDED` → `IMPLEMENTED`

Once all critical and high-priority decisions are made, this log will be used to generate the v0.0.2 documentation.

---

## Decision Summary

**Progress Tracker:**
- Critical Issues (5): ☑ 3/5 decided (2 moved to HIGH)
- High Priority (11): ☑ 11/11 decided (all complete!)
- Medium Priority (12): ☑ 10/12 decided
- Low Priority (12): ☑ 9/12 decided

**Total:** ☑ 30/30 decisions made (ALL COMPLETE!) 🎉

**Resolved:**
- `[D]` block marker completely removed - replaced by `[<]` in enumerations

---

## CRITICAL ISSUES (Must Decide Before Proceeding)

### Issue #1: Type System - Maps vs Enumerations
**Status:** DECIDED ✓
**Severity:** CRITICAL
**Reference:** inconsistencies-log.md lines 26-60
**Decided:** 2025-11-11

**Problem Summary:**
- `reference/02-type-system.md` says "Maps have been removed from Polyglot"
- Multiple other docs (language/syntax-reference.md, product-brief.md, etc.) extensively use and document maps
- Fundamentally incompatible: either maps exist or they don't

**Decision:**
- [x] Option A: Remove maps entirely, use enumerations only
- [ ] Option B: Keep maps, remove "maps removed" statement
- [ ] Option C: Deprecate maps but support temporarily
- [ ] Option D: Other (describe below)

**Rationale:**
Maps are completely removed from Polyglot. All key-value data structures will use enumerations instead. This provides:
- Simpler, more consistent type system
- Type-safe key access at compile time
- No duplicate/overlapping data structures
- Clear single approach for users

**Additional Feature:** Polyglot will have **reserved enumerations** that users can extend using `[#]` definition syntax.
*Note: User will provide details on reserved enumerations feature later - REMINDER NEEDED*

**Action Items:**
- [ ] Remove all map references from v0.0.2 documentation
- [ ] Update type system documentation to focus on enumerations only
- [ ] Convert all map examples to enumeration syntax
- [ ] Document enumeration syntax as the canonical approach
- [ ] Document reserved enumerations feature (pending user input)

---

### Issue #2: Type Separator - Forward Slash vs Backslash
**Status:** DECIDED ✓
**Severity:** CRITICAL
**Reference:** inconsistencies-log.md lines 62-98
**Decided:** 2025-11-11

**Problem Summary:**
- Most docs use `/` (forward slash): `pg/int`, `pg/string`
- product-brief.md uses `\` (backslash): `pg\int`, `pg\path`
- Mutable types use `\`: `pg.mutable\int`

**Decision:**
- [ ] Use `/` for all type separators
- [x] Use `\` for all type separators
- [ ] Use `/` for normal, `\` for mutable only
- [ ] Other (describe below)

**Canonical Type Syntax:**
```polyglot
language\type          // Normal types: pg\int, pg\string, pg\uint
language.mutable\type  // Mutable types: pg.mutable\int, pg.mutable\string
```

**Rationale:**
- **Consistent** with existing mutable type syntax already using `\`
- **Semantically appropriate** for path/URL types: `pg\path`, `pg\url`
- **No conflicts** with comments which use `//` (forward slash)
- **Single rule**: Always use backslash for type separator
- **Clear visual distinction** from other operators

**Additional Rules:**
- Comments use `//` (forward slash) - no conflict with type separator
- Paths and URLs use backslash: `pg\path`, `pg\url` (semantically matches path syntax)

**Action Items:**
- [ ] Update type system specification to use `\` exclusively
- [ ] Convert ALL code examples to backslash syntax
- [ ] Document separator rule clearly: "Type separator is always `\`"
- [ ] Update comment syntax documentation: `//` for single-line comments
- [ ] Update path/URL type examples to use backslash

---

### Issue #3: DateTime System - DT vs T Prefix
**Status:** DECIDED ✓
**Severity:** CRITICAL
**Reference:** inconsistencies-log.md lines 100-141
**Decided:** 2025-11-11

**Problem Summary:**
- `language/07-time literals.md` (1228 lines): Uses `DT"..."` with calendars, `DT.Ago`, `DT.Every`
- `language/syntax-reference.md`: Uses `T"..."` with simple timestamps only
- Completely different systems, incompatible

**Decision:**
- [x] Use DT prefix with full calendar system
- [ ] Use T prefix with simple timestamps only
- [ ] Hybrid: T for basic, DT for advanced (explain below)
- [ ] Other (describe below)

**Canonical DateTime Syntax:**
```polyglot
DT"2025-11-07:"                 // Date literal
DT"Mon"                          // Day of week
DT.Ago"2h"                       // Relative time
DT.Every"Sun:02:00:"            // Recurring pattern
DT.Hijri"1447-09-01:"           // Islamic calendar
// Plus other calendars: Chinese, Hebrew, Persian, etc.
```

**Type:** `pg\dt` (NOT `pg\time`)

**Rationale:**
- **Remove T"..." completely** - only DT"..." system exists
- **Automation-focused**: `DT.Every`, `DT.Ago` are essential for automation language
- **Multi-calendar support**: Important for international automation tasks
- **Consistent with language philosophy**: Rich, expressive literals for automation

**CRITICAL IMPLEMENTATION DETAIL - Literals as Syntax Sugar:**

DateTime literals (and all literals) are **syntax sugar for pipelines**:

**Example 1: String Interpolation**
```polyglot
"something {var:format}"
// Is actually syntax sugar for a pipeline:
// 1. Take var → |U.String.Language.Type.format
// 2. Substitute into string → |U.String.Substitute
// 3. All {} are processed in pg\serial
```

**Example 2: DateTime Literals**
```polyglot
DT.Hijri"1447-09-01:"
// Is syntax sugar for a pipeline that yields pg\dt
```

**Key Principle:** Literals in Polyglot are syntactic sugar for underlying pipeline operations. This is fundamental to implementation and will be important when coding the compiler/runtime.

**Action Items:**
- [ ] Remove ALL T"..." references from v0.0.2 documentation
- [ ] Use DT"..." exclusively for all temporal literals
- [ ] Document the full DT literal specification (from 07-time literals.md)
- [ ] Document supported calendars: Gregorian, Hijri, Chinese, Hebrew, Persian
- [ ] Document datatype as `pg\dt` (not `pg\time`)
- [ ] **Document literal-as-pipeline principle** in architecture/implementation docs
- [ ] Document string interpolation `{var:format}` as pipeline syntax sugar
- [ ] Document that all {} are processed in `pg\serial`

---

### Issue #4: Trigger Syntax - Multiple Conflicting Patterns
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 143-180
**Decided:** 2025-11-11

**Problem Summary:**
- Assignment operator varies: `=` vs `<<`
- Type separator varies: `\` vs `/` (RESOLVED by Decision #2)
- Queue marker varies: `[Q]` vs `[q]`

**Decision:**

**Canonical Trigger Syntax:**
```polyglot
[t] |T.Daily
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .time: pg\dt << DT"12:30:"
[<] .mb: pg\uint << 2048
```

**Assignment Operators:**
- `<<` - Left assignment (push value into variable)
- `>>` - Right assignment (documented purpose TBD)
- `=` - **NOT USED** - No equals sign for assignment in Polyglot

**Block Markers:**
- Queue configuration: `[Q]` ONLY (uppercase)
- Case sensitivity: Block markers are case-sensitive

**Rationale:**
- **Consistent assignment operators**: `<<` and `>>` ONLY throughout Polyglot
- **No `=` operator**: Prevents confusion with comparison, keeps syntax clean
- **Uppercase `[Q]`**: Maintains case-sensitivity convention for block markers
- **Visual clarity**: `<<` clearly shows direction of value assignment

**Action Items:**
- [ ] Remove ALL uses of `=` for assignment from v0.0.2 documentation
- [ ] Standardize on `<<` for all assignments (document `>>` usage separately)
- [ ] Use `[Q]` (uppercase) exclusively for queue blocks
- [ ] Update all trigger examples with correct syntax
- [ ] Document that block markers are case-sensitive
- [ ] Update type separators to `\` in trigger examples (per Decision #2)
- [ ] Update time literals to `DT"..."` in trigger examples (per Decision #3)

---

### Issue #5: Version Metadata Inconsistencies
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 182-213
**Decided:** 2025-11-11

**Problem Summary:**
- v0.0.1 files use v0.1.0, v1.0, v1.0.0 inconsistently
- Dates vary or are missing
- Status unclear

**Decision:**

**For v0.0.1 (archived documentation):**
- **Disregard all internal version numbers** - it's all v0.0.1, period
- The folder is v0.0.1, everything in it is v0.0.1
- Don't worry about what version numbers are written inside those files
- v0.0.1 is archived as a "mess" - that's the point

**For v0.0.2 (new documentation):**
- **Version:** 0.0.2
- **Clean slate:** Proper versioning starts with v0.0.2
- **Consistent metadata:** All v0.0.2 docs will have consistent version/date headers

**Rationale:**
- v0.0.1 is the "messy archive" - internal inconsistencies don't matter
- We're sorting the mess NOW by creating clean v0.0.2
- Don't waste time fixing v0.0.1 metadata - focus on creating proper v0.0.2
- Clear separation: v0.0.1 = legacy/archive, v0.0.2 = clean/current

**Action Items:**
- [ ] Leave v0.0.1 as-is (don't fix internal version numbers)
- [ ] All v0.0.2 documents show "Version: 0.0.2"
- [ ] Use consistent date format in v0.0.2: YYYY-MM-DD (e.g., 2025-11-11)
- [ ] Add consistent status metadata to v0.0.2 docs
- [ ] Document in v0.0.2 README that v0.0.1 internal versions are irrelevant

---

## HIGH PRIORITY ISSUES

### Issue #6: Pipeline Call Syntax Variations
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 214-249
**Decided:** 2025-11-11

**Problem Summary:**
- Inconsistent use of `|` prefix in pipeline calls
- Confusion between `|`, `~`, and `@` operators
- Handler syntax unclear

**Decision:**

**Operator Definitions:**

**1. `|` - Pipeline Operator**
- Calls predefined pipelines that are defined via `[|]`
- **Always required** when calling a pipeline
- Syntax: `|PipelineName`, `|T.Daily`, `|U.String`, `|Q.Pause`

**2. `~` - Unpack Operator**
- Unpacks `pg\array{}`, `pg\set{}`, and `#enumeration` types
- **NOT a pipeline call** - completely different operator
- Syntax: `~arrayVariable`, `~setVariable`, `~enumerationName`

**3. `@` - Package Operator**
- Accesses pipelines and enumerations from other packages
- Combined with `|` for pipelines, `#` for enumerations
- Syntax: `@package|Pipeline` or `@package#Enumeration`

**Canonical Syntax:**
```polyglot
// Pipeline calls (always use |):
[r] |ProcessData
[r] |T.Daily
[r] |U.String.Format
[r] |Q.Pause

// Unpack operator (separate from pipeline calls):
[r] ~myArray
[r] ~mySet
[r] ~MyEnumeration

// Package access:
[r] @otherPackage|SomePipeline
[i] @otherPackage#SomeEnumeration
```

**Important: Never combine `|~`** - These are separate operators with different purposes.

**Rationale:**
- `|` always indicates pipeline invocation
- `~` always indicates unpacking collections/enumerations
- `@` always indicates cross-package access
- Clear, unambiguous syntax - each operator has distinct purpose
- Consistent with block definition syntax `[|]` for defining pipelines

**Action Items:**
- [ ] Document `|` as pipeline operator - ALWAYS required for pipeline calls
- [ ] Document `~` as unpack operator for arrays, sets, and enumerations
- [ ] Document `@` as package access operator
- [ ] Update all examples to use `|PipelineName` syntax consistently
- [ ] Remove any instances of `|~` (incorrect combination)
- [ ] Document package syntax: `@package|Pipeline` and `@package#Enumeration`
- [ ] Clarify that pipelines are defined with `[|]` block marker


---

### Issue #7: Block Marker Case Sensitivity
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 250-278
**Decided:** 2025-11-11

**Problem Summary:**
- v0.0.1 claims "block markers are case-insensitive"
- But all examples use lowercase only
- Contradictory statements in documentation

**Decision:**

**Block Markers ARE Case-Sensitive**

**Complete List of Block Markers:**
- `[|]` - Pipeline definition
- `[i]` - Input
- `[r]` - Run sequential
- `[p]` - Parallel (replaces `[f]` from v0.0.1)
- `[t]` - Trigger
- `[Q]` - Queue configuration
- `[<]` - Passing input (dual purpose: trigger input AND enumeration fields)
- `[>]` - Passing output
- `[#]` - Enumeration definition
- `[A]` - Alias definition (for enumerations)
- `[X]` - End of pipeline/enumeration

**Removed/Changed:**
- `[f]` - Removed, use `[p]` for parallel instead
- `[D]` - **REMOVED COMPLETELY** (replaced by `[<]` in enumerations - see Decision #9)

**Case Sensitivity:**
- `[i]` ≠ `[I]`
- `[r]` ≠ `[R]`
- `[p]` ≠ `[P]`
- All block markers must use exact case as specified above

**Rationale:**
- **Case-sensitive** provides clarity and prevents ambiguity
- **Consistent capitalization** - `[Q]` uppercase, others lowercase
- **Simpler mental model** - one correct spelling for each marker
- **Compiler-friendly** - no need for case normalization

**Action Items:**
- [ ] Document all block markers as case-sensitive
- [ ] Remove claim of "case-insensitive" from v0.0.2
- [ ] Replace all `[f]` with `[p]` for parallel execution
- [ ] **Review all `[D]` uses in v0.0.1** - decide replacement for each scenario
- [ ] Document complete block marker reference with exact capitalization
- [ ] Update all examples to use correct case


---

### Issue #8: Reserved Keywords Incomplete List
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 279-314
**Decided:** 2025-11-11

**Problem Summary:**
- Multiple files show different reserved keyword lists
- v0.0.1 lists: `Default`, `Fixed`, `True`, `False`, `Exposed`, `Private`, `Public`
- Inconsistent and incomplete
- No clear categorization

**Decision:**

**Minimal Reserved Keywords - Philosophy:**

Polyglot intentionally **minimizes the use of reserved keywords**. Most language features use operators, block markers, or standard library conventions instead of keywords.

**Complete Reserved Keyword List for v0.0.2:**

**1. Boolean Literals:**
- `True` - Boolean true value
- `False` - Boolean false value

**2. Input Modifiers:**
- `Fixed` - Immutable constant input (Decision #10)
- `Default` - Optional input with default value (Decision #10)

**3. Macro System:**
- `Exposed` - Exposes macro variables into wrappers

**Total: 5 Reserved Keywords**

**Visibility System - NOT Keywords:**

`Public` and `Private` for package visibility are being **considered for conversion to reserved enumerations** instead of keywords. This aligns with Polyglot's philosophy of minimizing keywords.

**Potential Future Approach (Under Consideration):**
```polyglot
// Package visibility via reserved enumeration (not keywords)
#Package.Visibility.Public     // Public package
#Package.Visibility.Private    // Private package
```

This would further reduce keyword count and make visibility more consistent with other language features.

**Why Minimize Keywords?**

1. **Simpler language**: Fewer rules to remember
2. **More flexible**: Keywords can't be used as identifiers
3. **Consistent design**: Use operators and enumerations instead
4. **Easier parsing**: Less ambiguity in grammar
5. **Future-proof**: Fewer breaking changes when evolving language

**Examples:**

**Boolean Literals:**
```polyglot
[r] .is_valid: pg\bool << True
[r] .has_errors: pg\bool << False

[r] |ProcessData
[<] .enabled: pg\bool << True
```

**Input Modifiers:**
```polyglot
[i] Fixed .api_key: pg\string << "secret-123"
[i] Default .timeout: pg\int << 30
```

**Macro Exposure:**
```polyglot
// Exposed makes macro variables available in wrappers
// (Detailed macro syntax to be documented separately)
Exposed .macro_var
```

**NOT Reserved - Using Other Mechanisms:**

These are **NOT keywords** - they use alternative syntax:

**Block Markers (NOT keywords):**
- `[|]`, `[i]`, `[r]`, `[p]`, `[t]`, `[Q]`, `[<]`, `[>]`, etc. (Decision #7)

**Operators (NOT keywords):**
- `|` - Pipeline operator (Decision #6)
- `~` - Unpack operator (Decision #6)
- `@` - Package operator (Decision #6)
- `#` - Enumeration marker (Decision #9)
- `!` - Error type marker (Decision #13)
- `<<` - Push assignment (Decision #4)
- `>>` - Pull assignment (Decision #13)

**Type System (NOT keywords):**
- Types use `\` separator: `pg\int`, `pg\string`, `pg\path` (Decision #2)
- No `int`, `string`, `bool` keywords

**Control Flow (NOT keywords):**
- Loops via unpack operator: `~Array.ForEach`
- Conditionals via pipelines (to be documented)
- No `if`, `else`, `for`, `while` keywords

**Rationale:**
- **Minimalist approach**: Only 5 keywords for v0.0.2
- **Operator-driven**: Most features use operators instead
- **Enumeration-driven**: Visibility may become enumeration
- **Pipeline-driven**: Control flow via pipeline calls
- **Future flexibility**: Easy to add features without new keywords
- **Clear semantics**: Each keyword has exactly one purpose

**Open for Suggestions:**
The keyword list is intentionally minimal and open to community feedback. If better alternatives exist (e.g., converting keywords to operators or enumerations), they will be considered.

**Action Items:**
- [ ] Document 5 reserved keywords in v0.0.2 reference
- [ ] Document that `True`/`False` are boolean literals
- [ ] Document `Fixed`/`Default` as input modifiers (link to Decision #10)
- [ ] Document `Exposed` for macro system
- [ ] Clarify that block markers are NOT keywords
- [ ] Clarify that operators are NOT keywords
- [ ] Document minimalist keyword philosophy
- [ ] Research converting `Public`/`Private` to reserved enumeration
- [ ] If visibility becomes enumeration, document in separate decision
- [ ] Create comprehensive "What's NOT a keyword" reference
- [ ] Document that control flow uses pipelines, not keywords
- [ ] Add keyword list to syntax quick reference


---

### Issue #9: Enumeration Syntax - Two Different Systems
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 315-354
**Decided:** 2025-11-11

**Problem Summary:**
- Two completely different enumeration syntax patterns in v0.0.1
- System A uses `[D]` name with `[#]` entries
- System B uses `[#]` name with `[D]` entries
- Conflicting assignment operators (`=` vs `<<`)

**Decision:**

**Canonical Enumeration Syntax:**

**Structure:**
```polyglot
[#] Path.Identifiers.Name     // For fixed enumerations
[#] Path.Identifiers.Name.*   // For extendable/reserved enumerations
[A] AliasName                  // Optional alias using \\AliasName\\
[<] .fieldname: type << value  // Fields
[X]                            // End enumeration
```

**Complete Example:**
```polyglot
// Reserved enumeration - extendable with fixed schema
[#] Path.Identifiers.Hasan.ML.DataDirectory.*
// Alias \\DataDir\\
[A] DataDir
// Define path in unix systems
[<] .unix: pg\path << \\UnixRoot\\opt\ML\data\
// Only valid in Unix os
[<] .windows: pg\path << \\NoPath\\
[X]
```

**Key Elements:**

1. **`[#]` - Enumeration definition**
   - Hierarchical dot notation: `Path.Identifiers.Name`
   - Add `.*` suffix for extendable/reserved enumerations
   - Comment line above can indicate purpose

2. **`[A]` - Alias definition**
   - Provides shorthand reference
   - Special syntax: `\\AliasName\\` (double backslash wrapping)

3. **`[<]` - Field definition**
   - Same marker as "passing input" - dual purpose
   - Syntax: `[<] .fieldname: type << value`
   - Field names start with `.`
   - Uses `<<` assignment operator (per Decision #4)
   - Uses `\` type separator (per Decision #2)

4. **`[X]` - End enumeration**

**Extendable/Reserved Enumerations:**
- Marked with `.*` suffix on enumeration name
- Users can extend using `[#]` definition syntax
- Fixed schema (fields are defined)
- Related to "reserved enumerations" feature mentioned in Decision #1

**Rationale:**
- `[#]` logically marks enumeration definition
- `[<]` for fields follows "passing data into" semantic
- `[A]` provides alias capability for long hierarchical names
- Consistent with assignment (`<<`) and type separator (`\`) decisions
- Hierarchical dot notation allows organization
- `.*` clearly marks extendable enumerations

**Action Items:**
- [ ] Remove `[D]` block marker entirely from v0.0.2
- [ ] Document canonical enumeration syntax with `[#]...[X]`
- [ ] Document `[<]` dual purpose: trigger input AND enumeration fields
- [ ] Document `[A]` alias syntax with `\\Name\\` notation
- [ ] Document `.*` suffix for extendable/reserved enumerations
- [ ] Update all enumeration examples to canonical syntax
- [ ] Document hierarchical dot notation for enumeration names
- [ ] Connect to reserved enumerations feature (Decision #1 reminder)


---

### Issue #10: Input Declaration - Fixed vs Default Confusion
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 355-393
**Decided:** 2025-11-11

**Problem Summary:**
- `Fixed` and `Default` keywords used inconsistently
- Confusion about whether `Fixed` is for constants or multi-line formatting
- Examples show invalid patterns with assignments but no keywords

**Decision:**

**Canonical Input Syntax Rules:**
```polyglot
[i] .input_file: pg\path                              // Required input
[i] Fixed .api_key: pg\string << "secret-key-123"     // Fixed constant
[i] Default .chunk_size: pg\int << 1024               // Optional with default
```

**Syntax Rules:**

**1. Required Input (no keyword, no value)**
```polyglot
[i] .input_file: pg\path
```
- Caller **MUST** provide this value
- No default value
- Compilation error if not provided by caller

**2. Fixed Constant (`Fixed` keyword + value assignment)**
```polyglot
[i] Fixed .api_key: pg\string << "secret-key-123"
```
- Compile-time constant
- **CANNOT** be overridden by caller
- **MUST** have value assigned with `<<`
- Immutable

**3. Optional with Default (`Default` keyword + value assignment)**
```polyglot
[i] Default .chunk_size: pg\int << 1024
```
- Caller **CAN** override this value
- Falls back to default if not provided
- **MUST** have value assigned with `<<`

**Invalid Syntax (Compile Errors):**
```polyglot
// ❌ INVALID: Assignment without keyword
[i] .api_key: pg\string << "secret-key-123"
// Should be: [i] Default .api_key: pg\string << "secret-key-123"

// ❌ INVALID: Fixed without value
[i] Fixed .api_key2: pg\string
// Fixed REQUIRES value assignment

// ❌ INVALID: Default without value
[i] Default .chunk_size: pg\int
// Default REQUIRES value assignment
```

**Validation Rules (Compiler Enforcement):**
1. `Fixed` keyword **REQUIRES** value assignment (`<< value`)
2. `Default` keyword **REQUIRES** value assignment (`<< value`)
3. Plain `[i]` (required input) **MUST NOT** have value assignment
4. Any `[i]` with `<<` assignment **MUST** have `Fixed` or `Default` keyword

**Multi-line Literals:**

Multi-line literals work with both `Fixed` and `Default` using literal syntax sugar:

```polyglot
[i] Fixed .config: pg\serial << serial{
[^]  "host": "localhost",
[^]  "port": 8080
[^]}

[i] Default .settings: pg\array{pg\string} << array{
[^]  "option1",
[^]  "option2"
[^]}
```

**IMPORTANT - Literal Syntax Sugar Principle:**
- `array{}`, `set{}`, `serial{}` are syntax sugar for yielding those collection types
- Same principle as `DT.Hijri"..."` and `"{var:format}"` (documented in Decision #3)
- All literals in Polyglot are syntax sugar for underlying pipeline operations
- The `Fixed`/`Default` keyword determines mutability, NOT formatting

**Rationale:**
- **Clear semantics**: Each pattern has exactly one meaning
- **Compile-time validation**: Invalid combinations caught immediately
- **No ambiguity**: Keyword presence determines mutability and requirement
- **Consistent with decisions**: Uses `<<` assignment (Decision #4), `\` separator (Decision #2)
- **Consistent with literals-as-pipelines**: Follows principle from Decision #3

**Action Items:**
- [ ] Document input declaration rules in v0.0.2
- [ ] Create validation rules for compiler
- [ ] Update all examples to follow canonical syntax
- [ ] Remove any instances of `[i] .x: type << value` (missing keyword)
- [ ] Remove any instances of `[i] Fixed .x: type` (missing value)
- [ ] Remove any instances of `[i] Default .x: type` (missing value)
- [ ] Document compiler error messages for invalid combinations
- [ ] Add to syntax reference with clear examples of valid/invalid patterns
- [ ] Document `array{}`, `set{}`, `serial{}` as literal syntax sugar
- [ ] Cross-reference with Decision #3 (literals-as-pipeline principle)


---

### Issue #11: Queue Configuration - [Q] vs [q] Ambiguity
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 394-425
**Decided:** 2025-11-11

**Problem Summary:**
- Two different block markers `[Q]` and `[q]` appear in documentation
- Unclear when to use which
- Confusion about queue definition vs queue usage
- Reference syntax mismatch (`#Queues.Name` vs `Q|Name`)

**Decision:**

**`[Q]` is for Queue Control Blocks ONLY**

The `[Q]` block marker is used exclusively for **queue control operations** within triggers and pipeline execution - it controls queuing, dispatch, and running conditions.

**Canonical Queue Control Syntax:**
```polyglot
[t] |T.Daily
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 2048
[Q] |Q.ResumeIf.CPU.Idle.GreaterThan
[<] .percent: pg\uint << 50
```

**Queue Control Operations (|Q.* pipelines):**
- `|Q.PauseIf.*` - Conditional pausing based on system resources
- `|Q.ResumeIf.*` - Conditional resuming when resources available
- `|Q.Dispatch.*` - Dispatch conditions and priorities
- Additional queue control utilities in standard library

**There is NO `[q]` block marker** - Remove all lowercase `[q]` references from v0.0.2.

**Queue System Philosophy:**

Polyglot is a **precise automation language** that provides detailed control over automated tasks:
- **Resource management**: Control execution based on RAM, CPU, disk, network availability
- **Prevent resource overwhelming**: Pipelines can pause when resources are constrained
- **Dictate every scenario**: Explicit control over what happens in all execution conditions
- **Automation-first design**: Queue control is fundamental to reliable automation

**How Queue Control Works:**

1. **In Triggers** - Define conditions for when pipeline should pause/resume:
```polyglot
[t] |T.Every.Minute
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512
[Q] |Q.PauseIf.CPU.Usage.GreaterThan
[<] .percent: pg\uint << 80
```

2. **In Pipelines** - Dynamic queue control during execution:
```polyglot
[r] |ProcessLargeFile
[Q] |Q.Dispatch.Priority.High
[Q] |Q.PauseIf.Disk.Space.LessThan
[<] .gb: pg\uint << 10
```

3. **Queue States** - Pipelines move through queue system:
- **Pending Queue**: Waiting to execute
- **Dispatch Queue**: Ready to run, checking dispatch conditions
- **Running**: Currently executing
- **Paused**: Temporarily suspended due to `[Q]` conditions

**Queue References (Not Definitions):**

Queues themselves are **infrastructure** - they don't need explicit definition in user code. The `[Q]` block allows pipelines to interact with the queue system through control operations.

**Removed Confusion:**
- ~~`[Q]` for queue definition~~ ❌
- ~~`[q]` for queue assignment~~ ❌
- ~~`Q|QueueName` definition syntax~~ ❌
- ~~`#Queues.Name` reference syntax~~ ❌

**Clarified:**
- `[Q]` for queue control operations ✓
- Queue system is infrastructure (built-in) ✓
- Control via `|Q.*` standard library pipelines ✓

**Rationale:**
- **Consistent with case sensitivity** (Decision #7): Only `[Q]` uppercase exists
- **Aligned with automation philosophy**: Precise control over execution conditions
- **Resource management**: Critical for preventing system overload
- **Simpler mental model**: Queue control, not queue definition
- **Infrastructure abstraction**: Users control queue behavior, not queue structure

**Action Items:**
- [ ] Remove all `[q]` (lowercase) references from v0.0.2 documentation
- [ ] Document `[Q]` as queue control block marker
- [ ] Document queue control operations in standard library (`|Q.*`)
- [ ] Explain queue states: Pending, Dispatch, Running, Paused
- [ ] Document resource-based pause/resume conditions
- [ ] Provide examples of `[Q]` usage in triggers and pipelines
- [ ] Document queue system philosophy (precision automation, resource management)
- [ ] Remove any queue definition syntax (queues are infrastructure)
- [ ] Clarify that `[Q]` blocks use `|Q.*` pipeline calls for control


---

### Issue #12: Parallel Execution - Variable Scope Rules
**Status:** DECIDED ✓
**Severity:** HIGH
**Reference:** inconsistencies-log.md lines 426-458
**Decided:** 2025-11-11

**Problem Summary:**
- Unclear variable scope rules for parallel (`[p]`) blocks
- Confusion about whether parallel blocks can read/write outer scope
- Ambiguous join mechanism with `[Y]`
- No clear documentation on variable isolation

**Decision:**

**Parallel Blocks are Mini-Pipelines with Copy Semantics**

Parallel blocks (`[p]`) function as independent mini-pipelines with implicit copy-in and explicit copy-out behavior.

**Canonical Parallel Execution Syntax:**

```polyglot
// Outer scope
[r] .shared_data: pg\string << "input"
[r] .config: pg\int << 100
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

// Parallel block 1 - mini-pipeline
[p] |ProcessPartA
// Copy outer variables INTO parallel scope (at instantiation point)
[<] .data: pg\string << .shared_data
[<] .cfg: pg\int << .config
// Local variables within parallel scope
[<] .local_temp: pg\string << "temp A"
[<] .label: pg\string << "Result A"
// Copy result OUT to outer scope variable
[>] .output >> result1

// Parallel block 2 - mini-pipeline
[p] |ProcessPartB
[<] .data: pg\string << .shared_data
[<] .cfg: pg\int << .config
[<] .local_temp: pg\string << "temp B"
[<] .label: pg\string << "Result B"
[>] .output >> result2

// Join block - explicitly list variables to synchronize
[Y] |Y.Join
[<] result1    // Synchronize result1 from parallel scope to outer
[<] result2    // Synchronize result2 from parallel scope to outer
// Note: local_temp and label are NOT synchronized (not listed)
```

**Key Scope Rules:**

**1. Parallel Blocks are Mini-Pipelines**
- Each `[p]` block is an independent pipeline with its own scope
- Since each `[p]` is a single-pipeline block, `[~]` prefix is **NOT needed**
- `[~]` would only be needed if there were multiple `[r]` blocks inside `[p]`
- Variables are isolated within the parallel scope

**2. Implicit Input (Copy-In Semantics)**
```polyglot
[<] .data: pg\string << .shared_data
```
- **Implicit input behavior** - parallel blocks copy accessible outer variables
- Gets a **COPY** of outer variable `.shared_data` at instantiation point
- Changes to `.data` inside parallel block do NOT affect `.shared_data` in outer scope
- Snapshot taken at the moment parallel block starts
- Variables are implicitly available (like implicit `[i]` declarations)

**3. Copy-Out Semantics (Explicit Outputs)**
```polyglot
[>] .output >> result1
```
- Copies parallel scope variable to outer scope variable
- `result1` in outer scope receives value from parallel block's `.output`

**4. All Variables Inside Parallel Scope ARE Accessible**
- Variables created inside `[p]` blocks exist and are accessible
- However, they are NOT automatically synchronized to outer scope
- Only variables explicitly listed in `[Y]` join block are synchronized

**5. Join Block `[Y]` - Selective Synchronization**
```polyglot
[Y] |Y.Join
[<] result1
[<] result2
```
- **Explicitly lists** which variables to synchronize from parallel scopes
- Variables NOT listed (like `local_temp`, `label`) remain in parallel scope
- After join, unlisted variables are no longer accessible

**Variable Lifetime:**
- **Listed in `[Y]`**: Synchronized to outer scope, accessible after join
- **NOT listed in `[Y]`**: Discarded after join, not accessible

**When to Use `[~]` in Parallel Blocks:**

**NOT needed (single pipeline):**
```polyglot
[p] |ProcessPartA
[<] .data: pg\string << .shared_data    // No [~] needed
[>] .output >> result1                   // No [~] needed
```

**Needed (multiple operations in parallel block):**
```polyglot
[p] |ProcessPartA
[<] .data: pg\string << .shared_data
[r] |TransformData
[~][<] .input: pg\string << .data        // [~] needed for nested operation
[~][>] .transformed >> temp
[r] |ValidateData
[~][<] .value: pg\string << temp         // [~] needed for nested operation
[~][>] .validated >> result1
```

**Rationale:**
- **Explicit copy semantics**: Clear understanding of data flow
- **Isolation**: Parallel blocks cannot accidentally modify outer state
- **Selective synchronization**: Join only what's needed, discard temporaries
- **Thread-safe by design**: Copies prevent race conditions
- **Mini-pipeline model**: Consistent with Polyglot's pipeline philosophy
- **Implicit inputs**: Simpler syntax - no explicit `[i]` declarations needed
- **No unnecessary `[~]`**: Clean syntax for simple parallel blocks

**Open Question - Join Syntax:**

Two potential syntaxes for join block - **TO BE DECIDED LATER**:

**Option A: Without `...` (cleaner)**
```polyglot
[Y] |Y.Join
[<] result1
[<] result2
```

**Option B: With `...` prefix (more explicit)**
```polyglot
[Y] |Y.Join
[<] ... result1
[<] ... result2
```

**Deferred Decision:** This syntax choice will be discussed separately. Consider:
- If `...` is used elsewhere in the language for similar "reference" semantics, use it here
- If not, prefer cleaner syntax without `...`
- Document both options for now, finalize later

**Action Items:**
- [ ] Document parallel block mini-pipeline model in v0.0.2
- [ ] Document implicit input behavior (copy-in at instantiation)
- [ ] Document copy-out semantics with `[>]` inside `[p]` blocks
- [ ] Document `[Y]` join block for selective synchronization
- [ ] Explain variable lifetime (listed vs unlisted in join)
- [ ] Provide complete parallel execution examples
- [ ] Document that variables are copied at instantiation point
- [ ] Clarify when `[~]` is needed vs not needed in parallel blocks
- [ ] Update all parallel examples to use `[p]` instead of `[f]`
- [ ] **Decide on join syntax**: `[<] result` vs `[<] ... result` (DEFERRED)
- [ ] Cross-reference with Decision #6 (unpack operator `~`)
- [ ] Document that `[<]` in `[p]` blocks implies input without explicit `[i]`


---

### Issue #13: Error Handling - Output Syntax Inconsistency
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 459-491
**Decided:** 2025-11-11

**Problem Summary:**
- Inconsistent patterns for error handling in v0.0.1
- Pattern A shows error field extraction with `[>]` and `>>`
- Pattern B shows error handling without field extraction
- Unclear if error outputs are optional or required
- Purpose of `>>` operator undefined

**Decision:**

**Error Types (`!Error`) - Special Enumeration with Reserved Fields**

Error types are a special kind of enumeration marked with `!` operator (similar to `#` for enumerations). All errors have three **reserved fields** enforced by Polyglot:

**Reserved Error Fields (Immutable, Always Present):**
- `.message: pg\string` - Human-readable error message
- `.code: pg\int` - Numeric error code
- `.trace: pg\string` - Stack trace or error context

**Type System Comparison:**

**1. `pg\serial` - Serializable data structure**
- Schema/keys are **mutable** (can be changed at runtime)
- No restrictions on structure
- Dynamic key-value pairs

**2. `#Enumeration` - User-defined enumeration**
- Schema/keys are **immutable** (fixed at definition)
- Type-safe, compile-time checked
- Can be reserved (with `.*` suffix) - user can extend with fixed schema

**3. `!Error` - Error enumeration (special type)**
- Like `#Enumeration` but with additional error-handling responsibilities
- Has **reserved fields** enforced by Polyglot: `.message`, `.code`, `.trace`
- These three fields **cannot be altered or changed** by user
- `!` operator marks error types (similar to how `#` marks enumerations)
- Users can extend existing `!Error` types

**Canonical Error Handling Syntax:**

**Pattern 1: Catch error WITHOUT extracting fields (minimal handling)**
```polyglot
[r] |ReadFile
[<] .path: pg\path << "data.txt"

[!] !pg.FileSystem.NotFound
// Handle error without extracting fields
[r] |U.Log.Error
[<] .msg: pg\string << "File not found"
```

**Pattern 2: Catch error WITH field extraction (detailed handling)**
```polyglot
[r] |ReadFile
[<] .path: pg\path << "data.txt"

[!] !pg.FileSystem.NotFound
// Extract error fields - ALL OPTIONAL
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code
[>] .trace: pg\string >> err_trace

[r] |U.Log.Error
[<] .msg: pg\string << err_msg
[<] .code: pg\int << err_code
```

**Pattern 3: Partial field extraction (extract only what you need)**
```polyglot
[!] !pg.Network.Timeout
// Extract only message, ignore code and trace
[>] .message: pg\string >> err_msg

[r] |U.Log.Warning
[<] .text: pg\string << err_msg
```

**Key Rules:**

**1. Error Field Extraction is OPTIONAL**
- You can catch an error without extracting any fields
- Extract only the fields you need
- All three fields (`.message`, `.code`, `.trace`) are always available

**2. Assignment Direction with `>>` Operator**

**Assignment in Polyglot requires direction:**

**`<<` - Left/Push assignment (push value INTO variable)**
```polyglot
[<] .path: pg\path << "data.txt"    // Push "data.txt" INTO .path
```

**`>>` - Right/Pull assignment (pull value FROM source)**
```polyglot
[>] .message: pg\string >> err_msg  // Pull .message FROM error INTO err_msg
```

**This clarifies Decision #4 - the `>>` operator is for extracting/pulling values FROM objects.**

**3. Reserved Error Fields**
- `.message`, `.code`, `.trace` are **reserved by Polyglot**
- Users **cannot alter or change** these fields
- All `!Error` types automatically have these three fields

**4. Error Types Use `!` Operator**
```polyglot
!pg.FileSystem.NotFound       // Built-in error
!pg.Network.Timeout           // Built-in error
!MyApp.Database.ConnectionFailed  // User-defined error (extends existing)
```

**Nesting with `[~]`:**

When error handling contains multiple operations:
```polyglot
[!] !pg.Network.Timeout
[>] .message: pg\string >> err_msg
[r] |U.Log.Warning
[~][<] .text: pg\string << err_msg     // [~] needed for nested operation
[r] |U.Retry.After
[~][<] .seconds: pg\uint << 5          // [~] needed for nested operation
```

**Rationale:**
- **Optional extraction**: Flexibility - extract only what you need
- **Directional assignment**: `<<` push vs `>>` pull - clear data flow
- **Reserved fields**: Consistent error interface across all errors
- **Special enumeration**: Errors are structured, type-safe, immutable
- **User extensibility**: Can extend existing error types
- **Automation-friendly**: Errors contain all info needed for logging/recovery

**Deferred Decisions:**

**DEFERRED - Custom User Error Definition:**
- How users define brand new custom `!Error` types
- Syntax for adding custom fields beyond `.message`, `.code`, `.trace`
- Whether custom errors can have additional required fields
- Example syntax to be determined later

**Action Items:**
- [ ] Document `!Error` as special enumeration type with reserved fields
- [ ] Document `.message`, `.code`, `.trace` as reserved, immutable fields
- [ ] Clarify that error field extraction is OPTIONAL
- [ ] Document `>>` operator as "pull/extract FROM source"
- [ ] Update Decision #4 to include `>>` operator purpose
- [ ] Document difference between `pg\serial`, `#Enumeration`, and `!Error`
- [ ] Provide error handling examples (minimal, detailed, partial extraction)
- [ ] Document that users can extend existing `!Error` types
- [ ] Show error handling with nesting using `[~]`
- [ ] **DEFER custom error definition syntax** (to be decided later)
- [ ] Document type system comparison table in v0.0.2


---

### Issue #14: Path Identifiers - Inconsistent Set
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 492-516
**Decided:** 2025-11-11

**Problem Summary:**
- Different files list different sets of path identifiers (24 vs 5 vs unknown)
- Inconsistent syntax in v0.0.1 documentation (`//Name//`)
- No authoritative list of what path identifiers exist
- Unclear how path identifiers work

**Decision:**

**Path Identifiers Use `\\Name\\` Syntax (Same as Aliases)**

**Syntax Correction from v0.0.1:**
- v0.0.1 used `//Name//` (double forward slash) ❌
- v0.0.2 uses `\\Name\\` (double backslash) ✓
- **Same syntax as enumeration aliases** - both use `\\Name\\`
- Compiler raises error on name collision between alias and path identifier

**Path Type System:**

All `pg\path` values have **reserved fields** for OS-specific paths:
```polyglot
pg\path {
  .unix: pg\string     // Unix/Linux/macOS path representation
  .windows: pg\string  // Windows path representation
}
```

**Built-in Path Identifiers:**

Polyglot provides predefined path identifiers for common system paths:

```polyglot
\\C\\              // Windows C:\ drive
\\UnixRoot\\       // Unix/Linux root directory (/)
\\NoPath\\         // Special identifier for "path not available on this OS"
// Additional system identifiers (to be documented in standard library)
```

**User-Defined Path Identifiers:**

Users can define custom path identifiers using the **reserved enumeration** `#Path.Identifiers.*`:

**Canonical Custom Path Identifier Syntax:**
```polyglot
// #Path.Identifiers.* is extendable reserved enumeration
// Can be extended with fixed keys/schema (.unix, .windows)
[#] Path.Identifiers.Hasan.ML.DataDirectory
// Alias \\DataDir\\
[A] DataDir
// Define path in Unix/Linux/macOS systems
[<] .unix: pg\path << \\UnixRoot\\opt\ML\data\
// Only valid in Unix OS (NoPath = not available on this OS)
[<] .windows: pg\path << \\NoPath\\
[X]
```

**Using Custom Path Identifiers:**
```polyglot
[r] .data_file: pg\path << \\DataDir\\dataset.csv
// Or using full path:
[r] .data_file: pg\path << \\Path.Identifiers.Hasan.ML.DataDirectory\\dataset.csv
```

**Key Concepts:**

**1. Reserved Enumeration for Path Identifiers**
- `#Path.Identifiers.*` is a **reserved, extendable enumeration**
- Fixed schema with `.unix` and `.windows` fields
- Users extend it by defining new path identifiers
- Hierarchical dot notation for organization

**2. OS-Specific Path Resolution**
- Every path identifier defines both `.unix` and `.windows` variants
- Runtime resolves to appropriate variant based on OS
- Use `\\NoPath\\` when path doesn't exist on an OS

**3. Double Backslash Syntax**
- Path identifiers: `\\IdentifierName\\`
- Enumeration aliases: `\\AliasName\\`
- **Same syntax** - `\\Name\\` for both
- Compiler error on name collision

**4. Built-in System Identifiers**
- `\\C\\` - Windows C: drive
- `\\UnixRoot\\` - Unix/Linux root (/)
- `\\NoPath\\` - Path unavailable marker
- More system identifiers in standard library

**How Path Identifiers Work:**

**Example 1: Using built-in identifier**
```polyglot
[r] .config_path: pg\path << \\UnixRoot\\etc\myapp\config.json
// On Unix: resolves to /etc/myapp/config.json
// On Windows: would need Windows equivalent or use \\NoPath\\
```

**Example 2: Custom identifier for cross-platform paths**
```polyglot
[#] Path.Identifiers.MyApp.ConfigDir
[A] AppConfig
[<] .unix: pg\path << \\UnixRoot\\etc\myapp\
[<] .windows: pg\path << \\C\\ProgramData\MyApp\
[X]

// Usage:
[r] .config: pg\path << \\AppConfig\\settings.json
// On Unix: /etc/myapp/settings.json
// On Windows: C:\ProgramData\MyApp\settings.json
```

**Name Collision Handling:**
```polyglot
[#] SomeEnumeration
[A] MyName    // Alias \\MyName\\
[X]

[#] Path.Identifiers.MyName    // ❌ COMPILE ERROR
// Error: Name collision - \\MyName\\ already defined as alias
```

**Rationale:**
- **Unified syntax**: Same `\\Name\\` for aliases and path identifiers - simpler to learn
- **Collision detection**: Compiler enforces unique names across aliases and identifiers
- **Reserved enumeration**: Type-safe, organized, extensible by users
- **Fixed schema**: Enforces `.unix` and `.windows` for all path identifiers
- **Cross-platform**: Single identifier resolves to appropriate path per OS
- **Hierarchical organization**: Users can organize their path identifiers logically
- **\\NoPath\\**: Explicit marker for OS-incompatible paths

**Connection to Other Decisions:**
- **Decision #1**: Uses reserved enumerations concept
- **Decision #2**: Consistent with `\` (backslash) usage throughout language
- **Decision #9**: Path identifiers extend `#Path.Identifiers.*` enumeration, same `\\Name\\` syntax as aliases

**Action Items:**
- [ ] Change all `//PathName//` to `\\PathName\\` in v0.0.2 documentation
- [ ] Document `#Path.Identifiers.*` as reserved, extendable enumeration
- [ ] Document `.unix` and `.windows` as required fields for path identifiers
- [ ] Document built-in path identifiers: `\\C\\`, `\\UnixRoot\\`, `\\NoPath\\`
- [ ] Create authoritative list of system path identifiers in standard library
- [ ] Document custom path identifier creation syntax
- [ ] Explain OS-specific path resolution at runtime
- [ ] Provide cross-platform path examples
- [ ] Document `pg\path` reserved fields (`.unix`, `.windows`)
- [ ] Document name collision detection between aliases and path identifiers
- [ ] Cross-reference with Decision #1 (reserved enumerations)
- [ ] Cross-reference with Decision #9 (enumeration syntax and aliases)


---

### Issue #15: Expansion Operator - Nesting Rules
**Status:** DECIDED ✓
**Severity:** MEDIUM (High Priority)
**Reference:** inconsistencies-log.md lines 517-555
**Decided:** 2025-11-11

**Problem Summary:**
- The `[~]` expansion operator nesting rules are inconsistently shown
- Examples show varying nesting depths without clear rules
- Unclear when `[~]` prefix is needed vs implicit
- No documentation on variable scope at each nesting level

**Decision:**

**Block Element Parent-Child Relationships with Implicit Expansion**

All block elements have hierarchical relationships, and **all parent-child relationships have IMPLICIT `[~]` expansion** built-in. Explicit `[~]` prefix is only needed in specific scenarios.

**Block Element Hierarchy:**

**1. `[|]` Pipeline Definition Block - Parent of:**
- `[i]` - Input declaration
- `[t]` - Trigger
- `[Q]` - Queue control
- `[r]` - Run/operation
- `[p]` - Parallel execution
- `[b]` - Batch processing
- `[\]` - Setup block
- `[/]` - Cleanup block
- `[o]` - Output declaration

**2. Any Block with Pipeline Call - Parent of:**
- `[<]` - Input assignment (push INTO)
- `[>]` - Output assignment (pull FROM)

These blocks (`[<]` and `[>]`) are used to pass IO into the called pipeline.

**CRITICAL IMPLEMENTATION NOTE:**

These parent-child block relationships are **fundamental to the parser architecture**. Once the lexer completes tokenization, the parser uses these hierarchical relationships to construct the serialized Intermediate Representation (IR).

**Parser Implementation Strategy:**
- Block hierarchy defines the parse tree structure
- Parent-child relationships determine IR nesting
- Implicit expansion semantics are encoded in the IR
- Explicit `[~]` markers indicate scope nesting in the IR
- The IR will represent the semantic structure for the compiler/runtime

**Implicit vs Explicit `[~]` Rules:**

**IMPLICIT `[~]` (No prefix needed - Built into parent-child relationship):**

All parent-child relationships automatically have expansion semantics:

```polyglot
[r] |SomeOperation
[<] .input: pg\string << "value"    // Implicit expansion - no [~] needed
[>] .output >> result               // Implicit expansion - no [~] needed
```

The parent block `[r]` automatically expands to contain its children `[<]` and `[>]`.

**EXPLICIT `[~]` Required (When adding operations WITHIN an expanded context):**

**Case 1: Sequential Operations Within Parallel Blocks**

When you have **additional sequential operations** inside a `[p]` block, use `[~]` to indicate they run **WITHIN** the parallel block (not in parallel with it):

```polyglot
[p] |ProcessPartA
[<] .data: pg\string << .shared_data
[~][r] |TransformData                   // [~] means: runs WITHIN parallel block
[~][<] .input: pg\string << .data       // Child of [~][r] - implicit expansion
[~][r] |ValidateData                    // [~] means: runs WITHIN parallel block
[~][<] .value: pg\string << temp        // Child of [~][r] - implicit expansion
```

**Without `[~]` prefix**, the `[r]` blocks would run **in parallel** with `[p]`, not **within** it.

**Case 2: Error Handling with Nested Operations**

Error handling blocks `[!]` that contain multiple operations require explicit `[~]`:

```polyglot
[r] |SomeErrorPronePipeline
[~][!] !pg.Network.Timeout              // Error handler WITHIN [r]
[~][<] .timeout: pg\dt << DT"3m"        // Child of [~][!] - implicit expansion
[~][~][>] .message: pg\string >> err_msg   // Nested operation WITHIN error handler
[~][~][r] |U.Log.Warning                   // Sequential operation WITHIN error handler
[~][~][<] .text: pg\string << err_msg      // Child of [~][~][r] - implicit expansion
[~][!] !pg.*                            // Catch any other pg error WITHIN [r]
[~][~][r] |U.Log.Error                     // Handler WITHIN second error block
```

**Explanation:**
- First `[~]` - `[!]` is **within** the `[r]` block
- Second `[~]` - Operations are **within** the `[!]` error handling block
- `[<]` and `[>]` don't need additional `[~]` - they're children of their parent block (implicit expansion)

**Case 3: Nested Expansion Operators (e.g., ForEach inside ForEach)**

When using unpack operators like `~Array.ForEach` that expand collections:

```polyglot
[r] ~Array.ForEach                  // Level 0 - expanding outer array
[~][r] ~Array.ForEach               // Level 1 - expanding inner array WITHIN outer
[~][~][r] |ProcessNestedItem        // Level 2 - operation WITHIN inner expansion
[~][~][<] .item: pg\string << value // Child of [~][~][r] - implicit expansion
```

**Nesting Depth:**
- **Recommended**: Minimize nesting depth for readability
- **Not forbidden**: No maximum depth limit
- **Implicit expansion reduces nesting**: Common scenarios don't need explicit `[~]`

**Variable Scope Rules:**

**Each expansion level creates its own scope** - same scope rules apply at each level.

**Variable Accessibility Depends on Execution Model:**

**1. Parallel Execution (Mini-Pipeline Model):**

When branches run in **parallel** (e.g., `[p]` blocks) or from **unpack operator** (e.g., `~Array.ForEach`):
- **Implicit copy semantics** apply (like mini-pipelines in Decision #12)
- Variables from outer scope are **copied in** at instantiation
- Outputs must be **explicitly declared** via `[Y]` join block
- **Prevents race conditions** - no shared mutable state across parallel threads

```polyglot
[r] .data: pg\string << "shared"
[r] ~Array.ForEach                   // Parallel expansion
[~][r] |ProcessItem
[~][<] .input: pg\string << .data    // Copy of .data
// .data from outer scope is copied in
// Changes to .input don't affect outer .data
```

**2. Sequential Execution:**

When branches run **sequentially** (one after another):
- Variables from outer scopes are **directly accessible**
- Changes to variables **affect** outer scope
- No copy semantics needed

```polyglot
[r] .counter: pg\int << 0
[r] |IncrementCounter
[~][r] |AddOne                       // Sequential WITHIN IncrementCounter
[~][<] .value: pg\int << .counter    // Direct access to .counter
[~][>] .result >> counter            // Update outer .counter
```

**Why Different Scope Rules?**

This design **prevents race conditions** when variables are used with parallel threads:
- **Parallel/Unpack**: Copy semantics - each thread gets its own copy
- **Sequential**: Direct access - safe because no concurrent modification

**Visual Nesting Example:**

```polyglot
// Level 0 (outer scope)
[r] .data: pg\string << "input"

// Level 1 - operation expands
[r] ~Array.ForEach
[~][r] .item: pg\string              // Implicit copy from outer scope

// Level 2 - nested expansion
[~][r] ~String.Split
[~][~][r] |ProcessToken              // WITHIN nested expansion
[~][~][<] .token: pg\string          // Child of [~][~][r] - implicit

// Back to Level 1
[~][r] |ValidateItem                 // WITHIN first expansion
[~][<] .value: pg\string << .item    // Child of [~][r] - implicit
```

**Key Principles:**

1. **Implicit expansion is the default** - parent-child relationships don't need `[~]`
2. **Explicit `[~]` shows containment** - "this runs WITHIN that context"
3. **Each `[~]` adds one nesting level** - `[~][~]` is two levels deep
4. **Scope rules depend on execution model** - parallel uses copies, sequential uses direct access
5. **Design prevents race conditions** - parallel execution is thread-safe by default

**Corrected Examples from Decision #12:**

**Parallel Block with Sequential Operations:**
```polyglot
[p] |ProcessPartA
[<] .data: pg\string << .shared_data
[~][r] |TransformData                   // WITHIN parallel block
[~][<] .input: pg\string << .data       // Child of [~][r] - implicit
[~][>] .transformed >> temp
[~][r] |ValidateData                    // WITHIN parallel block
[~][<] .value: pg\string << temp        // Child of [~][r] - implicit
[~][>] .validated >> result1
```

**Corrected Examples from Decision #13:**

**Error Handling with Nested Operations:**
```polyglot
[r] |ReadFile
[<] .path: pg\path << "data.txt"
[~][!] !pg.FileSystem.NotFound          // Error handler WITHIN [r]
[~][~][>] .message: pg\string >> err_msg   // WITHIN error handler
[~][~][r] |U.Log.Error                     // WITHIN error handler
[~][~][<] .msg: pg\string << err_msg       // Child of [~][~][r] - implicit
[~][~][r] |U.Retry.After                   // WITHIN error handler
[~][~][<] .seconds: pg\uint << 5           // Child of [~][~][r] - implicit
```

**Rationale:**
- **Simplifies common cases**: Implicit expansion reduces `[~]` prefix noise
- **Explicit when needed**: `[~]` clarifies containment for complex scenarios
- **Thread-safe by design**: Scope rules prevent race conditions
- **Consistent model**: Same parent-child semantics throughout language
- **Flexible nesting**: No artificial depth limits
- **Clear visual structure**: Nesting depth is immediately visible from `[~]` count

**Connection to Other Decisions:**
- **Decision #6**: Unpack operator `~` creates expanded contexts requiring `[~]` nesting
- **Decision #12**: Parallel blocks follow mini-pipeline model with copy semantics
- **Decision #13**: Error handling uses `[~]` for nested operations

**Action Items:**
- [ ] Document block element hierarchy (`[|]` as parent)
- [ ] Document implicit expansion for all parent-child relationships
- [ ] Explain when explicit `[~]` is required vs implicit
- [ ] Document that `[~]` indicates "runs WITHIN" parent context
- [ ] Provide examples of parallel blocks with sequential operations using `[~]`
- [ ] Provide examples of error handling with `[~]` nesting
- [ ] Document variable scope rules for parallel vs sequential execution
- [ ] Explain copy semantics for parallel/unpack contexts
- [ ] Explain direct access for sequential contexts
- [ ] Document race condition prevention design
- [ ] Provide visual nesting examples showing depth
- [ ] Update Decision #12 examples to use `[~][r]` syntax
- [ ] Update Decision #13 examples to use `[~][!]` and `[~][~][r]` syntax
- [ ] Document recommended practice to minimize nesting depth
- [ ] Cross-reference with Decision #6 (unpack operator)
- [ ] Cross-reference with Decision #12 (parallel execution)
- [ ] Cross-reference with Decision #13 (error handling)


---

### Issue #16: Comment Syntax
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 556-578
**Decided:** 2025-11-11

**Problem Summary:**
- Inconsistent markdown escaping in v0.0.1 showing `\\` vs `\\\\` in comment examples
- Confusion about whether backslash `\\` or forward slash `//` is the comment marker
- No documentation of multi-line comment syntax
- Documentation formatting issue causing display inconsistencies

**Decision:**

**Canonical Comment Syntax:**

**1. Single-Line Comments:**
```polyglot
// This is a single-line comment
[r] .data: pg\string << "value"  // Comment at end of line
```

**2. Multi-Line Comments:**
```polyglot
/* This is a multi-line comment
   that spans multiple lines
   and can include detailed explanations */

[r] .config: pg\serial << serial{
  /* Configuration block for database
     with multi-line explanation */
[^]  "host": "localhost",
[^]  "port": 8080
[^]}
```

**Key Rules:**

1. **Single-line comments** use `//` (double forward slash)
   - Comment starts at `//` and continues to end of line
   - Can be on its own line or at the end of a code line

2. **Multi-line comments** use `/* ... */` (C-style block comments)
   - Start with `/*` and end with `*/`
   - Can span multiple lines
   - Useful for longer explanations, documentation, or temporarily disabling code blocks

3. **Not `\\` (backslash)** - v0.0.1 showed incorrect examples with `\\` due to markdown escaping issues
   - `\\` is NOT the comment marker
   - `\\Name\\` syntax is for path identifiers and aliases (Decision #14)

**Markdown Rendering Fix:**

The v0.0.1 documentation issue was a **markdown escaping problem**, not a language syntax ambiguity:
- Code blocks sometimes showed `\\\\` (double backslash escaped) when displaying `\\`
- This created confusion about comment syntax
- v0.0.2 will ensure all code blocks properly escape to show `//` for comments and `\\` for path identifiers

**Examples:**

**Single-line comments:**
```polyglot
// Define input parameters
[i] .file_path: pg\path

// Process the file
[r] |ReadFile
[<] .path: pg\path << .file_path  // Use input path
```

**Multi-line comments:**
```polyglot
/*
 * This pipeline processes customer data
 * by reading from a file, validating the content,
 * and then storing results in the database.
 */
[|] ProcessCustomerData
[i] .input_file: pg\path
[i] .db_connection: pg\string

[r] |ReadFile
[<] .path: pg\path << .input_file

/*
 * Validation step - ensures data meets requirements:
 * - Non-empty fields
 * - Valid email format
 * - Positive account balance
 */
[r] |ValidateData
[<] .data: pg\serial << .file_content
```

**Distinction from Other `\\` Usage:**

```polyglot
// This is a comment - uses forward slash
[r] .path: pg\path << \\DataDir\\file.txt  // Path identifier - uses backslash
```

**Rationale:**
- **Familiar syntax**: `//` and `/* */` are widely recognized from C, C++, Java, JavaScript, etc.
- **Clear distinction**: Forward slash for comments, backslash for type separators and identifiers
- **No ambiguity**: Different symbols prevent confusion
- **Tool support**: Standard comment syntax works with existing syntax highlighters
- **Documentation clarity**: Fixes markdown escaping issues from v0.0.1

**Action Items:**
- [ ] Update all v0.0.1 comment examples from `\\` to `//` in v0.0.2
- [ ] Document single-line comment syntax: `// comment`
- [ ] Document multi-line comment syntax: `/* comment */`
- [ ] Fix all markdown code blocks to properly escape and display `//` for comments
- [ ] Ensure path identifier examples clearly show `\\Name\\` (not comments)
- [ ] Add syntax highlighting rules for comments in documentation
- [ ] Provide examples showing comments, path identifiers, and type separators together
- [ ] Document that `\\` is NOT used for comments (clarify Decision #14 connection)
- [ ] Update syntax reference with comprehensive comment examples


---

## MEDIUM PRIORITY ISSUES

### Issue #17: File Naming Inconsistencies
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 579-596
**Decided:** 2025-11-11

**Problem Summary:**
- v0.0.1 documentation files use inconsistent naming formats
- Examples: `01-core-syntax.md` (kebab-case), `07-time literals.md` (dash then space), `06 collections.md` (space), `Syntax Reference.md` (Title case)
- No clear standard for file naming convention

**Decision:**

**Canonical File Naming Convention: kebab-case (lowercase with dashes)**

**Rules:**

1. **All documentation files use kebab-case** (lowercase letters with dashes)
   - `core-syntax.md`
   - `type-system.md`
   - `syntax-reference.md`
   - `getting-started.md`

2. **No numeric prefixes** - files are NOT numbered
   - ✓ `core-syntax.md`
   - ✗ `01-core-syntax.md`
   - ✗ `1-core-syntax.md`

3. **Exception: README.md remains uppercase**
   - `README.md` - Standard convention for repository root
   - All other files use kebab-case

**Examples:**

**Current v0.0.1 (inconsistent) → v0.0.2 (standardized):**
```
01-core-syntax.md        →  core-syntax.md
02-type-system.md        →  type-system.md
07-time literals.md      →  time-literals.md
06 collections.md        →  collections.md
Syntax Reference.md      →  syntax-reference.md
getting-started.md       →  getting-started.md (no change)
README.md                →  README.md (exception - stays uppercase)
```

**Directory Structure Example:**
```
docs/
├── README.md                    # Exception - uppercase
├── getting-started.md
├── core-syntax.md
├── type-system.md
├── enumerations.md
├── macros.md
├── database-integration.md
├── file-integration.md
├── examples.md
├── bnf-grammar.md
└── visual-syntax-guide.md
```

**Why kebab-case?**

1. **URL-friendly**: Works well in web documentation (no encoding needed)
2. **Unix convention**: Standard for Linux/Unix systems
3. **Git-friendly**: Case-insensitive filesystems handle it consistently
4. **Readable**: Dashes clearly separate words
5. **Consistent with project style**: Matches directory naming patterns
6. **No numbers**: Documentation organization should use directory structure and navigation, not file prefixes

**Rationale:**
- **Simplicity**: One clear rule for all files (except README.md)
- **Cross-platform**: Works identically on Windows, macOS, Linux
- **Standard practice**: Common in modern documentation projects
- **Easy to remember**: No ambiguity about spacing, casing, or numbering
- **Future-proof**: Adding/removing files doesn't require renumbering

**Action Items:**
- [ ] Rename all v0.0.1 files to kebab-case in v0.0.2
- [ ] Remove all numeric prefixes from filenames
- [ ] Update all file references and links in documentation
- [ ] Update navigation/index files with new filenames
- [ ] Document file naming convention in contributor guide
- [ ] Keep README.md as uppercase (standard exception)
- [ ] Ensure directory names also use kebab-case for consistency
- [ ] Update build scripts/tools that reference old filenames


---

### Issue #18: Code Block Language Tags
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 598-627
**Decided:** 2025-11-11

**Problem Summary:**
- v0.0.1 documentation uses inconsistent code block language tags
- Some use `polyglot`, some use `text`, some use no tag at all
- Inconsistent syntax highlighting in documentation

**Decision:**

**Canonical Code Block Language Tag: `polyglot`**

**Rules:**

1. **All Polyglot code examples use `polyglot` language tag**
   ````markdown
   ```polyglot
   [r] .x: pg\int << 5
   ```
   ````

2. **No untagged code blocks for Polyglot code**
   - ✓ Use: ` ```polyglot `
   - ✗ Avoid: ` ``` ` (no tag)
   - ✗ Avoid: ` ```text `

3. **Other language tags for non-Polyglot content**
   - Bash commands: ` ```bash `
   - JSON data: ` ```json `
   - Plain text output: ` ```text `

**Examples:**

**Correct usage:**
````markdown
```polyglot
[|] ProcessData
[i] .input_file: pg\path
[r] |ReadFile
[<] .path: pg\path << .input_file
[X]
```
````

**Incorrect usage (v0.0.1 inconsistencies):**
````markdown
```text
[r] .x: pg\int << 5
```

```
[r] .x: pg\int << 5
```
````

**Mixed content example:**
````markdown
Polyglot code:
```polyglot
[r] .config_path: pg\path << \\DataDir\\config.json
```

Shell command to run:
```bash
polyglot run pipeline.pg
```

Expected output:
```text
Pipeline executed successfully
```
````

**Benefits:**

1. **Future syntax highlighting**: Enables proper syntax highlighting in:
   - GitHub/GitLab markdown viewers
   - Documentation websites
   - IDEs with Polyglot support
   - Static site generators

2. **Consistency**: All Polyglot code clearly identified

3. **Tool integration**: External tools can parse and extract Polyglot code blocks

4. **Standard practice**: Follows markdown conventions for language-specific code blocks

**Rationale:**
- **Enables syntax highlighting**: Standard language tag allows future highlighter integration
- **Clear identification**: Readers immediately know which language code examples use
- **Consistency**: One tag for all Polyglot code throughout documentation
- **Future-proof**: Prepares documentation for Polyglot syntax highlighter plugins
- **Best practice**: Follows markdown conventions used by major documentation platforms

**Action Items:**
- [ ] Update all Polyglot code blocks to use `polyglot` language tag
- [ ] Remove all `text` tags from Polyglot code examples
- [ ] Add language tags to any untagged Polyglot code blocks
- [ ] Document code block convention in documentation style guide
- [ ] Create syntax highlighter definition for `polyglot` language (future)
- [ ] Ensure all new documentation uses `polyglot` tag consistently


---

### Issue #19: Duplicate Content with Variations
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 628-651
**Decided:** 2025-11-11

**Problem Summary:**
- Same content appears in multiple v0.0.1 files with slight variations
- Type System appears in 3 files with different examples/completeness
- Core Syntax appears in 3 files with different detail levels
- Creates confusion about which version is authoritative

**Decision:**

**Documentation Organization Principles for v0.0.2:**

**1. User (Hasan) is the Source of Truth**
- If any contradiction exists in v0.0.2 documentation, add to decision log to ask user for clarification
- User's word overrides any documentation inconsistency
- Document conflicts clearly and escalate for resolution

**2. DRY (Don't Repeat Yourself) Principle**
- Write content once, reference it from multiple places
- Avoid copying and pasting content between documents
- Use cross-references and links instead of duplication

**3. Different Audiences Allowed**
- If documents serve **different audiences**, variations are acceptable
  - Example: "Getting Started" (beginners) vs "Complete Reference" (advanced users)
  - Example: "Quick Tutorial" (hands-on learners) vs "Conceptual Overview" (theory-focused readers)
- If documents serve the **same audience**, NO duplication allowed
  - Example: Two "complete reference" documents with different content → consolidate into one

**4. Single Source of Truth Per Topic**
- Each technical topic has ONE authoritative document
- Other documents link to the authoritative source
- Example structure:
  - `type-system.md` - THE complete reference for types
  - `getting-started.md` - Links to type-system.md, doesn't duplicate it
  - `quick-reference.md` - Links to type-system.md, doesn't duplicate it

**Documentation Structure for v0.0.2:**

**Core Documents (Authoritative Sources):**
- Each topic gets ONE complete, authoritative reference document
- These are the "source of truth" for their respective topics
- Examples:
  - `type-system.md` - Complete type system reference
  - `syntax-reference.md` - Complete syntax reference
  - `enumerations.md` - Complete enumeration system reference

**Supporting Documents (Link to Core):**
- `getting-started.md` - Beginner tutorial (different audience)
- `quick-reference.md` - Cheat sheet for experienced users (different audience)
- `examples.md` - Practical examples with links to core docs
- All supporting documents LINK to core documents, don't duplicate content

**Handling Conflicts:**

When generating v0.0.2, if contradictions are found:
1. **Document the conflict** in decision log
2. **Flag for user review** - "Issue #XX: Contradiction found between..."
3. **DO NOT make assumptions** - always ask user which is correct
4. **User decides** - their decision becomes the canonical version

**Examples:**

**GOOD - Different Audiences:**
```
getting-started.md:
  "Polyglot uses types like pg\int. For complete details, see type-system.md"
  [Link to type-system.md#integers]

type-system.md:
  [Complete technical specification of pg\int with all details]
```

**BAD - Same Audience, Duplicated:**
```
reference-1.md:
  [Complete type system specification]

reference-2.md:
  [Complete type system specification with slight differences]
```

**GOOD - DRY with Cross-References:**
```
syntax-reference.md:
  "Enumerations use # marker. See enumerations.md for complete details."

enumerations.md:
  [Complete enumeration documentation - THE source of truth]
```

**Rationale:**
- **User as authority**: Prevents documentation drift from user's intent
- **DRY reduces maintenance**: Update content once, not in multiple places
- **Different audiences justified**: Beginners and experts need different depth
- **Single source prevents conflicts**: No ambiguity about which version is correct
- **Clear escalation path**: When conflicts arise, ask user for clarification

**Action Items:**
- [ ] Identify all authoritative source documents for v0.0.2
- [ ] Consolidate duplicate content serving same audience
- [ ] Convert duplications to cross-references and links
- [ ] Clearly label each document's intended audience
- [ ] Create documentation style guide specifying:
  - Which document is authoritative for each topic
  - When duplication is allowed (different audiences only)
  - How to handle cross-references
- [ ] When conflicts found during v0.0.2 generation:
  - Document in decision log
  - Flag for user review
  - DO NOT resolve automatically
- [ ] Review all v0.0.1 duplicate content and either:
  - Consolidate if same audience
  - Differentiate if different audiences
  - Link to authoritative source


---

### Issue #20: Redundant Examples
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 652-675
**Decided:** 2025-11-11

**Problem Summary:**
- Same examples repeated across multiple v0.0.1 files with variations
- "Hello World" pipeline appears in at least 3 files with different syntax
- Data processing pipelines show multiple variations with inconsistent syntax choices
- Creates confusion about which syntax is correct

**Decision:**

**Canonical Examples Repository with Educational Snippets**

**1. Single Authoritative Examples Repository**

Create ONE canonical examples document/directory that contains all official, validated Polyglot code examples. This is the **single source of truth** for all examples.

**Structure:**
```
docs/examples/
├── README.md                          # Index of all examples
├── hello-world.md                     # Hello World variations
├── data-processing.md                 # Data processing pipelines
├── file-operations.md                 # File I/O examples
├── error-handling.md                  # Error handling patterns
├── parallel-execution.md              # Parallel processing examples
├── database-integration.md            # Database examples
├── time-and-scheduling.md             # Time literals and triggers
└── complete-applications.md           # Full application examples
```

**Each example file contains:**
- **Complete, validated code** that compiles and runs
- **Clear documentation** of what the example demonstrates
- **Variations** showing different valid approaches (when applicable)
- **Comments** explaining key concepts
- **Links** to relevant reference documentation

**2. Educational Snippets in Other Documents**

Other documentation files (getting-started.md, syntax-reference.md, etc.) can include **abbreviated snippets** from canonical examples:

**Snippet Rules:**
- Must be **valid Polyglot syntax** (no pseudo-code)
- Should be **complete enough** to understand the concept
- Must **link back** to the full canonical example
- Can **simplify** for educational purposes (fewer inputs, shorter pipelines)
- Should **match** the syntax style of the canonical version

**Example of Educational Snippet:**
```markdown
getting-started.md:

## Your First Pipeline

Here's a simple "Hello World" pipeline:

```polyglot
[|] HelloWorld
[r] |U.Print
[<] .text: pg\string << "Hello, World!"
[X]
```

This demonstrates basic pipeline structure. For more variations and complete examples,
see [examples/hello-world.md](examples/hello-world.md).
```

**3. Varied Examples with User Approval Process**

When creating examples that show **multiple valid approaches**, document all variations and present them to the user (Hasan) for approval:

**Example Approval Workflow:**
1. **Identify the concept** to demonstrate (e.g., "error handling with retry")
2. **Generate 2-3 variations** showing different valid syntax choices
3. **Present to user** for approval
4. **User approves/modifies** which variations to include
5. **Document approved variations** in canonical examples

**Example: Multiple Valid Approaches**
```polyglot
// Variation 1: Minimal error handling
[r] |ReadFile
[<] .path: pg\path << "data.txt"
[!] !pg.FileSystem.NotFound
[r] |U.Log.Error
[<] .msg: pg\string << "File not found"

// Variation 2: Detailed error handling with field extraction
[r] |ReadFile
[<] .path: pg\path << "data.txt"
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code
[r] |U.Log.Error
[<] .msg: pg\string << err_msg
[<] .code: pg\int << err_code

// Variation 3: Error handling with retry logic
[r] |ReadFile
[<] .path: pg\path << "data.txt"
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Warning
[<] .text: pg\string << err_msg
[r] |U.Retry.After
[<] .seconds: pg\uint << 5
```

Each variation would be documented with:
- **What it demonstrates** (minimal vs detailed vs retry)
- **When to use it** (simple logging vs debugging vs resilience)
- **Why it's valid** (all follow canonical syntax rules)

**4. All Examples Must Use Valid Syntax**

**CRITICAL REQUIREMENT:** Every example in v0.0.2 documentation must:
- Follow **all canonical syntax rules** from decisions #1-#19
- Compile without errors (validated against language spec)
- Use **correct block markers** with proper casing (Decision #7)
- Use **correct operators** (`<<`, `>>`, `~`, `|`, `@`, `#`, `!`) (Decision #6)
- Follow **correct type syntax** (`pg\type`) (Decision #2)
- Use **correct comment syntax** (`//` and `/* */`) (Decision #16)
- Include proper **error handling** where appropriate (Decision #13)

**Invalid Examples Are NOT Allowed:**
- ❌ Pseudo-code approximations
- ❌ Simplified syntax that doesn't compile
- ❌ Outdated v0.0.1 syntax inconsistencies
- ❌ Examples with syntax errors
- ❌ Examples contradicting canonical decisions

**5. Documentation Process for Examples**

**Step 1: Create Canonical Example**
```markdown
examples/hello-world.md:

# Hello World Examples

## Basic Hello World

```polyglot
[|] HelloWorld
[r] |U.Print
[<] .text: pg\string << "Hello, World!"
[X]
```

**Demonstrates:** Basic pipeline structure, string literal, utility call

**Concepts:**
- `[|]` pipeline definition (Decision #7)
- `[r]` run block for operations (Decision #7)
- `[<]` input assignment with `<<` operator (Decision #4)
- `pg\string` type syntax (Decision #2)
- `[X]` pipeline end marker (Decision #7)

[... more variations ...]
```

**Step 2: Reference in Other Docs**
```markdown
getting-started.md:

See the complete [Hello World examples](examples/hello-world.md)
for more variations including:
- Hello World with input parameters
- Hello World with triggers
- Hello World with error handling
```

**Step 3: Use Snippets for Education**
```markdown
syntax-reference.md:

Pipeline definitions start with `[|]` and end with `[X]`:

```polyglot
[|] PipelineName
// ... pipeline contents ...
[X]
```

See [examples/](examples/) for complete examples.
```

**Benefits:**

1. **Single Source of Truth**: All examples validated in one place
2. **Valid Syntax Everywhere**: No pseudo-code or invalid examples
3. **User Approval**: Variations reviewed by user before documentation
4. **Educational Flexibility**: Snippets can simplify for learning while linking to complete versions
5. **Maintainability**: Update examples once, references stay correct
6. **Quality Assurance**: All examples compile and follow canonical syntax
7. **Progressive Learning**: Beginners see snippets, advanced users see complete examples

**Rationale:**
- **Consistency**: All examples follow canonical syntax decisions
- **User control**: User approves all example variations before inclusion
- **DRY principle**: Examples written once, referenced many times (connects to Decision #19)
- **Quality**: Valid syntax ensures examples work correctly
- **Educational value**: Snippets make learning easier while maintaining accuracy
- **Documentation integrity**: No conflicting syntax examples in v0.0.2

**Action Items:**
- [ ] Create `docs/examples/` directory structure
- [ ] Document all canonical examples with complete, valid syntax
- [ ] Validate all examples against language specification
- [ ] For each example showing variations:
  - [ ] Generate 2-3 valid variations
  - [ ] Present to user for approval
  - [ ] Document only approved variations
- [ ] Convert duplicate examples in v0.0.1 to:
  - [ ] Canonical versions in examples directory
  - [ ] Snippets with links in educational docs
- [ ] Create examples index (README.md in examples/)
- [ ] Review all v0.0.2 documentation to ensure:
  - [ ] No invalid syntax examples
  - [ ] No pseudo-code in place of valid examples
  - [ ] All snippets link back to canonical examples
- [ ] Establish example validation process (compile check)
- [ ] Document example contribution guidelines for future additions

**Decision:**


**Action Items:**


---

### Issue #21: Execution Model - Runtime vs Language Mismatch
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 676-706
**Decided:** 2025-11-11

**Problem Summary:**
- Language docs show `[w] |W.Python3.10` syntax but don't explain implementation
- Architecture docs mention `uv` tool not referenced in language specs
- Unclear relationship between user-facing syntax and backend implementation
- Confusion about whether runtime version is in code or config

**Decision:**

**Runtime Wrappers - Standard Library Pipelines for Environment Setup**

The `|W.*` namespace in Polyglot's standard library provides **runtime wrapper pipelines** that set up execution environments for different languages and versions.

**Canonical Runtime Wrapper Syntax:**

**1. Fixed Version Runtime Wrappers**
```polyglot
[w] |W.Python3.10      // Sets up Python 3.10 environment via uv
[w] |W.Python3.11      // Sets up Python 3.11 environment via uv
[w] |W.Python3.12      // Sets up Python 3.12 environment via uv
[w] |W.Node18          // Sets up Node.js 18 runtime
[w] |W.Node20          // Sets up Node.js 20 runtime
[w] |W.Rust            // Sets up Rust runtime environment
```

**2. Dynamic Version Runtime Wrappers**
```polyglot
[w] |W.Python
[<] .python_version: pg\string << "3.11"     // User specifies version as input

[w] |W.Node
[<] .node_version: pg\string << "20.5.0"     // User specifies Node version

[w] |W.Ruby
[<] .ruby_version: pg\string << "3.2"        // User specifies Ruby version
```

**3. Multiple Runtime Wrappers in Same Pipeline**
```polyglot
[|] DataProcessing
[i] .input_file: pg\path

// Python wrapper for data processing
[w] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "process_data.py"
[<] .data: pg\path << .input_file

// Node.js wrapper for API calls
[w] |W.Node20
[r] |RunNodeScript
[<] .script: pg\path << "upload_results.js"

[X]
```

**Key Concepts:**

**1. User-Facing Syntax: `|W.*` Pipelines**
- Users call standard library pipelines: `|W.Python3.10`, `|W.Rust`, etc.
- These are **standard library pipelines** just like `|U.*` utilities
- Users don't need to know about underlying implementation
- Clean, simple syntax: `[w] |W.RuntimeName`

**2. Implementation Detail: `uv` for Python**
- `uv` is a **Python packaging tool** that manages Python runtimes
- Polyglot uses `uv` behind the scenes to set up Python environments
- **Users don't interact with `uv` directly** - it's an implementation detail
- `uv` handles Python version management, virtual environments, package installation

**3. The `[w]` Block Marker - Wrapper Context**
- `[w]` establishes a **runtime wrapper context** for subsequent operations
- Code within wrapper context runs in specified environment
- Multiple `[w]` blocks can exist in same pipeline for different runtimes

**4. Abstraction Layer**
- **Language syntax** (`[w] |W.Python3.10`) - what users write
- **Standard library** (`|W.*` pipelines) - documented user-facing API
- **Implementation** (`uv`, language-specific tools) - hidden backend details
- Users control runtime **through syntax**, not configuration files

**How Runtime Wrappers Work:**

**Example: Python Script Execution**
```polyglot
[|] ProcessData
[i] .data_file: pg\path

// Setup Python 3.11 environment
[w] |W.Python3.11

// Run Python script within wrapper
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"
[<] .input: pg\path << .data_file
[>] .results >> analysis_results

[X]
```

**Behind the scenes:**
1. `|W.Python3.11` pipeline calls `uv` to ensure Python 3.11 is available
2. Creates isolated environment (virtual environment via `uv`)
3. Subsequent operations within `[w]` block run in that environment
4. Process isolation and sandboxing handled automatically

**Example: Dynamic Version Selection**
```polyglot
[|] RunTests
[i] .python_version: pg\string

// User provides version at runtime
[w] |W.Python
[<] .python_version: pg\string << .python_version

[r] |RunPythonScript
[<] .script: pg\path << "tests/run_tests.py"

[X]
```

**Standard Library Wrapper Pipelines:**

**Python Wrappers:**
- `|W.Python3.8`, `|W.Python3.9`, `|W.Python3.10`, `|W.Python3.11`, `|W.Python3.12`
- `|W.Python` (with `.python_version` input)
- Backend: Uses `uv` for environment management

**Node.js Wrappers:**
- `|W.Node16`, `|W.Node18`, `|W.Node20`, `|W.Node21`
- `|W.Node` (with `.node_version` input)
- Backend: Uses `nvm` or similar tool

**Other Runtime Wrappers:**
- `|W.Rust` - Rust toolchain
- `|W.Go` - Go runtime
- `|W.Ruby` - Ruby runtime
- `|W.Deno` - Deno runtime
- Additional wrappers in standard library documentation

**Implementation Tools (Hidden from Users):**

These tools are **implementation details** - users don't need to know about them:

- **`uv`**: Python environment and package management
- **`nvm`/`fnm`**: Node.js version management
- **`rustup`**: Rust toolchain management
- **Language-specific tools**: Each runtime has appropriate backend tooling
- **Process isolation**: Sandboxing and security handled by Polyglot runtime

**Documentation Separation:**

**Language Documentation (User-Facing):**
- `[w]` block marker syntax
- `|W.*` standard library pipeline reference
- How to use runtime wrappers in code
- Examples with different languages

**Architecture Documentation (Implementation):**
- How `uv` is used for Python management
- Process isolation mechanisms
- Sandboxing capabilities
- Security model for runtime execution

**Standard Library Documentation:**
- Complete `|W.*` wrapper reference
- Available runtime versions
- Input parameters for dynamic wrappers
- Configuration options

**Rationale:**

- **Clean abstraction**: Users write `|W.Python3.10`, not concerned with `uv`
- **Simple syntax**: Runtime selection in code, clear and readable
- **Flexible**: Both fixed and dynamic version selection supported
- **Standard library approach**: Consistent with `|U.*`, `|T.*`, `|Q.*` patterns
- **Implementation freedom**: Backend tools can change without breaking user code
- **Multiple runtimes**: Same pipeline can use Python, Node, Rust, etc.
- **Explicit control**: Users see exactly which runtime is used in code

**Connection to Other Decisions:**
- **Decision #6**: `|W.*` uses pipeline operator like all standard library calls
- **Decision #7**: `[w]` is a block marker for wrapper context
- **Polyglot philosophy**: Automation language needs to orchestrate multiple runtime environments

**Action Items:**
- [ ] Document `[w]` block marker in v0.0.2 syntax reference
- [ ] Create comprehensive `|W.*` wrapper reference in standard library docs
- [ ] Document available runtime wrappers: Python, Node, Rust, Go, Ruby, Deno
- [ ] Document fixed version wrappers: `|W.Python3.10`, `|W.Node20`, etc.
- [ ] Document dynamic version wrappers with inputs: `|W.Python`, `|W.Node`, etc.
- [ ] Separate user-facing docs (syntax) from architecture docs (implementation)
- [ ] Document that `uv` is implementation detail for Python (not in language docs)
- [ ] Provide examples of multiple runtime wrappers in single pipeline
- [ ] Document process isolation and sandboxing capabilities in architecture
- [ ] Cross-reference with standard library organization (`|U.*`, `|T.*`, `|Q.*`)
- [ ] Document wrapper context - what happens within `[w]` block
- [ ] Add complete block marker list update to include `[w]` (Decision #7)


---

### Issue #22: Queue System - Design vs Syntax Mismatch
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 707-733
**Decided:** 2025-11-11

**Problem Summary:**
- Architecture docs describe three queues (Pending, Dispatch, Pause) with operations (pause, resume, priority_bump, kill)
- Language syntax docs show queue definition and assignment but no queue control operations
- Gap: How do users control queue operations from Polyglot code?

**Decision:**

**Queue System - Built-in Pipelines and Enumeration Extension**

The Polyglot queue system consists of:
1. **Built-in queue control pipelines** in `|Q.*` namespace
2. **Queue enumeration** `#Queues.*` for defining custom queues
3. **Queue assignment** via `|Q.Queue.Assign`

**1. Built-in Queue Control Pipelines (`|Q.*`)**

These are **Polyglot built-in pipelines** in the standard library for controlling pipeline execution:

```polyglot
// Pause a pipeline (moves to Pause queue)
[r] |Q.Pause
[<] .pipeline_id: pg\string << pipeline_ref

// Resume a paused pipeline (moves back to Pending queue)
[r] |Q.Resume
[<] .pipeline_id: pg\string << pipeline_ref

// Kill a pipeline (removes from all queues)
[r] |Q.Kill
[<] .pipeline_id: pg\string << pipeline_ref

// Bump priority in queue
[r] |Q.PriorityBump
[<] .pipeline_id: pg\string << pipeline_ref
[<] .priority: pg\int << 10  // Higher number = higher priority

// Get queue status
[r] |Q.Status
[<] .pipeline_id: pg\string << pipeline_ref
[>] .queue_name: pg\string >> current_queue
[>] .position: pg\int >> queue_position
```

**2. Built-in System Queues**

Three system queues manage pipeline execution lifecycle:

- **`#Queues.Pending`**: Pipelines waiting to be dispatched
- **`#Queues.Dispatch`**: Pipelines currently executing
- **`#Queues.Pause`**: Pipelines paused by user or system

These are **built-in enumerations** - always available without definition.

**3. Custom Queue Definition**

Users can define custom queues by **extending the `#Queues` enumeration** using `[#]` block syntax:

```polyglot
// Define custom queue for background tasks
[#] #Queues.Background
[<] .description: pg\string << "Low priority background processing"
[<] .max_concurrent: pg\int << 5

// Define custom queue for high priority tasks
[#] #Queues.HighPriority
[<] .description: pg\string << "Critical operations"
[<] .max_concurrent: pg\int << 20
```

**4. Queue Assignment**

Assign a pipeline to a queue using `|Q.Queue.Assign`:

```polyglot
[|] BackgroundProcessor
[i] .data: pg\string

// Assign this pipeline to Background queue
[r] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.Background
[<] .pipeline_id: pg\string << @self  // Current pipeline

[r] |ProcessData
[<] .input: pg\string << .data

[X]
```

**5. Complete Example - Custom Queue with Control**

```polyglot
// Define custom queue for batch jobs
[#] #Queues.BatchJobs
[<] .max_concurrent: pg\int << 3

// Pipeline that processes batch jobs
[|] BatchProcessor
[i] .job_id: pg\string
[i] .data: pg\array

// Assign to BatchJobs queue
[r] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.BatchJobs
[<] .pipeline_id: pg\string << @self

// Store pipeline reference for later control
[r] .my_pipeline_ref: pg\string << @self

// Long-running batch processing
[r] |ProcessBatchData
[<] .items: pg\array << .data

[X]

// Later, from another pipeline - pause the batch job
[|] ControlBatchJob
[i] .action: pg\string
[i] .target_pipeline: pg\string

[?] .action == "pause"
[~][r] |Q.Pause
[~][<] .pipeline_id: pg\string << .target_pipeline

[?] .action == "resume"
[~][r] |Q.Resume
[~][<] .pipeline_id: pg\string << .target_pipeline

[?] .action == "kill"
[~][r] |Q.Kill
[~][<] .pipeline_id: pg\string << .target_pipeline

[X]
```

**6. Queue System Architecture**

**Three System Queues (Built-in):**

1. **Pending Queue** (`#Queues.Pending`):
   - New pipelines start here
   - Wait for available execution slots
   - FIFO ordering (unless priority_bump used)

2. **Dispatch Queue** (`#Queues.Dispatch`):
   - Currently executing pipelines
   - Limited by system concurrency settings
   - Pipelines moved here from Pending when slots available

3. **Pause Queue** (`#Queues.Pause`):
   - Pipelines paused by `|Q.Pause` operation
   - Can be resumed to Pending via `|Q.Resume`
   - Preserves pipeline state

**Custom Queues:**
- Extend `#Queues` enumeration with `[#]` syntax
- Define queue-specific settings (max_concurrent, priority, etc.)
- Pipelines assigned via `|Q.Queue.Assign`
- Still interact with system queues (Pending/Dispatch/Pause)

**Queue Control Operations:**
- **`|Q.Pause`**: Pending → Pause or Dispatch → Pause
- **`|Q.Resume`**: Pause → Pending
- **`|Q.Kill`**: Remove from any queue, terminate pipeline
- **`|Q.PriorityBump`**: Move up in Pending queue
- **`|Q.Queue.Assign`**: Assign pipeline to specific queue

**Key Concepts:**

1. **Built-in pipelines**: `|Q.*` namespace provides queue control operations
2. **Enumeration extension**: Define custom queues with `[#] #Queues.QueueName`
3. **Queue assignment**: Use `|Q.Queue.Assign` to assign pipelines to queues
4. **System queues**: Pending, Dispatch, Pause are always available
5. **Pipeline references**: Use `@self` to reference current pipeline or store references for later control
6. **Inter-pipeline control**: Any pipeline can control any other pipeline's queue state

**User-Accessible vs Internal:**

**User-Accessible:**
- `|Q.Pause`, `|Q.Resume`, `|Q.Kill` - queue control operations
- `|Q.PriorityBump` - priority management
- `|Q.Queue.Assign` - custom queue assignment
- `|Q.Status` - query pipeline queue state
- `[#] #Queues.*` - define custom queues
- System queues: `#Queues.Pending`, `#Queues.Dispatch`, `#Queues.Pause`

**Internal (Hidden from Users):**
- Queue scheduler implementation
- Concurrency management algorithms
- Pipeline state serialization
- Inter-queue transitions
- Resource allocation strategies

**Rationale:**
- **Consistent with standard library**: `|Q.*` follows same pattern as `|W.*`, `|U.*`, `|T.*`
- **Built-in pipelines**: Queue control is fundamental - should be built-in, not user-defined
- **Enumeration extension**: Leverages existing `[#]` syntax for type safety
- **Explicit assignment**: Clear syntax for queue membership
- **Pipeline-centric**: Operations use pipeline references, not queue-centric commands
- **Flexible control**: Any pipeline can control any other pipeline's queue state
- **System queues separate from custom**: Clear distinction between built-in lifecycle queues and user-defined queues

**Action Items:**
- [ ] Document `|Q.*` queue control pipelines in standard library reference
- [ ] Document `|Q.Pause`, `|Q.Resume`, `|Q.Kill`, `|Q.PriorityBump` operations
- [ ] Document `|Q.Queue.Assign` for custom queue assignment
- [ ] Document system queues: `#Queues.Pending`, `#Queues.Dispatch`, `#Queues.Pause`
- [ ] Document custom queue definition with `[#] #Queues.*` syntax
- [ ] Document pipeline reference syntax (`@self` and stored references)
- [ ] Document queue system architecture (three system queues + custom queues)
- [ ] Provide complete examples of queue control operations
- [ ] Document inter-pipeline control patterns
- [ ] Clarify user-accessible operations vs internal implementation
- [ ] Cross-reference with enumeration extension syntax (`[#]`)
- [ ] Document queue state transitions (Pending → Dispatch → Complete, Pause flows)
- [ ] Add examples of batch job processing with queue control


---

### Issue #23: Package Registry URLs
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 734-756
**Decided:** 2025-11-11

**Problem Summary:**
- Architecture docs mention package registries but don't specify URL formats
- No documentation on how users specify registry locations
- Authentication mechanisms unclear
- Private vs public registry handling undefined

**Decision:**

**Three-Tier Package Registry System**

Polyglot uses a **three-tier registry architecture** with distinct namespaces and access patterns:

**1. Local Registry** - `Local.*`

Local registries are hosted on the local network (localhost or LAN):

```polyglot
// Localhost registry
Local.localhost@PackageName

// LAN registry via DNS
Local.{DNS-hostname}@PackageName
```

**Usage Examples:**
```polyglot
// Import from localhost
[r] @Local.localhost@MyPackage|ProcessData

// Import from LAN server
[r] @Local.fileserver@SharedUtils|ValidateInput

// Import enumeration from local registry
[i] @Local.devbox@CommonTypes#Status
```

**Key Characteristics:**
- **Localhost**: `Local.localhost@*` - packages on developer's machine
- **LAN hosts**: `Local.{hostname}@*` - where `{hostname}` is DNS-resolvable on local network
- **DNS evaluation**: Hostname resolved via local DNS or hosts file
- **No authentication**: Trust-based access on local network
- **Development focus**: Primarily for development and internal testing

**2. Community Registry** - `Community.*`

Public community registry for open-source Polyglot packages:

```polyglot
// Community registry with username namespace
Community.{username}@PackageName
```

**Usage Examples:**
```polyglot
// Import from community user
[r] @Community.hasan@DataUtils|Transform

// Import enumeration from community
[i] @Community.alice@WebTypes#HTTPStatus

// Import from community organization
[r] @Community.polyglot-team@StandardUtils|Format
```

**Key Characteristics:**
- **Username namespace**: `Community.{username}@*` - each user has their own package namespace
- **Public access**: All packages in community registry are publicly accessible
- **Open source**: Community packages are expected to be open source
- **Central registry**: Hosted by Polyglot project/foundation
- **No authentication for read**: Anyone can import community packages
- **Authentication for publish**: Users must authenticate to publish packages to their namespace

**3. Company Registry** - `Company.*`

Private enterprise registries with access control:

```polyglot
// Company registry with access control
Company.{company-id}@PackageName
```

**Usage Examples:**
```polyglot
// Import from company registry
[r] @Company.acme-corp@InternalAPI|ProcessOrder

// Import from company with team namespace
[r] @Company.techcorp.payments@PaymentUtils|ValidateCard

// Import company enumeration
[i] @Company.startup-xyz@BusinessRules#ApprovalLevels
```

**Key Characteristics:**
- **Company namespace**: `Company.{company-id}@*` - company reserves unique identifier
- **Access control**: Companies control who can read/write packages
- **Authentication required**: Both read and write require authentication
- **Private packages**: Not publicly accessible
- **Hierarchical namespaces**: Companies can create sub-namespaces (e.g., `Company.acme.payments@*`)
- **Enterprise hosting**: Companies host their own registry or use hosted service

**Registry URL Structure:**

**General Pattern:**
```
{RegistryType}.{namespace}@{PackageName}
```

**Components:**
- **`{RegistryType}`**: `Local`, `Community`, or `Company`
- **`{namespace}`**:
  - For Local: `localhost` or DNS hostname
  - For Community: username or organization name
  - For Company: company identifier (can be hierarchical with `.` separator)
- **`{PackageName}`**: The package name

**Complete Package Reference Syntax:**

When using packages in Polyglot code, combine registry URL with pipeline/enumeration access:

```polyglot
// Pipeline from package
@{Registry}.{namespace}@{Package}|{Pipeline}

// Enumeration from package
@{Registry}.{namespace}@{Package}#{Enumeration}

// Examples:
@Local.localhost@Utils|ProcessData
@Community.hasan@WebTools|FetchURL
@Company.acme@InternalLib#ErrorCodes
```

**Complete Example - Multi-Registry Usage:**

```polyglot
[|] ProcessOrder
[i] .order_data: pg\serial

// Import from local development package
[r] @Local.localhost@DevUtils|ValidateOrder
[<] .order: pg\serial << .order_data

// Import from community package
[r] @Community.hasan@PaymentGateway|ProcessPayment
[<] .amount: pg\uint << .order_data.total
[<] .currency: pg\string << "USD"

// Import from company internal package
[r] @Company.acme.billing@InvoiceGen|CreateInvoice
[<] .order_id: pg\string << .order_data.id
[<] .customer: pg\serial << .order_data.customer

// Use company enumeration
[i] @Company.acme@BusinessRules#OrderStatus
[r] .status: #OrderStatus << #OrderStatus.Processing

[X]
```

**Registry URL Examples Summary:**

```
Local.localhost@MyPackage              // Localhost package
Local.fileserver@SharedLib             // LAN server package
Local.dev-machine.local@TestUtils      // LAN with FQDN

Community.hasan@DataProcessing         // Community user package
Community.polyglot-team@StandardLib    // Community org package

Company.acme-corp@InternalAPI          // Company package
Company.techcorp.payments@PaymentLib   // Company with subnamespace
```

**Key Concepts:**

1. **Three-tier architecture**: Local (development), Community (open source), Company (enterprise)
2. **Namespace isolation**: Each tier has distinct namespace structure
3. **DNS evaluation**: Local registries use DNS for hostname resolution
4. **Username namespaces**: Community registry uses username-based namespacing
5. **Company control**: Companies control access to their private packages
6. **Standard syntax**: `@{Registry}.{namespace}@{Package}|{Pipeline}` or `#{Enumeration}`
7. **Wildcard notation**: `@*` represents all packages in namespace

**Rationale:**
- **Three tiers meet all use cases**: Development (Local), open source (Community), enterprise (Company)
- **Clear namespace separation**: No ambiguity about package source
- **DNS-based local discovery**: Leverages existing network infrastructure
- **Community username namespaces**: Familiar pattern from npm, GitHub, etc.
- **Company flexibility**: Companies can host privately with full access control
- **Consistent syntax**: Same `@` operator for all registry types
- **Security by design**: Authentication required for private/write access
- **Scalable**: Each tier can scale independently

**Action Items:**
- [ ] Document three-tier registry architecture in v0.0.2
- [ ] Document Local registry with localhost and LAN hostname support
- [ ] Document Community registry with username namespaces
- [ ] Document Company registry with access control
- [ ] Document registry URL syntax: `{Type}.{namespace}@{Package}`
- [ ] Document package reference syntax with pipelines and enumerations
- [ ] Document DNS hostname resolution for Local registries
- [ ] Provide examples of multi-registry usage in single pipeline
- [ ] Document wildcard `@*` notation for namespace access
- [ ] Cross-reference with package operator `@` (Decision #6)
- [ ] Document authentication and publishing mechanisms in separate section
- [ ] Create registry administration guide for company setup


---

### Issue #24: CLI Command Reference Incomplete
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 757-788
**Decided:** 2025-11-11

**Problem Summary:**
- Several documents mention CLI commands without formal specification
- No documentation on command-line flags, options, or behavior
- Unclear how to configure Polyglot behavior via CLI
- No reference for CI/CD integration

**Decision:**

**Polyglot CLI Workflow - Compilation and Activation Model**

Polyglot uses a **compile-register-activate workflow** for pipeline execution. This separates code compilation from runtime activation, enabling efficient trigger monitoring and queue-based execution.

**Development Workflow Steps:**

**1. Write Code**
   - Write Polyglot code (`.pg` files)
   - Write associated runtime code (Python, Rust, etc.)
   - Organize as packages with multiple pipelines

**2. Compile Code** - `polyglot compile`
   - Compiles Polyglot code and associated runtime code into **Intermediate Representation (IR)**
   - IR is stored in database for efficient execution
   - Validates syntax and type checking

**Syntax:**
```bash
polyglot compile file.pg
```

**Example:**
```bash
# Compile a single Polyglot file
polyglot compile process-orders.pg

# This compiles:
# - The Polyglot pipeline definitions
# - Associated Python/Rust/etc. code
# - Generates IR (Intermediate Representation)
# - Stores IR in database
```

**What Happens:**
- Parses `.pg` file syntax
- Validates pipeline structure
- Compiles associated runtime code (Python, Rust, etc.)
- Generates optimized IR
- Stores IR in Polyglot database
- Reports compilation errors if any

**3. Register Package** - `polyglot register`
   - Registers the package, making its pipelines **available for activation**
   - Compiles all associated files of that package
   - Makes pipelines discoverable in the system

**Syntax:**
```bash
# Register package by name (if .pg file exists in current directory)
polyglot register {packagename}

# Register specific .pg file
polyglot register file.pg
```

**Examples:**
```bash
# Register package by name (searches for package.pg in current directory)
polyglot register OrderProcessing

# Register specific file (compiles and registers all pipelines in file)
polyglot register src/process-orders.pg

# Register package with all dependencies
polyglot register DataPipeline
```

**What Happens:**
- Locates package file (`.pg` file with matching package name)
- Compiles all associated files (Python, Rust, etc.)
- Registers all pipelines defined in package
- Makes pipelines available for activation
- Updates package registry (Local.localhost by default)

**4. Activate Pipeline** - `polyglot activate`
   - Activates a specific pipeline for **trigger monitoring**
   - Trigger monitor actively watches for pipeline triggers
   - When trigger fires, pipeline is queued and executed according to queue rules

**Syntax:**
```bash
polyglot activate {package}@{pipeline}
```

**Examples:**
```bash
# Activate a pipeline
polyglot activate OrderProcessing@ProcessNewOrders

# Activate pipeline from community package
polyglot activate Community.hasan@DataUtils@TransformData

# Activate multiple pipelines
polyglot activate OrderProcessing@ProcessNewOrders
polyglot activate OrderProcessing@SendNotifications
```

**What Happens:**
- Loads pipeline IR from database
- Starts trigger monitor for pipeline
- Monitor watches for trigger conditions (time, file, event, etc.)
- When trigger fires:
  - Pipeline added to appropriate queue
  - Queue system schedules execution
  - Pipeline executes with monitored inputs

**Key Concepts:**
- **Active pipeline**: Trigger is being monitored
- **Inactive pipeline**: Exists in registry but not monitoring triggers
- **Activation enables**: Real-time monitoring and automatic execution
- **Multiple activations**: Same pipeline can be activated multiple times with different inputs

**5. Test Pipeline** - `polyglot test`
   - Confirms pipeline works as expected
   - Testing methodology will be defined in later development stage

**Syntax:**
```bash
polyglot test {package}@{pipeline}
```

**Examples:**
```bash
# Test a pipeline
polyglot test OrderProcessing@ProcessNewOrders

# Test with specific inputs (syntax TBD)
polyglot test DataPipeline@Transform --input test-data.json
```

**What Happens:**
- Loads pipeline from registry
- Executes pipeline in test mode
- Reports success/failure
- Details of testing methodology: **DEFERRED** to later development stage

**Complete Workflow Example:**

```bash
# Step 1: Write code (manual - create process-orders.pg and associated Python files)

# Step 2: Compile the code
polyglot compile process-orders.pg
# Output: Compiled successfully. IR stored in database.

# Step 3: Register the package
polyglot register OrderProcessing
# Output: Package 'OrderProcessing' registered.
#         Pipelines available:
#         - ProcessNewOrders
#         - SendNotifications
#         - UpdateInventory

# Step 4: Activate pipelines
polyglot activate OrderProcessing@ProcessNewOrders
# Output: Pipeline activated. Monitoring triggers...

polyglot activate OrderProcessing@SendNotifications
# Output: Pipeline activated. Monitoring triggers...

# Step 5: Test pipeline (optional)
polyglot test OrderProcessing@ProcessNewOrders
# Output: Testing methodology TBD in later development stage
```

**CLI Command Reference:**

**`polyglot compile <file.pg>`**
- **Purpose**: Compile Polyglot code and associated runtime code to IR
- **Input**: Path to `.pg` file
- **Output**: IR stored in database
- **Validates**: Syntax, types, pipeline structure
- **Compiles**: Polyglot + Python/Rust/etc. code

**`polyglot register <packagename>` or `polyglot register <file.pg>`**
- **Purpose**: Register package and make pipelines available
- **Input**: Package name (searches for `.pg` file) OR direct file path
- **Output**: Package registered, pipelines discoverable
- **Compiles**: All associated files in package
- **Registry**: Adds to Local.localhost registry by default

**`polyglot activate <package>@<pipeline>`**
- **Purpose**: Activate pipeline trigger monitoring
- **Input**: Fully qualified pipeline reference
- **Output**: Trigger monitoring active
- **Behavior**: Monitor watches for trigger conditions, queues pipeline when fired
- **Multiple activations**: Supported for same pipeline

**`polyglot test <package>@<pipeline>`**
- **Purpose**: Test pipeline execution
- **Input**: Fully qualified pipeline reference
- **Output**: Test results (methodology TBD)
- **Status**: Testing methodology deferred to later development stage

**Compilation to Activation Flow:**

```
┌──────────────┐
│ Write Code   │
│ (.pg files)  │
└─��────┬───────┘
       │
       ▼
┌──────────────────┐
│ polyglot compile │
│ Syntax: compile file.pg
└──────┬───────────┘
       │
       ▼
┌────────────────────┐
│ IR in Database     │
│ (Optimized, ready) │
└──────┬─────────────┘
       │
       ▼
┌───────────────────┐
│ polyglot register │
│ Syntax: register {pkg} or register file.pg
└──────┬────────────┘
       │
       ▼
┌──────────────────────┐
│ Package Registry     │
│ (Pipelines available)│
└──────┬───────────────┘
       │
       ▼
┌────────────────────┐
│ polyglot activate  │
│ Syntax: activate {pkg}@{pipeline}
└──────┬─────────────┘
       │
       ▼
┌─────────────────────┐
│ Trigger Monitoring  │
│ (Active pipeline)   │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Queue & Execute     │
│ (When trigger fires)│
└─────────────────────┘
```

**Key Design Principles:**

1. **Separation of Concerns**:
   - **Compile**: Syntax validation + IR generation
   - **Register**: Package management + discovery
   - **Activate**: Trigger monitoring + execution

2. **Database-Backed IR**:
   - Compiled IR stored in database
   - Fast pipeline loading
   - No re-compilation on activation

3. **Explicit Activation**:
   - Pipelines don't auto-activate after registration
   - User controls which pipelines are monitoring triggers
   - Enables fine-grained control over system resources

4. **Package-Centric**:
   - Register entire packages
   - Activate individual pipelines
   - Clear namespace: `{package}@{pipeline}`

5. **Registry Integration**:
   - Register adds to Local.localhost by default
   - Can register to other registries (Community, Company)
   - Activate works with any registry tier

**Rationale:**

- **Compile step**: Validates code early, generates optimized IR, catches errors before registration
- **Register step**: Makes pipelines discoverable, manages package namespace, enables import/reference
- **Activate step**: Explicit control over trigger monitoring, resource management, enables/disables automation
- **Test step**: Validates pipeline behavior, ensures correctness, reserved for future enhancement
- **IR in database**: Fast execution, no re-compilation, efficient pipeline loading
- **Separation of concerns**: Clear workflow stages, debugging friendly, predictable behavior
- **Package @ Pipeline syntax**: Consistent with import syntax, clear namespace, unambiguous reference

**Action Items:**
- [ ] Document CLI workflow in v0.0.2 command reference
- [ ] Document `polyglot compile` command with syntax and examples
- [ ] Document `polyglot register` command with both syntaxes (package name and file path)
- [ ] Document `polyglot activate` command with trigger monitoring behavior
- [ ] Document `polyglot test` command (mark testing methodology as TBD)
- [ ] Document IR (Intermediate Representation) storage in database
- [ ] Document workflow diagram: Write → Compile → Register → Activate
- [ ] Document separation between registration (available) and activation (monitoring)
- [ ] Document trigger monitoring behavior when pipeline is activated
- [ ] Document queue-based execution when triggers fire
- [ ] Cross-reference with queue system (Decision #22)
- [ ] Cross-reference with package registry (Decision #23)
- [ ] Add examples of complete workflows from code to execution
- [ ] Document CLI error messages and exit codes (future enhancement)
- [ ] Document CLI flags and options (future enhancement)
- [ ] Create CLI command quick reference sheet
- [ ] Note: Testing methodology deferred to later development stage


---

### Issue #25: Standard Library Documentation Missing
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 789-819
**Decided:** 2025-11-11

**Problem Summary:**
- Many pipeline calls reference `|U.*` utilities and other standard library namespaces
- No complete standard library API reference exists in v0.0.1
- Examples use utilities like `|U.String.*`, `|U.File.*`, `|U.Database.*`, etc. without documentation
- Unclear which utilities exist vs. planned for future
- Users cannot effectively write code without this reference

**Decision:**

**Deferred Standard Library Documentation - Catalog First, Design Later**

The standard library API reference will be **documented after syntax documentation is completed**.

**Approach for v0.0.2:**

**1. Document All Standard Library Instances**
   - Catalog every `|U.*`, `|T.*`, `|W.*`, `|Q.*` reference found in v0.0.1 examples
   - Create comprehensive list of all standard library namespaces and pipelines mentioned
   - Mark each as "referenced in examples" without full API specification yet

**2. Standard Library Namespaces Identified:**

**Fully Documented (from previous decisions):**
- `|W.*` - Runtime wrappers (Decision #21)
- `|Q.*` - Queue control operations (Decision #22)
- `|Y.*` - Join operations (Decision #12)

**Referenced but API Not Yet Defined:**
- `|U.*` - Utility functions (String, File, Database, HTTP, Path, Array, Set, Convert, Math, Compare, Log, Print, Retry, etc.)
- `|T.*` - Triggers (Daily, Every.*, File.*, Event.*, time-based patterns)

**3. Phased Documentation Strategy:**

**Phase 1: Complete Syntax Documentation (Current - Issues #1-#30)**
- Finalize all language syntax rules
- Establish canonical patterns and decisions

**Phase 2: Catalog Standard Library References (After Phase 1)**
- Extract all standard library references from v0.0.1 examples
- Group by namespace and functionality
- Present catalog to user for review

**Phase 3: Design Standard Library APIs (After Phase 2)**
- User reviews catalog and prioritizes utilities
- Design input/output signatures for each pipeline
- Document expected behavior

**Phase 4: Create API Reference (After Phase 3)**
- Complete API documentation with inputs, outputs, types
- Usage examples for each utility
- Implementation status markers

**4. Temporary Approach in v0.0.2:**

Examples can use standard library pipelines to demonstrate syntax:
```polyglot
[r] |U.Print
[<] .text: pg\string << "Hello, World!"

[r] |U.Log.Error
[<] .msg: pg\string << err_msg
```

These show **syntax patterns**, not API documentation. Complete API specs will be added later after syntax is stable.

**Rationale:**

- **Syntax first**: Establish foundation before designing APIs
- **Avoid premature design**: Don't design APIs on unstable syntax
- **User-driven priorities**: User decides which utilities are essential
- **Iterative approach**: Syntax → catalog → design → document
- **Examples remain valid**: Can demonstrate patterns using standard library
- **Clear roadmap**: Phased approach with defined next steps

**Action Items:**
- [ ] Add standard library placeholder section to v0.0.2
- [ ] Note that `|W.*`, `|Q.*`, `|Y.*` are fully specified (cross-reference)
- [ ] Note that `|U.*`, `|T.*` are referenced but APIs are TBD
- [ ] After completing all 30 decisions, catalog all standard library references
- [ ] Extract all namespace usages from v0.0.1 examples
- [ ] Group catalog by namespace and functionality
- [ ] Present catalog to user for prioritization and API design
- [ ] Create complete standard library API reference after design phase


---

### Issue #26: Pipeline vs Workflow vs Function
**Status:** DECIDED ✓
**Severity:** MEDIUM
**Reference:** inconsistencies-log.md lines 820-850
**Decided:** 2025-11-11

**Decision:**

**Terminology Standardization - Pipeline, Workflow, and Function**

Polyglot is an **asynchronous automation language** where all operations are pipelines. To maintain consistency across documentation, use the following terminology:

**1. Pipeline - Individual Unit of Execution**

A **pipeline** is the fundamental unit of execution in Polyglot:
- Individual blackbox with defined inputs and outputs
- Defined using `[|]...[X]` block syntax
- Can be called from other pipelines using `|PipelineName` syntax
- Asynchronous by nature

**Canonical Definition:**
```polyglot
[|] ProcessData
[i] .input_file: pg\path
[r] |ReadFile
[<] .path: pg\path << .input_file
[X]
```

**Usage in Documentation:**
- "This **pipeline** processes customer data"
- "Call the `|ProcessData` **pipeline**"
- "Define a **pipeline** using `[|]` block marker"

**2. Workflow - Collection of Interconnected Pipelines**

A **workflow** is the complete automation consisting of multiple interconnected pipelines and their triggers:
- Multiple pipelines working together
- Includes triggers that activate pipelines
- Represents end-to-end automation process
- May span multiple `.pg` files and packages

**Example Workflow:**
```polyglot
// Workflow: "Order Processing System"
// Consists of 3 interconnected pipelines

[|] ProcessNewOrders
[t] |T.Every.Minute
[r] |FetchOrders
[r] |ValidateOrders
[X]

[|] SendNotifications
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[r] |GetPendingOrders
[r] |ComposeEmails
[r] |SendEmails
[X]

[|] UpdateInventory
[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\inventory.csv
[r] |ReadInventoryFile
[r] |SyncToDatabase
[X]
```

This entire system is the **"Order Processing Workflow"** - it consists of 3 pipelines with their triggers.

**Usage in Documentation:**
- "This **workflow** automates order processing from receipt to fulfillment"
- "The **workflow** consists of 5 pipelines that handle different stages"
- "Trigger the **workflow** by activating its pipelines"

**3. Function - Context-Dependent Term**

The term **function** can be used **interchangeably with pipeline** when discussing Polyglot code, but has a specific meaning when referring to other codebases:

**When discussing Polyglot:**
- "Function" = "Pipeline" (interchangeable)
- "This function processes data" = "This pipeline processes data"
- Prefer "pipeline" for clarity in Polyglot-specific documentation

**When discussing other languages (Python, Rust, JavaScript, etc.):**
- "Function" refers to functions in those languages
- Distinct from Polyglot pipelines
- Used to clarify what code runs inside runtime wrappers

**Examples:**

**Polyglot context (function = pipeline):**
```polyglot
// This pipeline/function validates input
[|] ValidateInput
[i] .data: pg\string
[r] |CheckFormat
[X]
```
"The `ValidateInput` **function** (pipeline) checks data format."

**Other language context (function ≠ pipeline):**
```polyglot
[w] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"
```
"This pipeline calls Python **functions** defined in `analyze.py`."

**4. Terminology Hierarchy**

```
Workflow (highest level - complete automation)
  ├── Pipeline 1 (individual unit)
  ├── Pipeline 2 (individual unit)
  └── Pipeline 3 (individual unit)
      ├── Calls other pipelines
      └── May invoke runtime functions (Python/Rust/etc.)
```

**5. Standard Usage Rules for v0.0.2**

**Use "Pipeline" when:**
- Discussing individual `[|]...[X]` definitions
- Explaining pipeline calls with `|PipelineName`
- Documenting inputs/outputs of a single unit
- Referring to standard library utilities (`|U.*`, `|Q.*`, etc.)

**Use "Workflow" when:**
- Discussing complete automation systems
- Describing end-to-end processes
- Explaining how multiple pipelines work together
- Documenting trigger-based execution flows
- Referring to packages with multiple pipelines

**Use "Function" when:**
- Referring to code in other languages (Python functions, Rust functions)
- Discussing runtime wrapper behavior (`[w]` blocks)
- Can use interchangeably with "pipeline" in Polyglot context (but prefer "pipeline" for clarity)

**6. Examples of Correct Usage**

**Correct - Pipeline:**
```markdown
The `ProcessData` **pipeline** accepts a file path as input and returns processed results.

To call this **pipeline** from another **pipeline**:
```polyglot
[r] |ProcessData
[<] .input: pg\path << "data.csv"
```
```

**Correct - Workflow:**
```markdown
The "Customer Management" **workflow** consists of three **pipelines**:
1. `ImportCustomers` - Triggered daily at midnight
2. `ValidateData` - Triggered on file modification
3. `SyncToDatabase` - Triggered after validation completes

This **workflow** ensures customer data stays synchronized across systems.
```

**Correct - Function (other languages):**
```markdown
The `[w] |W.Python3.11` wrapper executes Python **functions** from `analysis.py`.

The **pipeline** `RunAnalysis` calls multiple Python **functions** within the wrapper context.
```

**7. Async Nature of Pipelines**

Because Polyglot is an **asynchronous language**, all pipelines are inherently async:
- Pipelines don't block waiting for completion
- Triggers fire pipelines asynchronously
- Queue system manages async execution
- Parallel blocks (`[p]`) run pipelines concurrently

This is fundamental to Polyglot's design as an automation language - pipelines execute asynchronously in response to triggers and events.

**Rationale:**

- **Pipeline = individual unit**: Clear, atomic concept - one blackbox with I/O
- **Workflow = connected system**: Describes complete automation from trigger to completion
- **Function flexibility**: Can substitute for "pipeline" in Polyglot context, distinct when discussing other languages
- **Consistent hierarchy**: Workflow > Pipeline > (optionally calls other language functions)
- **Async foundation**: All operations are async pipelines, enabling automation use cases
- **User clarity**: Readers immediately understand scope (pipeline vs workflow)
- **Documentation consistency**: One term per concept across all v0.0.2 docs

**Action Items:**
- [ ] Document "pipeline" as the canonical term for individual units of execution
- [ ] Document "workflow" as the term for interconnected pipeline systems
- [ ] Document "function" usage rules (Polyglot vs other languages)
- [ ] Update glossary with clear definitions of pipeline, workflow, and function
- [ ] Review all v0.0.1 documentation and standardize terminology
- [ ] Replace inconsistent "workflow" with "pipeline" when referring to individual units
- [ ] Replace inconsistent "pipeline" with "workflow" when referring to complete systems
- [ ] Document async nature of all pipelines in architecture docs


---

### Issue #27: Instance vs Execution vs Run
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 851-871
**Decided:** 2025-11-11

**Decision:**

**Runtime Execution Terminology - Instance, Execution, and Running**

Polyglot pipelines follow a **class/object model** for runtime execution. Understanding the distinction between pipeline definitions and their runtime instances is critical for clear documentation.

**1. Pipeline vs Pipeline Instance - The Class/Object Analogy**

Think of the difference between a **class** and an **object**:
- **Pipeline definition** = Class (the template/blueprint)
- **Pipeline instance** = Object (one instantiation of the template)

**Pipeline Definition (Template):**
```polyglot
[|] ProcessData
[i] .input_file: pg\path
[r] |ReadFile
[<] .path: pg\path << .input_file
[>] .content: pg\string >> file_data
[X]
```

This is the **template** - like a class definition in OOP languages.

**Pipeline Instance (Runtime Object):**

When you call this pipeline, you create an **instance**:
```polyglot
[r] |ProcessData
[<] .input_file: pg\path << "data.csv"
```

This creates **one pipeline instance** - like creating an object from a class.

**Key Concept:**
- **One pipeline definition** can have **many pipeline instances** running concurrently
- Each instance has its own state, variables, and lifecycle
- Instances are independent - they don't share state

**2. Pipeline Instance Lifecycle**

A pipeline instance goes through several states from creation to exit:

```
┌─────────────────────┐
│ Pipeline Definition │ (Template - like a class)
│ [|] ProcessData     │
└──────────┬──────────┘
           │
           ▼ (Call creates instance)
┌─────────────────────┐
│ Pipeline Instance   │ (Object instantiated from template)
│ Instance ID: #12345 │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Queue for Dispatch  │ (Pending Queue - waiting to execute)
│ #Queues.Pending     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Running             │ (Includes both execution and paused states)
│ (Dispatch → Exit)   │
└──────────┬──────────┘
           │
    ┌──────┴──────┐
    ▼             ▼
┌─────────┐   ┌──────────┐
│Executing│   │  Paused  │ (Both are "Running")
│ Queue   │   │  Queue   │
└─────────┘   └──────────┘
           │
           ▼
┌─────────────────────┐
│ Exit                │ (Graceful or forceful termination)
└─────────────────────┘
```

**3. Queue Flow for Pipeline Instances**

When a pipeline instance is created, it enters the queue system:

**Step 1: Instance Created**
```polyglot
[r] |ProcessData
[<] .input_file: pg\path << "data.csv"
```
- Creates pipeline instance from `ProcessData` template
- Instance assigned unique ID (e.g., `#12345`)
- Instance immediately goes to **Queue for Dispatch**

**Step 2: Queue for Dispatch (Pending)**
- Instance enters `#Queues.Pending` queue
- Waits for available execution slot
- Queue manages priority and scheduling

**Step 3: Dispatched to Execution**
- When slot available, instance moves to `#Queues.Dispatch`
- Instance starts executing
- State = **"Running"**

**Step 4: Paused (Optional)**
```polyglot
[r] |Q.Pause
[<] .pipeline_id: pg\string << "#12345"
```
- Instance can be paused during execution
- Moves from `#Queues.Dispatch` to `#Queues.Pause`
- State still = **"Running"** (paused is a running state)

**Step 5: Resume from Pause (Optional)**
```polyglot
[r] |Q.Resume
[<] .pipeline_id: pg\string << "#12345"
```
- Instance moves from `#Queues.Pause` back to `#Queues.Pending`
- Will be re-dispatched when slot available
- State still = **"Running"**

**Step 6: Exit**
- Instance completes (graceful exit) or is killed (forceful exit)
- Removed from all queues
- State = **"Exited"**
- Instance lifecycle complete

**4. Terminology Definitions**

**Pipeline Instance:**
- One runtime instantiation of a pipeline definition
- Like an object created from a class
- Has unique instance ID
- Has its own state and variables
- Goes to queue for dispatch when created

**Execution:**
- The act of running code within a pipeline instance
- Occurs when instance is in `#Queues.Dispatch` queue
- Can be paused and resumed
- Part of the "Running" state

**Running:**
- **Broad state** that includes both execution and paused
- Starts when instance is dispatched from `#Queues.Pending`
- Continues through execution AND paused states
- Ends when instance exits (gracefully or forcefully)
- **Running = from dispatch until exit**

**5. "Running" State Includes Both Execution and Paused**

This is a critical distinction:

**Running State:**
- Dispatched from pending queue → **Running starts**
- Executing in `#Queues.Dispatch` → **Running** (actively executing)
- Paused in `#Queues.Pause` → **Running** (paused but not exited)
- Resumed to `#Queues.Pending` → **Running** (waiting to resume)
- Exits (graceful or forceful) → **Running ends**

**Example:**
```polyglot
[|] LongRunningProcess
[i] .data: pg\array

[r] |ProcessBatch
[<] .items: pg\array << .data

// Instance can be paused mid-execution
[r] .instance_id: pg\string << @self

[X]

// From another pipeline - pause the instance
[r] |Q.Pause
[<] .pipeline_id: pg\string << instance_id

// Instance is now in Paused Queue
// But it's still "Running" - not exited
// It can be resumed later

[r] |Q.Resume
[<] .pipeline_id: pg\string << instance_id

// Instance returns to Pending Queue
// Still "Running" - will execute when dispatched
```

**6. Exit Types**

**Graceful Exit:**
- Pipeline instance completes all operations
- Reaches `[X]` end marker
- Exits cleanly with return values

**Forceful Exit:**
- Pipeline instance is killed using `|Q.Kill`
- Immediate termination
- Resources cleaned up
- No return values

**Example - Forceful Exit:**
```polyglot
[r] |Q.Kill
[<] .pipeline_id: pg\string << "#12345"
```

**7. Multiple Instances of Same Pipeline**

One pipeline definition can have many instances running concurrently:

```polyglot
// Definition (template)
[|] ProcessFile
[i] .filename: pg\path
[r] |ReadFile
[<] .path: pg\path << .filename
[X]

// Create 3 instances (3 objects from same class)
[r] |ProcessFile
[<] .filename: pg\path << "file1.csv"  // Instance #1

[r] |ProcessFile
[<] .filename: pg\path << "file2.csv"  // Instance #2

[r] |ProcessFile
[<] .filename: pg\path << "file3.csv"  // Instance #3
```

Each instance:
- Has unique instance ID
- Has its own copy of variables
- Goes to queue independently
- Runs independently
- Exits independently

**8. Standard Usage Rules for v0.0.2**

**Use "Pipeline Instance" when:**
- Referring to one runtime instantiation of a pipeline
- Discussing instance IDs (e.g., `#12345`)
- Explaining queue operations (`|Q.Pause`, `|Q.Resume`, `|Q.Kill`)
- Describing instance lifecycle (created → queued → running → exit)
- Emphasizing that multiple instances can exist from one definition

**Use "Execution" when:**
- Referring to the act of running code
- Discussing active processing (not paused)
- Describing what happens when instance is in `#Queues.Dispatch`
- Explaining performance or timing

**Use "Running" when:**
- Referring to the broad state from dispatch to exit
- Including both execution and paused states
- Describing instance status (running vs exited)
- Explaining that paused instances are still "running" (not exited)

**9. Examples of Correct Usage**

**Correct - Pipeline Instance:**
```markdown
When you call a pipeline, you create a **pipeline instance**. This instance goes to the queue for dispatch and receives a unique ID.

Multiple **pipeline instances** of the same pipeline can run concurrently, each with its own state.
```

**Correct - Execution:**
```markdown
When the **pipeline instance** is dispatched from the pending queue, **execution** begins. The instance actively processes data until it completes or is paused.
```

**Correct - Running:**
```markdown
A **pipeline instance** is **running** from the moment it's dispatched until it exits. This includes both active **execution** and **paused** states.

You can pause a **running** instance using `|Q.Pause`, but it remains **running** (not exited) until you kill it or it completes.
```

**10. Queue State Transitions**

Complete state diagram:

```
Pipeline Definition (Template)
        │
        ▼ (Call creates instance)
Pipeline Instance Created
        │
        ▼
#Queues.Pending (Queue for dispatch)
        │
        ▼ (Dispatch when slot available)
#Queues.Dispatch (Executing) ◄──┐
        │                        │
        ├─► #Queues.Pause (Paused) ──► #Queues.Pending (Resume)
        │
        ▼
Exit (Graceful or Forceful)

Running State = [Dispatch → Exit]
Includes: Dispatch Queue, Pause Queue, Pending (after resume)
```

**Rationale:**

- **Class/object analogy**: Familiar mental model for understanding pipeline vs instance
- **Clear lifecycle**: From definition → instance → queue → running → exit
- **"Running" includes paused**: Critical for understanding queue operations
- **Queue flow**: Explicit path through Pending → Dispatch → (optional Pause) → Exit
- **Multiple instances**: One definition can spawn many concurrent instances
- **Instance independence**: Each instance has its own state and lifecycle
- **Graceful vs forceful exit**: Clear distinction between completion and termination
- **Documentation consistency**: One term per concept across all v0.0.2 docs

**Action Items:**
- [ ] Document pipeline definition vs pipeline instance distinction
- [ ] Document class/object analogy for understanding instances
- [ ] Document complete instance lifecycle (created → queue → running → exit)
- [ ] Document queue flow (Pending → Dispatch → Pause → Resume → Exit)
- [ ] Document "Running" state includes both execution and paused
- [ ] Document graceful vs forceful exit
- [ ] Document multiple instances from one definition
- [ ] Update glossary with clear definitions of instance, execution, and running
- [ ] Provide queue state transition diagrams
- [ ] Review all v0.0.1 documentation and standardize terminology
- [ ] Provide examples showing hierarchy: workflow → pipelines → (other language functions)
- [ ] Create terminology quick reference for documentation contributors
- [ ] Update product brief to use "workflow" for complete automation systems
- [ ] Update language docs to use "pipeline" for individual units
- [ ] Update architecture docs to clarify "workflow execution" as complete system execution


---

## LOW PRIORITY ISSUES

### Issue #28: Internal Document Links
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 872-890
**Decided:** 2025-11-11

**Problem Summary:**
- Some documents reference other documents that don't exist or have wrong paths
- Links to `09-examples.md` from language/README.md, but file doesn't exist
- References to "Complete Examples" throughout, but location varies
- Links use relative paths that may break depending on context

**Decision:**

**Validate and fix all internal links in v0.0.2 documentation**

- Use consistent relative path format from docs root
- Add "See Also" sections with working links
- Consider adding a documentation index/navigation file
- Validate links as part of documentation build process

**Rationale:**
- Broken links create poor user experience
- Consistent path format improves maintainability
- Documentation navigation helps users find related content
- Automated validation prevents future link rot

**Action Items:**
- [ ] Audit all internal links in v0.0.2 documentation
- [ ] Fix broken links using consistent relative paths
- [ ] Create documentation index/navigation file
- [ ] Add "See Also" sections to related documents
- [ ] Add link validation to documentation build process
- [ ] Document link format convention in v0.0.2 style guide

---

### Issue #29: Planning Document Cross-References
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 891-913
**Decided:** 2025-11-11

**Problem Summary:**
- Epic files reference architecture files with relative paths
- Paths may not work from all contexts
- Example: `[Compiler Architecture](../architecture/02-compiler.md)` works from epic file but not when rendered in different context

**Decision:**

**Standardize on absolute paths from repository root**

**Pattern:**
```markdown
[Link Text](/docs/v0.0.2/planning/architecture/02-compiler.md)
```

**Alternative approaches:**
- OR use a documentation link resolver/index
- Consider adding a docs navigation/sitemap
- Validate links in CI/CD pipeline

**Rationale:**
- Absolute paths work regardless of rendering context
- Clear, unambiguous path specification
- Easier to maintain and validate
- Works across different documentation tools

**Action Items:**
- [ ] Convert all relative cross-references to absolute paths from repo root
- [ ] Use pattern: `/docs/v0.0.2/section/file.md`
- [ ] Add documentation sitemap/navigation
- [ ] Validate links in CI/CD pipeline
- [ ] Document link format convention in v0.0.2 style guide

---

### Issue #30: Hello World Variations
**Status:** DECIDED ✓
**Severity:** LOW
**Reference:** inconsistencies-log.md lines 914-964
**Decided:** 2025-11-11

**Problem Summary:**
- "Hello World" example appears in multiple files with variations
- Different pipeline names (`Greet` vs `GreetUser`)
- Different variable names (`.name` vs `.user_name`)
- With/without package declaration
- Creates confusion about canonical syntax

**Decision:**

**Create ONE canonical Hello World example**

**Canonical Hello World Example:**

```polyglot
[@] Local@HelloWorld::1.0.0
[X]

[|] Greet
// trigger via termial

[t] |T.Cli
[<] .cmd: pg\string << "Hello"

[i] |i.Cli
[>] .args.name: pg\string >> .name

// Direct console outputs to the current terminal
[W] |W.Cli

[p] |U.Python3.10.RunScript
[<] .code: pg\string << "print('Hello {.name}')"

[p] |U.Rust1.8.Run
[<] .code: pg\string << "fn main() {"
[^] >"       println!(\"Hello, world!\");"
[^] >"    }"

[o] #None
[X]
```

**Key Features Demonstrated:**
- Package declaration: `[@] Local@HelloWorld::1.0.0`
- CLI trigger: `[t] |T.Cli`
- CLI input: `[i] |i.Cli` with output to variable
- Wrapper configuration: `[W] |W.Cli`
- Parallel execution: `[p]` blocks for Python and Rust
- Multi-line string continuations: `[^] >`
- String interpolation: `{.name}`
- Output configuration: `[o] #None`

**Rationale:**
- Demonstrates realistic CLI-triggered workflow
- Shows multiple language wrappers (Python, Rust)
- Illustrates parallel execution
- Uses modern syntax decisions (backslash separator, DT/comment conventions)
- More representative of actual Polyglot automation use cases

**Action Items:**
- [ ] Place canonical example in centralized examples directory
- [ ] Reference this example consistently across all v0.0.2 documentation
- [ ] Create additional examples for other common patterns
- [ ] Document what each line/block demonstrates
- [ ] Add to v0.0.2 quick start guide


---

## PENDING USER DECISIONS

The following items require user input before v0.0.2 documentation is complete:

### Pending #1: Reserved Enumerations Feature Details
**Status:** DECIDED ✓
**Related Decision:** Issue #1 (Type System - Maps vs Enumerations)
**Line Reference:** 64
**Decided:** 2025-11-11

**Context:**
Decision #1 mentions that Polyglot will have **reserved enumerations** that users can extend using `[#]` definition syntax. This decision clarifies how reserved enumerations work and which ones exist in Polyglot.

**Decision:**

**Reserved Enumerations - System-Defined Extensible Schemas**

**Terminology:**
- **Schema**: The complete key structure of a serial data type
- **Keys**: The leaf fields of the serial data type

**1. What Are Reserved Enumerations?**

Polyglot has three types of enumeration-like structures:

**Type 1: Regular Enumerations**
```polyglot
[#] MyApp.Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]
```
- Schema/keys are defined by user at compile-time
- Schema/keys cannot be changed at runtime
- User has full control over structure

**Type 2: Reserved Enumerations (Non-Extendable)**
```polyglot
// #Status.* is built-in by Polyglot
// Users CANNOT extend or modify
```
- Schema/keys are predefined by Polyglot (before compile-time)
- Schema/keys are NOT subject to change even at compile-time
- Users can USE but cannot EXTEND

**Type 3: Reserved Enumerations (Extendable)**
```polyglot
[#] Path.Identifiers.MyCustomPath
[<] .unix: pg\path << \\UnixRoot\\custom\path\
[<] .windows: pg\path << \\C\\Custom\Path\
[X]
```
- Schema/keys are predefined by Polyglot (before compile-time)
- Users can define additional entries WITH STRICT SCHEMA enforcement
- Users CANNOT modify the schema/keys
- Users CANNOT overwrite existing entries

**2. Built-in Reserved Enumerations**

**Extendable Reserved Enumerations:**

1. **`#Path.Identifiers.*`** (Decision #14)
   - Required schema: `.unix: pg\path`, `.windows: pg\path`
   - Users can define custom path identifiers

2. **`#Queues.*`** (Decision #22)
   - Schema: TBD (see Pending Decision Log for Enumeration Schemas)
   - Users can define custom queues

3. **`#DT.Business.Week.*`**
   - Schema: TBD (see Pending Decision Log for Enumeration Schemas)
   - Users can define custom business week configurations
   - Part of the `#DT.*` family of date/time enumerations

**Non-Extendable Reserved Enumerations:**

1. **`#Status.*`**
   - Users CANNOT extend or modify
   - Read-only, system-defined

**Deprecated/Replaced:**

1. **`#Errors.*`**
   - Early draft of error system
   - **REPLACED by `!Error` syntax** (Decision #13)
   - See Pending Decision Log for Enumeration Schemas for final status

**3. Syntax Rules**

**Extending an Extendable Reserved Enumeration:**
```polyglot
[#] Path.Identifiers.MyCustomPath
[A] MyPath
[<] .unix: pg\path << \\UnixRoot\\custom\path\
[<] .windows: pg\path << \\C\\Custom\Path\
[X]
```

**Key Rules:**
- Use `[#]` block marker (same as regular enumerations)
- Follow the predefined schema EXACTLY
- ALL required keys MUST be provided
- NO additional keys allowed
- NO changing key names
- NO changing key types

**4. Schema Enforcement**

**ALLOWED - Correct Schema:**
```polyglot
[#] Path.Identifiers.MyDataPath
[<] .unix: pg\path << \\UnixRoot\\data\
[<] .windows: pg\path << \\C\\Data\
[X]
```

**FORBIDDEN - Wrong Key Names:**
```polyglot
[#] Path.Identifiers.MyDataPath
[<] .linux: pg\path << \\UnixRoot\\data\     // ❌ Should be .unix
[<] .mac: pg\path << \\UnixRoot\\data\       // ❌ Should be .windows
[X]
```
**Compile Error:** "Cannot change reserved enumeration schema. Required keys: .unix, .windows"

**FORBIDDEN - Adding Extra Keys:**
```polyglot
[#] Path.Identifiers.MyDataPath
[<] .unix: pg\path << \\UnixRoot\\data\
[<] .windows: pg\path << \\C\\Data\
[<] .description: pg\string << "Extra field"  // ❌ Extra key not allowed
[X]
```
**Compile Error:** "Cannot add keys to reserved enumeration. Required keys: .unix, .windows"

**FORBIDDEN - Missing Required Keys:**
```polyglot
[#] Path.Identifiers.MyDataPath
[<] .unix: pg\path << \\UnixRoot\\data\
// Missing .windows
[X]
```
**Compile Error:** "Missing required key .windows for reserved enumeration #Path.Identifiers.*"

**FORBIDDEN - Extending Non-Extendable:**
```polyglot
[#] Status.MyCustomStatus
[<] .value: pg\string << "custom"
[X]
```
**Compile Error:** "Cannot extend Polyglot reserved enumeration #Status.*"

**5. User-Defined vs Reserved**

**Question:** Can users create their own reserved enumerations?

**Answer:** NO. Reservation is by Polyglot, not by the user.

**What Users CAN Do:**
- Define regular enumerations with `[#]`
- Load serialized files into `pg\serial` objects (mutable schema)

**What Users CANNOT Do:**
- Create new reserved enumerations (with `.*` suffix)
- Modify existing reserved enumeration schemas
- Extend non-extendable reserved enumerations

**6. Complete List of Built-in Reserved Enumerations**

**Status: TBD** - Schemas and complete list to be documented in separate decision.

**Known Reserved Enumerations:**
- ✓ `#Path.Identifiers.*` - Extendable (schema documented)
- ✓ `#Queues.*` - Extendable (schema TBD)
- ✓ `#Status.*` - Non-extendable
- ✓ `#DT.Business.Week.*` - Extendable (schema TBD)
- ✓ `#DT.*` family - Various date/time enumerations (schemas TBD)
- ? `#Errors.*` - Status TBD (replaced by `!Error`?)

**Rationale:**

- **System control**: Polyglot reserves critical enumerations for consistency
- **Type safety**: Strict schema enforcement prevents runtime errors
- **Extensibility**: Users can extend some reserved enumerations with predefined structure
- **Consistency**: All extensions follow same schema, ensuring compatibility
- **Compile-time validation**: Schema violations caught early
- **User empowerment**: Users can define custom entries while maintaining system integrity

**Action Items:**
- [ ] Document all built-in reserved enumerations in v0.0.2 standard library reference
- [ ] Document schema for each extendable reserved enumeration
- [ ] Document compile error messages for schema violations
- [ ] Create comprehensive examples of extending reserved enumerations
- [ ] Document that users cannot create their own reserved enumerations
- [ ] Clarify `#Errors.*` status (deprecated/replaced by `!Error`)
- [ ] Cross-reference with Decision #1 (enumerations vs maps)
- [ ] Cross-reference with Decision #9 (enumeration syntax)
- [ ] Cross-reference with Decision #13 (`!Error` types)
- [ ] Cross-reference with Decision #14 (`#Path.Identifiers.*`)
- [ ] Cross-reference with Decision #22 (`#Queues.*`)
- [ ] Create new Pending Decision for Reserved Enumeration Schemas (document all schemas and finalize `#Errors.*` status)

---

### Pending #2: Join Block Syntax - Variable Synchronization from Parallel Scopes
**Status:** DECIDED ✓
**Related Decision:** Issue #12 (Parallel Execution - Variable Scope Rules)
**Line Reference:** 975, 990
**Decided:** 2025-11-11

**Context:**
Decision #12 presented multiple potential syntaxes for the `[Y]` join block when synchronizing variables from parallel scopes back to the outer scope. The decision was deferred for separate discussion.

**Decision:**

**Canonical Join Block Syntax - Use `[>]` (Pull/Extract Semantics)**

```polyglot
[Y] |Y.Join
[>] result1
[>] result2
```

**Complete Example:**

```polyglot
// Outer scope
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

// Parallel block 1
[p] |ProcessPartA
[<] .data: pg\string << "input A"
[>] .output >> result1

// Parallel block 2
[p] |ProcessPartB
[<] .data: pg\string << "input B"
[>] .output >> result2

// Join block - synchronize results from parallel scopes
[Y] |Y.Join
[>] result1
[>] result2

// After join, result1 and result2 are synchronized and accessible
[r] |ProcessResults
[<] .r1: pg\string << result1
[<] .r2: pg\string << result2
```

**Key Points:**

1. **Use `[>]` block element** - Consistent with pull/extract semantics from Decision #13
2. **No `...` prefix** - Simple variable listing without extra syntax
3. **Semantically consistent** - `[>]` means "pull value FROM source", which matches join behavior (pulling variables from parallel scopes)
4. **Block element required** - All valid Polyglot code MUST start with a block element (fundamental syntactic rule)

**Why `[>]` instead of `[<]`:**

- `[<]` means "push value INTO" (input direction)
- `[>]` means "pull value FROM" (output/extract direction)
- Join is extracting/pulling variables FROM parallel scopes TO outer scope
- Semantically accurate: we're pulling synchronized state

**Rejected Options:**

**Option A: `[<]` without `...`**
```polyglot
[Y] |Y.Join
[<] result1
[<] result2
```
❌ Rejected: `[<]` means "push INTO" but join is pulling FROM parallel scopes

**Option B: `[<] ...` with prefix**
```polyglot
[Y] |Y.Join
[<] ... result1
[<] ... result2
```
❌ Rejected: Most verbose, introduces new `...` syntax, wrong semantic direction

**Option C: No block element prefix**
```polyglot
[Y] |Y.Join
result1
result2
```
❌ Rejected: Violates fundamental Polyglot rule - all valid code must start with block element

**Rationale:**

- **Semantic consistency**: `[>]` = pull/extract, which matches join behavior
- **Syntactic consistency**: Uses existing block element, no new syntax needed
- **Language rule compliance**: All Polyglot code starts with block element
- **Visual clarity**: `[>]` clearly shows directionality (FROM parallel scopes)
- **Consistent with Decision #13**: `[>]` established as extraction operator
- **Simple**: No extra prefix syntax like `...` needed

**Connection to Other Decisions:**
- **Decision #12**: Parallel execution and join blocks
- **Decision #13**: `[>]` operator semantics (pull/extract FROM source)
- **Decision #15**: Block element relationships and nesting

**Action Items:**
- [ ] Update Decision #12 examples to use `[>]` in join blocks
- [ ] Document `[>]` usage in `[Y]` join blocks
- [ ] Update all parallel execution examples with correct join syntax
- [ ] Document that join block lists variables to synchronize using `[>]`
- [ ] Cross-reference with `[>]` operator documentation from Decision #13
- [ ] Add examples showing multiple variables synchronized in join block

---

### Pending #3: Custom User Error Definition Syntax
**Status:** DECIDED ✓
**Related Decision:** Issue #13 (Error Handling - Output Syntax), Decision #1 (Enumeration Extension)
**Line Reference:** 1136
**Decided:** 2025-11-11

**Context:**
Decision #13 established that `!Error` types are special enumerations with three reserved fields (`.message`, `.code`, `.trace`). This decision establishes how users define brand new custom error types.

**Decision:**

**Custom Error Definition Uses Same Syntax as Enumeration Extension with `[!]` Block Element**

Custom error definitions follow the exact same pattern as extending enumerations (Decision #1), with the key difference being the use of `[!]` block element instead of `[#]`.

**Canonical Syntax:**

```polyglot
[!] !Errors.*
```

Where `*` represents the full error name.

**Complete Example - Defining Custom Error with Additional Fields:**

```polyglot
[!] !MyApp.DatabaseError
[<] .message: pg\string << "Database operation failed"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[<] .query: pg\string << ""
[<] .affected_rows: pg\int << 0
[X]
```

**Example - Minimal Custom Error:**

```polyglot
[!] !MyApp.ValidationError
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[X]
```

**Example - Custom Error with Alias (Package-Scoped):**

```polyglot
[!] !MyApp.Authentication.InvalidCredentials
[A] !InvalidCreds  // Alias usable only within this package
[<] .message: pg\string << "Invalid username or password"
[<] .code: pg\int << 4010
[<] .trace: pg\string << ""
[<] .username_attempted: pg\string << ""
[X]
```

**Key Points:**

1. **Same syntax as enumeration extension** - Use `[!]` instead of `[#]`
2. **Three reserved fields required** - `.message`, `.code`, `.trace` must be explicitly declared
3. **Custom fields allowed** - Users can add additional fields beyond the three reserved ones
4. **Alias support** - `[A]` provides package-scoped alias (follows same pattern as enumerations)
5. **Full namespace required** - Error names use namespaced format: `!Package.Context.ErrorName`

**Consistency with Decision #1:**
- Enumeration extension: `[#] #EnumName.*` with `[<]` for new keys
- Error definition: `[!] !ErrorName` with `[<]` for fields (including reserved three)
- Both use `[A]` for aliases
- Both require `[X]` to close the definition block

**Why Three Reserved Fields Must Be Explicit:**
Unlike enumeration extension where you only add new keys, error definitions are creating a new type structure. Making `.message`, `.code`, `.trace` explicit:
- Provides clarity about default values
- Maintains consistency with the extension syntax pattern
- Allows customization of default values per error type
- Documents the complete error structure in one place

---

### Pending #4: Testing Methodology (Can be deferred)
**Status:** DEFERRED TO LATER DEVELOPMENT STAGE
**Related Decision:** Issue #24 (CLI Command Reference)
**Line Reference:** 3042, 3100

**Context:**
The `polyglot test` command was documented in Decision #24, but the testing methodology details are explicitly marked as "deferred to later development stage."

**Current Documentation:**
```bash
polyglot test {package}@{pipeline}
```

**What's Unclear:**
1. How do users write tests for pipelines?
2. What's the test file format/structure?
3. How are test inputs specified?
4. How are expected outputs validated?
5. Are there built-in test assertions or utilities?

**Note:**
This was already marked as deferred, so it may not need immediate decision unless you want to address it now.

---

### Pending #5: Reserved Enumeration Schemas Documentation

**Status:** PENDING USER INPUT

**Context:**
From Decision #1 (Pending), we established that Polyglot provides several built-in reserved enumerations that are extendable. However, the exact schema/keys for some of these enumerations have not been fully documented.

**Built-in Reserved Enumerations That Need Schema Documentation:**

1. **`#Path.Identifiers.*`** - CONFIRMED SCHEMA:
   - `.unix: pg\path` - Path for Unix-based systems
   - `.windows: pg\path` - Path for Windows systems

2. **`#Queues.*`** - SCHEMA TBD:
   - From Decision #22, we see usage of `.max_concurrent` field
   - Need to confirm if this is the complete required schema
   - Are there other required or optional fields?

3. **`#DT.Business.Week.*`** - SCHEMA TBD:
   - No current examples in documentation
   - What fields are required for business week definitions?
   - Example use cases needed

4. **`#Errors.*`** - DEPRECATED STATUS:
   - User confirmed: "Yes and include that also the new decision log for enumerations"
   - `#Errors.*` has been completely replaced by `!Error` syntax
   - Should be documented as deprecated/removed in v0.0.2

**Questions:**

1. **For `#Queues.*`:** What is the complete required schema? Is it just `.max_concurrent: pg\int`, or are there other fields like `.priority`, `.timeout`, etc.?

2. **For `#DT.Business.Week.*`:** What fields are required? Examples might include:
   - `.start_day: pg\string` (e.g., "Monday")
   - `.work_days: pg\int` (e.g., 5)
   - Other fields?

3. **For `#Errors.*`:** Should we:
   - Explicitly document it as REMOVED in v0.0.2?
   - Add migration guidance from `#Errors.*` to `!Error`?
   - Include this in the error handling decision?

4. **Are there any other built-in reserved enumerations** not yet listed that need documentation?

**Example Documentation Needed:**

```polyglot
// #Queues.* - TBD
[#] Queues.HighPriority
[<] .max_concurrent: pg\int << 10
// Are there other required fields?
[X]

// #DT.Business.Week.* - TBD
[#] DT.Business.Week.Standard
// What fields are required here?
[X]
```

**Decision Needed:**
Document the complete schemas for all extendable reserved enumerations so users know exactly what fields are required when extending them.

---

## Next Steps

### Phase 1: Critical Decisions (REQUIRED)
☐ Make decisions for all 5 critical issues (#1-5)
☐ Review decisions for consistency
☐ Document rationale clearly

### Phase 2: High Priority Decisions
☐ Make decisions for all 11 high-priority issues (#6-16)
☐ Ensure alignment with critical decisions

### Phase 3: Medium & Low Priority
☐ Address medium priority issues (#17-27)
☐ Address low priority issues (#28-30)

### Phase 4: Generate v0.0.2 Documentation
☐ Use this decision log to create consistent v0.0.2 docs
☐ Follow BMM workflow (product-brief → prd → architecture)
☐ Validate all decisions are implemented

---

## Notes

**Document Convention:**
- Use checkboxes `[ ]` for pending items, `[x]` for completed
- Update progress tracker at top as decisions are made
- Add date stamps when decisions are made
- Link to relevant v0.0.2 documentation when created

**Decision Quality:**
- Every decision should have clear rationale
- Consider implementation complexity
- Think about user experience
- Ensure consistency across decisions
