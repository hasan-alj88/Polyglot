---
# BMAD Agent Context Block
# Schema: bmad-context-v1
id: parser-quick-reference
type: reference
status: stable
updated: 2025-12-18
version: 0.0.4
---

# Polyglot v0.0.4 Parser Quick Reference Card

**For:** Parser developers implementing v0.0.4
**Keep this handy:** Print or keep open while coding

---

## Token Count: 121 Total

| Category | Count | Examples |
|----------|-------|----------|
| Block Markers | 30 | `[r]`, `[p]`, `[|]`, `[~]`, `[*]` |
| Push Operators | 6 | `<<`, `>>`, `<<<`, `>>>`, `<~`, `~>` |
| Compare Operators | 18 | `=?`, `>?`, `<?`, `>=?`, `<=?` + `!` variants |
| String Operators | 1 | `+"` |
| Other Operators | 6 | `|>`, `in?`, `re?`, `*?` + `!` variants |
| Identifiers | 11 | `$var`, `#enum`, `|pipe`, `!error`, `%meta` |
| Delimiters | 15 | `;`, `.`, `:`, `,`, `(`, `)`, etc. |
| Keywords | 8 | `if`, `then`, `else`, `match`, etc. |

---

## Operator Precedence (11 Levels)

| # | Operators | Assoc | Notes |
|---|-----------|-------|-------|
| 1 | `|>` | Left | Pipeline composition (tightest) |
| 2 | `in?`, `in!?` | Left | Collection membership |
| 3 | `re?`, `re!?` | Left | Regex matching |
| 4 | `*?` | Left | Wildcard matching |
| 5 | `=?`, `=!?` | Left | Equality |
| 6 | `>?`, `<?` + `!` | Left | Relational |
| 7 | `>=?`, `<=?` + `!` | Left | Relational + equality |
| 8 | `+"` | Left | String concat |
| 9 | `<<`, `>>` | Right | Push (right-assoc!) |
| 10 | `<<<`, `>>>` | Right | Variadic push |
| 11 | `<~`, `~>` | Right | Default push (loosest) |

**Critical:** Push operators are RIGHT-associative!

---

## Critical Parsing Rules

### Rule 1: Line Structure
**Every line:** `[Indent] + [Marker(s)] + [Expression]`

```polyglot
[r] $x << 5              # 0 indent, 1 marker, 1 expr
   [?] $x >? 10          # 3 spaces = 1 level deep
      [r] $log << "Hi"   # 6 spaces = 2 levels deep
```

### Rule 2: Reserved Indication
**Pattern:** Delimiter determines NEXT segment type

```polyglot
#DT.Business;Week.CustomWeek;RestDays
  ^         ^    ^          ^
  |         |    |          └─ reserved (after ;)
  |         |    └─ custom (after .)
  |         └─ reserved (after ;)
  └─ reserved (first after ;)
```

**Parser:** Track current delimiter (`;` or `.`)

### Rule 3: Inline Pipelines
**Pattern:** `|Identifier` + `"formatted string"`

```polyglot
|FormatName"{$first} {$last}"
            └──────────────────┘
            String IS the input (no space!)
```

**Parser:** Lookahead for StringStart after PipelineIdent

### Rule 4: Indentation Nesting
**Rule:** 3 spaces = 1 nesting level

```polyglot
[m] $value        # Level 0
   [?] 1 ? #A     # Level 1 (3 spaces)
   [?] 2 ? #B     # Level 1 (sibling)
      [r] $x      # Level 2 (6 spaces)
```

**Parser:** Stack-based indentation tracking

### Rule 5: Dual-Context Markers
**Same marker, different meanings:**

```polyglot
# DEFINITION CONTEXT
{|} |Pipeline
[|] <input :type      # [|] DECLARES parameter

# INVOCATION CONTEXT
[r] |Pipeline
[|] <input << $val    # [|] BINDS value
```

**Parser:** Track state (DEFINITION_BLOCK vs INVOCATION)

---

## Context-Sensitive Token Recognition

### Colon `:` Disambiguation

| Context | Example | Meaning |
|---------|---------|---------|
| After `@` or `::` | `@Local::Pkg:1.0` | Version delimiter |
| Before type path | `<input :pg.string` | Type prefix |

### Curly Brace `{` Disambiguation

| Next Token | Example | Meaning |
|------------|---------|---------|
| Prefix char | `{|} |Pipe` | Definition block |
| Expression | `{1, 2, 3}` | Collection literal |

### Marker `[X]` Disambiguation

| Parser State | Meaning | Example |
|--------------|---------|---------|
| DEFINITION_BLOCK | Declare component | `{|} ... [|] <input` |
| INVOCATION | Bind argument | `[r] |P ... [|] <in << $x` |

---

## Common Token Sequences

### Variable Assignment
```
BlockSequential Variable OperatorPush Expression
[r]            $x       <<           5
```

### Pipeline Call with Input
```
BlockSequential IdentifierPipeline
[r]            |Calculate
BlockPipelineStart IdentifierInput OperatorPush Variable
[|]               <value            <<           $x
```

### Reserved Indication
```
IdentifierEnum DelimSemicolon Identifier DelimSemicolon Identifier
#              ;              Boolean    ;              True
```

### Inline Pipeline
```
IdentifierPipeline StringStart StringContent ...
|FormatName        "           {$name} is...
```

### Indentation Nesting
```
BlockMatch Variable                    # Level 0
$status
   BlockConditional Expression         # Level 1 (indent=3)
   [?]              1 ? #Active
      BlockSequential Variable ...     # Level 2 (indent=6)
      [r]            $log << "..."
```

---

## Error Recovery Strategies

| Error Type | Recovery Action |
|------------|-----------------|
| Missing `{x}` closer | Sync on next block/EOF, close implicitly |
| Bad indentation | Round to nearest 3-multiple, warn |
| Ambiguous token | Try parent context, skip if invalid |

---

## Implementation Phases

**Phase 1 (MVP):**
- [ ] Variable assignment
- [ ] Literals
- [ ] Binary expressions
- [ ] Simple conditionals
- [ ] Basic pipeline calls

**Phase 2 (Control Flow):**
- [ ] Indentation tracking
- [ ] Match expressions
- [ ] Error handling
- [ ] Boolean markers

**Phase 3 (Advanced):**
- [ ] Pipeline definitions
- [ ] I/O binding
- [ ] Enum/error definitions
- [ ] Loop constructs

**Phase 4 (Complex):**
- [ ] Reserved indication
- [ ] Inline pipelines
- [ ] Dual-context markers
- [ ] Pipeline composition

---

## Key Files Reference

- **This Card:** Quick lookup during implementation
- **[PARSER-IMPLEMENTATION-GUIDE.md](./PARSER-IMPLEMENTATION-GUIDE.md)** - Full guide
- **[reference/token-patterns.md](./User/reference/token-patterns.md)** - All 121 tokens
- **[reference/syntax-patterns.md](./User/reference/syntax-patterns.md)** - All patterns + EBNF
- **[reference/README.md](./User/reference/README.md)** - Precedence + rules
- **[language/syntax/README.md](./User/language/syntax/README.md)** - Syntax deep dive

---

**Last Updated:** 2025-12-18
**Print this card** for quick reference during parser implementation!
