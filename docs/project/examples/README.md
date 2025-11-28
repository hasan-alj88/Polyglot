# Polyglot v0.0.2 Examples

**Generated:** 2025-11-26
**AI Context Package:** `/docs/ai-context/`
**Compliance:** v0.0.2 Syntax Standards

---

## Overview

This directory contains **compliant Polyglot v0.0.2 code examples** generated from the updated AI context package. All examples follow the latest syntax rules, including:

- ✅ **MANDATORY PIPELINE SECTIONS** - ALL pipelines have `[i]`, `[t]`, `[\][/]` or `[W]`, `[o]`
- ✅ Block marker hierarchy (`[r]` for declarations, `[<]`/`[>]` within parent blocks)
- ✅ Inline pipeline string literals (`DT.Now""` not `DT.Now`)
- ✅ Collection syntax (`{}` not `[]`)
- ✅ Variable state semantics (Declared, DefaultReady, Pending, Ready, Faulted)
- ✅ Input variations (`[i]` with `<<`, `<~`, or nothing - **no `[=]` marker**)
- ✅ Exhaustive conditionals (`[?] *?` catch-all required)
- ✅ PUSH/PULL terminology (not assign/mutable)

---

## Examples

### 1. Basic Pipeline (`01-basic-pipeline.pg`)

**Concepts:**
- Core syntax (package, pipeline, inputs, outputs)
- Input variations: Required, Default, Constant
- Block markers: `[@]`, `[|]`, `[i]`, `[r]`, `[o]`, `[X]`
- String interpolation: `"{.variable}"`
- No keywords rule (all identifiers use operator prefix)

**Key Pattern:**
```polyglot
[|] InputTypesDemo
[i] .user_input: pg\string               // Required (Declared)
[i] .optional_param: pg\int <~ 42        // Default (DefaultReady)
[i] .constant_value: pg\string << "v1.0" // Constant (Ready)
```

**Demonstrates:**
- Three input state variations
- Variable declarations with `[r]`
- Inline pipelines: `DT.Now""`
- Exhaustive conditionals

---

### 2. Variable States (`02-variable-states.pg`)

**Concepts:**
- Variable state transitions: Declared → DefaultReady → Pending → Ready/Faulted
- Push count semantics (enforcement of immutability)
- Auto-await when pulling from Pending variables
- Reserved namespace: `.*.pgvar.*` (state, errors, history)
- State introspection and checking

**Key Pattern:**
```polyglot
// Declared State (ready for pushes)
[r] .user_name: pg\string                // PUSH_COUNT: 0, ALLOWED: 1
[r] .user_name << "Alice"                // Now Ready (push count: final)

// DefaultReady State (allows override)
[r] .timeout: pg\int <~ 30               // PUSH_COUNT: 1, ALLOWED: 2
[<] .timeout << 60                       // Override default

// Pending → Ready/Faulted
[r] |FetchUserData
[>] .user_data: pg\serial >> .data       // .data becomes Pending

[?] .data.pgvar.state =? #PgVar.States.Ready
[~][o] .data: pg\serial
[~]
```

**Demonstrates:**
- All five variable states
- Push count enforcement
- Reserved namespace usage
- Error access via `.pgvar.errors`
- Timestamp tracking via `.pgvar.history`

---

### 3. Conditional Logic (`03-conditional-logic.pg`)

**Concepts:**
- Exhaustive conditions (required `[?] *?` catch-all)
- Boolean logic operators: `[&]` AND, `[+]` OR
- Comparison operators: `=?`, `=!?`, `>?`, `<?`, `=>?`, `=<?`
- Range operators: `?[a,b]`, `?(a,b)`, `?(a,b]`, `?[a,b)`
- Pattern matching: `re?` for regex
- Nested conditionals

**Key Pattern:**
```polyglot
[?] .score >? 90
[~][r] .grade: pg\string << "A"
[~]

[?] .score >? 80
[~][r] .grade: pg\string << "B"
[~]

[?] *?                                   // EXHAUSTIVE: REQUIRED!
[~][r] .grade: pg\string << "F"
[~]
```

