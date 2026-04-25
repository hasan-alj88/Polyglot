---
user-story: "US001-Lexer-Line-Parsing"
github-issue-link: "https://github.com/hasan-alj88/Polyglot/issues/365"
status: "in-progress"
assignee: "@developer"
---
# Task: Identify Tokens

## Instructions
1. Process the isolated `ExpressionToken` emitted from the line tokenizer.
2. Scan the expression string to identify the expected sub-tokens (e.g., variables, types, literals, operators).
3. Utilize the known Polyglot object prefixes to rapidly identify and classify token types accurately without arbitrary guessing.
4. Emit these structured sub-tokens to the final Token Stream, ensuring each retains precise line and column location metadata.

## Token List

### Layout & Structural
| Token | Lexeme | Description |
|-------|--------|-------------|
| `TOK_NEWLINE` | `\n` | Line terminator |
| `TOK_INDENT` | `   ` (3 spaces) | Block scope depth at line start |
| `TOK_SPACE` | ` ` | Inline space (not discarded — enforces spacing rules) |
| `TOK_COMMENT_TEXT` | *(rest of line)* | Text payload after a comment marker (`[ ]`, `{ }`, `( )`) |
| `TOK_FOREIGN_CODE` | *(rest of line)* | Raw foreign code text after `[C]` marker |

### Definition Blocks `{X}`
| Token | Lexeme |
|-------|--------|
| `DEF_PACKAGE` | `{@}` |
| `DEF_DATA` | `{#}` |
| `DEF_PIPELINE` | `{-}` |
| `DEF_TRIGGER` | `{T}` |
| `DEF_WRAPPER` | `{W}` |
| `DEF_NATIVE` | `{N}` |
| `DEF_QUEUE` | `{Q}` |
| `DEF_ERROR` | `{!}` |
| `DEF_PERMISSION` | `{_}` |
| `DEF_COLLECTOR` | `{*}` |
| `DEF_CONSTRUCTOR` | `{$}` |
| `DEF_COMMENT` | `{ }` |

### Action Elements `[X]`
| Token | Lexeme |
|-------|--------|
| `ACTION_REGISTRY` | `[@]` |
| `ACTION_EXEC_SEQ` | `[-]` |
| `ACTION_EXEC_PAR` | `[=]` |
| `ACTION_EXEC_BG` | `[b]` |
| `ACTION_DATA_LOAD` | `[#]` |
| `ACTION_TYPE_BIND` | `[$]` |
| `ACTION_COND_SWITCH` | `[?]` |
| `ACTION_ERROR` | `[!]` |
| `ACTION_TRIGGER` | `[T]` |
| `ACTION_QUEUE` | `[Q]` |
| `ACTION_WRAPPER` | `[W]` |
| `ACTION_SCOPE_IN` | `[\]` |
| `ACTION_SCOPE_OUT` | `[/]` |
| `ACTION_DATA_ACCESS_FIXED` | `[.]` |
| `ACTION_DATA_ACCESS_FLEX` | `[:]` |
| `ACTION_LOGICAL_AND` | `[&]` |
| `ACTION_LOGICAL_OR` | `[+]` |
| `ACTION_LOGICAL_XOR` | `[^]` |
| `ACTION_CONTINUATION` | `[~]` |
| `ACTION_FOREIGN_CODE` | `[C]` |
| `ACTION_METADATA` | `[%]` |
| `ACTION_COMMENT` | `[ ]` |

### IO Brackets `(X)`
| Token | Lexeme |
|-------|--------|
| `IO_PIPELINE` | `(-)` |
| `IO_EXPANDER` | `(=)` |
| `IO_COLLECTOR` | `(*)` |
| `IO_PERMISSION` | `(_)` |
| `IO_PARAM_OUT_FALLBACK` | `(>)` |
| `IO_PARAM_IN_FALLBACK` | `(<)` |
| `IO_OPERATION_LABEL` | `($)` |
| `IO_CHAIN_STEP` | `(.)` |
| `IO_COMMENT` | `( )` |

### Identifiers (Monolithic)
The lexer consumes the prefix sigil and all chained field names as a single token. The **prefix is stripped** from the token value — only the name/path is stored.

| Token | Prefix | Example Source | Token Value |
|-------|--------|----------------|-------------|
| `PACKAGE` | `@` | `@Local:999.MyPackage` | `"Local:999.MyPackage"` |
| `DATA` | `#` | `#WeatherReport` | `"WeatherReport"` |
| `DATATYPE` | `#` | `#string` (type annotation) | `"string"` |
| `SCHEMA` | `##` | `##Scalar` | `"Scalar"` |
| `TERMINAL_DATA` | `###` | `###Value` | `"Value"` |
| `PIPELINE` | `-` | `-T.Daily` | `"T.Daily"` |
| `EXPANDER` | `=` | `=ForEach.Array` | `"ForEach.Array"` |
| `COLLECTOR` | `*` | `*Into.Map` | `"Into.Map"` |
| `REASSEMBLE` | `=*` | `=*Agg.Sum` | `"Agg.Sum"` |
| `INPUT_PARAMETER` | `<` | `<city` | `"city"` |
| `OUTPUT_PARAMETER` | `>` | `>report` | `"report"` |
| `VARIABLE` | `$` | `$config.db:host` | `"config.db:host"` |
| `VARIABLE_DISCARD` | `$*` | `$*` | *(no value)* |
| `METADATA` | `%` | `%FallbackMessage` | `"FallbackMessage"` |
| `ERROR` | `!` | `!Error.Name` | `"Error.Name"` |
| `RAISE_ERROR` | `>> !` | `>> !Some.Error` | `"Some.Error"` |
| `PERMISSION` | `_` | `_DataCeiling` | `"DataCeiling"` |
| `PERM_DESCRIPTOR` | `__` | `__Permission` | `"Permission"` |
| `PERM_CONSTRAINT` | `___` | `___Unix` | `"Unix"` |

