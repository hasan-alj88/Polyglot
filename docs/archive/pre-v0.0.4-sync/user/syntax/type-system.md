---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/type-system.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Type System

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Type Namespace System

Polyglot uses **backslash separator** (`\` for type namespaces:

```polyglot
:pg.string              // Polyglot native string
rs\HashMap            // Rust HashMap
py\dict               // Python dictionary
js\Object             // JavaScript Object
```

**CRITICAL:** Use `\` (backslash, NOT `/` or `:`!

---

## Primitive Types (`:pg.*`

| Type | Description | Example |
|------|-------------|---------|
| `:pg.int` | Signed integer | `42`, `-10` |
| `:pg.uint` | Unsigned integer | `100` |
| `:pg.float` | Floating point | `3.14`, `-0.5` |
| `:pg.string` | UTF-8 string | `"Hello"` |
| `:pg.bool` | Boolean | `#Boolean.True` |
| `:pg.path` | File path | `"/home/user"` |
| `:pg.dt` | DateTime/Duration | `DT.Now""` |
| `:pg.serial` | Hierarchical data | Dynamic fields |

---

## Collection Types

### `:pg.array{T`

Ordered collection of type `T`.

```polyglot
.numbers:pg.array.pg.int << {1, 2, 3
.names:pg.array.pg.string << {"Alice", "Bob"
```

**CRITICAL:** Use `{` for literals, NOT `[]`!

### `:pg.set{T`

Unordered unique collection.

```polyglot
.unique_ids:pg.set.pg.int << {1, 2, 3
```

### No Nested Collections

**INVALID:**
```polyglot
:pg.array.pg.array.pg.int      // ERROR!
```

**VALID (wrap in enum:**
```polyglot
[#] #Row
[<] <cells:pg.array.pg.int
[X]

.matrix:pg.array.#Row           // ✓ Valid
```

---

## Foreign Types

### Rust Types (`rs\*`

```polyglot
.data: rs\Vec
.map: rs\HashMap
.result: rs\Result
```

### Python Types (`py\*`

```polyglot
.config: py\dict
.items: py\list
.value: py\int
```

### JavaScript/Node Types (`js\*`, `node\*`

```polyglot
.obj: js\Object
.arr: js\Array
.promise: node\Promise
```

### Go Types (`go\*`

```polyglot
.slice: go\Slice
.map: go\Map
.channel: go\Channel
```

---

## Type Declaration Syntax

```polyglot
.variable: Type                    // Declared state
.variable: Type << value           // Ready state (immediate
.variable: Type <~ default         // DefaultReady state
```

---

## Enumeration Types

Enumerations are types themselves:

```polyglot
[#] #UserRole
[<] .Admin
[<] .User
[X]

.role: #UserRole << #UserRole.Admin
```

---

## Error Types

Errors are special types with required fields:

```polyglot
[!] NetworkTimeout
[<] <message:pg.string << "Timeout"
[<] <code:pg.int << 1001
[<] <trace:pg.string << ""
[X]

.error: !NetworkTimeout
```

---

## Serial Type (`:pg.serial`

Hierarchical key-value structure (like JSON:

```polyglot
.user:pg.serial
[<] .user.name:pg.string << "Alice"
[<] .user.age:pg.int << 30
[<] .user.email:pg.string << "alice@example.com"
```

---

## Type Inference

Polyglot infers types from literals and context:

```polyglot
[r] .count << 42                   // Inferred:pg.int
[r] .name << "Alice"               // Inferred:pg.string
[r] .flag << #Boolean.True         // Inferred: #Boolean
```

---

## Reserved Types

- `#Boolean` - `#Boolean.True`, `#Boolean.False`
- `#None` - Unit type
- `#PgVar.States.*` - Variable states
- `!No.Input` - No input marker (error marker, not enumeration
- `!No.Output` - No output marker (error marker

---

**Next:** [Enumerations →](enumerations.md
