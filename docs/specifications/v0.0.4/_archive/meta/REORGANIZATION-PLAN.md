<!-- ARCHIVED: 2025-12-16 | Reason: Planning document - reorganization complete | Superseded by: N/A (organizational doc) -->

# v0.0.4 Documentation Reorganization Plan

**Created:** 2025-12-14
**Updated:** 2025-12-14 (Rev 2 - Added standard library, categorized features)
**Status:** Awaiting approval
**Estimated Files:** 60+ new documentation files
**Estimated Total Size:** ~300KB

---

## Current Structure (Before)

```
v0.0.4/
├── COMPLETE-SPEC.md (10KB) - monolithic spec
├── README.md (9KB) - outdated overview
├── features/
│   ├── loop-system.md (13KB)
│   ├── reserved-indication-system.md (12KB)
│   └── metadata-system.md (13KB)
├── core-syntax/ (empty)
├── loop-system/ (7 design docs) → to be moved
└── syntax-refinement/ (8+ docs) → to be moved
```

**Issues:**
- ❌ COMPLETE-SPEC.md too large (10KB), hard to navigate
- ❌ Missing critical documentation (serial-load-block, variable lifecycle, markers with corrections)
- ❌ No quick reference materials
- ❌ No examples or migration guides
- ❌ Design history mixed with current docs

---

## Proposed Structure (After)

