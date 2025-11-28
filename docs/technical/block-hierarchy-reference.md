# Polyglot Complete Block Hierarchy & Context Roles

**Date:** 2025-11-20
**Author:** Paige (Tech Writer)
**Status:** 🟡 DRAFT - Awaiting hhj Verification

---

## File Scope (Root Level)

**Syntactic Structure:**

```
Polyglot Source File (.pg)
│
├─ [@] Package Declaration          (MANDATORY, FIRST, ONE instance)
│  ├─ Role: Declares package name
│  ├─ Header: [@] Registry@Package.Full.Name:Major.Minor.Patch
│  ├─ Contains:
│  │  ├─ [#] FileNumber             (File number within package)
│  │  └─ [<] @ImportAlias << Registry@Package.Full.Name:Major.Minor.Patch  (Import other packages)
│  └─ Ends: [X]
│
├─ [#] Enumeration Definition        (Optional, multiple allowed)
│  ├─ Role: Define custom enumeration type
│  ├─ Header: [#] Enumeration.Full.Path (Must be not reserved or extendable)
│  ├─ Contains:
│  │  ├─ [<] .variant: type << value   (Enumeration variants)
│  │  └─ [A] AliasName                 (Optional aliases)
│  └─ Ends: [X]
│
├─ [!] Error Definition              (Optional, multiple allowed)
│  ├─ Role: Define custom error type
│  ├─ Header: [!] Error.Full.Path (Must be not reserved or extendable)
│  ├─ Contains:
│  │  └─ [<] .field: type << value     (Error fields/data)
│  └─ Ends: [X]
│
├─ [M] Macro Definition              (Optional, multiple allowed)
│  ├─ Role: Define reusable code template
│  ├─ Header: [M] Macro.Full.Name
│  ├─ Contains: (see "Macro Internal Structure" below)
│  └─ Ends: [X]
│
└─ [|] Pipeline Definition           (Optional, multiple allowed)
   ├─ Role: Define executable pipeline
   ├─ Header: [|] Pipeline.Full.Name
   ├─ Contains: (see "Pipeline Internal Structure" below)
   └─ Ends: [X]
```

**Key Points:**
- `[@]` package declaration **CONTAINS** `[#]` FileNumber and `[<]` imports (they are NOT siblings)
- `[@]` package declaration **ENDS with [X]** (not implicit)
- All definitions at file scope are siblings: `[#]`, `[!]`, `[M]`, `[|]`
- Import syntax: `[<] @ImportAlias << Registry@Package.Full.Name:Major.Minor.Patch`

---

## Macro Internal Structure `[M]`

**Context:** Inside `[M] MacroName` at file scope

```
[M] MacroName
│
├─ [<] Macro.include"<chars+"       (REQUIRED, declares block types)
│  └─ Role: Explicit declaration of which block types macro contains
│
├─ [{] .variable: type              (Optional, multiple allowed)
│  └─ Role: Scope INPUT - variables required FROM caller
│
├─ [}] .variable: type              (Optional, multiple allowed)
│  └─ Role: Scope OUTPUT - variables provided TO caller
│
└─ Pipeline Sub-Elements (same as [|]):
   ├─ [i] .param: type              (Optional in macro - macro parameters)
   ├─ [i] .const: type << value     (Optional in macro - constants)
   ├─ [t] Trigger                   (Optional in macro - trigger conditions)
   ├─ [\] Setup                     (Optional in macro - setup operations)
   ├─ [r] Run sequential            (Optional in macro - operations)
   ├─ [p] Parallel                  (Optional in macro - parallel ops)
   ├─ [s] Serial load               (Optional in macro - file loading)
   ├─ [b] Background                (Optional in macro - fire-and-forget)
   ├─ [Y] Join                      (Optional in macro - join point)
   ├─ [/] Cleanup                   (Optional in macro - cleanup operations)
   └─ [o] .output: type             (Optional in macro - outputs)
```

**Key Points:**
- `[{]` and `[}]` are UNIQUE to macros (not in pipelines)
- Macro can contain same execution blocks as pipelines
- `[<] Macro.include` validates that macro only uses declared block types
- Macros are fragments - they don't need to satisfy canonical requirements alone

