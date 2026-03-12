---
last-redoc-date: 2025-12-18
---

# Polyglot v0.0.4 Syntax Reference

This directory contains the complete syntax specification for Polyglot v0.0.4, a language designed for asynchronous automation with cross-language FFI capabilities.

## Overview

Polyglot syntax is built on three core principles:

1. **Predictable Structure**: Every line follows the pattern `[Optional Indentation] + [Marker(s)] + [One Expression]`
2. **Prefix-Based Identifiers**: All identifiers use prefix characters (`.`, `#`, `|`, `!`, `~`, `%`, etc.) to indicate their type
3. **Context-Dependent Markers**: The same marker (e.g., `[|]`) serves different purposes based on context (definition vs invocation)

## Core Syntax Files

### [prefix-system.md](./prefix-system.md)
**Purpose**: Reference for all prefix operators that begin identifiers.

**Key Prefixes**:
- `.` - Variables (e.g., `$varName`)
- `#` - Enumerations (e.g., `#Status.Active`)
- `|` - Pipelines (e.g., `|ProcessData`)
- `!` - Errors (e.g., `!Network.Timeout`)
- `~` - Unpack operators (e.g., `~ForEach.Array`)
- `%` - Metadata (e.g., `%Doc`, `%Author`)
- `<` - Input arguments (e.g., `<input1`)
- `>` - Output arguments (e.g., `>result`)
- `:` - Type identifiers (e.g., `:pg.string`)
- `@` - Package specs (e.g., `@Local::MyApp:1.0.0`)

### [markers.md](./markers.md)
**Purpose**: Complete reference for block markers `[X]` that control execution flow and structure.

**Key Marker Categories**:
- **Execution**: `[r]` (sequential), `[p]` (parallel), `[b]` (background)
- **Control Flow**: `[?]` (conditional), `[!]` (error catch), `[m]` (match/select)
- **Structure**: `[|]` (pipeline section), `[i]` (input), `[o]` (output)
- **Definitions**: `{|}` (define pipeline), `{#}` (define enum), `{!}` (define error)
- **Boolean Logic**: `[+]` (OR), `[&]` (AND), `[-]` (NOT), `[^]` (XOR)

### [operators.md](./operators.md)
**Purpose**: Complete catalog of all operators with precedence rules.

**Operator Categories**:
- **Push Operators**: `<<`, `>>`, `<<<`, `>>>` (variadic), `<~`, `~>` (default)
- **Comparison**: `=?`, `>?`, `<?`, `>=?`, `<=?` (and negated variants with `!`)
- **Pattern Matching**: `re?`, `re!?`, `in?`, `in!?`, `*?`
- **Composition**: `|>` (pipeline composition)
- **String**: `+"` (string concatenation)

### [io-operators.md](./io-operators.md)
**Purpose**: Detailed specification for input/output binding operators.

**Key Concepts**:
- Input binding: `<input << $value` (push value to input)
- Output binding: `>output >> $target` (pull value from output)
- Variadic I/O: `<<<` and `>>>` for arrays

## Critical Patterns for Parser Implementation

### 1. Reserved vs Custom Hierarchy

**Rule**: Semicolon (`;`) marks reserved segments, dot (`.`) marks custom segments.

```polyglot
#DT.Business;Week.CustomWeek;RestDays
├─ DT          (reserved - after ;)
├─ Business    (reserved - after ;)
├─ Week        (reserved - after ;)
├─ CustomWeek  (custom - after .)
└─ RestDays    (reserved - after ;)
```

**Token Sequence**: `#` → `;` → `Ident` → `;` → `Ident` → `.` → `Ident` → `;` → `Ident`

**Parser Rule**: Each delimiter (`;` or `.`) determines the nature of the **next** segment.

### 2. Definition vs Invocation Pattern

**Dual-Context Markers**: Markers like `[|]` have different meanings in different contexts.

#### Context A: Definition (inside `{X}...{x}`)

```polyglot
{|} |MyPipeline              ← Define |MyPipeline
[|] <input1 :pg.string       ← [|] means "parameter OF |MyPipeline"
[|] <input2 :pg.int
... code body ...
[|] >output :pg.int << $x    ← [|] means "output OF |MyPipeline"
{x}                          ← End definition
```

**Reading**: `[|]` declares parameters/outputs of the pipeline being defined.

#### Context B: Invocation (after execution marker)

```polyglot
[r] |MyPipeline              ← Invoke |MyPipeline
[|] <input1 << $x            ← [|] means "bind TO |MyPipeline's <input1"
[|] >output >> $y            ← [|] means "bind FROM |MyPipeline's >output"
```

**Reading**: `[|]` binds values to/from the pipeline being invoked.