```
v0.0.4/
├── README.md (5KB) ⭐ NEW - Navigation hub
│
├── quick-reference/
│   ├── README.md (2KB)
│   ├── syntax-cheatsheet.md (4KB) ⭐ NEW
│   ├── markers-quick-ref.md (3KB) ⭐ NEW
│   ├── operators-quick-ref.md (3KB) ⭐ NEW
│   └── migration-quick-ref.md (2KB) ⭐ NEW
│
├── core-syntax/
│   ├── README.md (2KB) ⭐ NEW
│   ├── prefix-system.md (4KB) ⭐ NEW - All prefixes: $, :, #, |, !, @, %
│   ├── markers.md (6KB) ⭐ NEW - All markers WITH CORRECTIONS
│   ├── operators.md (5KB) ⭐ NEW - Assignment, I/O, conditionals
│   ├── blocks.md (3KB) ⭐ NEW - {|}, {#}, {!}, {@}, {x}
│   ├── variables-lifecycle.md (5KB) ⭐ NEW - 5 states, transitions
│   ├── types.md (4KB) ⭐ NEW - Type notation :pg.type
│   ├── enums-structs.md (5KB) ⭐ NEW - Enum/struct definitions
│   ├── pipeline-structure.md (6KB) ⭐ NEW - [t], [Q], [W], execution order
│   └── io-system.md (5KB) ⭐ NEW - Definition vs call, implicit triggers
│
├── features/
│   ├── README.md (3KB) ⭐ NEW - CATEGORIZED
│   │
│   ├── core-features/ ⭐ NEW CATEGORY
│   │   ├── loop-system.md (13KB) ✅ EXISTS - MOVED
│   │   ├── reserved-indication-system.md (12KB) ✅ EXISTS - MOVED
│   │   ├── metadata-system.md (13KB) ✅ EXISTS - MOVED
│   │   └── serial-load-block.md (8KB) ⭐ NEW - CRITICAL!
│   │
│   ├── data-structures/ ⭐ NEW CATEGORY
│   │   ├── enums-with-fields.md (4KB) ⭐ NEW
│   │   ├── struct-shorthand.md (3KB) ⭐ NEW
│   │   └── collection-literals.md (3KB) ⭐ NEW
│   │
│   ├── control-flow/ ⭐ NEW CATEGORY
│   │   ├── match-expressions.md (4KB) ⭐ NEW
│   │   ├── early-return.md (3KB) ⭐ NEW
│   │   └── boolean-markers.md (4KB) ⭐ NEW
│   │
│   ├── string-handling/ ⭐ NEW CATEGORY
│   │   ├── multi-line-strings.md (3KB) ⭐ NEW
│   │   └── inline-pipelines.md (3KB) ⭐ NEW
│   │
│   ├── operators/ ⭐ NEW CATEGORY
│   │   ├── range-operators.md (4KB) ⭐ NEW
│   │   ├── operator-negation.md (3KB) ⭐ NEW
│   │   └── collection-membership.md (3KB) ⭐ NEW
│   │
│   ├── pipeline-features/ ⭐ NEW CATEGORY
│   │   ├── pipeline-composition.md (3KB) ⭐ NEW
│   │   ├── variadic-input.md (4KB) ⭐ NEW
│   │   └── trigger-or.md (3KB) ⭐ NEW
│   │
│   └── error-handling/ ⭐ NEW CATEGORY
│       └── error-handling.md (5KB) ⭐ NEW
│
├── standard-library/ ⭐ NEW SECTION - CRITICAL!
│   ├── README.md (8KB) ⭐ NEW - Tree diagram index
│   │
│   ├── utilities/ (|U.*)
│   │   ├── README.md (4KB) ⭐ NEW - |U.* tree
│   │   ├── math/
│   │   │   ├── README.md (2KB)
│   │   │   ├── double.md
│   │   │   ├── add.md
│   │   │   ├── subtract.md
│   │   │   └── ...
│   │   ├── string/
│   │   │   ├── README.md
│   │   │   ├── concat.md
│   │   │   ├── split.md
│   │   │   └── ...
│   │   ├── datetime/
│   │   │   ├── README.md
│   │   │   ├── now.md
│   │   │   ├── parse.md
│   │   │   └── ...
│   │   └── data/
│   │       ├── README.md
│   │       ├── yaml-load.md
│   │       ├── yaml-parse.md
│   │       ├── json-load.md
│   │       └── ...
│   │
│   ├── wrappers/ (|W.*)
│   │   ├── README.md (4KB) ⭐ NEW - |W.* tree
│   │   ├── polyglot/
│   │   │   ├── README.md
│   │   │   └── scope.md
│   │   ├── runtime/
│   │   │   ├── README.md
│   │   │   ├── python.md
│   │   │   ├── rust.md
│   │   │   ├── javascript.md
│   │   │   └── ...
│   │   └── ...
│   │
│   ├── unpack-operators/ (~*)
│   │   ├── README.md (3KB) ⭐ NEW - ~* tree
│   │   ├── foreach.md
│   │   ├── foreach-array.md
│   │   ├── foreach-range.md
│   │   ├── enumerate.md
│   │   ├── zip.md
│   │   ├── window.md
│   │   ├── chunk.md
│   │   ├── chained.md
│   │   ├── reduce.md
│   │   └── ...
│   │
│   └── pack-operators/ (**)
│       ├── README.md (3KB) ⭐ NEW - ** tree
│       ├── into-array.md
│       ├── into-set.md
│       ├── collect-last.md
│       ├── join-all.md
│       ├── join-first.md
│       ├── to-next-iteration.md
│       ├── partition-status.md
│       └── ...
│
├── examples/
│   ├── README.md (2KB) ⭐ NEW
│   ├── hello-world.md (2KB) ⭐ NEW
│   ├── basic-pipeline.md (3KB) ⭐ NEW
│   ├── loops.md (5KB) ⭐ NEW
│   ├── config-loading.md (4KB) ⭐ NEW - Using [s]
│   ├── error-handling.md (4KB) ⭐ NEW
│   └── complete-application.md (6KB) ⭐ NEW
│
├── changes-from-v0.0.3/ ⭐ RENAMED (v0.0.3 was draft)
│   ├── README.md (1KB) ⭐ NEW
│   └── syntax-changes.md (8KB) ⭐ NEW - Breaking changes + new features
│
└── design-history/
    ├── README.md (1KB) ⭐ NEW
    ├── loop-system/ ✅ MOVED from root
    └── syntax-refinement/ ✅ MOVED from root
```

**Legend:**
- ⭐ NEW - To be created
- ✅ EXISTS - Already exists, keep
- ✅ MOVED - Moved from another location

---

## Detailed File Outlines

### 1. Quick Reference Files

#### quick-reference/syntax-cheatsheet.md
**Size:** ~4KB
**Content:**
- One-page syntax reference
- All prefixes, markers, operators
- Common patterns
- Code examples for each construct

#### quick-reference/markers-quick-ref.md
**Size:** ~3KB
**Content:**
- All markers with corrected descriptions
- [v] = Join (NOT vacuum!)
- [y] = Fork
- [Q] = Queue control
- [s] = Serial load
- Grouped by category

#### quick-reference/operators-quick-ref.md
**Size:** ~3KB
**Content:**
- All operators categorized
- Assignment: <<, >>, <~, ~>
- Conditionals: =?, !=?, >?, etc.
- Range: ?[,], ?(,], ?[,), ?(,)
- Collection: in?, !in?
- Composition: |>

