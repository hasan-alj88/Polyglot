---
user-story: "US001-Lexer-Line-Parsing"
github-issue-link: "https://github.com/hasan-alj88/Aljam3/issues/365"
status: "done"
assignee: "@developer"
---
# Task: Identify Tokens

## Instructions
1. Process the isolated `ExpressionToken` emitted from the line tokenizer.
2. Scan the expression string to identify the expected sub-tokens (e.g., variables, types, literals, operators).
3. Utilize the known Aljam3 object prefixes to rapidly identify and classify token types accurately without arbitrary guessing.
4. Emit these structured sub-tokens to the final Token Stream, ensuring each retains precise line and column location metadata.

## Token List

### Layout & Structural
| Token | Lexeme | Description |
|-------|--------|-------------|
| `TOK_NEWLINE` | `\n` | Line terminator |
| `TOK_INDENT` | `   ` (3 spaces) | Block scope depth at line start |
| `TOK_SPACE` | ` +` | Inline spaces (1 or more consecutive spaces condense into one token) |
| `TOK_COMMENT_TEXT` | *(rest of line)* | Text payload after a comment marker (`[ ]`, `{ }`, `( )`) |
| `TOK_FOREIGN_CODE` | *(rest of line)* | Raw foreign code text after `[C]` marker |
| `TOK_UNRECOGNIZED` | *(unmatched char/seq)* | Catch-all for invalid syntax to prevent lexer panics; delegated to the compiler for error generation |

### Definition Blocks `{X}`
| Token | Pattern Regex |
|-------|---------------|
| `DEF_PACKAGE` | `^\{@\}` |
| `DEF_DATA` | `^\{#\}` |
| `DEF_PIPELINE` | `^\{-\}` |
| `DEF_TRIGGER` | `^\{T\}` |
| `DEF_WRAPPER` | `^\{W\}` |
| `DEF_NATIVE` | `^\{N\}` |
| `DEF_QUEUE` | `^\{Q\}` |
| `DEF_ERROR` | `^\{!\}` |
| `DEF_PERMISSION` | `^\{_\}` |
| `DEF_COLLECTOR` | `^\{\*\}` |
| `DEF_CONSTRUCTOR` | `^\{\$\}` |
| `DEF_COMMENT` | `^\{ \}` |

### Action Elements `[X]`
| Token | Pattern Regex |
|-------|---------------|
| `ACTION_REGISTRY` | `^\[@\]` |
| `ACTION_EXEC_SEQ` | `^\[-\]` |
| `ACTION_EXEC_PAR` | `^\[=\]` |
| `ACTION_EXEC_BG` | `^\[b\]` |
| `ACTION_DATA_LOAD` | `^\[#\]` |
| `ACTION_TYPE_BIND` | `^\[\$\]` |
| `ACTION_COND_SWITCH` | `^\[\?\]` |
| `ACTION_ERROR` | `^\[!\]` |
| `ACTION_TRIGGER` | `^\[T\]` |
| `ACTION_QUEUE` | `^\[Q\]` |
| `ACTION_WRAPPER` | `^\[W\]` |
| `ACTION_SCOPE_IN` | `^\[\\\]` |
| `ACTION_SCOPE_OUT` | `^\[/\]` |
| `ACTION_DATA_ACCESS_FIXED` | `^\[\.\]` |
| `ACTION_DATA_ACCESS_FLEX` | `^\[:\]` |
| `ACTION_LOGICAL_AND` | `^\[&\]` |
| `ACTION_LOGICAL_OR` | `^\[\+\]` |
| `ACTION_LOGICAL_XOR` | `^\[\^\]` |
| `ACTION_CONTINUATION` | `^\[~\]` |
| `ACTION_FOREIGN_CODE` | `^\[C\]` |
| `ACTION_METADATA` | `^\[%\]` |
| `ACTION_COMMENT` | `^\[ \]` |

