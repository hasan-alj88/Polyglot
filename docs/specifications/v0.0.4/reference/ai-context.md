---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "ai-context"
shard: false

# --- Classification ---
type: reference
topic: AI Context for Polyglot v0.0.4
summary: "Reference: AI Context for Polyglot v0.0.4"
keywords:
  - reference
  - documentation

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: any
workflow: any
module: any
complexity: medium

# --- Dependency Chain ---
prereqs:
  []
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#reference"
---
# AI Context for Polyglot v0.0.4

**Machine-readable formal specifications for AI models and tooling**

---

## Purpose

This directory contains formal grammar definitions and structured context designed specifically for:

1. **AI Code Assistants** - Training and fine-tuning language models on Polyglot syntax
2. **Parser Generators** - Implementing parsers from formal grammar
3. **Syntax Validators** - Building linters and syntax checkers
4. **Code Completion Tools** - Providing accurate autocomplete suggestions
5. **Documentation Generation** - Auto-generating reference docs from grammar

---

## Contents

### Grammar Files

- **[polyglot-ebnf-grammar.md](./polyglot-ebnf-grammar.md)** - Complete EBNF grammar for Polyglot v0.0.4

### Coming Soon

- **syntax-patterns.md** - Common code patterns and idioms
- **type-inference-rules.md** - Type system inference rules
- **scope-resolution.md** - Variable and identifier scope rules
- **operator-precedence.md** - Detailed operator precedence table

---

## Using the EBNF Grammar

### For AI Models

**When generating Polyglot code, follow these rules:**

1. **Markers are ALWAYS in square brackets**: `[r]`, `[y]`, `[|]`, `[!]`
2. **Block delimiters use curly braces with markers**: `{@}`, `{|}`, `{#}`, `{x}`
3. **Operators are prefix symbols**: `|Pipeline`, `#Enum`, `!Error`, `$variable`
4. **Indentation is significant** - scope determined by indentation level
5. **Comparison operators end with `?`**: `=?`, `>?`, `>=?`
6. **IO operators**: `<input`, `>output`, `<<`, `>>`, `<~`

### For Parser Implementation

The EBNF grammar in this directory follows standard Extended Backus-Naur Form notation:

- `::=` defines a production rule
- `|` represents alternation (choice)
- `[ ]` indicates optional elements
- `{ }` indicates zero or more repetitions
- `( )` groups elements

**Example production rule:**
```ebnf
conditional ::= fork_branch { continuation_branch } [ join ]
```

This means: A conditional consists of one fork_branch, followed by zero or more continuation_branches, optionally ending with a join.

### For Syntax Highlighting

Key token types to highlight:

**Markers (Keywords):**
- Execution: `[r]`, `[p]`, `[b]`, `[y]`, `[v]`
- I/O: `[|]`, `[~]`, `[*]`
- Control: `[m]`, `[?]`, `[!]`, `[z]`
- Structure: `[.]`, `[s]`, `[t]`, `[Q]`, `[W]`, `[A]`, `[<]`, `[%]`
- Boolean: `[&]`, `[^]`

**Blocks:**
- Open: `{@}`, `{|}`, `{#}`, `{!}`, `{A}`
- Close: `{x}`

**Operators (Prefixes):**
- Pipeline: `|` (e.g., `|ProcessOrder`)
- Enum: `#` (e.g., `#Status`)
- Error: `!` (e.g., `!ValidationError`)
- Unpack: `~` (e.g., `~ForEach`)
- Pack: `*` (e.g., `*Into.Array`)
- Variable: `$` (e.g., `$my_var`)

**IO Operators:**
- `<`, `>`, `<<`, `>>`, `<~`

**Comparison Operators:**
- `=?`, `>?`, `<?`, `>=?`, `<=?`, `!=?`

**Types:**
- `:pg.string`, `:pg.int`, `:pg.float`, `:pg.bool`, `:pg.datetime`, `:pg.serial`
- `:pg.array.*`
- `:#EnumType`

### For Code Completion

**Context-aware completion suggestions:**

After `[` → Suggest markers based on context:
- After newline: `[r]`, `[p]`, `[b]`, `[y]`, `[!]`, `[m]`
- In conditional chain: `[&]`, `[^]`, `[v]`
- In loop: `[~]`, `[*]`, `[v]`
- In pipeline definition: `[|]`, `[t]`, `[Q]`, `[W]`
- In enum definition: `[.]`, `[A]`, `[s]`

After `|` → Suggest pipeline names from scope

After `#` → Suggest enum names from scope

After `!` → Suggest error names from scope

After `$` → Suggest variable names from scope

After `:pg.` → Suggest type: `string`, `int`, `float`, `bool`, `datetime`, `serial`, `array.`