#### quick-reference/migration-quick-ref.md
**Size:** ~2KB
**Content:**
- Side-by-side v0.0.3 vs v0.0.4
- Quick regex patterns for migration
- Breaking changes checklist

---

### 2. Core Syntax Files

#### language/syntax/prefix-system.md
**Size:** ~4KB
**Content:**
- `$` - Variable prefix (with comma ambiguity explanation)
- `:` - Type prefix
- `#` - Enum/struct prefix
- `|` - Pipeline prefix
- `!` - Error prefix
- `@` - Registry prefix
- `%` - Metadata prefix
- Examples for each
- Hierarchy rules (dot separator)

#### language/syntax/markers.md ⭐ CRITICAL - WITH CORRECTIONS
**Size:** ~6KB
**Content:**
- **Execution Markers:**
  - `[r]` - Sequential execution (run)
  - `[p]` - Parallel execution
  - `[b]` - Background execution
  - `[y]` - Forked branch (y = visual fork) ✅ CORRECTED
  - `[v]` - Join (V = visual join) ✅ CORRECTED NOT VACUUM!

- **Pipeline Structure:**
  - `[t]` - Trigger (REQUIRED)
  - `[Q]` - Queue control (optional) ✅ ADDED
  - `[W]` - Wrapper (REQUIRED) ✅ Contains setup + cleanup
  - `[|]` - Pipeline I/O marker

- **Loop Markers:**
  - `[~]` - Unpack/expand marker
  - `[*]` - Pack/collect marker

- **Data Markers:**
  - `[s]` - Serial load block ✅ CRITICAL NEW FEATURE
  - `[.]` - Field marker
  - `[+]` - Multi-line continuation

- **Error Markers:**
  - `[z]` - Try block
  - `[!]` - Error catch

- **Other Markers:**
  - `[m]` - Match expression
  - `[?]` - Match case
  - `[&]` - AND condition
  - `[^]` - XOR condition

#### language/types/variables-lifecycle.md ⭐ CRITICAL
**Size:** ~5KB
**Content:**
- **5 States:** Pending, Default, Final, Faulted, Released ✅ CORRECTED
- **State Transitions:**
  ```
  Enter scope → Declaration → Pending
  Pending → Push (<<, >>) → Final
  Pending → Default Push (<~, ~>) → Default (1 more push allowed)
  Default → Push (<<, >>) → Final
  Pending → Error → Faulted
  Final → Exit scope → Released
  Faulted → Exit scope → Released
  ```
- **Transition Mechanism:** PULL from Final → PUSH to Pending
- **Examples for each transition**
- **Loop iteration scopes**

#### language/control-flow/pipeline-structure.md ⭐ CRITICAL
**Size:** ~6KB
**Content:**
- **Execution Order:**
  1. Inputs
  2. Trigger `[t]` (REQUIRED)
  3. Queue `[Q]` (optional, has default)
  4. Wrapper `[W]` (REQUIRED - setup + cleanup)
  5. Logic
  6. Outputs

- **Input Parameters as Implicit Triggers:**
  - Pipeline executes ONLY when all inputs are Final or Default
  - If input is Pending (no value, no default), pipeline never executes
  - Pipeline body ALWAYS sees inputs as Final or Default, never Pending

- **Required vs Optional Markers**
- **Default Implementations**
- **Examples**

#### core-syntax/io-system.md
**Size:** ~5KB
**Content:**
- **Pipeline Definition (Signature):**
  ```polyglot
  [|] <input_param :type
  [|] >output_param :type
  ```

- **Pipeline Call (Usage):**
  ```polyglot
  [|] <input_param :type << $value
  [|] >output_param :type >> $result
  ```

- **Distinction:** No assignment operators in definition, `<<`/`>>` in call
- **Default values:** `<~` operator
- **Examples**

---

### 3. Features Files

#### features/serial-load-block.md ⭐ CRITICAL NEW FEATURE
**Size:** ~8KB
**Content:**
- **Two Contexts:**

  **A. Struct/Enum Definition (field mapping):**
  ```polyglot
  {#} #Config
  [s] << |YAML.Load"file.yaml"
     [.] .field :type << .yaml.path
     [.] .field_with_default :type
        [.] <~ "default"
        [.] << .yaml.path
  [s][!] *!
  {x}
  ```

  **B. Pipeline Execution (entire content):**
  ```polyglot
  [s] $variable << |YAML.Load"file.yaml"
     [.] << *  // Load entire content
  [s][!] *!
  ```