### IO Brackets `(X)`
| Token | Pattern Regex |
|-------|---------------|
| `IO_PIPELINE` | `^\(-\)` |
| `IO_EXPANDER` | `^\(=\)` |
| `IO_COLLECTOR` | `^\(\*\)` |
| `IO_PERMISSION` | `^\(_\)` |
| `IO_PARAM_OUT_FALLBACK` | `^\(>\)` |
| `IO_PARAM_IN_FALLBACK` | `^\(<\)` |
| `IO_OPERATION_LABEL` | `^\(\$\)` |
| `IO_CHAIN_STEP` | `^\(\.\)` |
| `IO_COMMENT` | `^\( \)` |

### Identifiers (Monolithic)
The lexer consumes the prefix sigil and all chained field names as a single token. The **prefix is stripped** from the token value — only the name/path is stored.
*Note: Regex capture group 1 `(...)` defines the Extracted Token Value. Identifiers are strictly enforced to begin with a letter or underscore, and sub-segments must be populated.*

| Token | Prefix | Example Source | Token Value | Extraction Regex |
|-------|--------|----------------|-------------|------------------|
| `PACKAGE` | `@` | `@Local:999.MyPackage` | `"Local:999.MyPackage"` | `^@([a-zA-Z_]\w*(?:[.:]\w+)*)` |
| `DATA` | `#` | ` #WeatherReport` | `"WeatherReport"` | `(?:^\|(?<=\s))#([a-zA-Z_]\w*(?:\.\w+)*)` |
| `DATATYPE` | `#` | `$Var#string` (type annotation) | `"string"` | `(?<=\S)#([a-zA-Z_]\w*(?:\.\w+)*)` |
| `SCHEMA` | `##` | `##Scalar` | `"Scalar"` | `^##([a-zA-Z_]\w*(?:\.\w+)*)` |
| `TERMINAL_DATA` | `###` | `###Value` | `"Value"` | `^###([a-zA-Z_]\w*(?:\.\w+)*)` |
| `PIPELINE` | `-` | `-T.Daily` | `"T.Daily"` | `^-([a-zA-Z_]\w*(?:\.\w+)*)` |
| `EXPANDER` | `=` | `=ForEach.Array` | `"ForEach.Array"` | `^=([a-zA-Z_]\w*(?:\.\w+)*)` |
| `COLLECTOR` | `*` | `*Into.Map` | `"Into.Map"` | `^\*([a-zA-Z_]\w*(?:\.\w+)*)` |
| `REASSEMBLE` | `=*` | `=*Agg.Sum` | `"Agg.Sum"` | `^\=\*([a-zA-Z_]\w*(?:\.\w+)*)` |
| `INPUT_PARAMETER` | `<` | `<city` | `"city"` | `^<([a-zA-Z_]\w*(?:\.\w+)*)` |
| `OUTPUT_PARAMETER` | `>` | `>report` | `"report"` | `^>([a-zA-Z_]\w*(?:\.\w+)*)` |
| `VARIABLE` | `$` | `$config.db:host` | `"config.db:host"` | `^\$([a-zA-Z_]\w*(?:[.:]\w+)*)` |
| `VARIABLE_DISCARD` | `$*` | `$*` | *(no value)* | `^\$\*$` |
| `METADATA` | `%` | `%FallbackMessage` | `"FallbackMessage"` | `^%([a-zA-Z_]\w*(?:\.\w+)*)` |
| `ERROR` | `!` | `!Error.Name` | `"Error.Name"` | `^!([a-zA-Z_]\w*(?:\.\w+)*)` |
| `RAISE_ERROR` | `>> !` | `>> !Some.Error` | `"Some.Error"` | `^>>\s+!([a-zA-Z_]\w*(?:\.\w+)*)` |
| `PERMISSION` | `_` | `_DataCeiling` | `"DataCeiling"` | `^_([a-zA-Z_]\w*(?:\.\w+)*)` |
| `PERM_DESCRIPTOR` | `__` | `__Permission` | `"Permission"` | `^__([a-zA-Z_]\w*(?:\.\w+)*)` |
| `PERM_CONSTRAINT` | `___` | `___Unix` | `"Unix"` | `^___([a-zA-Z_]\w*(?:\.\w+)*)` |

