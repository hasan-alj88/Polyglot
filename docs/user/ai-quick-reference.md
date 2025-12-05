# AI Quick Reference

Ultra-compressed Polyglot reference for AI code generation. Maximum density, minimum fluff.

## Core Syntax Rules

| Rule | Description |
|------|-------------|
| **Dot prefix** | ALL variables start with `.` (e.g., `.myvar` |
| **Case sensitive** | `.Name` ≠ `.name` |
| **PFG formatting** | 3 blank lines before `[&#124;Pipeline]`, `[r]`, `[#]` definitions |
| **Type separator** | Backslash `\` for foreign types: `py\dict`, `rs\Vec` |
| **Block markers** | `[X]` format for all blocks |
| **No semicolons** | Lines don't end with `;` |
| **Comments** | `//` single-line, `/* */` multi-line |

## Block Markers

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `[&#124;]` | Pipeline | Define pipeline | `[&#124;Pipeline] .my_pipeline` |
| `[X]` | Macro | Macro definition | `[X] .my_macro` |
| `[i]` | Input | Pipeline input | `[i] .param: String` |
| `[o]` | Output | Pipeline output | `[o] .result: Integer` |
| `[r]` | Wrapper | Foreign function wrapper | `[r] py\my_func` |
| `[p]` | Parallel | Parallel block | `[p] .parallel_work` |
| `[<]` | Field | Struct/enum field, scope input, macro unwrap | `[<] <field: String` |
| `[>]` | Copy-Out | Parallel copy-out, scope output | `[>] .result` |
| `[Y]` | Join | Join parallel threads | `[Y] .parallel_block` |
| `[t]` | Trigger | Event trigger | `[t] .trigger: #T.Daily(...` |
| `[Q]` | Queue | Queue control | `[Q] .priority: 10` |
| `[W]` | Runtime | Runtime wrapper (internal | `[W] .wrapper` |
| `[#]` | Enumeration | Define enumeration | `[#] .Status` |
| `[!]` | Error | Error handler block | `[!] .error_handler` |
| `[A]` | Alias | Enum alias | `[A] .NewName: .OldName` |
| `[~]` | Expansion | Explicit expansion | `.arr &#124; [~] .x -> .x * 2` |
| `[M]` | Macro | Macro definition | `[M] .my_macro` |
| `[{]` | Scope-In | Macro scope input | `[{] .input` |
| `[]` | Scope-Out | Macro scope output | `[] .output` |
| `[&]` | Pipeline-Marker | Mark pipeline in scope | `[&] .pipeline_ref` |
| `[+]` | Setup-Add | Add setup step | `[+] .init` |
| `[-]` | Setup-Remove | Remove setup step | `[-] .cleanup` |
| `[^]` | Exception | Exception handling | `[^] .exception` |
| `[.]` | Comment-Block | Block comment | `[.] Documentation` |
| `[*]` | Continuation | Line continuation | `[*]` |
| `[\]` | Setup | Macro setup (FIFO | `[\] .setup_step` |
| `[/]` | Cleanup | Macro cleanup (LIFO | `[/] .cleanup_step` |
| `[b]` | Begin-Handler | Begin error handler body | `[b] .error_handler` |
| `[s]` | Separator | Internal separator | `[s]` |
| `[?]` | Conditional | Conditional block | `[?] .condition` |

## Operators

### Assignment & Access

| Operator | Name | Usage | Description |
|----------|------|-------|-------------|
| `.` | Variable Prefix | `.myvar` | Prefix for variables in current scope |
| `<` | Input Argument | `<input` | Prefix for pipeline input arguments |
| `>` | Output Argument | `>output` | Prefix for pipeline output arguments |
| `<<` | PUSH | `.x << expr` | Start async operation, var becomes Pending |
| `>>` | PULL | `>output >> .var` | Pipeline output to variable |
| `<~` | Set Default | `.x <~ value` | Set default if Declared |
| `~>` | Extract Default | `.x ~> default` | Use x if Ready, else default |

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
| `DT` | `DT"2025-01-15"` |

### Collections

| Type | Example |
|------|---------|
| `Array` | `[1, 2, 3]` |
| `Set` | `{1, 2, 3` |
| `Serial` | `[1, 2, 3]` (ordered, unique |

### Foreign Types

| Syntax | Example | Language |
|--------|---------|----------|
| `py\type` | `py\dict`, `py\ndarray` | Python |
| `rs\type` | `rs\Vec`, `rs\String` | Rust |
| `go\type` | `go\[]int`, `go\string` | Go |
| `js\type` | `js\Array`, `js\Object` | Node.js |
| `jl\type` | `jl\Array`, `jl\Dict` | Julia |

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
[|Pipeline] .name
[i] .input: Type
[o] .output: Type
[t] .trigger: #T.Type (optional

.temp << compute(.input
.output << transform(.temp
```

### Foreign Function Call

```polyglot
[r] py\function_name
[i] .param: py\type
[o] .result: py\type
[<] <module: "module_name"
[<] <function: "func"
```

### Parallel Execution

```polyglot
[p] .parallel_work
    .result1 << compute1(
    .result2 << compute2(
    [>] .result1  // Copy out
    [>] .result2
[Y] .parallel_work

.final << combine(.result1, .result2
```

### Error Handling

```polyglot
[!] .handle_error
[<] <error_type: !NetworkError

.data << py\requests.get(.url

[b] .handle_error
    .data << fallback_data(
[/]
```

### Pipeline Operator

```polyglot
// Transform each element
.results << .array | [~] .x -> process(.x

// Filter
.filtered << .array | [~] .x -> .x >? 10

// Map-reduce
.sum << .array | [~] .x -> .x * 2 | sum(
```

### Macro Definition

```polyglot
[M] .retry_macro
[{] .operation
[\] .setup_logging
[\] .init_metrics

// Macro body
.result << .operation !? .operation !? .operation

[/] .cleanup_metrics
[/] .cleanup_logging
[] .result
```

## Enumeration Definition

```polyglot
[#] #Status
[<] <Success: Integer
[<] <Failure: Integer
[<] <Pending: Integer

#Status.Success: 0
#Status.Failure: 1
#Status.Pending: 2
```

### Extendable Enumeration

```polyglot
[#] #HTTPStatus.*
[<] <code: Integer
[<] <message: String

#HTTPStatus.OK(.code: 200, .message: "OK"
#HTTPStatus.NotFound(.code: 404, .message: "Not Found"
```

### Loading from Files (with Safety Mechanism

```polyglot
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30
[s] "config.yaml"               // Load from file
[s][!] *                        // Safety: required error handling
[X]
```

**⚠️ Safety:** All `[s]` blocks must declare error handling: `[s][!] *` (default or `[s][!]` (custom.

## Critical Rules

1. **All variables use dot prefix:** `.var` not `var`
2. **PUSH `<<` doesn't wait:** Returns immediately, var becomes Pending
3. **PULL `>>` waits if needed:** Automatic async synchronization
4. **Pipe escape in docs:** Use `&#124;` for `|` in markdown tables
5. **PFG requires 3 blank lines** before `[|]`, `[r]`, `[#]` definitions
6. **Type separator is backslash:** `py\type` not `py::type` or `py.type`
7. **No semicolons:** Line endings are implicit
8. **Errors propagate automatically:** Faulted variables propagate through chains
9. **Parallel blocks have copy semantics:** Use `[>]` to copy-out results
10. **Macros: FIFO setup, LIFO cleanup:** Setup in order, cleanup in reverse
11. **`[s]` blocks require error handling:** Must use `[s][!] *` or `[s][!]` with custom handler

## Example: Complete Pipeline

```polyglot
[|Pipeline] .daily_report
[t] .trigger: #T.Daily(.hour: 9, .minute: 0
[i] .recipients: Array
[o] .sent: Boolean

// Fetch data (Python - easy libraries
.data << py\analytics.fetch_yesterday(

// Process (Rust - performance
.processed << rs\processor::compute_metrics(.data

// Generate report (Node - rich ecosystem
.pdf << js\pdf_gen.create(.processed

// Send (Go - robust networking
.sent << go\mailer.send(.pdf, .recipients
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