### Sub-field Identifiers
Field references after `.` or `:` separators. Prefix is stripped.

| Token | Prefix | Example Source | Token Value |
|-------|--------|----------------|-------------|
| `FIXED_SUB_FIELD` | `.` | `.city` | `"city"` |
| `FLEXIBLE_SUB_FIELD` | `:` | `:host` | `"host"` |

### Assignment Operators
| Token | Lexeme |
|-------|--------|
| `OP_PUSH_LEFT` | `<<` |
| `OP_PUSH_RIGHT` | `>>` |
| `OP_DEFAULT_PUSH_LEFT` | `<~` |
| `OP_DEFAULT_PUSH_RIGHT` | `~>` |
| `OP_FALLBACK_PUSH_LEFT` | `!<` |
| `OP_FALLBACK_PUSH_RIGHT` | `!>` |

### Comparison Operators (ISIT)
| Token | Lexeme |
|-------|--------|
| `OP_ISIT_EQ` | `=?` |
| `OP_ISIT_GT` | `>?` |
| `OP_ISIT_LT` | `<?` |
| `OP_ISIT_GTE` | `>=?` |
| `OP_ISIT_LTE` | `<=?` |
| `OP_ISIT_NEQ` | `=!?` |
| `OP_ISIT_NOT_LT` | `<!?` |
| `OP_ISIT_NOT_GT` | `>!?` |
| `OP_ISIT_NOT_LTE` | `<=!?` |
| `OP_ISIT_NOT_GTE` | `>=!?` |
| `OP_ISIT_ELSE` | `*?` |

### Range Operators
| Token | Lexeme |
|-------|--------|
| `OP_RANGE_LEFT_INC` | `?[` |
| `OP_RANGE_LEFT_EXC` | `?(` |
| `OP_RANGE_COMMA` | `,` |
| `OP_RANGE_RIGHT_INC` | `]` |
| `OP_RANGE_RIGHT_EXC` | `)` |

### Wildcards & Symbols
| Token | Lexeme | Description |
|-------|--------|-------------|
| `ERROR_ELSE` | `!*` | Matches all declared errors |
| `ALL_INPUT` | `<*` | All inputs of target pipeline |
| `ALL_OUTPUTS` | `>*` | All outputs of labeled operation |
| `SYM_SUPPRESS` | `-` | Error suppression suffix (`!*-`) |

### Literals
| Token | Example |
|-------|---------|
| `LITERAL_STRING` | `"hello"` |
| `LITERAL_INT` | `42`, `-7` |
| `LITERAL_FLOAT` | `3.14`, `-0.01` |

Note: `#Boolean.True` and `#Boolean.False` are tokenized as `DATA("Boolean.True")` and `DATA("Boolean.False")` — no special literal type.

### Inline Boundaries
| Token | Lexeme |
|-------|--------|
| `SYM_COMMA` | `,` |
| `SYM_INTERP_START` | `{` (inside strings) |
| `SYM_INTERP_END` | `}` (inside strings) |

## Disambiguation Rules

| Ambiguity | Rule |
|-----------|------|
| `#name` → `DATA` vs `DATATYPE` | No space before `#` = `DATATYPE` (type annotation glued to prev token). Space before `#` = `DATA` (standalone value reference). |
| `-7` → `PIPELINE` vs `LITERAL_INT` | After an assignment operator (`<<`, `>>`, `<~`, `~>`) = `LITERAL_INT`. After a block marker = `PIPELINE`. |
| `.name` → `FIXED_SUB_FIELD` vs `SEP_FIXED` | `.` followed by a letter = `FIXED_SUB_FIELD`. Standalone `.` between names inside a monolithic identifier is consumed as part of that identifier. |
| `>>` → `OP_PUSH_RIGHT` vs two `>` | Greedy: always match the longest operator first. |
| `=*Name` → `REASSEMBLE` vs `EXPANDER` | Greedy: check `=*` before `=`. |
| `!*` → `ERROR_ELSE` vs `ERROR` + `COLLECTOR` | `!*` is always `ERROR_ELSE`. |
| `[!] >> !Name` | `ACTION_ERROR` + `TOK_SPACE` + `RAISE_ERROR("Name")`. The `>> !` sequence is recognized as the raise pattern. |
| `[C] code text` | `ACTION_FOREIGN_CODE` + `TOK_SPACE` + `TOK_FOREIGN_CODE("code text")`. Remainder of line is raw text. |