---

## Pipeline Internal Structure `[|]`

**Context:** Inside `[|] PipelineName` at file scope

**Canonical Order (from runtime-execution.md):**
```
[t] → [i],[i] → [Q] → [\] → [r],[p],[s],[b],[Y] → [/] → [o] → [X]
```

**REQUIRED Elements in Canonical Form (after macro unwrapping):**

All pipelines MUST contain these elements in final rendered form:

1. **Input:** `[i]` and/or `[i]`
   - If no input needed: `[i] #None`

2. **Trigger:** `[t]`
   - ALWAYS REQUIRED

3. **Setup and Cleanup:** `[\]` and `[/]`
   - BOTH required together

4. **Execution:** At least ONE of the following (REQUIRED)
   - `[r]` Sequential
   - `[p]` Parallel (if used, MUST join via `[Y]`)
   - `[s]` Serial load
   - `[b]` Background (fire-and-forget)

5. **Output:** `[o]`
   - If no output: `[o] !NoError`

**Note:** Requirements apply to FINAL RENDERED pipeline including unwrapped macros via `[W]`

---

### Canonical Pipeline Examples

**Example 1: Minimal Canonical Pipeline (no input, no output)**
```polyglot
[|] MinimalPipeline
[i] #None                       // No input - REQUIRED placeholder
[t] |T.Call                     // Trigger - REQUIRED
[\] |Code.NoOp                  // Setup - REQUIRED
[r] |U.DoSomething              // Sequential execution - REQUIRED
[/] |Code.NoOp                  // Cleanup - REQUIRED
[o] !NoError                    // No output - REQUIRED placeholder
[X]
```

**Example 2: Canonical Pipeline with Input/Output**
```polyglot
[|] ProcessData
[i] .data: pg\string            // Input - REQUIRED
[t] |T.Call                     // Trigger - REQUIRED
[\] |U.Log.Start                // Setup - REQUIRED
[r] |U.Process                  // Sequential execution - REQUIRED
[<] .input: pg\string << .data
[>] .result: pg\string >> .output
[/] |U.Log.Stop                 // Cleanup - REQUIRED
[o] .output: pg\string          // Output - REQUIRED
[X]
```

**Example 3: Pipeline with Macro (satisfies requirements via macro)**
```polyglot
[M] LoggingWrapper
[<] Macro.include"\/"
[\] |U.Log.Start                // Macro provides setup
[/] |U.Log.Stop                 // Macro provides cleanup
[X]

[|] ProcessWithMacro
[i] .data: pg\string            // Input - REQUIRED
[t] |T.Call                     // Trigger - REQUIRED
[W] |LoggingWrapper             // Unwrap macro (provides [\] and [/])
[r] |U.Process                  // Sequential execution - REQUIRED
[<] .input: pg\string << .data
[>] .result: pg\string >> .output
[o] .output: pg\string          // Output - REQUIRED
[X]

// After macro unwrapping, this becomes canonical:
// [i] .data
// [t] |T.Call
// [\] |U.Log.Start    ← FROM MACRO
// [r] |U.Process
// [/] |U.Log.Stop     ← FROM MACRO
// [o] .output
```

**Example 4: Pipeline with Parallel (requires Join)**
```polyglot
[|] ParallelPipeline
[i] .data: pg\string
[t] |T.Call
[\] |Code.NoOp
[p] |U.ProcessParallel          // Parallel - requires [Y]
[<] .input: pg\string << .data
[Y]                             // Join - REQUIRED after [p]
[r] |U.Finalize                 // Sequential execution
[/] |Code.NoOp
[o] !NoError
[X]
```

**Example 5: Pipeline with only Serial Load (no [r] required)**
```polyglot
[|] LoadConfiguration
[i] #None
[t] |T.Call
[\] |Code.NoOp
[s] .config: pg\serial << JSON"\\Config\\app.json"    // Serial load (execution)
[s][!] !File.NotFound
[r] |U.Log.Error
[/] |Code.NoOp
[o] .config: pg\serial
[X]
```

