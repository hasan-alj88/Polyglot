# Aljam3 Syntax Highlighting Test for Markdown

This document tests syntax highlighting of Aljam3 code blocks in markdown.

## Basic Pipeline

Here's a simple pipeline:

```aljam3
[@] @example/hello-world

[|] |HelloWorld
  [i] !No.Input
  [t] |T.Manual

  [r] .message: pg\string << "Hello, World!"

  [o] .message: pg\string
[X]
```

## Pipeline with Input/Output

Example with variables:

```aljam3
[|] |ProcessData
  [i] .input: pg\string
  [i] .count: pg\int <~ 10
  [t] |T.Manual

  [r] .result: pg\string << .input
  [r] .length: pg\int << .result

  [o] .result: pg\string
  [o] .length: pg\int
[X]
```

## Error Handling

Demonstrating error definitions and catch blocks:

```aljam3
[!] !ValidationError
  .message: pg\string
  .code: pg\int
  .trace: pg\string
[X]

[|] |SafeProcess
  [i] .data: pg\string
  [t] |T.Manual

  [r] .result: pg\string << .data

  [~][!] !ValidationError
    [r] .result << "fallback"
  [X]

  [o] .result: pg\string
[X]
```

## Operators Showcase

All the different operators:

```aljam3
[|] |Operators
  [i] .a: pg\int
  [i] .b: pg\int
  [t] |T.Manual

  // Comparison
  [r] .equal: pg\bool << (.a =? .b)
  [r] .greater: pg\bool << (.a >? .b)

  // Range
  [r] .in_range: pg\bool << .a ?[10, 20]

  // Data flow
  [r] .push: pg\int << 42      // PUSH
  [r] .pull: pg\int >> .a      // PULL
  [r] .default: pg\int <~ 100  // DEFAULT

  [o] !No.Output
[X]
```

## DateTime System

Using datetime literals and triggers:

```aljam3
[|] |ScheduledTask
  [i] !No.Input
  [t] |T.DT.Daily
    .time: pg\string << "09:00"
  [X]

  [r] .now: pg\datetime << DT"now"
  [r] .specific: pg\datetime << DT"2025-12-03T10:30:00Z"

  [o] .now: pg\datetime
[X]
```

## Multi-Language Types

Different namespace types:

```aljam3
[|] |MultiLang
  [i] !No.Input
  [t] |T.Manual

  // Aljam3 native
  [r] .jm3_str: pg\string << "hello"
  [r] .jm3_int: pg\int << 42

  // Rust types
  [r] .rs_vec: rs\Vec{i32} << [1, 2, 3]

  // Python types
  [r] .py_list: py\list << [1, 2, 3]
  [r] .py_dict: py\dict << {"key": "value"}

  // JavaScript types
  [r] .js_array: js\Array << [1, 2, 3]

  // Go types
  [r] .go_slice: go\[]int << [1, 2, 3]

  [o] !No.Output
[X]
```

## Parallel Execution and Join Points

Processing collections:

```aljam3
[|] |ProcessBatch
  [i] .items: pg\array{pg\string}
  [t] |T.Manual

  [Y] ~.items
    [p] .processed: pg\string << .items
  [X]

  [r] .results: pg\array{pg\string} << ~Y.*

  [o] .results: pg\array{pg\string}
[X]
```

## Enumerations

Custom and reserved enums:

```aljam3
[#] #Status
  .Pending
  .Processing
  .Completed
  .Failed
[X]

[|] |UseEnum
  [i] !No.Input
  [t] |T.Manual

  [r] .status: #Status << #Status.Pending
  [r] .bool_val: pg\bool << #Boolean.True
  [r] .state: pg\string << #PgVar.States.Ready

  [o] .status: #Status
[X]
```

## Nested Scopes and Conditionals

Complex logic with nesting:

```aljam3
[|] |Nested
  [i] .value: pg\int
  [t] |T.Manual

  [r] .category: pg\string

  [~]
    [?] .value >? 100
      [r] .category << "high"
    [X]
    [?] (.value >=? 50) [&] (.value <=? 100)
      [r] .category << "medium"
    [X]
    [?] .value <? 50
      [r] .category << "low"
    [X]
  [X]

  [o] .category: pg\string
[X]
```

## Testing with `pg` Identifier

Using shorter identifier:

```pg
[|] |ShortIdentifier
  [i] .input: pg\string
  [t] |T.Manual

  [r] .output: pg\string << .input

  [o] .output: pg\string
[X]
```

## Inline Code

Inline syntax elements: `[|]` pipeline marker, `.variable` identifier, `<<` push operator, `#Boolean.True` enumeration, `!No.Input` error marker, `DT"2025-12-03"` datetime literal.

## Summary

All major Aljam3 syntax elements should be highlighted:

- **Block markers**: `[@]`, `[|]`, `[#]`, `[!]`, `[M]`, `[X]`, `[i]`, `[o]`, `[r]`, `[p]`, `[?]`, `[t]`, `[~]`, etc.
- **Operators**: `<<`, `>>`, `<~`, `=?`, `>?`, `?[`, `~*`, etc.
- **Identifiers**: `.variable`, `|Pipeline`, `#Enum`, `!Error`, `@package`
- **Types**: `pg\string`, `rs\Vec`, `py\dict`, `js\Array`, `go\[]int`
- **Literals**: `"string"`, `DT"datetime"`, `42`, `3.14`
- **Reserved**: `#Boolean.True`, `#None`, `#PgVar.States.Ready`, `!No.Input`, `!No.Output`
- **Comments**: `// comment`
