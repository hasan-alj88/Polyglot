---
last-redoc-date: 2025-12-18
---

# Polyglot v0.0.4 Reference Documentation

This directory contains formal specifications and reference materials for implementing Polyglot language tooling, including parsers, compilers, and IDE support.

## Core Reference Files

### [grammar.md](./grammar.md)
**Purpose**: Complete EBNF (Extended Backus-Naur Form) grammar specification for Polyglot v0.0.4.

**Contents**:
- Formal syntax definition in EBNF notation
- Program structure and statement types
- Block definitions (`{@}`, `{|}`, `{#}`, `{!}`)
- Expression grammar
- Type system notation

**Use Cases**:
- Parser implementation
- Syntax validation
- Language tooling development
- AI model training for code generation

### [ai-context.md](./ai-context.md)
**Purpose**: AI-optimized context for language understanding and code generation.

**Contents**:
- Quick reference patterns
- Common idioms
- Anti-patterns to avoid
- Code examples with annotations

## Operator Precedence Table

**Critical for Expression Parsing**

| Precedence | Operators | Associativity | Description |
|------------|-----------|---------------|-------------|
| 1 (Highest) | `|>` | Left | Pipeline composition |
| 2 | `in?`, `in!?` | Left | Collection membership |
| 3 | `re?`, `re!?` | Left | Regular expression matching |
| 4 | `*?` | Left | Wildcard pattern matching |
| 5 | `=?`, `=!?` | Left | Equality comparison |
| 6 | `>?`, `>!?`, `<?`, `<!?` | Left | Relational comparison |
| 7 | `>=?`, `>=!?`, `<=?`, `<=!?` | Left | Relational with equality |
| 8 | `+"` | Left | String concatenation |
| 9 | `<<`, `>>` | Right | Push operators |
| 10 | `<<<`, `>>>` | Right | Variadic push operators |
| 11 (Lowest) | `<~`, `~>` | Right | Default push operators |

**Important Notes**:
- **Push operators are right-associative**: `$a << $b << $c` parses as `$a << ($b << $c)`
- **Comparison operators are non-associative**: `$a >? $b >? $c` is invalid (use explicit grouping)
- **Pipeline composition binds tightest**: `$x << |A |> |B` parses as `$x << (|A |> |B)`

## Token Sequence Patterns

**Parser Implementation Guide**

### Reserved Indication Pattern

**Syntax**: `Prefix + Semicolon + Segments`

```yaml
pattern: "# ; IDENT (; IDENT)* (. IDENT)* (; IDENT)*"
recognition: "If enum/error prefix followed by semicolon, parse as reserved indication"

token_sequence:
  input: "#Boolean.True"
  tokens: [IdentifierEnum("#"), DelimiterSemicolon, Identifier("Boolean"), DelimiterSemicolon, Identifier("True")]

parsing_rule:
  - Track current delimiter (';' or '.')
  - Each delimiter determines next segment type:
    - After ';' → reserved segment
    - After '.' → custom segment

ast_output:
  type: "HierarchicalIdentifier"
  prefix: "#"
  segments:
    - name: "Boolean"
      reserved: true
    - name: "True"
      reserved: true
```

### Inline Pipeline Pattern

**Syntax**: `PipelineIdentifier + FormattedString`

```yaml
pattern: "| IDENT STRING_START ... STRING_END"
recognition: "PipelineIdentifier immediately followed by StringStart (no whitespace)"

token_sequence:
  input: "|FormatName\"{$first} {$last}\""
  tokens: [IdentifierPipeline("|FormatName"), StringStart, StringContent(" "), InterpolationStart, ...]

parsing_rule:
  - Lookahead: If PipelineIdent followed by StringStart, parse as inline pipeline
  - Formatted string is the ONLY input to the pipeline
  - Pipeline extracts arguments from interpolations

ast_output:
  type: "InlinePipelineCall"
  pipeline: "|FormatName"
  format_string: "{$first} {$last}"
  arguments_extracted: ["$first", "$last"]
```

### Definition Block Pattern

**Syntax**: `{Prefix} Identifier ... {x}`

```yaml
pattern: "{ PREFIX } IDENT { [PREFIX] component } { x }"
recognition: "Open brace followed by prefix character indicates definition"

examples:
  pipeline_definition:
    opener: "{|}"
    component_marker: "[|]"
    closer: "{x}"

  enum_definition:
    opener: "{#}"
    component_marker: "[#]"
    closer: "{x}"

context_rule:
  - Inside {X}...{x}: Markers declare components
  - Outside: Markers bind arguments

state_tracking:
  - Parser maintains "in_definition_block" flag
  - Flag determines how [X] markers are interpreted
```

### Indentation Pattern

**Syntax**: `INDENT + Marker + Expression`