**Example 6: Pipeline with only Background (no [r] required)**
```polyglot
[|] FireAndForgetTask
[i] .task_id: pg\string
[t] |T.Call
[\] |U.Log.TaskStarted
[b] |U.ProcessInBackground      // Background only (execution)
[<] .id: pg\string << .task_id
[/] |U.Log.TaskQueued
[o] !NoError
[X]
```

---

**Detailed Pipeline Structure:**

```
[|] PipelineName
│
├─ [i] .param: type                 (Optional - input parameters)
│  ├─ Role: Define input that caller MUST provide
│  ├─ Contains:
│  │  └─ [<] .param << default      (Optional - default value)
│  └─ Note: Without [<], parameter is REQUIRED
│
├─ [i] .const: type << value        (Optional - immutable constants)
│  └─ Role: Fixed value, caller CANNOT override
│
├─ [t] Trigger                      (Optional/REQUIRED in canonical)
│  ├─ Role: Determine IF pipeline executes
│  ├─ Contains:
│  │  ├─ [<] .input << value        (Pass inputs to trigger)
│  │  └─ Boolean Logic Blocks:
│  │     ├─ [+] OR block
│  │     ├─ [&] AND block
│  │     ├─ [^] XOR block
│  │     ├─ [.] Group start
│  │     ├─ [~] Group member
│  │     └─ [-] NOT block
│  └─ Note: Trigger continuously runs to check condition
│
├─ [Q] Queue Assignment             (Optional)
│  ├─ Role: Route pipeline to execution queue
│  └─ Contains:
│     └─ [<] .param << value        (Queue parameters)
│
├─ [W] Wrapper/Unwrap               (Optional - UNIQUE to pipelines)
│  ├─ Role: Inline a macro's blocks into this pipeline
│  ├─ Syntax: [W] |MacroName
│  ├─ Contains:
│  │  ├─ [<] .input << source       (Wire [{] inputs FROM macro)
│  │  └─ [>] .output >> target      (Wire [}] outputs FROM macro)
│  └─ Note: Compile-time inline, blocks merge by TYPE
│
├─ [\] Setup                        (Optional)
│  ├─ Role: Allocate resources, initialize state
│  └─ Contains:
│     ├─ [<] .input << value        (Pass inputs)
│     └─ [>] .output >> var         (Capture outputs)
│
├─ Execution Blocks                 (Optional, multiple allowed)
│  │
│  ├─ [r] Run Sequential
│  │  ├─ Role: Execute operation sequentially
│  │  ├─ Syntax: [r] |U.Package.Pipeline
│  │  ├─ Contains:
│  │  │  ├─ [<] .input << value
│  │  │  └─ [>] .output >> var
│  │  └─ Can be nested with [~]
│  │
│  ├─ [p] Parallel
│  │  ├─ Role: Execute operations in parallel
│  │  ├─ Contains:
│  │  │  ├─ [<] .input << value
│  │  │  ├─ [>] .output >> var
│  │  │  └─ [~][r] Nested operations (explicit)
│  │  └─ Can nest [p], [r], [?], [b] inside
│  │
│  ├─ [s] Serial Load
│  │  ├─ Role: Load serialized data from files (parallel execution)
│  │  ├─ Syntax: [s] .var: type << Format"path"
│  │  ├─ Contains:
│  │  │  └─ [s][!] Error handler (scope-level)
│  │  └─ Three-step: collect paths → load parallel → assign
│  │
│  ├─ [b] Background
│  │  ├─ Role: Fire-and-forget async operation
│  │  ├─ Contains:
│  │  │  └─ [<] .input << value
│  │  └─ Note: NO [>] output - never joins back
│  │
│  └─ [Y] Join
│     ├─ Role: Explicit join point for parallel operations
│     └─ Note: Waits for parallel/background to complete
│
├─ [?] Switch/Conditional           (Optional, multiple allowed)
│  ├─ Role: Conditional branching
│  ├─ Syntax: [?] .var =? value  OR  [?] .var ?[min, max]
│  ├─ Contains:
│  │  ├─ Boolean Logic Blocks (same as [t]):
│  │  │  ├─ [+] OR
│  │  │  ├─ [&] AND
│  │  │  ├─ [^] XOR
│  │  │  ├─ [.] Group
│  │  │  └─ [-] NOT
│  │  └─ [~][r] Nested operations (execute if condition true)
│  └─ Can nest [p], [r], [s], [b] inside with [~]
│
├─ [!] Error Handling               (Optional, multiple allowed)
│  ├─ Role: Catch and handle errors
│  ├─ Context-dependent:
│  │  ├─ [!] .var.error =? !ErrorType   (Variable-level)
│  │  ├─ [s][!] !ErrorType              (Serial block scope-level)
│  │  └─ [~][!] !ErrorType              (Previous block)
│  ├─ Contains:
│  │  ├─ [>] .field >> var              (Extract error data)
│  │  └─ [r], [p], [o] (Handle error)
│  └─ Precedence: [~][!] (specific) > [s][!] (general)
│
├─ [/] Cleanup                      (Optional)
│  ├─ Role: Release resources, finalize state
│  └─ Contains:
│     ├─ [<] .input << value
│     └─ [>] .output >> var
│
└─ [o] .output: type                (Optional - output declaration)
   ├─ Role: Define output that pipeline provides
   └─ Note: Caller receives this via [>]
```