- **Implicit Behavior:**
  - All `[s]` blocks at same level run IN PARALLEL
  - Auto collection of results
  - Single error handler for all: `[s][!] *!`

- **Multiple File Loading Example**
- **Error Handling Semantics**
- **vs Multi-step Explicit Approach**

#### features/error-handling.md
**Size:** ~5KB
**Content:**
- Try blocks: `[z]`
- Error catch: `[!] !ErrorType >> $error`
- Error propagation
- Faulted state
- Error handling in loops
- Serial load block errors

---

### 4. Examples Files

#### examples/config-loading.md
**Size:** ~4KB
**Content:**
- Loading single YAML file
- Loading multiple configs in parallel with `[s]`
- Field mapping with defaults
- Error handling
- Complete working example

#### examples/complete-application.md
**Size:** ~6KB
**Content:**
- Full application using:
  - Registry definition
  - Serial load blocks for config
  - Pipelines with triggers and queue
  - Loop system for processing
  - Error handling
  - Metadata annotations
- Shows all major v0.0.4 features in one example

---

### 5. Standard Library Files ⭐ NEW CRITICAL SECTION

#### stdlib/index.md - Tree Diagram Index
**Size:** ~8KB
**Content:**
- **Complete Package Tree:**
  ```
  Polyglot Standard Library v0.0.4
  │
  ├── |U.* (Utilities)
  │   ├── |U.Math.*
  │   │   ├── |U.Math.Double
  │   │   ├── |U.Math.Add
  │   │   └── ...
  │   ├── |U.String.*
  │   ├── |U.DateTime.* (|DT.*)
  │   └── |U.Data.* (YAML, JSON)
  │
  ├── |W.* (Wrappers)
  │   ├── |W.Polyglot.Scope
  │   ├── |W.RT.Python*
  │   ├── |W.RT.Rust*
  │   └── ...
  │
  ├── ~* (Unpack Operators)
  │   ├── ~ForEach.Array
  │   ├── ~ForEach.Range
  │   ├── ~Enumerate
  │   └── ...
  │
  └── ** (Pack Operators)
      ├── *Into.Array
      ├── *Join.All
      └── ...
  ```

- **Quick Navigation by Category**
- **Usage Statistics** (most common pipelines)
- **Links to All Subsections**

#### Individual Pipeline/Operator Documentation Template
**Each file follows this structure:**

```markdown
# |U.Math.Double

**Category:** Utilities > Math
**Purpose:** Doubles a numeric value
**Since:** v0.0.1

## Signature

{|} |U.Math.Double
[|] <value :pg.float
[|] >result :pg.float
{x}

## Parameters

**Inputs:**
- `<value` :pg.float - The number to double

**Outputs:**
- `>result` :pg.float - The doubled value

## Description

Multiplies the input value by 2. Handles both integer and float types.

## Examples

### Basic Usage
[r] $doubled :pg.float << |U.Math.Double"{5.0}"
// $doubled = 10.0

### In Pipeline
{|} |ProcessNumbers
[|] <numbers :pg.array.pg.float

[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $doubled :pg.float << |U.Math.Double"{$num}"
   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $results
{x}

## See Also
- [Math Package](../README.md) - All math utilities
- [|U.Math.Add](add.md) - Addition
- [Inline Pipelines](../../../features/string-handling/inline-pipelines.md)
```

#### stdlib/utilities/README.md
**Size:** ~4KB
**Content:**
- Tree of all |U.* packages
- Math, String, DateTime, Data categories
- Quick links to each utility
- Common patterns

#### stdlib/loops/unpack/README.md
**Size:** ~3KB
**Content:**
- Tree of all ~* operators
- Grouped by use case:
  - Iteration: ~ForEach.*, ~Enumerate
  - Transformation: ~Map, ~Filter
  - Aggregation: ~Reduce
  - Chaining: ~Chained
  - Windowing: ~Window, ~Chunk
- Quick comparison table
- Links to loop-system.md

#### stdlib/loops/pack/README.md
**Size:** ~3KB
**Content:**
- Tree of all ** operators
- Grouped by use case:
  - Collection: *Into.Array, *Into.Set
  - Joining: *Join.All, *Join.First
  - Chaining: *To.Next.Iteration
  - Conditional: *Collect.Last, *Break, *Continue
- Quick comparison table
- Links to loop-system.md

---

### 6. Changes from v0.0.3 Files (v0.0.3 was draft)