```yaml
pattern: "(SPACE{3})* [MARKER] EXPRESSION"
recognition: "Count leading spaces, divide by 3 for nesting level"

parsing_rule:
  - Track indentation level as stack
  - Each 3 spaces = one nesting level
  - Indentation creates sub-marker relationship

examples:
  level_0:
    spaces: 0
    marker: "[m]"
    relationship: "root"

  level_1:
    spaces: 3
    marker: "[?]"
    relationship: "child of level 0"

  level_2:
    spaces: 6
    marker: "[r]"
    relationship: "child of level 1"

ast_output:
  type: "NestedBlock"
  marker: "[m]"
  expression: "$value"
  children:
    - marker: "[?]"
      expression: "5 ? #Small"
      children: []
```

## Context-Sensitive Parsing Rules

### Colon `:` Disambiguation

```yaml
package_spec_context:
  pattern: "@Registry::Package:Version"
  rule: "In package spec, : is delimiter after Package name"

type_annotation_context:
  pattern: "<input :pg.string"
  rule: "After parameter name or before type path, : starts type identifier"

lookahead_strategy:
  - If previous token is @ or ::, next : is delimiter
  - Otherwise, : starts type identifier
```

### Curly Brace `{` Disambiguation

```yaml
definition_block:
  pattern: "{ PREFIX }"
  rule: "Open brace followed by prefix character"
  example: "{|} |Pipeline"

collection_literal:
  pattern: "{ EXPR (, EXPR)* }"
  rule: "Open brace followed by expression"
  example: "{1, 2, 3}"

lookahead_strategy:
  - Peek next character after {
  - If prefix character (#, |, !, @, :) → definition block
  - Otherwise → collection literal
```

### Marker `[X]` Disambiguation

**Context 1: Inside Definition Block**
```yaml
context: "Inside {X}...{x}"
meaning: "Declare component of X"
example: "{|} |Pipeline\n[|] <input :pg.string"
interpretation: "Declare input parameter of |Pipeline"
```

**Context 2: After Invocation**
```yaml
context: "After [exec] Identifier"
meaning: "Bind argument to component"
example: "[r] |Pipeline\n[|] <input << $value"
interpretation: "Bind $value to |Pipeline's input"
```

**State Tracking Required**:
- Parser maintains `current_context` flag
- Set to "definition" when entering `{X}`
- Set to "invocation" when parsing `[exec] Ident`
- Reset when exiting block or completing invocation

## Error Recovery Strategies

### Missing Closer

```yaml
error: "Definition block {X} not closed with {x}"
recovery:
  - Look ahead for next definition block or EOF
  - Implicitly close current block
  - Report error but continue parsing

strategy: "Synchronization on block boundaries"
```

### Invalid Indentation

```yaml
error: "Indentation not multiple of 3 spaces"
recovery:
  - Round down to nearest multiple of 3
  - Report warning
  - Continue parsing

strategy: "Graceful degradation"
```

### Ambiguous Token Sequence

```yaml
error: "Unexpected token in context"
recovery:
  - Check if token is valid in parent context
  - If yes, pop context and retry
  - If no, skip token and report error

strategy: "Context stack unwinding"
```

## Type Inference Context

```yaml
explicit_type_required:
  - Input parameter declarations: "<input :pg.string"
  - Output parameter declarations: ">output :pg.int"
  - Variable declarations without initializer: "$var :pg.float"

type_can_be_inferred:
  - Variable with literal initializer: "$x << 5" (infer :pg.int)
  - Variable from pipeline output: "$result << |Calculate" (infer from pipeline signature)
  - Parameter binding: "[|] <input << $value" (type already declared in pipeline definition)

inference_rules:
  literal_int: ":pg.int"
  literal_float: ":pg.float"
  literal_string: ":pg.string"
  literal_collection: ":pg.array.TYPE" (infer from elements)
  pipeline_output: "Lookup pipeline signature"
```

## Missing Documentation

The following reference files should be created:

- **operator-precedence.md** - Detailed precedence and associativity rules with examples
- **parsing-tables.md** - Complete token sequence → AST node mapping tables
- **pattern-catalog.md** - Exhaustive list of all valid syntax patterns
- **error-recovery.md** - Comprehensive error recovery strategies
- **type-inference.md** - Complete type inference algorithm specification
- **token-relationships.md** - Which tokens can follow which tokens (for error detection)

## Quick Reference

**Operator Summary**:
- Push: `<<` `>>` `<<<` `>>>` `<~` `~>`
- Compare: `=?` `>?` `<?` `>=?` `<=?` (+ `!` variants)
- Pattern: `re?` `in?` `*?` (+ `!` variants)
- Compose: `|>`
- String: `+"`

**Marker Summary**:
- Execute: `[r]` `[p]` `[b]`
- Control: `[?]` `[!]` `[m]`
- Define: `{|}` `{#}` `{!}` `{@}`
- Boolean: `[+]` `[&]` `[-]` `[^]`

**Prefix Summary**:
- Variable: `$`
- Enum: `#`
- Pipeline: `|`
- Error: `!`
- Unpack: `~`
- Pack: `*` (markers only, e.g., `[*]`, `*Collect`)
- Metadata: `%`
- Input: `<`
- Output: `>`
- Type: `:`
- Package: `@`

---

**For complete syntax details, see**: `../User/language/syntax/README.md`

**For examples and tutorials, see**: `../User/getting-started/` and `../User/examples/`
