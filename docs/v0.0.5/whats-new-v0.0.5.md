# What's New in v0.0.5

**Version:** 0.0.5
**Release Date:** 2026-01-04
**Status:** Official Release Notes
**Previous Version:** v0.0.4

---

## 🎉 Overview

Polyglot v0.0.5 represents a major evolution of the language, introducing powerful new features, improved syntax clarity, and enhanced type safety. This release focuses on making the language more expressive, safer, and easier to use.

---

## ✨ Major New Features

### 1. Loop System - Unpack & Pack Operators

**The biggest addition to v0.0.5** - a comprehensive iteration system with collection operations.

#### Unpack Operators (`~`)

Iterate over collections with multiple specialized operators:

```polyglot
%% Iterate over array
[r] $items >> ~ForEach.Array
   [r] $processed << |Transform"{$current}"

%% Iterate with index
[r] $items >> ~ForEach.IndexedArray
   [r] |Log"[{$current.index}] = {$current.value}"

%% Iterate over set
[r] $uniqueIds >> ~ForEach.Set
   [r] |ProcessId"{$current}"

%% Iterate over serial fields
[r] $config >> ~ForEach.Serial
   [r] |Log"Key: {$current.key}, Value: {$current.value}"
```

#### Pack Operators (`*`)

Collect and aggregate iteration results:

```polyglot
%% Build collections
[*] *Into.Array
 *  <item.name:string << $current.name
 *  <item.value:uint << $current.value
 *  >array >> $results

%% Aggregate values
[*] *Aggregate.Sum
 *  <value:uint << $current.amount
 *  >sum:uint >> $total

%% String operations
[*] *String.Lines
 *  <line:string << "[{$current.timestamp}] {$current.message}"
 *  >result:string >> $logText
```

**Benefits:**
- Clean, expressive iteration syntax
- Type-safe collection operations
- Parallel or sequential execution
- Nested loop support
- Built-in aggregation functions

**Learn more:** [Loop System Guide](./language/loop-system.md)

---

### 2. Collection Literals

Create collections inline with concise syntax:

```polyglot
%% Empty collections
[r] $emptyArray:array.string << ( )
[r] $emptySet:set.uint << { }
[r] $emptySerial:serial << {:}

%% Inline arrays
[r] $numbers:array.uint << ( 1, 2, 3, 4, 5 )
[r] $names:array.string << ( "Alice", "Bob", "Charlie" )

%% Inline sets (unique values)
[r] $tags:set.string << { "urgent", "bug", "priority" }

%% Inline serials
[r] $config:serial << {
[+]  .host: "localhost",
[+]  .port: 5432,
[+]  .timeout: 30
[+] }
```

**Benefits:**
- No need for `|Array.Empty` or `|Array.From`
- More readable and concise
- Type-safe with explicit element types

---

### 3. Runtime Orchestration System

Execute code in multiple programming languages from Polyglot pipelines:

```polyglot
%% Python runtime
[w] |W.RT.Python
 |  <version:string << "3.11"
 |  >environment-RTenv-python >> $pyEnv

[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.log_file:string << $logPath
 |  <code:string << |Python""
[c] import os
[c] log_file = os.environ['log_file']
[c] with open(log_file, 'a') as f:
[c]     f.write('Hello from Python\n')
 |  >exit_code:uint >> $exitCode

%% Rust runtime
[w] |W.RT.Rust
 |  >environment-RTenv-rust >> $rustEnv

[r] |RT.Rust.Code
 |  <env.lang-RTenv-rust << $rustEnv
 |  <code:string << |Rust""
[c] println!("Hello from Rust");

%% JavaScript runtime
[w] |W.RT.JavaScript
 |  >environment-RTenv-javascript >> $jsEnv

[r] |RT.JavaScript.Code
 |  <env.lang-RTenv-javascript << $jsEnv
 |  <code:string << |JavaScript""
[c] console.log("Hello from JavaScript");
```

**Supported Languages:**
- Python (via `|W.RT.Python` and `|RT.Python.Code`)
- Rust (via `|W.RT.Rust` and `|RT.Rust.Code`)
- JavaScript/Node.js (via `|W.RT.JavaScript` and `|RT.JavaScript.Code`)

**Features:**
- Environment variable passing (all `:string` type)
- Native type kwargs for function-based code
- Automatic resource cleanup
- Error handling with `!RT.{Language}.Error`

**Learn more:** [Runtime Orchestration Quick Reference](./quick-reference/runtime-orchestration.md)

---

### 4. Code Block Marker `[c]`

Cleaner multi-line code syntax:

```polyglot
%% v0.0.4 (verbose)
<code:string << |SQL""
[+] +"SELECT id, name"
[+] +"FROM customers"
[+] +"WHERE active = true"

%% v0.0.5 (clean)
<code:string << |SQL""
[c] SELECT id, name
[c] FROM customers
[c] WHERE active = true
```

**Benefits:**
- More natural appearance
- Less visual noise
- Variable substitution: `{$variable}`
- Indentation preserved

**Note:** `[+]` still works for backward compatibility, but `[c]` is recommended.

---

### 5. Reserved Schema System

Type-safe configuration schemas for common patterns:

```polyglot
%% Database settings schema
{#} #MyApp.Database-DB-Settings
[A] #MyAppDB
[s] << |TOML.Load"\\FileDir\\config\\database.toml"
   [.] .host:string << .connection.host
   [.] .port:uint << .connection.port
   [.] .username:string << .credentials.user
   [.] .password:string << .credentials.pass
   [.] .database:string << .database_name
[s][!] !*
{x}

%% Use schema in wrapper
[w] |W.DB.Postgresql
 |  <settings-DB-Settings#MyAppDB
 |  >db-DB-Connection >> $dbConn
```

**Available Schemas:**
- `-DB-Settings` - Database connection configuration
- `-RT-Environment-{Language}` - Runtime environments
- More schemas in development

**Benefits:**
- Compile-time field validation
- Less verbose wrapper calls
- Self-documenting configuration
- Reusable across pipelines

**Learn more:** [DB Settings Schema](./stdlib/schemas/db-settings.yaml)

---

### 6. Input Shorthand for Enum-Typed Inputs

Reduce boilerplate when input value is implied:

```polyglot
%% v0.0.4
{#} #Config
[A] #MyConfig
{x}

[<] <config#Config << #MyConfig

%% v0.0.5 (implied value)
{#} #Config
[A] #MyConfig
{x}

[<] <config#Config  %% Value #MyConfig implied by [A] alias
```

**Benefits:**
- Less repetition
- Cleaner pipeline signatures
- Value automatically inferred from enum definition

---

## 🔧 Syntax Improvements

### 1. Clearer Reserved Enum Prefix

**v0.0.4:** Mixed `#` prefix for both user and reserved enums (ambiguous)
**v0.0.5:** Distinct `-` prefix for reserved enums

```polyglot
%% v0.0.4
[r] $now << #DT.Now
[r] $status << #Email.Status.Success

%% v0.0.5
[r] $now << -DT-Now              %% Reserved enum (stdlib)
[r] $status << #Email.Status.Success  %% User enum (your code)
```

**Benefits:**
- Clear visual distinction
- Easier to identify stdlib vs user code
- Prevents naming conflicts

---

### 2. Field Naming Convention

**v0.0.4:** Dashes allowed in field names (created ambiguity with reserved enums)
**v0.0.5:** Underscores required for compound names

```polyglot
%% v0.0.4 (ambiguous)
[.] .total-customers:uint  %% Looks like -customers enum!

%% v0.0.5 (clear)
[.] .total_customers:uint  %% Clearly a compound field name
```

**Rule:**
- `field_name` → underscores (your fields)
- `field-ReservedType` → dashes (reserved enum references)

**Learn more:** [Field Naming Conventions](./style-guide/field-naming-conventions.md)

---

### 3. I/O Marker Syntax

**v0.0.4:** `(|)` for pipeline parameters
**v0.0.5:** ` | ` (space-wrapped) for cleaner appearance

```polyglot
%% v0.0.4
[r] |DB.Query
(|) <db:serial << $dbConn
(|) <query:string << $sql
(|) >results:array.serial >> $results

%% v0.0.5
[r] |DB.Query
 |  <db:serial << $dbConn
 |  <query:string << $sql
 |  >results:array.serial >> $results
```

**Benefits:**
- Better visual alignment
- Less parenthesis clutter
- Consistent indentation

---

### 4. Comment Syntax

**v0.0.4:** C-style `//` and `/* */`
**v0.0.5:** Polyglot-specific `%%` and `%{ }%`

```polyglot
%% v0.0.4
// Single-line comment
/* Multi-line
   comment */

%% v0.0.5
%% Single-line comment
%{ Multi-line
   comment }%
```

**Benefits:**
- Distinct from embedded code comments (Python, JS, etc.)
- Consistent with metadata `%` prefix
- Easier to parse

---

### 5. Boolean Type System

**v0.0.4:** `#Boolean.True` and `#Boolean.False`
**v0.0.5:** `-Boolean-True` / `-Boolean-False` with `-True` / `-False` aliases