---

## Boolean Logic Blocks (Context: Inside `[t]` or `[?]`)

**Can appear under:** `[t]` Trigger blocks OR `[?]` Switch blocks

```
Boolean Logic Structure:
│
├─ [+] OR Modifier                  (Alternative condition)
│  ├─ Role: At least one condition must be true
│  ├─ Closes by: Context (when nesting level changes)
│  └─ Used for alternate conditions
│
├─ [&] AND Modifier                 (Required condition)
│  ├─ Role: All conditions must be true
│  ├─ Closes by: Context (when nesting level changes)
│  └─ Explicit or implicit at first trigger level
│
├─ [^] XOR Modifier                 (Exclusive OR)
│  ├─ Role: Exactly one condition must be true
│  ├─ Closes by: Context (when nesting level changes)
│  └─ Mutually exclusive conditions
│
├─ [.] Group Start                  (Explicit grouping)
│  ├─ Role: Begin condition group
│  ├─ Closes by: Context (when nesting level changes)
│  ├─ Contains:
│  │  └─ [~] Group member prefix
│  └─ Used for complex nested boolean logic
│
├─ [-] NOT Modifier                 (Logical negation)
│  ├─ Role: Negate condition result
│  └─ Closes by: Context (when nesting level changes)
│
└─ [~] Group Member                 (Inside group)
   ├─ Role: Mark condition as part of group above
   ├─ Nesting depth: number of [~] prefixes
   └─ Example: [~][~][t] = doubly nested
```

---

## Context-Dependent Blocks

### `[#]` - Dual Meaning by Context

**Context 1: Inside `[@]` Package Declaration**
```
[@] Registry@Package.Full.Name:1.0.0
[#] 001                 // File number within package
[X]
```
- Role: File number within package
- Type: Numeric identifier
- Required: Yes (inside `[@]`)

**Context 2: File Scope (sibling to `[@]`)**
```
[#] Enumeration.Full.Path     // Enumeration definition
[<] .variant1: type << value
[<] .variant2: type << value
[X]
```
- Role: Define enumeration type
- Type: Type definition
- Constraint: Must not be reserved or extendable enumeration

### `[!]` - Dual Meaning by Context

**Context 1: File Scope (sibling to `[@]`)**
```
[!] ErrorTypeName       // Error definition
[<] .field: type
[X]
```
- Role: Define error type
- Type: Type definition

**Context 2: Inside Pipeline `[|]`**
```
[!] .var.error =? !Type     // Error handler
[r] |HandleError
```
- Role: Catch and handle error
- Type: Control flow

### `[<]` - Multiple Contexts