**Demonstrates:**
- Exhaustive conditions prevent indefinite waiting
- Boolean logic with multiple conditions
- Range checking (closed, open, half-open intervals)
- Pattern matching with regex
- No short-circuit evaluation (async environment)

---

### 4. Unpack Operators (`04-unpack-operators.pg`)

**Concepts:**
- Unpack operators: `~ForEach`, `~Enumerate`, `~Zip`
- Execution modes: `[p]` Parallel, `[r]` Sequential, `[b]` Background
- Join operators: `~Y.IntoArray`, `~Y.IntoSerial`, `~Y.IntoSet`
- Type inference from collection element types
- Mini-scopes with `[~]` for each iteration
- Nested unpack operators

**Key Pattern:**
```polyglot
[r] .numbers: pg\array{pg\int} << {1, 2, 3}

[p] ~ForEach                             // PARALLEL execution
[<] .numbers                             // Input: variable to unpack
[>] .number                              // Type inferred as pg\int

[~][r] |DoubleValue                      // Mini-scope: process each item
[~][<] .value: pg\int << .number
[~][>] .result: pg\int >> .doubled

[~][Y] ~Y.IntoArray                      // JOIN: Convergence point
[~][<] .doubled
[~][>] .all_doubled                      // Type: pg\array{pg\int}
[~]
```

**Demonstrates:**
- Parallel, sequential, and background execution
- All three join operators (Array, Serial, Set)
- Index + item unpacking with `~Enumerate`
- Combining collections with `~Zip`
- Conditional pushes to Declared arrays
- Nested matrix processing

---

### 5. Comprehensive Example (`05-comprehensive-example.pg`)

**Concepts:**
- Multiple pipelines working together
- Custom enumerations and serial structures
- Runtime wrappers (`RT.Python`) for external code
- Triggers (`TG.Email`) for notifications
- Error handling with state checking
- Reserved namespace for monitoring
- Real-world data processing scenario

**Key Pattern:**
```polyglot
// Main pipeline
[|] ProcessDataFile
[i] .file_path: pg\path
[i] .batch_size: pg\int <~ 100

// Call runtime wrapper
[r] RT.Python"read_json_file"
[<] .wrapper_input.file_path << .file_path
[>] .file_content: pg\serial >> .raw_data

// Check state
[?] .raw_data.pgvar.state =? #PgVar.States.Faulted
[~][r] .errors: pg\array{!} << .raw_data.pgvar.errors
[~][o] .errors: pg\array{!}
[~]

// Process batch
[r] |ProcessRecordBatch
[<] .records << .records
[>] .valid_records >> .valid

// Background notification
[b] TG.Email""
[<] .trigger_input.to << "admin@example.com"
```

**Demonstrates:**
- Multi-pipeline architecture (5 pipelines total)
- Helper pipelines for validation and transformation
- Batch processing with unpack operators
- State-based error handling
- Runtime wrapper integration
- Background triggers
- Reserved namespace monitoring
- All concepts from examples 1-4 combined

---

## Suggested Reading Order

1. **Start Here:** `01-basic-pipeline.pg` - Core syntax fundamentals
2. **States:** `02-variable-states.pg` - Understanding async behavior
3. **Control Flow:** `03-conditional-logic.pg` - Decision making
4. **Collections:** `04-unpack-operators.pg` - Processing arrays/sets
5. **Real World:** `05-comprehensive-example.pg` - Everything together

---

## Mandatory Pipeline Sections (CRITICAL!)

**Every `[|]` pipeline MUST have ALL of these sections:**

1. **Inputs `[i]`** - At least one (use `[i] #Pipeline.NoInput` if none)
2. **Triggers `[t]`** - At least one (pipeline will NEVER run without triggers!)
3. **Wrapper OR Setup/Cleanup** - `[W]` wrapper (includes `[\][/]` in macro) OR explicit `[\][/]`
4. **Outputs `[o]`** - At least one (use `[o] !NoError` if no output)

### Minimal Valid Pipeline

