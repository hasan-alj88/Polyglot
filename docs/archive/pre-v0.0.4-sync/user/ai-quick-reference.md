---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/ai-quick-reference.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# AI Quick Reference

Ultra-compressed Polyglot reference for AI code generation. Maximum density, minimum fluff.

## Core Syntax Rules

| Rule | Description |
|------|-------------|
| **Dot prefix** | ALL variables start with `.` (e.g., `.myvar` |
| **Case sensitive** | `.Name` ≠ `.name` |
| **PFG formatting** | 3 blank lines before `{&#124;}`, `{W}`, `{#}` definitions |
| **Type separator** | Dot `.` for foreign types: `py.dict`, `rs.Vec` |
| **Block markers** | `[X]` format for all blocks |
| **No semicolons** | Lines don't end with `;` |
| **Comments** | `//` single-line, `/* */` multi-line |

## Block Markers

### Registry Markers (Define Operators)

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `{@}` | Registry | Registry/package declaration (must start file) | `{@} @Local::Examples.MyApp:1.0.0.0` |
| `{&#124;}` | Pipeline | Define pipeline operator `&#124;` | `{&#124;} &#124;Pipeline.MyPipeline` |
| `{#}` | Enumeration | Define enumeration operator `#` | `{#} #Status` |
| `{!}` | Error | Define error operator `!` | `{!} !Error` |
| `{W}` | Wrapper | Define wrapper operator `&#124;W` | `{W} &#124;W.RT.Python3.14` |

**Note:** All registry markers close with `{x}`: `{@}...{x}`, `{|}...{x}`, `{#}...{x}`, `{!}...{x}`, `{W}...{x}`

**Registry Declaration Example:**
```polyglot
// Must start with registry declaration
{@} @Local::Examples.MyApp:1.0.0.0
[A] @MyApp
[.] @DataProcess << @Community.user123::DataProcessing:2.0.2.1
[.] @Stats << @Community.user143::Statistics:latest
{x}
```

### I/O Markers

| Marker | Parent | Name | Purpose | Example |
|--------|--------|------|---------|---------|
| `[<}` | `{&#124;}` | Input | Define pipeline input parameter | `[<} .param:pg.string` |
| `[>}` | `{&#124;}` | Output | Define pipeline output parameter | `[>} .result:pg.int` |
| `\<\` | Any* | Pass-In | Pass input to predefined pipeline | `\<\ <array << .source` |
| `\>\` | Any* | Pass-Out | Pass output from predefined pipeline | `\>\ >result >> .target` |
| `[{]` | `{W}` | Scope-In | Wrapper scope input | `[{] .input:pg.int` |
| `[}]` | `{W}` | Scope-Out | Wrapper scope output | `[}] .output` |

*Any marker with predefined pipeline call

### Execution Markers

**Context:** Found in `{|}` between `[\]` and `[/]`, or between `[W]` and closing `{x}`

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `[r]` | Sequential | Sequential execution block | `[r] @Lib&#124;Pipeline` |
| `[p]` | Parallel | Parallel execution block | `[p] ~ForEach` |
| `[v]` | Join | Join parallel variables from `[p]` (merge ∨) | `[v] ~V.JoinFirst` |
| `[b]` | Background | Fire-and-forget execution | `[b] &#124;U.Log.Info"{.msg}"` |
| `[y]` | Fork-Switch | Conditional fork (Y branch) | `[y] .age <? 18` |
| `[s]` | Serial | Serial file data load for push/pull | `[s] .data << &#124;YAML.Load"\\FileDir\\data.yaml"` |

**Note:** `[s]` can be used in `{|}` for push/pull assignment, or in `{#}` and `{!}` to load data from files into fields.

**Example - Loading data into enumeration:**
```polyglot
{#}
[.] .service_name:pg.string << "MyAppService"
[s] |JSON.Load"\\FileDir\\data.json"
\.\ >> .api_key:pg.string
\.\ >> .url:pg.url
\.\ ~> .username:pg.string  // default assignment from file
{x}
```

**Example - Loading data in pipeline:**
```polyglot
{|}
[s] .data:pg.serial << |TOML.Load"\\FileDir\\data.toml"
{x}
```

### Structure Markers

| Marker | Parent | Name | Purpose | Example |
|--------|--------|------|---------|---------|
| `[.]` | `{#}`, `{!}`, `{@}` | Field | Field definition or dependency import | `[.] .field:pg.string` or `[.] @Dependency` |
| `\.\` | `[s]` | Load-Field | Load field from serial data | `\.\ >> .api_key:pg.string` |

### Control Flow Markers

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `[t]` | Trigger | Pipeline trigger | `[t] &#124;T.Daily"2AM"` |
| `[Q]` | Queue | Queue control | `[Q] &#124;Q.Assign.Priority"10"` |
| `[!]` | Error-Handler | Error handler block | `[!] .error_handler` |
| `[A]` | Alias | Create alias for registry entry | `[A] #NewName` |

**`[A]` Alias usage by registry type:**
- `{@}` → `[A] @NewName` (alias for current registry)
- `{#}` → `[A] #NewName` (enumeration alias)
- `{|}` → `[A] |NewName` (pipeline alias)
- `{!}` → `[A] !NewName` (error alias)
- `{W}` → `[A] |W.NewName` (wrapper alias)

### Boolean & Grouping Markers

**Note:** Boolean markers use `\` prefix when nested. Used with `[t]` and `[y]`.

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `[*]` | Boolean-Grouping | Group boolean expressions | `\~\[*]` (nested under parent) |
| `\&#124;\` | OR | Boolean OR (nested) | `\&#124;\ .condition_b` |
| `\&\` | AND | Boolean AND (nested) | `\&\ .condition_c` |
| `\^\` | XOR | Boolean XOR (nested) | `\^\ .condition_d` |

### Utility Markers

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `\~\` | Expand-Above | Expand parent marker above with nested content | `[y] .condition` then `\~\[r]` |
| `[+]` | Continuation | Line continuation | `[+]` |
| `[\]` | Setup | Setup step (FIFO) | `[\] &#124;Setup.Step` |
| `[/]` | Cleanup | Cleanup step (LIFO) | `[/] &#124;Cleanup.Step` |
| `[W]` | Runtime | Runtime wrapper (internal) | `[W] .wrapper` |

### Nesting Pattern

**Base markers** use `[X]` or `[X}` syntax. When **nested** under another marker, they use `\X\` syntax with backslash prefix and suffix.

### Push/Pull Operators

**Syntax:**
- `.source >> .destination` (push right)
- `.destination << .source` (pull left)
- `|String.Literal"{.args}" >> .destination`
- `.destination << |String.Literal"{.args}"`
- `\.\ ~> .destination` (default assignment from file via `[s]`)
- `.destination <~ .source` (set default value, can be overridden by push)
- `.destination ~> .source` (extract value if Ready, else use default)

**Rules:**
- Source must be variable (`.var`) or string literal (`|Pipeline"string"`)
- Variable types must be convertible
- Conversion pipeline: `|U.Convert.{source_lang}.{source_type}.to.{dest_lang}.{dest_type}`
- Example: `|U.Convert.Python.Str.to.Polyglot.String`
- Default operators allow one more push to override the default value

### Naming Conventions

- **Registry:** `@Scope::Namespace.Name:Version` (e.g., `@Local::Examples.MyApp:1.0.0.0`)
- **Pipelines:** `|Pipeline.Name.Hierarchy` (dot hierarchy, CamelCase)
- **Enumerations:** `#Enumeration.Name.Hierarchy` (dot hierarchy, CamelCase)
- **Errors:** `!Error.Name.Hierarchy` (dot hierarchy, CamelCase)
- **Variables:** `.variable_name:languagecode.type` (dot hierarchy, snake_case, no capitalization)
  - Example: `.param:pg.string`, `.user_data:py.dict`

## Operators

### Assignment & Access

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `.` | Variable Prefix | `.myvar` | Prefix for variables in current scope |
| `<` | Input Argument | `<input` | Prefix for pipeline input arguments |
| `>` | Output Argument | `>output` | Prefix for pipeline output arguments |
| `<<` | PULL | `.destination << .source` | Pull value from source to destination |
| `>>` | PUSH | `.source >> .destination` | Push value from source to destination |
| `<~` | Set Default | `.destination <~ .source` | Set default value (can be overridden by push) |
| `~>` | Extract Default | `.destination ~> .source` | Use destination if Ready, else use source |

### Pipeline & Data

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `&#124;` | Pipeline | `.arr &#124; .x -> expr` | Pipeline data through transform |
| `~` | Unpack | `~.array` | Unpack array elements |
| `@` | Package | `@package/module` | Import package |
| `#` | Enumeration | `#Status.Success` | Enum value |
| `!` | Error | `!ErrorType` | Error type or check |

### Comparison

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `>?` | Greater Than | `.a >? .b` | Returns Boolean |
| `<?` | Less Than | `.a <? .b` | Returns Boolean |
| `=?` | Equal | `.a =? .b` | Returns Boolean |
| `=!?` | Not Equal | `.a =!? .b` | Returns Boolean |
| `!?` | Is Error | `.x !?` | Check if Faulted |
| `!?` | Error Coalesce | `.a !? .b` | Use .a if Ready, .b if Faulted |

### String & Pattern

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `{.var` | Interpolation | `"Hello {.name"` | String interpolation |
| `+"` | Concatenation | `"part1" +" "part2"` | Concatenate strings |
| `*?` | Glob Match | `.path *? "*.csv"` | Glob pattern match |
| `re?` | Regex Match | `.text re? /\d+/` | Regex match |

### Range

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `?[a,b]` | Range | `?[1, 10]` | Range from a to b (inclusive |
| `?[a,b` | Range Exclusive | `?[1, 10` | Range excluding b |

## Variable States

| State | Description | Can Use? |
|-------|-------------|----------|
| **Declared** | Exists, no value | ❌ |
| **Pending** | Computing | ⏳ Waits when used |
| **Ready** | Value available | ✅ |
| **Faulted** | Error occurred | ❌ Propagates error |

## Type System

### Primitives

| Type | Example Literal |
|------|-----------------|
| `Integer` | `42`, `-10` |
| `Float` | `3.14`, `-0.5` |
| `String` | `"text"`, `'text'` |
| `Boolean` | `#True`, `#False` (aliases for `#Boolean.True`, `#Boolean.False` |
| `DT` | `|DT"2025-01-15"` |

### Collections

| Type | Example |
|------|---------|
| `Array` | `[1, 2, 3]` |
| `Set` | `{1, 2, 3` |
| `Serial` | `[1, 2, 3]` (ordered, unique |

### Foreign Types

| Syntax | Example | Language |
|--------|---------|----------|
| `py.type` | `py.dict`, `py.ndarray` | Python |
| `rs.type` | `rs.Vec`, `rs.String` | Rust |
| `go.type` | `go.slice`, `go.string` | Go |
| `js.type` | `js.Array`, `js.Object` | Node.js |
| `jl.type` | `jl.Array`, `jl.Dict` | Julia |

## Reserved Enumerations

### Triggers

| Enum | Fields | Example |
|------|--------|---------|
| `#T.Daily` | `.hour`, `.minute` | `#T.Daily(.hour: 9, .minute: 0` |
| `#T.Hourly` | `.minute` | `#T.Hourly(.minute: 30` |
| `#T.Weekly` | `.day`, `.hour`, `.minute` | `#T.Weekly(.day: #DayOfWeek.Monday, .hour: 9` |
| `#T.Cron` | `.expression` | `#T.Cron(.expression: "0 */6 * * *"` |
| `#T.FileCreated` | `.path` | `#T.FileCreated(.path: "/uploads/*.csv"` |
| `#T.FileModified` | `.path` | `#T.FileModified(.path: "/config/*"` |
| `#T.HTTP` | `.port`, `.path` | `#T.HTTP(.port: 8080, .path: "/webhook"` |
| `#T.Queue` | `.topic` | `#T.Queue(.topic: "events.user"` |

### DayOfWeek

```
#DayOfWeek.Monday | .Tuesday | .Wednesday | .Thursday | .Friday | .Saturday | .Sunday
```

### Errors

All error types start with `!`:

| Error Type | Fields | Example |
|------------|--------|---------|
| `!Error` | `.message`, `.code`, `.trace` | `!Error(.message: "failed"` |
| `!NetworkError` | `.message`, `.code`, `.trace` | `!NetworkError(.code: 404` |
| `!FileError` | `.message`, `.code`, `.trace`, `.path` | `!FileError(.path: "/missing"` |

## Common Patterns

### Pipeline Definition

```polyglot
{|} |Pipeline.Name
[<} .input:pg.type
[>} .output:pg.type
[t] |T.Type (optional)

[\] |Setup.Step
[r] |DoSomething
\<\ <in << .input
\>\ >out >> .temp
[/] |Cleanup.Step

.output << .temp
```

### Foreign Function Call

```polyglot
{W} |W.Python.CustomFunction
[{] .param:py.dict
[}] .result:py.list

[r] |U.Python.Call
\<\ <module << "module_name"
\<\ <function << "func_name"
\<\ <args << .param
\>\ >result >> .result
```

### Parallel Execution

```polyglot
[p] ~ForEach
\<\ <array << .input_array
\>\ >item >> .current_item
\~\[r] |ProcessItem
\~\\<\ <in << .current_item
\~\\>\ >out >> .result
\~\[v] ~V.JoinAll
\~\\<\ <append << .result
\~\\>\ >array >> .final_results
```

### Error Handling

```polyglot
{|} |FetchData
[<} .url:pg.string
[>} .data:pg.string

[!] .handle_network_error
\<\ <error_type << !NetworkError

[r] |U.HTTP.Get
\<\ <url << .url
\>\ >data >> .data

[b] |U.Log.Error"{.error}"
.data << |GetFallbackData
```

### Parallel Foreach Pattern

```polyglot
// Transform each element in parallel
[p] ~ForEach
\<\ <array << .input_array
\>\ >item >> .item
\~\[r] |ProcessItem
\~\\<\ <in << .item
\~\\>\ >out >> .result
\~\[v] ~V.JoinAll
\~\\<\ <append << .result
\~\\>\ >array >> .results
```

### Wrapper Definition

```polyglot
{W} |W.RetryWrapper
[{] .operation:pg.pipeline
[}] .result:pg.any

[\] |Setup.Logging
[\] |Setup.Metrics

[r] |DoOperation
\<\ <op << .operation
\>\ >res >> .result

[/] |Cleanup.Metrics
[/] |Cleanup.Logging
{x}
```

## Enumeration Definition

```polyglot
{#} #Status
[.] .success:pg.int << 0
[.] .failure:pg.int << 1
[.] .pending:pg.int << 2
{x}
```

### Extendable Enumeration

```polyglot
{#} #HTTPStatus.*
[.] .code:pg.int
[.] .message:pg.string

#HTTPStatus.OK << (.code: 200, .message: "OK")
#HTTPStatus.NotFound << (.code: 404, .message: "Not Found")
{x}
```

### Loading from Files

```polyglot
{#} #Config
[.] .api_key:pg.string
[.] .timeout:pg.int <~ 30
[s] |YAML.Load"\\FileDir\\config.yaml"
\.\ >> .api_key
\.\ ~> .timeout  // Use file value or default
{x}
```

## Critical Rules

1. **All variables use dot prefix:** `.var` not `var`
2. **PULL `<<` assigns from source:** `.destination << .source`
3. **PUSH `>>` assigns to destination:** `.source >> .destination`
4. **Pipe escape in docs:** Use `&#124;` for `|` in markdown tables
5. **PFG requires 3 blank lines** before `{|}`, `{W}`, `{#}` definitions
6. **Type separator is dot:** `py.type` not `py\type` or `py::type`
7. **No semicolons:** Line endings are implicit
8. **Registry markers close with {x}:** `{|}...{x}`, `{#}...{x}`, etc.
9. **Nesting uses backslash prefix/suffix:** `\<\`, `\>\`, `\~\`, etc.
10. **Wrappers: FIFO setup, LIFO cleanup:** `[\]` setup in order, `[/]` cleanup in reverse
11. **Actions use pipelines or literals:** Not bare variables

## Example: Complete Pipeline

```polyglot
{|} |Pipeline.DailyReport
[t] |T.Daily"9:00AM"
[<} .recipients:pg.array
[>} .sent:pg.bool

[\] |Setup.Logging

// Fetch data (Python - easy libraries)
[r] |W.Python.Analytics.FetchYesterday
\>\ >data >> .data

// Process (Rust - performance)
[r] |W.Rust.Processor.ComputeMetrics
\<\ <input << .data
\>\ >output >> .processed

// Generate report (Node - rich ecosystem)
[r] |W.NodeJS.PdfGen.Create
\<\ <data << .processed
\>\ >pdf >> .pdf

// Send (Go - robust networking)
[r] |W.Go.Mailer.Send
\<\ <pdf << .pdf
\<\ <recipients << .recipients
\>\ >success >> .sent

[/] |Cleanup.Logging
{x}
```

## CLI Quick Reference

```bash
polyglot compile file.pg              # Compile
polyglot run file.pg name --input '{' # Run once
polyglot register file.pg name        # Register
polyglot activate name                # Activate triggers
polyglot logs name                    # View logs
polyglot status                       # Service status
```

## Common Mistakes

| ❌ Wrong | ✅ Correct | Reason |
|---------|-----------|--------|
| `var << value` | `.var << value` | Missing dot prefix |
| `py::func(` | `py\func(` | Wrong type separator |
| `.x >> field;` | `.x >> field` | No semicolons |
| `await .result` | `.result` (auto-waits | No await keyword |
| `.x = value` | `.x << value` | Use PUSH operator |
| `[Pipeline]` | `[&#124;Pipeline]` | Wrong marker |
| `.arr &#124; x -> x` | `.arr &#124; .x -> .x` | Missing dot in lambda |

## Async Model (Critical

- **Everything is async by default**
- **PUSH `<<` starts computation, returns immediately**
- **Using a variable waits automatically if Pending**
- **Independent operations run in parallel**
- **No `await`, `async`, promises - handled automatically**
- **Errors propagate through Faulted state**

This reference contains ONLY the essentials. For full docs, see the complete user documentation.
