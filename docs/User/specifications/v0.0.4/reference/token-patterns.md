---
last-redoc-date: 2025-12-18
---

# Complete Token List and Patterns for Parser Implementation

**Version**: v0.0.4
**Total Tokens**: 121 types
**Purpose**: Comprehensive reference for parser developers

---

## Table of Contents

1. [Token Inventory by Category](#token-inventory-by-category)
2. [Token Sequence Patterns](#token-sequence-patterns)
3. [Pattern Recognition Rules](#pattern-recognition-rules)
4. [AST Node Mapping](#ast-node-mapping)

---

## Token Inventory by Category

### Block Markers (30 tokens)

| Token | Lexeme | Purpose | Context |
|-------|--------|---------|---------|
| `BlockPackageStart` | `[@]` | Package definition start | Definition block |
| `BlockVersionEnum` | `[#]` | Version/enum definition | Definition block |
| `BlockEnd` | `[X]` | Block end marker | Any block |
| `BlockPipelineStart` | `[|]` | Pipeline section marker | Definition/Invocation |
| `BlockInput` | `[i]` | Input block | Pipeline definition |
| `BlockTrigger` | `[t]` | Trigger block | Pipeline definition |
| `BlockQueue` | `[Q]` | Queue block | Pipeline definition |
| `BlockWrapper` | `[W]` | Wrapper block | Pipeline definition |
| `BlockSetup` | `[\]` | Setup block | Pipeline definition |
| `BlockCleanup` | `[/]` | Cleanup block | Pipeline definition |
| `BlockOutput` | `[o]` | Output block | Pipeline definition |
| `BlockSequential` | `[r]` | Sequential execution | Execution context |
| `BlockInputBinding` | `[<]` | Input binding marker | Pipeline invocation |
| `BlockOutputBinding` | `[>]` | Output binding marker | Pipeline invocation |
| `BlockParallel` | `[p]` | Parallel execution | Execution context |
| `BlockJoin` | `[v]` | Join marker | Parallel context |
| `BlockBackground` | `[b]` | Background execution | Execution context |
| `BlockSerialLoad` | `[s]` | Serial file load | File I/O context |
| `BlockErrorCatch` | `[!]` | Error catch block | Error handling |
| `BlockConditional` | `[?]` | Conditional marker | Control flow |
| `BlockBody` | `[~]` | Body marker | Loop/conditional |
| `BlockBoolOr` | `[+]` | Boolean OR | Boolean logic |
| `BlockBoolAnd` | `[&]` | Boolean AND | Boolean logic |
| `BlockBoolNot` | `[-]` | Boolean NOT | Boolean logic |
| `BlockBoolXor` | `[^]` | Boolean XOR | Boolean logic |
| `BlockSubfield` | `[.]` | Subfield definition | Struct/enum context |
| `BlockLineContinuation` | `[*]` | Line continuation | Multi-line expressions |
| `BlockMacroDefinition` | `[M]` | Macro definition | Definition block |
| `BlockScopeInput` | `[{]` | Scope input | Scope definition |
| `BlockScopeOutput` | `[}]` | Scope output | Scope definition |
| `BlockAliasDefinition` | `[A]` | Alias definition | Definition block |

### Assignment/Push Operators (6 tokens)

| Token | Lexeme | Associativity | Precedence | Purpose |
|-------|--------|---------------|------------|---------|
| `OpPushLeft` | `<<` | Right | 9 | Push value to left target |
| `OpPushRight` | `>>` | Right | 9 | Push value to right target |
| `OpDefaultPushLeft` | `<~` | Right | 11 (Lowest) | Default push left |
| `OpDefaultPushRight` | `~>` | Right | 11 (Lowest) | Default push right |
| `OpVariadicPushLeft` | `<<<` | Right | 10 | Variadic push left |
| `OpVariadicPushRight` | `>>>` | Right | 10 | Variadic push right |

### Comparison Operators (11 tokens)

| Token | Lexeme | Associativity | Precedence | Negated Version |
|-------|--------|---------------|------------|-----------------|
| `OpEqual` | `=?` | Left | 5 | `=!?` (OpNotEqual) |
| `OpNotEqual` | `=!?` | Left | 5 | - |
| `OpGreater` | `>?` | Left | 6 | `>!?` (OpNotGreater) |
| `OpNotGreater` | `>!?` | Left | 6 | - |
| `OpLess` | `<?` | Left | 6 | `<!?` (OpNotLess) |
| `OpNotLess` | `<!?` | Left | 6 | - |
| `OpGreaterEqual` | `>=?` | Left | 7 | `>=!?` (OpNotGreaterEqual) |
| `OpNotGreaterEqual` | `>=!?` | Left | 7 | - |
| `OpLessEqual` | `<=?` | Left | 7 | `<=!?` (OpNotLessEqual) |
| `OpNotLessEqual` | `<=!?` | Left | 7 | - |

### Pattern Matching Operators (5 tokens)

| Token | Lexeme | Associativity | Precedence | Purpose |
|-------|--------|---------------|------------|---------|
| `OpWildcard` | `*?` | Left | 4 | Wildcard pattern match |
| `OpRegex` | `re?` | Left | 3 | Regex match |
| `OpNotRegex` | `re!?` | Left | 3 | Negated regex match |
| `OpInCollection` | `in?` | Left | 2 | Collection membership |
| `OpNotInCollection` | `in!?` | Left | 2 | Negated membership |

### Range Operators (4 tokens)

| Token | Lexeme | Purpose | Inclusive? |
|-------|--------|---------|------------|
| `OpRangeClosed` | `?[` | Closed range | Both ends inclusive |
| `OpRangeOpen` | `?(` | Open range | Both ends exclusive |
| `OpRangeHalfRight` | `?]` | Half-right range | Right inclusive |
| `OpRangeHalfLeft` | `?)` | Half-left range | Left inclusive |

### Other Operators (2 tokens)

| Token | Lexeme | Associativity | Precedence | Purpose |
|-------|--------|---------------|------------|---------|
| `OpPipelineCompose` | `|>` | Left | 1 (Highest) | Pipeline composition |
| `OpStringConcat` | `+"` | Left | 8 | String concatenation |

### Delimiters (15 tokens)

| Token | Lexeme | Purpose | Paired With |
|-------|--------|---------|-------------|
| `DelimiterBraceOpen` | `{` | Open brace | `}` |
| `DelimiterBraceClose` | `}` | Close brace | `{` |
| `DelimiterParenOpen` | `(` | Open paren | `)` |
| `DelimiterParenClose` | `)` | Close paren | `(` |
| `DelimiterSquareBracketClose` | `]` | Close bracket | Range operators |
| `DelimiterQuote` | `"` | Quote | String literals |
| `DelimiterComma` | `,` | Comma | List separator |
| `DelimiterColon` | `:` | Colon | Type annotation |
| `DelimiterSemicolon` | `;` | Semicolon | Reserved indication |
| `DelimiterAt` | `@` | At sign | Package specs |
| `DelimiterBackslash` | `\` | Backslash | Escape |
| `DelimiterPipe` | `|` | Pipe | Explicit blocks |
| `DelimiterDot` | `.` | Dot | Hierarchy separator |
| `DelimiterInputPrefix` | `<` | Input prefix | Standalone |
| `DelimiterOutputPrefix` | `>` | Output prefix | Standalone |

### String Literal Tokens (6 tokens)

| Token | Purpose | Emitted When |
|-------|---------|--------------|
| `StringStart` | Opening quote | `"` encountered |
| `StringContent` | Static text | Between interpolations |
| `StringEnd` | Closing quote | Terminating `"` |
| `InterpolationStart` | Begin interpolation | `{` inside string |
| `InterpolationEnd` | End interpolation | `}` inside string |
| `FormatIdentifier` | Format specifier | After `:` in interpolation |

### Identifiers (11 categories)

| Token | Prefix | Example | Hierarchy Support |
|-------|--------|---------|-------------------|
| `IdentifierVariable` | `$` | `$userName` | Yes (`.`) |
| `IdentifierEnum` | `#` | `#Status.Active` | Yes (`.` and `;`) |
| `IdentifierPipeline` | `|` | `|U.Log.Info` | Yes (`.`) |
| `IdentifierError` | `!` | `!Network.Timeout` | Yes (`.` and `;`) |
| `IdentifierUnpack` | `~` | `~ForEach.Array` | Yes (`.`) |
| `IdentifierJoin` | `~Y.` | `~Y.MergeResults` | Yes (`.`) |
| `IdentifierInputArgument` | `<` | `<input1` | Yes (`.`) |
| `IdentifierOutputArgument` | `>` | `>result` | Yes (`.`) |
| `IdentifierDataType` | `:` | `:pg.string` | Yes (`.`) |
| `IdentifierPackageSpec` | `@` | `@Local::MyApp:1.0.0` | Special (`::` and `:`) |
| `IdentifierMetadata` | `%` | `%Doc` | Yes (`.`) |
| `Identifier` | None | `fieldName` | No (DEPRECATED) |

### Reserved Enumerations (10 tokens)

| Token | Lexeme | Category | Purpose |
|-------|--------|----------|---------|
| `ReservedPgVarDeclared` | `#PgVar.States.Declared` | Variable state | Variable declared |
| `ReservedPgVarDefaultReady` | `#PgVar.States.DefaultReady` | Variable state | Default value ready |
| `ReservedPgVarPending` | `#PgVar.States.Pending` | Variable state | Awaiting value |
| `ReservedPgVarReady` | `#PgVar.States.Ready` | Variable state | Value ready |
| `ReservedPgVarFaulted` | `#PgVar.States.Faulted` | Variable state | Error state |
| `ReservedBooleanTrue` | `#Boolean.True` | Boolean | True value |
| `ReservedBooleanFalse` | `#Boolean.False` | Boolean | False value |
| `ReservedNone` | `#None` | Special | Null/None value |
| `ReservedPipelineNoInput` | `#Pipeline.NoInput` | Pipeline state | No input received |
| `ReservedNoError` | `!NoError` | Error | Success/no error |

### Literals (6 types)

| Token | Example | Pattern |
|-------|---------|---------|
| `LiteralInteger` | `42`, `-10` | `[+-]?[0-9]+` |
| `LiteralFloat` | `3.14`, `-0.5` | `[+-]?[0-9]+\.[0-9]+` |
| `LiteralDatetime` | `DT"2024-01-15T14:30:00Z"` | `DT"..."` |
| `LiteralDuration` | `DT.Minutes"5"` | `DT.UNIT"..."` |
| `LiteralCollection` | `{1, 2, 3}` | `{expr, ...}` |
| `LiteralPipelineFormatted` | `|Pipe"{$var}"` | `|IDENT"..."` |

### Special Identifiers (5 types)

| Token | Prefix | Example | Purpose |
|-------|--------|---------|---------|
| `SpecialDatetime` | `DT.` | `DT.Now` | Datetime operations |
| `SpecialRuntime` | `RT.` | `RT.Python` | Runtime wrappers |
| `SpecialTrigger` | `TG.` | `TG.HTTP` | Trigger types |
| `SpecialTriggerType` | `|T.` | `|T.File.Watch` | Trigger pipelines |
| `SpecialWrapper` | `|W.` | `|W.Polyglot.Scope` | Wrapper pipelines |

### Comments (2 tokens)

| Token | Syntax | Skipped by Lexer? |
|-------|--------|-------------------|
| `CommentSingle` | `// comment` | Yes |
| `CommentMulti` | `/* comment */` | Yes |

### Whitespace (2 tokens)

| Token | Characters | Skipped by Lexer? |
|-------|------------|-------------------|
| `Newline` | `\n` | No (significant!) |
| `Whitespace` | Space, Tab, `\r` | Yes |

### Control Tokens (2 tokens)

| Token | Purpose |
|-------|---------|
| `Version` | Version numbers (e.g., `1.0.0`) |
| `Eof` | End of file marker |

---

## Token Sequence Patterns

### Pattern 1: Variable Assignment

```
Sequence: BlockSequential IdentifierVariable OpPushLeft Expression
Example:  [r] $userName << "Alice"
Tokens:   BlockSequential, IdentifierVariable("$userName"), OpPushLeft, StringStart, StringContent("Alice"), StringEnd
AST:      AssignmentStatement { target: Variable("userName"), operator: PushLeft, value: StringLiteral("Alice") }
```

### Pattern 2: Pipeline Definition

```
Sequence: DelimiterBraceOpen DelimiterPipe DelimiterBraceClose IdentifierPipeline
          BlockPipelineStart IdentifierInputArgument IdentifierDataType
          ...
          BlockPipelineStart IdentifierOutputArgument IdentifierDataType OpPushLeft Expression
          DelimiterBraceOpen Identifier DelimiterBraceClose

Example:  {|} |ProcessUser
          [|] <userId :pg.int
          [|] >userName :pg.string << $result
          {x}

Tokens:
  DelimiterBraceOpen("{"), DelimiterPipe, DelimiterBraceClose("}"), IdentifierPipeline("|ProcessUser"), Newline,
  BlockPipelineStart("[|]"), IdentifierInputArgument("<userId"), IdentifierDataType(":pg.int"), Newline,
  BlockPipelineStart("[|]"), IdentifierOutputArgument(">userName"), IdentifierDataType(":pg.string"),
    OpPushLeft, IdentifierVariable("$result"), Newline,
  DelimiterBraceOpen("{"), Identifier("x"), DelimiterBraceClose("}")

AST:      PipelineDefinition {
            name: "ProcessUser",
            inputs: [Parameter { name: "userId", type: "pg.int" }],
            outputs: [Parameter { name: "userName", type: "pg.string", expression: Variable("result") }]
          }
```

### Pattern 3: Pipeline Invocation

```
Sequence: BlockSequential IdentifierPipeline
          BlockPipelineStart IdentifierInputArgument OpPushLeft Expression
          BlockPipelineStart IdentifierOutputArgument OpPushRight IdentifierVariable

Example:  [r] |ProcessUser
          [|] <userId << 123
          [|] >userName >> $name

Tokens:
  BlockSequential, IdentifierPipeline("|ProcessUser"), Newline,
  BlockPipelineStart, IdentifierInputArgument("<userId"), OpPushLeft, LiteralInteger("123"), Newline,
  BlockPipelineStart, IdentifierOutputArgument(">userName"), OpPushRight, IdentifierVariable("$name")

AST:      PipelineCall {
            pipeline: "ProcessUser",
            input_bindings: [Binding { param: "userId", value: IntegerLiteral(123) }],
            output_bindings: [Binding { param: "userName", target: Variable("name") }]
          }
```

### Pattern 4: Reserved Indication

```
Sequence: IdentifierEnum DelimiterSemicolon Identifier DelimiterSemicolon Identifier

Example:  #Boolean.True

Tokens:   IdentifierEnum("#"), DelimiterSemicolon, Identifier("Boolean"), DelimiterSemicolon, Identifier("True")

Parsing:
  1. See IdentifierEnum("#")
  2. Lookahead: next is DelimiterSemicolon → this is reserved indication
  3. Consume segments alternating DelimiterSemicolon and Identifier
  4. Each segment after ; is marked as "reserved"

AST:      HierarchicalIdentifier {
            prefix: "#",
            segments: [
              Segment { name: "Boolean", reserved: true },
              Segment { name: "True", reserved: true }
            ]
          }
```

### Pattern 5: Mixed Reserved/Custom Hierarchy

```
Sequence: IdentifierEnum DelimiterSemicolon Identifier DelimiterSemicolon Identifier
          DelimiterDot Identifier DelimiterSemicolon Identifier

Example:  #DT.Business;Week.CustomWeek;RestDays

Tokens:
  IdentifierEnum("#"), DelimiterSemicolon, Identifier("DT"), DelimiterSemicolon, Identifier("Business"),
  DelimiterSemicolon, Identifier("Week"), DelimiterDot, Identifier("CustomWeek"),
  DelimiterSemicolon, Identifier("RestDays")

Parsing:
  Track current delimiter:
  - After ; → next segment is reserved
  - After . → next segment is custom

AST:      HierarchicalIdentifier {
            prefix: "#",
            segments: [
              Segment { name: "DT", reserved: true },
              Segment { name: "Business", reserved: true },
              Segment { name: "Week", reserved: true },
              Segment { name: "CustomWeek", reserved: false },  // after .
              Segment { name: "RestDays", reserved: true }      // after ;
            ]
          }
```

### Pattern 6: Inline Pipeline

```
Sequence: IdentifierPipeline StringStart ... StringEnd

Example:  |FormatName"{$first} {$last}"

Tokens:
  IdentifierPipeline("|FormatName"), StringStart, StringContent(" "), InterpolationStart,
  IdentifierVariable("$first"), InterpolationEnd, StringContent(" "), InterpolationStart,
  IdentifierVariable("$last"), InterpolationEnd, StringEnd

Recognition: PipelineIdentifier IMMEDIATELY followed by StringStart (no whitespace between tokens)

AST:      InlinePipelineCall {
            pipeline: "FormatName",
            format_string: "{$first} {$last}",
            arguments: [Variable("first"), Variable("last")]
          }
```

### Pattern 7: Conditional Expression

```
Sequence: BlockConditional Expression OpEqual IdentifierEnum

Example:  [?] $status =? #Active

Tokens:   BlockConditional, IdentifierVariable("$status"), OpEqual, IdentifierEnum("#Active")

AST:      Conditional {
            marker: BlockConditional,
            condition: ComparisonExpression {
              left: Variable("status"),
              operator: Equal,
              right: EnumValue("Active")
            }
          }
```

### Pattern 8: Match/Select Pattern

```
Sequence: BlockMatch Expression
          (Newline Indentation BlockConditional Expression OpConditional Expression)+
          Newline Indentation BlockConditional OpWildcard OpConditional Expression

Example:  [m] $value
             [?] 1 ? #Small
             [?] 10 ? #Medium
             [?] * ? #Large

Tokens:
  BlockMatch, IdentifierVariable("$value"), Newline,
  (3 spaces), BlockConditional, LiteralInteger("1"), (? operator - wait, how is ? tokenized?), IdentifierEnum("#Small"), Newline,
  (3 spaces), BlockConditional, LiteralInteger("10"), (?), IdentifierEnum("#Medium"), Newline,
  (3 spaces), BlockConditional, OpWildcard, (?), IdentifierEnum("#Large")

Note: Need clarification on conditional operator tokenization

AST:      MatchExpression {
            value: Variable("value"),
            cases: [
              MatchCase { pattern: IntegerLiteral(1), result: EnumValue("Small") },
              MatchCase { pattern: IntegerLiteral(10), result: EnumValue("Medium") },
              MatchCase { pattern: Wildcard, result: EnumValue("Large") }
            ]
          }
```

### Pattern 9: Loop with Unpack

```
Sequence: BlockSequential IdentifierUnpack
          BlockBody IdentifierInputArgument OpPushLeft Expression
          BlockBody IdentifierOutputArgument OpPushRight IdentifierVariable
          (Newline Indentation BlockSequential ...)*

Example:  [r] ~ForEach.Array
          [~] <array << $items
          [~] >item >> $currentItem
             [r] $processed << |Transform <data << $currentItem

Tokens:
  BlockSequential, IdentifierUnpack("~ForEach.Array"), Newline,
  BlockBody, IdentifierInputArgument("<array"), OpPushLeft, IdentifierVariable("$items"), Newline,
  BlockBody, IdentifierOutputArgument(">item"), OpPushRight, IdentifierVariable("$currentItem"), Newline,
  (3 spaces), BlockSequential, IdentifierVariable("$processed"), OpPushLeft, ...

AST:      UnpackLoop {
            operator: "ForEach.Array",
            input_bindings: [Binding { param: "array", value: Variable("items") }],
            output_bindings: [Binding { param: "item", target: Variable("currentItem") }],
            body: [AssignmentStatement { ... }]
          }
```

### Pattern 10: Type Annotation

```
Sequence: IdentifierVariable IdentifierDataType

Example:  $count :pg.int

Tokens:   IdentifierVariable("$count"), IdentifierDataType(":pg.int")

AST:      VariableDeclaration {
            name: "count",
            type: TypeAnnotation { namespace: "pg", type: "int" }
          }
```

---

## Pattern Recognition Rules

### Rule 1: Definition Block Detection

```
Trigger: DelimiterBraceOpen followed by (DelimiterPipe | IdentifierEnum | IdentifierError | DelimiterAt)

Pattern: { PREFIX } ... { x }

Action:
  1. Set parser state to IN_DEFINITION_BLOCK
  2. Track definition type from prefix (|, #, !, @)
  3. Parse components using [PREFIX] markers
  4. Exit state when { x } encountered
```

### Rule 2: Reserved Indication Detection

```
Trigger: (IdentifierEnum | IdentifierError) followed by DelimiterSemicolon

Action:
  1. Mark this as reserved indication path
  2. Collect segments alternating Semicolon/Dot and Identifier
  3. Track delimiter for each segment:
     - After ; → reserved
     - After . → custom
  4. Stop when non-delimiter/non-identifier token encountered
```

### Rule 3: Inline Pipeline Detection

```
Trigger: IdentifierPipeline at position N, StringStart at position N+1 (no Whitespace/Newline between)

Action:
  1. Mark as inline pipeline call
  2. Parse formatted string
  3. Extract interpolated variables as arguments
  4. Create InlinePipelineCall AST node
```

### Rule 4: Indentation-Based Nesting

```
Trigger: Newline followed by Whitespace (count must be multiple of 3)

Action:
  1. Count leading spaces
  2. Divide by 3 to get nesting level
  3. If level > current_level: push new scope
  4. If level < current_level: pop scopes until level matches
  5. If level == current_level: sibling node
```

### Rule 5: Invocation vs Definition Context

```
Parser State: Track current context

IN_DEFINITION_BLOCK:
  - [|] means "declare parameter/output of pipeline"
  - Parse as parameter declaration

AFTER_INVOCATION:
  - Set when BlockSequential/BlockParallel/etc. followed by IdentifierPipeline
  - [|] means "bind value to/from parameter"
  - Parse as argument binding

NORMAL:
  - Default state
  - Most tokens interpreted in standard way
```

---

## AST Node Mapping

### Statements

| Token Pattern | AST Node | Example |
|---------------|----------|---------|
| `[r] $var << expr` | `AssignmentStatement` | `[r] $x << 5` |
| `[r] \|Pipeline ...` | `PipelineCall` | `[r] \|Process ...` |
| `{|} \|Name ...` | `PipelineDefinition` | `{|} \|MyPipeline ...` |
| `[?] expr op expr` | `ConditionalStatement` | `[?] $x >? 5` |
| `[m] expr ...` | `MatchExpression` | `[m] $status ...` |
| `[!] * ? expr` | `ErrorCatch` | `[!] * ? \|Handler` |

### Expressions

| Token Pattern | AST Node | Example |
|---------------|----------|---------|
| `LiteralInteger` | `IntegerLiteral` | `42` |
| `LiteralFloat` | `FloatLiteral` | `3.14` |
| `IdentifierVariable` | `VariableReference` | `$userName` |
| `IdentifierEnum` | `EnumReference` | `#Status.Active` |
| `expr op expr` | `BinaryExpression` | `$x >? 5` |
| `{ expr, expr }` | `CollectionLiteral` | `{1, 2, 3}` |
| `\|Pipe"{$var}"` | `InlinePipelineCall` | `\|Format"{$name}"` |

### Type Annotations

| Token Pattern | AST Node | Example |
|---------------|----------|---------|
| `:pg.type` | `TypeAnnotation` | `:pg.string` |
| `$var :type` | `TypedVariable` | `$count :pg.int` |

### Hierarchical Identifiers

| Token Pattern | AST Node | Example |
|---------------|----------|---------|
| `# . ident . ident` | `HierarchicalEnum` (custom) | `#MyEnum.Field1.Field2` |
| `# ; ident ; ident` | `HierarchicalEnum` (reserved) | `#Boolean.True` |
| `# ; ident . ident ; ident` | `HierarchicalEnum` (mixed) | `#DT.Week.Custom;Field` |

---

## Quick Reference Tables

### Operator Precedence (Complete)

| Level | Operators | Associativity | Tokens |
|-------|-----------|---------------|--------|
| 1 | `|>` | Left | `OpPipelineCompose` |
| 2 | `in?`, `in!?` | Left | `OpInCollection`, `OpNotInCollection` |
| 3 | `re?`, `re!?` | Left | `OpRegex`, `OpNotRegex` |
| 4 | `*?` | Left | `OpWildcard` |
| 5 | `=?`, `=!?` | Left | `OpEqual`, `OpNotEqual` |
| 6 | `>?`, `>!?`, `<?`, `<!?` | Left | `OpGreater`, `OpNotGreater`, `OpLess`, `OpNotLess` |
| 7 | `>=?`, `>=!?`, `<=?`, `<=!?` | Left | `OpGreaterEqual`, `OpNotGreaterEqual`, `OpLessEqual`, `OpNotLessEqual` |
| 8 | `+"` | Left | `OpStringConcat` |
| 9 | `<<`, `>>` | Right | `OpPushLeft`, `OpPushRight` |
| 10 | `<<<`, `>>>` | Right | `OpVariadicPushLeft`, `OpVariadicPushRight` |
| 11 | `<~`, `~>` | Right | `OpDefaultPushLeft`, `OpDefaultPushRight` |

### Token Categories Summary

| Category | Count | Purpose |
|----------|-------|---------|
| Block Markers | 30 | Execution control, structure definition |
| Operators | 27 | Expressions, assignments, comparisons |
| Delimiters | 15 | Syntax structure, separators |
| Identifiers | 11 | Named references with type prefixes |
| Reserved Values | 10 | Built-in enum/error values |
| Literals | 6 | Constant values |
| String Tokens | 6 | String literal parsing |
| Special Identifiers | 5 | System-level constructs |
| Comments | 2 | Documentation (skipped) |
| Whitespace | 2 | Formatting (Newline significant!) |
| Control | 2 | Version, EOF |
| **Total** | **121** | **Complete token set** |

---

## Implementation Checklist

**For Parser Developers**, ensure you handle:

- ✅ All 121 token types
- ✅ Operator precedence (11 levels)
- ✅ Context-dependent marker interpretation (definition vs invocation)
- ✅ Reserved indication (`;` vs `.` in hierarchies)
- ✅ Inline pipeline detection (IdentifierPipeline + StringStart)
- ✅ Indentation tracking (3-space units)
- ✅ Definition block state machine (`{X}...{x}`)
- ✅ String interpolation parsing (nested expressions in strings)
- ✅ Error recovery at block boundaries
- ✅ Type inference for untyped variables

---

**Last Updated**: 2025-12-18
**Source**: `polyglot-lexer/src/token.rs`
**Lexer Tests**: 45/45 passing ✅