```polyglot
%% Equivalence
:bool ≡ -Boolean

%% Usage
[r] $success:bool << -True
[r] $failed:bool << -False

%% In conditionals
[f] $success ?= -True
   [r] |Celebrate
```

**Benefits:**
- Consistent with reserved enum system
- Shorter aliases available
- Type safety maintained

---

## 📚 Type System Enhancements

### 1. Shorter DateTime Type

**v0.0.4:** `:datetime`
**v0.0.5:** `:dt`

```polyglot
[.] .created_at:dt << |DT.Now""
[.] .updated_at:dt << $timestamp
```

---

### 2. Native Type System

Support for language-specific types in runtime kwargs:

```polyglot
%% Python types
<kwargs.user_id:py.int << $userId
<kwargs.message:py.str << $message
<kwargs.active:py.bool << $isActive

%% Rust types
<kwargs.count:rust.i32 << $count
<kwargs.name:rust.String << $name

%% JavaScript types
<kwargs.value:js.number << $value
<kwargs.text:js.string << $text
```

**Mapping:**

| Polyglot | Python | Rust | JavaScript |
|----------|--------|------|------------|
| `:string` | `:py.str` | `:rust.String` | `:js.string` |
| `:uint` | `:py.int` | `:rust.u32` | `:js.number` |
| `:int` | `:py.int` | `:rust.i32` | `:js.number` |
| `:bool` | `:py.bool` | `:rust.bool` | `:js.boolean` |

---

### 3. Schema-Based Types

Enums can implement reserved schemas for type safety:

```polyglot
{#} #MyApp.Config-DB-Settings  %% Implements -DB-Settings schema
[A] #AppDB

%% Compiler validates all required fields present
[s] << |TOML.Load"config.toml"
   [.] .host:string << .db.host        %% Required
   [.] .port:uint << .db.port          %% Required
   [.] .username:string << .db.user    %% Required
   [.] .password:string << .db.pass    %% Required
   [.] .database:string << .db.name    %% Required
[s][!] !*
{x}
```

---

## 🛠️ Developer Experience Improvements

### 1. Better Error Messages

v0.0.5 provides clearer compile-time errors:

```
ERROR: Field naming conflict
  --> pipeline.pg:23:5
   |
23 | [.] .total-customers:uint
   |     ^^^^^^^^^^^^^^^^
   |
   = Compound field names must use underscores, not dashes
   = Help: Change to .total_customers:uint
   = Reason: Dashes are reserved for enum references like .status-Email-Status
```

---

### 2. Dot Notation for Serials

Recommended pattern for constructing serial values in pack operations:

```polyglot
%% Recommended (v0.0.5)
[*] *Into.Array
 *  <item.runtime:string << "Python"
 *  <item.success:bool << -True
 *  <item.timestamp:dt << |DT.Now""
 *  >array >> $results

%% Alternative (verbose)
[*] *Into.Array
 *  <item:serial
    [.] .runtime:string << "Python"
    [.] .success:bool << -True
 *  >array >> $results
```

---

### 3. Exhaustive Error Handling

Error handling must be exhaustive (compile-time checked):

```polyglot
[r] |RiskyOperation
 |  <input:string << $data
   [!] !RiskyOperation.Error
      [r] $status << "failed"
   [!] !*  %% Catch-all REQUIRED for exhaustiveness
      [r] $status << "success"
```

---

## 📖 Documentation Improvements

### New Documentation

- **Loop System Guide** - Comprehensive iteration patterns
- **Runtime Orchestration** - Multi-language code execution
- **Field Naming Conventions** - Style guide
- **Migration Guide** - v0.0.4 → v0.0.5 conversion
- **Quick References** - Runtime orchestration patterns
- **Training Sessions** - 7 documented examples with corrections

### Enhanced Stdlib Documentation

- **Reserved Enums** - Complete catalog with aliases
- **Standard Wrappers** - Runtime, database, HTTP, file
- **Standard Pipelines** - Runtime execution, utilities
- **Standard Operators** - Pack and unpack operators
- **Schemas** - Database settings and runtime environments

---

## 🚀 Performance & Reliability

### Parallel Loop Execution

```polyglot
[p] $urls >> ~ForEach.Array  %% Parallel execution
   [r] |HTTP.Get"{$current}"
```

**Benefits:**
- Automatic concurrency management
- Better resource utilization
- Faster I/O-bound operations

### Automatic Resource Cleanup

Wrappers automatically manage resource lifecycle:

```polyglot
[w] |W.DB.Postgresql
 |  <settings-DB-Settings#Config
 |  >db-DB-Connection >> $dbConn

%% Connection automatically closed when pipeline exits
%% No manual cleanup needed!
```