**Context 1: Import Package (inside `[@]`)**
```
[@] Registry@Package.Full.Name:1.0.0
[<] @ImportAlias << Registry@OtherPackage.Full.Name:1.0.0
[X]
```
- Role: Import dependency package
- Syntax: `[<] @ImportAlias << Registry@Package.Full.Name:Major.Minor.Patch`

**Context 2: Define Field/Variant (inside `[#]`, `[!]`)**
```
[#] Status
[<] .Success: pg\int << 0            // Define enum variant
[<] .Failed: pg\int << 1
[X]

[!] DatabaseError
[<] .message: pg\string              // Define error field
[X]
```
- Role: Define enumeration variant or error field

**Context 3: Macro Include (inside `[M]`)**
```
[M] MacroName
[<] Macro.include"{\/"
```
- Role: Declare macro block types (REQUIRED in macros)

**Context 4: Pass Input (inside operation blocks)**
```
[r] |U.Function
[<] .param: type << value
```
- Role: Pass input to operation

**Context 5: Wire Scope Input (inside `[W]`)**
```
[W] |MacroName
[<] .input << source    // Wire [{] from macro
```
- Role: Wire macro scope input

### `[>]` - Multiple Contexts

**Context 1: Capture Output (inside operation blocks)**
```
[r] |U.Function
[>] .result >> variable
```
- Role: Capture output from operation

**Context 2: Wire Scope Output (inside `[W]`)**
```
[W] |MacroName
[>] .output >> target   // Wire [}] from macro
```
- Role: Wire macro scope output

**Context 3: Extract Error Data (inside `[!]`)**
```
[s][!] !File.NotFound
[>] .message >> err_msg
```
- Role: Extract error information

---

## Expansion/Nesting Marker `[~]`

**Role:** Marks explicit nesting/expansion

**Context 1: Nested Operations (inside `[p]`, `[?]`)**
```
[p] |ParallelOp
[~][r] |NestedOperation     // Explicit nesting
```

**Context 2: Group Member (inside boolean logic)**
```
[.]                         // Group start
[~][t] |Condition1          // Group member
[~][+] |Condition2          // OR within group
```

**Context 3: File Scope (implicit)**
```
All top-level definitions ([|], [M], [#], [!]) have implicit first [~]
```

---

## Special Markers

### `[X]` - End Marker
**Role:** Close scope for file-level definitions

**Required for:**
- `[|]` Pipeline ... `[X]`
- `[M]` Macro ... `[X]`
- `[#]` Enumeration ... `[X]`
- `[!]` Error ... `[X]`

**NOT required for:**
- `[@]` Package declaration (closes with `[X]`)
- Operation blocks `[r]`, `[p]`, `[s]`, `[b]` (single-line or scoped by parent)
- Boolean blocks `[+]`, `[&]`, `[^]`, `[-]`, `[.]` (close by context, not explicit markers)

### `[*]` - Line Continuation
**Role:** Continue previous line

**Contexts:**
```
[<] .url: pg\string << "postgresql://"
[*] +"admin:pass@"              // String concatenation
[*] +"localhost:5432/"
```

**NOT for:**
- Adding new parameters (use `[<]` for each)
- New block elements (each gets own block marker)

---

## Complete Hierarchy Summary

```
Polyglot File (.pg)
│
├─ File Scope (all siblings):
│  ├─ [@] Package (MANDATORY FIRST)
│  │  ├─ [<] @import
│  │  └─ [#] FileNumber
│  │
│  ├─ [#] Enumeration
│  │  ├─ [<] .variant
│  │  ├─ [A] Alias
│  │  └─ [X]
│  │
│  ├─ [!] Error
│  │  ├─ [<] .field
│  │  └─ [X]
│  │
│  ├─ [M] Macro
│  │  ├─ [<] Macro.include
│  │  ├─ [{] Scope input
│  │  ├─ [}] Scope output
│  │  ├─ [i], [i], [t], [\], [r], [p], [s], [b], [Y], [/], [o]
│  │  └─ [X]
│  │
│  └─ [|] Pipeline
│     ├─ [i], [i], [t], [Q], [W], [\], [r], [p], [s], [b], [Y], [?], [!], [/], [o]
│     └─ [X]
│
├─ Trigger/Switch Context (inside [t] or [?]):
│  ├─ [+] OR (closes by context)
│  ├─ [&] AND (closes by context)
│  ├─ [^] XOR (closes by context)
│  ├─ [.] Group (closes by context)
│  ├─ [~] Member
│  └─ [-] NOT (closes by context)
│
├─ Macro-Specific (inside [M]):
│  ├─ [{] Scope input
│  └─ [}] Scope output
│
└─ Pipeline-Specific (inside [|]):
   └─ [W] Wrapper/Unwrap
      ├─ [<] Wire input
      └─ [>] Wire output
```