After `#` for boolean values → Suggest: `#True`, `#False`, `#;Boolean;True`, `#;Boolean;False`

### Reserved Boolean Aliases

**Built-in aliases (no import required):**
```polyglot
#True   → #;Boolean;True   // Shorthand
#False  → #;Boolean;False  // Shorthand
```

**When to suggest:**
- After boolean comparisons: `$flag =? #True`
- In metadata assignments: `%Inline.Output << #True`
- In boolean variable initialization: `$active :pg.bool << #True`

**See:** [Reserved Boolean Aliases](../language/types/enums-serial.md#reserved-boolean-aliases)

---

## Grammar Coverage

### ✓ Covered in EBNF Grammar

- [x] Block definitions (app, pipeline, enum, error, alias)
- [x] All markers (execution, I/O, control flow, structure, boolean)
- [x] Variable declarations and assignments
- [x] Type annotations (primitives, arrays, serial, enums)
- [x] Expressions (literals, variables, pipeline calls, field access)
- [x] Serial construction with subfields
- [x] Enum definitions (variants and data subfields)
- [x] Pipeline definitions with I/O parameters
- [x] Conditionals with boolean marker chains
- [x] Match expressions
- [x] Loop constructs (unpack operators)
- [x] Error handling and raising
- [x] Serial load blocks
- [x] Import statements
- [x] Metadata annotations
- [x] Array construction
- [x] Indentation rules
- [x] Operator precedence

### Future Additions

- [ ] Function definitions (future feature)
- [ ] Class definitions (future feature)
- [ ] Module system details
- [ ] Macro system (if added)
- [ ] Advanced type constraints

---

## Examples with Grammar References

### Simple Variable Declaration

**Code:**
```polyglot
[r] $name :pg.string << "Alice"
```

**Grammar Path:**
```
statement → variable_declaration
  → execution_marker variable_name type_annotation assignment_operator expression
    → "[r]" "$name" ":pg.string" "<<" string_literal
```

### Enum with Subfields

**Code:**
```polyglot
{#} #Status
[.] .Pending
[.] .Active
   [.] .started_at :pg.datetime
{x}
```

**Grammar Path:**
```
block_definition → enum_block
  → "{#}" enum_name { enum_field } "{x}"
    → "{#}" "#Status"
      enum_field → "[.]" ".Pending"
      enum_field → "[.]" ".Active" { value_field }
        value_field → "[.]" ".started_at" ":pg.datetime"
    → "{x}"
```

### Conditional with Boolean Chain

**Code:**
```polyglot
[y] $age >=? 18
[&] $has_license =? true
   [r] $can_drive << true
```

**Grammar Path:**
```
statement → conditional
  → fork_branch continuation_branch
    → "[y]" condition → comparison "$age" ">=?" "18"
    → "[&]" condition → comparison "$has_license" "=?" "true"
      indented_statement → "[r]" variable_declaration
```

### Pipeline Definition

**Code:**
```polyglot
{|} |ProcessOrder
[|] <order_id :pg.string
[|] >result :pg.string
[t] |T.Call
[W] |W.Polyglot.Scope
   [r] $value << "processed"
{x}
```

**Grammar Path:**
```
block_definition → pipeline_block
  → "{|}" pipeline_name io_parameters trigger_marker wrapper_marker { statement } "{x}"
```

---

## Validation and Testing

### Grammar Validation Tools

**Recommended tools for validating EBNF grammar:**

1. **ANTLR** - Generate parser from grammar
2. **Bison/Yacc** - Classic parser generator
3. **PEG.js** - Parser generator for JavaScript
4. **Tree-sitter** - Incremental parser for code editors

### Test Cases

**The grammar has been validated against:**

- 200+ code examples from specification
- Complex nested structures
- Error handling patterns
- Serial and enum constructions
- Real-world pipeline compositions

---

## Contributing

When updating the grammar:

1. **Maintain EBNF standard notation**
2. **Validate against existing examples** in `../examples/`
3. **Update this README** if new sections are added
4. **Version the grammar** - include v0.0.4 in filenames
5. **Test with parser generators** before committing

---

## Resources

**External References:**

- [EBNF Standard (ISO/IEC 14977)](https://www.iso.org/standard/26153.html)
- [Wikipedia: Extended Backus-Naur Form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
- [ANTLR Documentation](https://www.antlr.org/)

**Related Polyglot Documentation:**

- [Core Syntax](../core-syntax/) - Human-readable syntax documentation
- [Examples](../examples/) - Code examples for testing
- [Standard Library](../standard-library/) - Library specifications

---

**Version:** v0.0.4
**Last Updated:** 2025-12-15
**Maintainer:** Polyglot Language Team
**Part of:** [v0.0.4 Specification](../README.md)