---

## 🔄 Migration from v0.0.4

**Estimated Effort:**
- Small projects (< 1000 lines): 1-2 hours
- Medium projects (1000-10000 lines): 1-2 days
- Large projects (10000+ lines): 1 week+

**Migration Steps:**
1. Update field names (dashes → underscores)
2. Update datetime type (`:datetime` → `:dt`)
3. Update reserved enum prefixes (`#` → `-`)
4. Update comments syntax (`//` → `%%`)
5. Update I/O markers (`(|)` → ` | `)
6. Update pack marker (`[v]` → `[*]`)
7. Optional: Add collection literals
8. Optional: Convert `[+]` to `[c]` for code blocks

**Tools:**
- Automated migration scripts provided
- Syntax validator with upgrade suggestions
- Step-by-step migration guide

**Learn more:** [Migration Guide](./migration-guide-v0.0.4-to-v0.0.5.md)

---

## 📊 Breaking Changes Summary

| Category | Impact | Migration Effort |
|----------|--------|------------------|
| Field naming | High | Medium - Find/replace |
| Reserved enum prefix | High | Low - Automated |
| I/O markers | Medium | Low - Automated |
| Comments | Medium | Low - Automated |
| DateTime type | Medium | Low - Find/replace |
| Pack marker | Low | Low - Automated |

**Note:** While these are breaking changes, automated migration tools handle ~80% of the work.

---

## 🎯 What This Means for You

### If You're New to Polyglot

**Great timing!** v0.0.5 is the most polished version yet:
- Clearer syntax with less ambiguity
- More powerful features (loops, runtime orchestration)
- Better documentation and examples
- Consistent conventions

Start with:
1. [Quick Start Guide](./quick-start.md)
2. [Loop System Guide](./language/loop-system.md)
3. [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
4. [Hello World Example](./examples/hello-world-multi-runtime.pg)

### If You're Migrating from v0.0.4

**Worth the effort!** v0.0.5 provides:
- New capabilities (loops, runtime orchestration)
- Clearer, more maintainable code
- Better type safety and error catching
- Future-proof syntax

Follow the [Migration Guide](./migration-guide-v0.0.4-to-v0.0.5.md) for a smooth transition.

---

## 🔮 Looking Ahead

### Upcoming Features (Future Versions)

- More runtime language support (Go, C#, etc.)
- Advanced pattern matching
- Async/await patterns
- Module system enhancements
- IDE plugins and tooling

### Community Feedback

We're listening! Share your experience with v0.0.5:
- What features do you love?
- What could be improved?
- What examples would help?

---

## 📚 Resources

### Documentation
- [Loop System Guide](./language/loop-system.md)
- [Runtime Orchestration](./quick-reference/runtime-orchestration.md)
- [Variable Lifecycle](./language/variable-lifecycle.md)
- [Error Handling](./language/error-handling.md)
- [Field Naming Conventions](./style-guide/field-naming-conventions.md)

### Examples
- [Hello World Multi-Runtime](./examples/hello-world-multi-runtime.pg)
- [Training Session Examples](./training-sessions/session-001-2026-01-02.md)

### Reference
- [Stdlib Documentation](./stdlib/README.md)
- [Reserved Enums](./stdlib/reserved-enums.yaml)
- [Standard Wrappers](./stdlib/standard-wrappers.yaml)
- [Standard Pipelines](./stdlib/standard-pipelines.yaml)

### Guides
- [Migration Guide v0.0.4 → v0.0.5](./migration-guide-v0.0.4-to-v0.0.5.md)

---

## ✅ Summary

v0.0.5 is a **major leap forward** for Polyglot:

**New Capabilities:**
- ✅ Loop system with unpack/pack operators
- ✅ Runtime orchestration (Python, Rust, JavaScript)
- ✅ Collection literals
- ✅ Reserved schema system
- ✅ Code block marker `[c]`

**Syntax Improvements:**
- ✅ Clearer reserved enum prefix (`-`)
- ✅ Better field naming (underscores)
- ✅ Improved I/O syntax
- ✅ Polyglot-specific comments
- ✅ Type system enhancements

**Developer Experience:**
- ✅ Comprehensive documentation
- ✅ Migration tools and guides
- ✅ Training examples
- ✅ Better error messages

**Get started with v0.0.5 today!** 🚀

---

**Version:** 0.0.5
**Release Date:** 2026-01-04
**Status:** ✅ Official Release
**Documentation:** Complete

**Welcome to Polyglot v0.0.5!**