### Sub-field Identifiers
Field references after `.` or `:` separators. Prefix is stripped.
*Note: Regex capture group 1 `(...)` defines the Extracted Token Value.*

| Token | Prefix | Example Source | Token Value | Extraction Regex |
|-------|--------|----------------|-------------|------------------|
| `FIXED_SUB_FIELD` | `.` | `.city` | `"city"` | `^\.([a-zA-Z_]\w*)` |
| `FLEXIBLE_SUB_FIELD` | `:` | `:host` | `"host"` | `^:([a-zA-Z_]\w*)` |

### Assignment Operators
| Token | Pattern Regex |
|-------|---------------|
| `OP_PUSH_LEFT` | `^<<` |
| `OP_PUSH_RIGHT` | `^>>` |
| `OP_DEFAULT_PUSH_LEFT` | `^<~` |
| `OP_DEFAULT_PUSH_RIGHT` | `^~>` |
| `OP_FALLBACK_PUSH_LEFT` | `^!<` |
| `OP_FALLBACK_PUSH_RIGHT` | `^!>` |

### Comparison Operators (ISIT)
| Token | Pattern Regex |
|-------|---------------|
| `OP_ISIT_EQ` | `^=\?` |
| `OP_ISIT_GT` | `^>\?` |
| `OP_ISIT_LT` | `^<\?` |
| `OP_ISIT_GTE` | `^>=\?` |
| `OP_ISIT_LTE` | `^<=\?` |
| `OP_ISIT_NEQ` | `^=!\?` |
| `OP_ISIT_NOT_LT` | `^<!\?` |
| `OP_ISIT_NOT_GT` | `^>!\?` |
| `OP_ISIT_NOT_LTE` | `^<=!\?` |
| `OP_ISIT_NOT_GTE` | `^>=!\?` |
| `OP_ISIT_ELSE` | `^\*\?` |

### Range Operators
| Token | Pattern Regex |
|-------|---------------|
| `OP_RANGE_LEFT_INC` | `^\?\[` |
| `OP_RANGE_LEFT_EXC` | `^\?\(` |
| `OP_RANGE_COMMA` | `^,` |
| `OP_RANGE_RIGHT_INC` | `^\]` |
| `OP_RANGE_RIGHT_EXC` | `^\)` |

### Wildcards & Symbols
| Token | Pattern Regex | Description |
|-------|---------------|-------------|
| `ERROR_ELSE` | `^!\*` | Matches all declared errors |
| `ALL_INPUT` | `^<\*` | All inputs of target pipeline |
| `ALL_OUTPUTS` | `^>\*` | All outputs of labeled operation |

### Literals
| Token | Example | Pattern Regex |
|-------|---------|---------------|
| `LITERAL_STRING` | `"hello"` | `^"([^"\\]\|\\.)*"` |
| `LITERAL_INT` | `42`, `-7` | `^-?[0-9]+` |
| `LITERAL_FLOAT` | `3.14`, `-0.01` | `^-?[0-9]+\.[0-9]+` |

Note: `#Boolean.True` and `#Boolean.False` are tokenized as `DATA("Boolean.True")` and `DATA("Boolean.False")` — no special literal type.

### Inline Boundaries
| Token | Pattern Regex |
|-------|---------------|
| `SYM_COMMA` | `^,` |
| `SYM_INTERP_START` | `^\{` |
| `SYM_INTERP_END` | `^\}` |

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

## EBNF Lexical Patterns

To optimize and simplify the lexer algorithm, the engine can construct composite regex sequences from EBNF primitives. Rather than relying on rigid loop cycles and lookbehinds, it can match common "glued" syntax structures on a line-by-line basis, successfully yielding arrays of tokens from a single regex execution.