---

## Block Role Matrix

| Block | File Scope | [@] | [#] | [!] | [M] | [|] | [t]/[?] | [r]/[p]/[s] |
|-------|-----------|-----|-----|-----|-----|-----|---------|-------------|
| [@] | ✅ FIRST | - | - | - | - | - | - | - |
| [#] | ✅ Enum OR FileNum | ✅ FileNum | - | - | - | - | - | - |
| [!] | ✅ Error | - | - | - | - | - | - | - |
| [M] | ✅ Macro | - | - | - | - | - | - | - |
| [|] | ✅ Pipeline | - | - | - | - | - | - | - |
| [<] | - | ✅ Import | ✅ Variant | ✅ Field | ✅ Include/Input | ✅ Input | - | ✅ Input |
| [>] | - | - | - | - | ✅ Output | ✅ Output | - | ✅ Output |
| [{] | - | - | - | - | ✅ ScopeIn | - | - | - |
| [}] | - | - | - | - | ✅ ScopeOut | - | - | - |
| [W] | - | - | - | - | - | ✅ Unwrap | - | - |
| [i] | - | - | - | - | ✅ Params | ✅ Params | - | - |
| [i] | - | - | - | - | ✅ Const | ✅ Const | - | - |
| [t] | - | - | - | - | ✅ Trigger | ✅ Trigger | ✅ Condition | - |
| [Q] | - | - | - | - | ✅ Queue | ✅ Queue | - | - |
| [\] | - | - | - | - | ✅ Setup | ✅ Setup | - | - |
| [r] | - | - | - | - | ✅ Run | ✅ Run | ✅ Nested | - |
| [p] | - | - | - | - | ✅ Parallel | ✅ Parallel | ✅ Nested | ✅ Nested |
| [s] | - | - | - | - | ✅ Serial | ✅ Serial | ✅ Nested | - |
| [b] | - | - | - | - | ✅ Background | ✅ Background | ✅ Nested | - |
| [Y] | - | - | - | - | ✅ Join | ✅ Join | - | - |
| [?] | - | - | - | - | ✅ Switch | ✅ Switch | - | ✅ Nested |
| [!] | ✅ ErrorDef | - | - | - | ✅ Handler | ✅ Handler | - | - |
| [/] | - | - | - | - | ✅ Cleanup | ✅ Cleanup | - | - |
| [o] | - | - | - | - | ✅ Output | ✅ Output | - | - |
| [X] | - | - | ✅ | ✅ | ✅ | ✅ | - | - |
| [+] | - | - | - | - | - | - | ✅ OR | - |
| [&] | - | - | - | - | - | - | ✅ AND | - |
| [^] | - | - | - | - | - | - | ✅ XOR | - |
| [.] | - | - | - | - | - | - | ✅ Group | - |
| [~] | - | - | - | - | - | - | ✅ Member | ✅ Nested |
| [-] | - | - | - | - | - | - | ✅ NOT | - |
| [*] | - | - | - | - | ✅ Continue | ✅ Continue | - | ✅ Continue |
| [A] | - | - | ✅ Alias | - | - | - | - | - |

---

**Status:** 🟡 DRAFT - Please review hierarchy and context roles for accuracy

**Questions:**
1. Is the file scope structure correct (all siblings)?
2. Are the context-dependent blocks complete?
3. Is the macro system hierarchy accurate?
4. Any missing blocks or incorrect parent-child relationships?