```polyglot
[|] MinimalPipeline
[i] #Pipeline.NoInput            // MANDATORY: No inputs
[t] |T.Call                      // MANDATORY: Trigger (will NOT run without this)
[W] |W.Polyglot.Scope            // MANDATORY: No setup/cleanup (RAII placeholder)
[o] !NoError                     // MANDATORY: No output
[X]
```

### Pipeline with Runtime Wrapper

```polyglot
[|] WrapperPipeline
[i] .data: pg\string             // MANDATORY: Input
[t] |T.Call                      // MANDATORY: Trigger
[W] RT.Python"process.py"        // MANDATORY: Runtime wrapper (includes [\][/])
[o] .result: pg\string           // MANDATORY: Output
[X]
```

### Wrapper Options

**1. Scope Placeholder** (use when no explicit setup/cleanup needed):
```polyglot
[W] |W.Polyglot.Scope            // Preferred syntax
[W] W.Polyglot.Scope""           // Alternative syntax
```
- RAII-style scope cleanup
- Makes "no setup/cleanup" explicit (not accidental omission)
- Replaces old `|W.NoSetup.NoCleanup`

**2. Runtime Wrappers** (for external code integration):
```polyglot
[W] RT.Python"script.py"
[W] RT.Rust"module::function"
[W] RT.Node"module.js"
[W] RT.Go"package.Function"
```

**3. Explicit Setup/Cleanup** (for custom logic):
```polyglot
[\]
[r] .setup_var: pg\int << 0
[/]
```

### Common Triggers

- `[t] |T.Call` - Manual call (for pipelines called via `|PipelineName`)
- `[t] TG.Cron""` - Time-based trigger
- `[t] TG.FileWatch""` - File system events
- `[t] TG.HTTP""` - HTTP endpoints

**Why Mandatory:**
- Without `[t]` triggers, pipeline will **never execute**
- Without `[W]`/`[\][/]`, no execution environment
- Without `[i]`, ambiguous input requirements
- Without `[o]`, ambiguous output contract

### Pipeline Execution Workflow

**IMPORTANT:** Declaration order in file ≠ Runtime execution order

**Declaration Order** (how you write it in the file):
```
[|] PipelineName
  [i] inputs
  [t] triggers
  [Q] queue config (optional)
  [W] wrapper OR [\] setup
  execution blocks: [r], {[p],[Y]}, [b], [s]
  [/] cleanup (if using explicit [\])
  [o] outputs
[X]
```

**Runtime Execution Order** (what happens when pipeline runs):
```
[t],[i] → [\],[Q] → [r],{[p],[Y]},[b],[s] → [/] → [o] → [X]
```

**Execution Phases:**

1. **Trigger & Inputs** `[t],[i]`
   - Triggers fire (time-based, file watch, HTTP, manual call)
   - Inputs provided by external caller

2. **Setup & Queue** `[\],[Q]`
   - Setup block executes (RAII-style or explicit)
   - Queue configuration applied (if present)

3. **Execution Blocks** `[r],{[p],[Y]},[b],[s]`
   - Sequential blocks `[r]` run in order
   - Parallel blocks `{[p],[Y]}` fork and join
   - Background blocks `[b]` fire and forget
   - Streaming blocks `[s]` process data streams

4. **Cleanup** `[/]`
   - Cleanup block executes (explicit or RAII from wrapper)
   - Variables cleaned up at scope end

5. **Outputs** `[o]`
   - Outputs returned to caller

6. **Complete** `[X]`
   - Pipeline execution complete

---

## Key Syntax Rules (Quick Reference)

### Block Markers

| Marker | Usage | Context |
|--------|-------|---------|
| `[r]` | Variable declaration | Pipeline scope |
| `[r]` | Sequential execution | Execution flow |
| `[<]` | Input binding | Within parent block ONLY |
| `[>]` | Output binding | Within parent block ONLY |
| `[i]` | Input declaration | Pipeline scope |
| `[o]` | Output declaration | Pipeline scope |

### Input Variations (NO `[=]` MARKER)