### 1. Primitive EBNF Components
| Rule | Regex Equivalent | Description |
|------|------------------|-------------|
| `Segment` | `[a-zA-Z][a-zA-Z0-9]*` | Standard alphanumeric name block (strictly no underscores). |
| `DotPath` | `Segment(?:\.Segment)*` | Identifier path separated exclusively by dots. |
| `FlexPath` | `Segment(?:[.:]Segment)*` | Identifier path separated by dots or colons. |
| `Space` | ` +` | 1 or more consecutive spaces. |

### 2. Composite Pattern Extractors
By scanning line-by-line using these aggregated patterns, the matching algorithm dynamically yields **multiple tokens and identifiers simultaneously**. Capture groups (e.g. `$1`, `$2`) automatically map to the respective token payloads.

| Logical Pattern Structure | Example Source | Composite Regex | Extracted Token Stream |
|---------------------------|----------------|-----------------|------------------------|
| `Execution_Seq` | `[-] -Transform` | `^\[-\] +-(DotPath)` | `[ ACTION_EXEC_SEQ, TOK_SPACE, PIPELINE($1) ]` |
| `Execution_Par` | `[=] -Transform` | `^\[=\] +-(DotPath)` | `[ ACTION_EXEC_PAR, TOK_SPACE, PIPELINE($1) ]` |
| `Def_Pipeline` | `{-} -Transform` | `^\{-\} +-(DotPath)` | `[ DEF_PIPELINE, TOK_SPACE, PIPELINE($1) ]` |
| `Def_Data` | `{#} #Config` | `^\{#\} +#(DotPath)` | `[ DEF_DATA, TOK_SPACE, DATA($1) ]` |
| `Data_Load` | `[#] $hire << #p` | `^\[#\] +\$(FlexPath) +<< +#(DotPath)` | `[ ACTION_DATA_LOAD, TOK_SPACE, VARIABLE($1), TOK_SPACE, OP_PUSH_LEFT, TOK_SPACE, DATA($2) ]` |
| `IO_Pipeline_Input` | `(-) <user << $u`| `^\(-\) +<(DotPath) +<< +\$(FlexPath)`| `[ IO_PIPELINE, TOK_SPACE, INPUT_PARAMETER($1), TOK_SPACE, OP_PUSH_LEFT, TOK_SPACE, VARIABLE($2) ]` |
| `IO_Pipeline_Out` | `(-) >> $user` | `^\(-\) +>> +\$(FlexPath)` | `[ IO_PIPELINE, TOK_SPACE, OP_PUSH_RIGHT, TOK_SPACE, VARIABLE($1) ]` |
| `IO_Fallback_Out` | `(>) !> $log` | `^\(>\) +!> +\$(FlexPath)` | `[ IO_PARAM_OUT_FALLBACK, TOK_SPACE, OP_FALLBACK_PUSH_RIGHT, TOK_SPACE, VARIABLE($1) ]` |
| `IO_Fallback_In` | `(<) !< #Def` | `^\(<\) +!< +#(DotPath)` | `[ IO_PARAM_IN_FALLBACK, TOK_SPACE, OP_FALLBACK_PUSH_LEFT, TOK_SPACE, DATA($1) ]` |
| `Assignment` | `$ip << #Local` | `^\$(FlexPath) +<< +#(DotPath)` | `[ VARIABLE($1), TOK_SPACE, OP_PUSH_LEFT, TOK_SPACE, DATA($2) ]` |
| `Typed_Variable` | `$db#Mongo` | `^\$(FlexPath)#(DotPath)` | `[ VARIABLE($1), DATATYPE($2) ]` |
| `Raise_Error_Macro` | `>> !Conn.Timeout` | `^>> +!(DotPath)` | `[ OP_PUSH_RIGHT, TOK_SPACE, ERROR($1) ]` |
| `Isolated_Terminal` | ` ###Value` | `^(?:^\|(?<=\s))###(DotPath)` | `[ TERMINAL_DATA($1) ]` |
| `Standalone_Variable` | `$cache:timeout` | `^\$(FlexPath)` | `[ VARIABLE($1) ]` |