#### changes-from-v0.0.3/syntax-changes.md
**Size:** ~8KB
**Content:**
- **Breaking Changes (Automated):**
  - Variable prefix: `,` → `$`
  - Indentation: `\~\` → 3 spaces
  - Types: `pg\type` → `:pg.type`
  - Reserved: Add `;` where needed

- **Breaking Changes (Manual Review):**
  - IO markers: `[i]`/`[o]` → `[|] <param`/`[|] >param`
  - Loop patterns → Unpack/pack system

- **New Features to Adopt:**
  - Serial load blocks `[s]`
  - Queue control `[Q]`
  - Metadata system `%`
  - And 20+ more features

- **Side-by-side examples**
- **Migration checklist**
- **Common pitfalls**

---

## Implementation Priority

### Phase 1: Critical Files (Priority 1)
1. ✅ README.md - Main navigation hub with all 6 sections
2. ✅ language/syntax/markers.md - WITH ALL CORRECTIONS
3. ✅ language/types/variables-lifecycle.md - 5 states + transitions
4. ✅ language/control-flow/pipeline-structure.md - Execution order, implicit triggers
5. ✅ language/advanced/serial-load-block.md - NEW CRITICAL FEATURE

### Phase 2: Core Documentation (Priority 2)
6. language/syntax/prefix-system.md
7. language/syntax/operators.md
8. core-syntax/io-system.md
9. core-syntax/enums-structs.md
10. quick-reference/syntax-cheatsheet.md
11. features/README.md - CATEGORIZED index

### Phase 3: Standard Library Foundation (Priority 3)
12. stdlib/index.md - Tree diagram index
13. stdlib/utilities/README.md - |U.* tree
14. stdlib/wrappers/README.md - |W.* tree
15. stdlib/loops/unpack/README.md - ~* tree
16. stdlib/loops/pack/README.md - ** tree
17. Sample pipeline docs (5-10 most common)

### Phase 4: Features & Examples (Priority 4)
18. All categorized feature files (18 files)
19. examples/config-loading.md
20. examples/complete-application.md
21. changes-from-v0.0.3/syntax-changes.md

### Phase 5: Standard Library Details (Priority 5)
22. Individual pipeline documentation (30-50 files)
23. Individual operator documentation (20-30 files)
24. Usage examples for each

### Phase 6: Completion (Priority 6)
25. All remaining quick-reference files
26. All remaining examples
27. All README.md files for subdirectories
28. Design history organization

---

## Cross-Reference Matrix

Every file will include "See Also" sections with links to related files:

**Example for language/syntax/markers.md:**
```markdown
## See Also
- [Prefix System](prefix-system.md) - Operators that markers work with
- [Pipeline Structure](pipeline-structure.md) - Pipeline-specific markers
- [Loop System](../language/advanced/loop-system.md) - Loop markers [~], [*], [v]
- [Serial Load Block](../features/serial-load-block.md) - [s] marker
- [Quick Reference](../quick-reference/markers-quick-ref.md) - Quick lookup
```

---

## Navigation Strategy

### Top-Level Navigation
- Main README.md → Points to all 5 major sections
- Each section has its own README.md → Points to files within

### Bottom-Up Navigation
- Every file has breadcrumbs at top
- Every file has "See Also" at bottom
- Quick reference files link to detailed docs
- Detailed docs link to quick reference

### Search-Friendly
- Clear file names (no abbreviations)
- Descriptive headers
- Keyword-rich content
- Examples in every file

---

## Estimated Effort

**Total Files to Create:** ~30 new files
**Total Size:** ~150KB
**Estimated Time:**
- Phase 1 (Critical): 2 hours
- Phase 2 (Core): 2 hours
- Phase 3 (Features): 2 hours
- Phase 4 (Completion): 2 hours
- **Total:** 8 hours of documentation writing

---

## Review Questions

1. **Structure:** Does the folder organization make sense?
2. **Priority:** Should any files be created before others?
3. **Scope:** Are there files missing or files that shouldn't be created?
4. **Naming:** Are file names clear and consistent?
5. **Size:** Are any files too large and need further subdivision?

---

## Approval Checklist

Before proceeding, confirm:

- [ ] Folder structure approved
- [ ] File list approved
- [ ] Priority order approved
- [ ] Content outlines approved
- [ ] Ready to proceed with Phase 1

---

**Status:** ⏸️ Awaiting Review and Approval

Once approved, I will proceed with Phase 1 (Critical Files) and work through the phases systematically.