```polyglot
[i] .var: Type           // Required (Declared state)
[i] .var: Type <~ value  // Default (DefaultReady state)
[i] .var: Type << value  // Constant (Ready state)
```

### Inline Pipelines

**ALL inline pipeline calls are string literals:**
```polyglot
DT.Now""                 // ✅ Correct
DT.Minutes"5"            // ✅ Correct
DT.Now                   // ❌ Wrong (missing "")
DT.Minutes(5)            // ❌ Wrong (not string literal)
```

### Collections

**Use `{}` delimiters, NOT `[]`:**
```polyglot
<< {}                    // ✅ Correct (empty)
<< {1, 2, 3}             // ✅ Correct (array)
<< []                    // ❌ Wrong (invalid delimiter)
<< [1, 2, 3]             // ❌ Wrong (invalid delimiter)
```

### Declared State for Collections

**Don't push empty collections:**
```polyglot
// ❌ Wrong - Ready immediately, cannot push more
[r] .items: pg\array{T} << {}

// ✅ Correct - Declared state, ready for pushes
[r] .items: pg\array{T}

[?] .condition =? #Boolean.True
[~][<] .items << .value  // PUSH to Declared array
[~]
```

---

## Terminology

Polyglot is an **async-centric language**. Use async-aware terminology:

| ✅ Use | ❌ Avoid |
|--------|----------|
| PUSH to variable | assign |
| PULL from variable | read/get |
| Variable States | mutable/immutable |
| Declared → Ready | uninitialized → initialized |
| Pending (async operation) | loading/waiting |
| Faulted (error state) | error/exception |

---

## Reserved Namespace

All variables have a reserved namespace **always in Ready state**:

```polyglot
.variable.pgvar.state                    // Current state (#PgVar.States)
.variable.pgvar.errors                   // Error array (if Faulted)
.variable.pgvar.history.Pending.at       // Timestamp entered Pending
.variable.pgvar.history.Ready.at         // Timestamp became Ready
.variable.pgvar.history.Faulted.at       // Timestamp became Faulted
```

---

## AI Context Package

These examples were generated from the updated AI context package located at:

**`/docs/ai-context/`**

The package includes:
- `grammar.ebnf` - Formal EBNF grammar
- `constraints.yaml` - Validation rules and edge cases
- `state-machine.yaml` - Variable state transitions
- `operators.json` - Complete operator reference
- `examples-annotated.pg` - Canonical patterns with annotations
- `README.md` - Package guide

All examples comply with the rules defined in the AI context package.

---

## Breaking Changes from v0.0.1

1. **`[=]` Block Marker Removed**
   - **Old:** `[=] .constant: Type << value`
   - **New:** `[i] .constant: Type << value`
   - **Migration:** Simple find-replace `[=]` → `[i]`

2. **Block Marker Hierarchy Clarified**
   - `[r]` for variable declarations at pipeline scope
   - `[<]`/`[>]` ONLY within parent blocks (pipeline calls, conditionals, loops)

3. **Inline Pipeline Syntax Enforced**
   - ALL inline pipelines must use string literal syntax with `""`

4. **Collection Syntax Standardized**
   - Use `{}` delimiters for all collections (arrays, sets)

See `/docs/BLOCK-MARKER-REMOVAL-SUMMARY.md` for complete migration guide.

---

## Testing Examples

To validate these examples against the Polyglot parser (when implemented):

```bash
# Parse single example
polyglot parse 01-basic-pipeline.pg

# Parse all examples
polyglot parse examples/*.pg

# Validate syntax only
polyglot check --syntax-only examples/
```

---

## Contributing

When adding new examples:

1. ✅ Follow all syntax rules from AI context package
2. ✅ Use `[r]` for variable declarations
3. ✅ Use `""` for all inline pipelines
4. ✅ Use `{}` for all collections
5. ✅ Include `[?] *?` catch-all in conditionals
6. ✅ Use PUSH/PULL terminology in comments
7. ✅ Add comprehensive concept demonstration comments
8. ✅ Test against parser (when available)

---

**Last Updated:** 2025-11-26
**Compliance Level:** v0.0.2 ✅
**Status:** Complete