**This pattern applies to ALL identifiers**: `[|]`, `[~]`, `[*]`, `[#]`, `[!]`, etc.

### 3. Indentation as Sub-Marker Relationship

**Rule**: 3-space indentation creates a sub-marker relationship (equivalent to `~\` in v0.0.3).

```polyglot
[m] $value
   [?] 1 ? #Small              ← 3 spaces: sub-marker of [m]
   [?] 10 ? #Medium
      [r] $log << "Medium"     ← 6 spaces: sub-marker of [?]
   [?] * ? #Large
```

**Important**: All Polyglot code is `marker + expression` (not Python-style statement-only lines).

**Curly Bracket Exception**: `{X}...{x}` blocks do NOT use indentation (closed by `{x}`).

### 4. Inline Pipeline Syntax

**Rule**: Inline pipelines receive a formatted string as their ONLY input.

```polyglot
$result << |FormatName"{$first} {$last}"
           └─ Formatted string with interpolations
```

**Valid Forms**:
- With arguments: `|Call"{$arg1:fmt} text {$arg2}"`
- No arguments: `|Call""`

**Why**: The formatted string IS the input; pipeline extracts arguments from interpolations.

## Pattern Catalog

Every valid Polyglot line matches one of these patterns:

### Single-Line Patterns

```yaml
basic:
  pattern: "[marker] expression"
  examples:
    - "[r] $x << 5"
    - "[?] $x >? 10"
    - "[!] * ? |ErrorHandler"

nested:
  pattern: "INDENT + [marker] expression"
  examples:
    - "   [r] $y << 2           # 1 level (3 spaces)"
    - "      [?] $y >? 0         # 2 levels (6 spaces)"

definition:
  pattern: "{marker} identifier"
  examples:
    - "{|} |PipelineName"
    - "{#} #EnumName"
    - "{!} !ErrorName"
```

### Multi-Line Patterns

```yaml
pipeline_definition:
  pattern: |
    {|} |Name
    [|] <input type
    [|] <input type
    ... code body ...
    [|] >output type << expression
    {x}

pipeline_invocation:
  pattern: |
    [execution_marker] |Name
    [|] <input << value
    [|] >output >> target

match_selection:
  pattern: |
    [m] expression
       [?] condition ? result
       [?] condition ? result
       [?] * ? default

loop_pattern:
  pattern: |
    [execution_marker] ~UnpackOperator
    [~] <input << source
    [~] >output >> target
       ... iteration body (indented) ...
```

## Parser Implementation Guidance

### Token Sequence Recognition

1. **Reserved Indication**: Look for `Prefix + Semicolon` to detect reserved segments
2. **Inline Pipelines**: Look for `PipelineIdent + StringStart` (no whitespace)
3. **Definitions**: Look for `{` + `Prefix` + `}` to detect definition blocks
4. **Invocation**: Look for `[ExecutionMarker]` + `Identifier` to detect calls

### Context Tracking

The parser must maintain context:
- **In Definition Block**: `{X}` encountered → markers declare components
- **After Invocation**: `[exec] Ident` encountered → markers bind arguments
- **Indentation Level**: Track spaces to build nesting structure

### Critical Ambiguities

1. **Curly Braces**: `{1, 2, 3}` (collection) vs `{|} |Name` (definition)
   - Solution: Check next char after `{` - if prefix char, it's definition

2. **Colon**: `:` in `@Reg::Pkg:Ver` (delimiter) vs `:pg.string` (type)
   - Solution: Package spec context uses `::`, type context uses single `:`

3. **Markers**: `[|]` parameter vs `[|]` binding
   - Solution: Track whether inside `{|}...{x}` definition block

## Missing Documentation

The following topics need dedicated documentation files:

- **reserved-indication.md** - Detailed parsing rules for `;` hierarchies
- **inline-pipelines.md** - Complete specification for formatted string pipelines
- **indentation.md** - Indentation rules and sub-marker relationships
- **definition-patterns.md** - All definition syntaxes (`{X}`)
- **pattern-catalog.md** - Exhaustive list of all valid line patterns
- **operator-precedence.md** - Complete precedence table
- **parsing-tables.md** - Token sequence → AST mappings

## Quick Reference

**File Structure**:
```
syntax/
├── README.md (this file)
├── prefix-system.md (all prefixes: ., #, |, !, ~, %, etc.)
├── markers.md (block markers: [r], [p], [?], etc.)
├── operators.md (all operators with precedence)
└── io-operators.md (input/output binding details)
```

**Next Steps**: For comprehensive parser implementation guidance, see `../User/reference/` directory for parsing tables and precedence rules.

---

**Note**: This is v0.0.4 syntax. v0.0.3 and earlier used different patterns (e.g., `,` for variables instead of `$`, explicit `~\` instead of indentation).
