---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: type-system
shard: false

# --- Classification ---
type: spec
topic: Types System
summary: Specification for Types System
keywords:
  - types
  - type-system
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: solutioning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - language-syntax
unlocks:
  - advanced-features

# --- Relationships ---
related:
  []
parent: language-types

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#types"
  - "#spec"
---
# Types System

**What You'll Learn:**
- Complete Polyglot type system
- Built-in types and aliases
- Collection types (arrays, sets, maps)
- Custom types (enums, structs)
- Type notation and hierarchy
- Type compatibility rules

---

## Type Notation

**All types use the `:` prefix:**

```polyglot
:type_name
:namespace.type
:collection.element_type
```

**Examples:**
```polyglot
[r] $name :string << "Alice"
[r] $count :int << 42
[r] $items :array.string << {"item1", "item2"}
[r] $user :User << #User
```

---

## Built-in Types

### Polyglot Namespace (`pg.*`)

**Full type names:**

| Type | Full Form | Description |
|------|-----------|-------------|
| `string` | `:pg.string` | Text/string data |
| `int` | `:pg.int` | Integer numbers |
| `float` | `:pg.float` | Floating-point numbers |
| `bool` | `:pg.bool` | Boolean values |
| `serial` | `:pg.serial` | Serialized data (JSON, YAML, etc.) |
| `file` | `:pg.file` | File reference |
| `stream` | `:pg.stream` | Data stream |

### Type Aliases

**Short form = Long form:**

```polyglot
:string  = :pg.string
:int     = :pg.int
:float   = :pg.float
:bool    = :pg.bool
```

**Both forms are valid:**

```polyglot
[r] $name :string << "Alice"              // Short form
[r] $name :pg.string << "Alice"           // Long form (equivalent)

[r] $count :int << 42                     // Short form
[r] $count :pg.int << 42                  // Long form (equivalent)
```

**Recommendation:** Use short form for readability.

**See Also:** [Prefix System - Type Aliases](./prefix-system.md#type-aliases) for the `:` prefix notation.

---

## Primitive Types

### `:string` / `:pg.string`

**Purpose:** Text data

**Examples:**
```polyglot
[r] $name :string << "Alice"
[r] $greeting :string << "Hello, World!"
[r] $empty :string << ""
```

**Operations:**
```polyglot
[r] $upper :string << |U.String.Upper"{$name}"
[r] $concat :string << |U.String.Concat <parts << {"Hello", " ", "World"}
```

### `:int` / `:pg.int`

**Purpose:** Integer numbers (whole numbers)

**Examples:**
```polyglot
[r] $age :int << 30
[r] $count :int << 0
[r] $negative :int << -42
```

**Range:** Implementation-defined (typically 64-bit signed)

**Operations:**
```polyglot
[r] $sum :int << |U.Math.Add <x << 5 <y << 3
[r] $doubled :int << |U.Math.Multiply <x << $count <y << 2
```

### `:float` / `:pg.float`

**Purpose:** Floating-point numbers (decimal numbers)

**Examples:**
```polyglot
[r] $price :float << 19.99
[r] $ratio :float << 0.75
[r] $negative :float << -3.14
```

**Precision:** Implementation-defined (typically 64-bit IEEE 754)

**Operations:**
```polyglot
[r] $total :float << |U.Math.Multiply <x << $price <y << $quantity
[r] $rounded :float << |U.Math.Round <value << $price <decimals << 2
```

### `:bool` / `:pg.bool`

**Purpose:** Boolean values (true/false)

**Values:**
```polyglot
[r] $is_active :bool << #True
[r] $is_deleted :bool << #False
```

**Note:** Polyglot uses **enum values**, not keywords:
- ✅ `#True` (correct - shorthand for `#;Boolean;True`)
- ❌ `true` (not a keyword)

**Operations:**
```polyglot
[y] $is_active =? #True
   [r] $can_access << #True
```

---

## Special Types

### `:pg.serial`

**Purpose:** Serialized/deserialized data (JSON, YAML, TOML, etc.)

**Usage:**
```polyglot
[s] |YAML.Load
[s] <file << "/config.yaml"
[s] >content >> $config :pg.serial
   [.] << *

[r] $db_host :string << $config.database.host
[r] $db_port :int << $config.database.port
```

**Characteristics:**
- Dynamic structure
- Field access via dot notation
- Type checking at runtime

**See:** [Serial Load Block](../language/advanced/serial-load-block.md)

### `:pg.file`

**Purpose:** File reference

**Usage:**
```polyglot
{#} #Config
[s] .config_file :pg.file.yaml
   [.] .database :string
   [.] .port :int
{x}
```

**Subtypes:**
- `:pg.file.yaml` - YAML files
- `:pg.file.json` - JSON files
- `:pg.file.toml` - TOML files
- `:pg.file.xml` - XML files
- `:pg.file.csv` - CSV files

### `:pg.stream`

**Purpose:** Data stream (future feature)

**Usage:**
```polyglot
[r] $input_stream :pg.stream << |File.OpenStream <path << "/data.txt"
```

---

## Collection Types

### `:array.*` - Arrays

**Purpose:** Ordered collection of elements (allows duplicates)

**Syntax:**
```polyglot
:array.element_type
```

**Examples:**
```polyglot
[r] $names :array.string << {"Alice", "Bob", "Charlie"}
[r] $scores :array.int << {95, 87, 92, 87}  // Duplicates allowed
[r] $prices :array.float << {19.99, 29.99, 9.99}
```

**Literal syntax:**
```polyglot
[r] $items :array.string << {"item1", "item2", "item3"}
```

**Operations:**
```polyglot
[r] $length :int << |Array.Length <array << $names
[r] $first :string << |Array.First <array << $names
[r] $sorted :array.int << |Array.Sort <array << $scores
```

**Nested arrays:**
```polyglot
[r] $matrix :array.array.int << {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
//          ^     ^      ^
//         array  array  int
```

### `:set.*` - Sets

**Purpose:** Unordered collection of unique elements (no duplicates)

**Syntax:**
```polyglot
:set.element_type
```

**Examples:**
```polyglot
[r] $unique_tags :set.string << {"important", "urgent", "review"}
[r] $user_ids :set.int << {1, 2, 3, 2, 1}  // Stored as {1, 2, 3}
```

**Literal syntax:**
```polyglot
[r] $tags :set.string << {"tag1", "tag2", "tag3"}
```

**Operations:**
```polyglot
[r] $contains :bool << |Set.Contains <set << $tags <value << "tag1"
[r] $union :set.string << |Set.Union <set1 << $tags1 <set2 << $tags2
```

### `:map.*` - Maps (Dictionaries)

**Purpose:** Key-value pairs

**Syntax:**
```polyglot
:map.key_type.value_type
```

**Examples:**
```polyglot
[r] $user_ages :map.string.int << {
   {"Alice", 30},
   {"Bob", 25},
   {"Charlie", 35}
}

[r] $config :map.string.string << {
   {"host", "localhost"},
   {"port", "5432"},
   {"database", "mydb"}
}
```

**Operations:**
```polyglot
[r] $age :int << |Map.Get <map << $user_ages <key << "Alice"
[r] $has_key :bool << |Map.ContainsKey <map << $config <key << "host"
```

---

## Custom Types

### Enum Types

**Definition:**
```polyglot
{#} #OrderStatus
[.] .pending
[.] .processing
[.] .completed
{x}
```

**Usage:**
```polyglot
[r] $status :OrderStatus << #OrderStatus.Processing
```

**Type notation:** Use enum name directly (no `:` prefix in declaration)

```polyglot
[r] $status :OrderStatus << #OrderStatus.Processing
//          ^              ^
//        Type name      Enum value (with # prefix)
```

### Struct Types

**Definition:**
```polyglot
{#} #User
[.] .name :string
[.] .email :string
[.] .age :int
{x}
```

**Usage:**
```polyglot
[r] $user :User << #User
   [.] .name << "Alice"
   [.] .email << "alice@example.com"
   [.] .age << 30
```

**Type notation:** Use struct name directly

```polyglot
[r] $user :User << #User
//        ^        ^
//     Type name  Struct constructor (with # prefix)
```

---

## Type Hierarchy

### Nested Collections

**Array of arrays:**
```polyglot
[r] $matrix :array.array.int << {{1, 2}, {3, 4}}
```

**Array of sets:**
```polyglot
[r] $tag_groups :array.set.string << {
   {"tag1", "tag2"},
   {"tag3", "tag4"}
}
```

**Map with array values:**
```polyglot
[r] $user_permissions :map.string.array.string << {
   {"admin", {"read", "write", "delete"}},
   {"user", {"read"}}
}
```

### Parsing Type Notation

**Read from right to left:**

```polyglot
:array.array.int
        ^     ^
      array  of int

:map.string.array.int
 ^    ^       ^    ^
map  with   array of int
     keys  values
     of
   string
```

**Examples:**

| Type | Reads As |
|------|----------|
| `:array.string` | Array of strings |
| `:array.array.int` | Array of (arrays of ints) |
| `:set.int` | Set of ints |
| `:map.string.int` | Map with string keys and int values |
| `:map.string.array.int` | Map with string keys and (array of int) values |

---

## Type Compatibility

### Assignment Rules

**Variables must match declared type:**

```polyglot
[r] $name :string << "Alice"       ✅ Type matches
[r] $name :string << 42            ❌ Type mismatch (int, not string)
```

### Struct Field Types

**Field assignments must match field type:**

```polyglot
{#} #User
[.] .name :string
[.] .age :int
{x}

[r] $user :User << #User
   [.] .name << "Alice"            ✅ string matches
   [.] .age << 30                  ✅ int matches
   [.] .age << "thirty"            ❌ string doesn't match :int
```

### Collection Element Types

**All elements must match declared element type:**

```polyglot
[r] $numbers :array.int << {1, 2, 3, 4}        ✅ All ints
[r] $numbers :array.int << {1, 2, "three", 4}  ❌ "three" is string
```

### No Implicit Conversion

**Polyglot does NOT auto-convert types:**

```polyglot
[r] $count :int << 42
[r] $count_str :string << $count   ❌ No implicit int → string

// Fix: Use explicit conversion
[r] $count_str :string << |Convert.ToString <value << $count  ✅
```

---

## Type Inference (Limited)

**First declaration MUST have type:**

```polyglot
[r] $name :string << "Alice"       ✅ Type declared
[r] $name << "Alice"               ❌ ERROR: Missing type
```

**Subsequent uses don't need type:**

```polyglot
[r] $result :string << "initial"   // First declaration with type

// Later (same scope or nested):
[r] $result << "updated"           // Type already known (from first decl)
```

**Note:** In practice, always include type for clarity.

---

## Common Patterns

### Pattern 1: Collection of Custom Types

```polyglot
{#} #User
[.] .name :string
[.] .email :string
{x}

[r] $users :array.User << {
   #User { .name << "Alice", .email << "alice@example.com" },
   #User { .name << "Bob", .email << "bob@example.com" }
}
```

### Pattern 2: Nested Data Structures

```polyglot
[r] $organization :map.string.array.User << {
   {"Engineering", {
      #User { .name << "Alice", .email << "alice@example.com" },
      #User { .name << "Bob", .email << "bob@example.com" }
   }},
   {"Marketing", {
      #User { .name << "Charlie", .email << "charlie@example.com" }
   }}
}
```

### Pattern 3: Optional Values (with Enum)

```polyglot
{#} #Optional
[.] .some
   [.] .value :pg.serial
[.] .none
{x}

[r] $maybe_value :Optional << #Optional.some
   [.] .value << "data"

[r] $empty_value :Optional << #Optional.none
```

### Pattern 4: Generic Response

```polyglot
{#} #Response
[.] .success
   [.] .data :pg.serial
   [.] .status :int
[.] .error
   [.] .message :string
   [.] .code :int
{x}

{|} |FetchData
[|] >response :Response

[t] |T.Call
[W] |W.Polyglot.Scope

   [z] $data :pg.serial << |Database.Query
   [z][!] *! >> $error :!

   [y] $error.state =? :pg.state.faulted
      [|] >response << #Response.error
         [.] .message << "Query failed"
         [.] .code << 500
   [y] *?
      [|] >response << #Response.success
         [.] .data << $data
         [.] .status << 200

{x}
```

---

## Type Metadata

### Documenting Types

```polyglot
{#} #User
[%] %Doc << "Represents a user in the system"

[.] .name :string
   [%] %description << "Full name of the user"
   [%] %example << "Alice Smith"

[.] .email :string
   [%] %description << "Email address"
   [%] %format << "email"
   [%] %example << "alice@example.com"

[.] .age :int
   [%] %description << "Age in years"
   [%] %range << "0-150"
{x}
```

---

## Best Practices

### 1. Always Declare Types on First Use

**Good:**
```polyglot
[r] $user_name :string << "Alice"  ✅
```

**Bad:**
```polyglot
[r] $user_name << "Alice"          ❌ Missing type
```

### 2. Use Descriptive Type Names

**Good:**
```polyglot
{#} #UserRegistrationResult
[.] .success
[.] .error
{x}
```

**Bad:**
```polyglot
{#} #Data
[.] .ok
[.] .err
{x}
```

### 3. Prefer Specific Types Over `pg.serial`

**Good:**
```polyglot
{#} #Config
[.] .database :string
[.] .port :int
{x}
```

**Acceptable:**
```polyglot
[r] $dynamic_config :pg.serial << $loaded_yaml
```

**Use `pg.serial` when:**
- Structure is truly dynamic
- Loading external data
- Prototyping

### 4. Use Collection Types Appropriately

**Arrays - when order matters:**
```polyglot
[r] $steps :array.string << {"step1", "step2", "step3"}
```

**Sets - when uniqueness matters:**
```polyglot
[r] $unique_ids :set.int << {1, 2, 3, 2}  // Stores {1, 2, 3}
```

**Maps - when key-value lookup matters:**
```polyglot
[r] $settings :map.string.string << {{"theme", "dark"}, {"lang", "en"}}
```

---

## Type Summary

### Built-in Types

| Type | Alias | Purpose |
|------|-------|---------|
| `:pg.string` | `:string` | Text data |
| `:pg.int` | `:int` | Integer numbers |
| `:pg.float` | `:float` | Decimal numbers |
| `:pg.bool` | `:bool` | Boolean values |
| `:pg.serial` | - | Serialized data |
| `:pg.file` | - | File reference |
| `:pg.stream` | - | Data stream |

### Collection Types

| Type | Purpose |
|------|---------|
| `:array.T` | Ordered collection (allows duplicates) |
| `:set.T` | Unordered collection (unique elements) |
| `:map.K.V` | Key-value pairs |

### Custom Types

| Defined With | Type Notation | Example |
|--------------|---------------|---------|
| `{#} #EnumName` | `:EnumName` | `:OrderStatus` |
| `{#} #StructName` | `:StructName` | `:User` |

### Type Rules

1. **First declaration MUST have type**
2. **No implicit type conversion**
3. **All collection elements must match element type**
4. **Struct fields must match declared types**
5. **Use `:` prefix for type notation**
6. **Use `#` prefix for enum/struct values**

---

## Related Documentation

- [Prefix System](./prefix-system.md) - `:` prefix for types
- [Enums & Structs](./enums-structs.md) - Custom type definitions
- [Variables & Lifecycle](./variables-lifecycle.md) - Type declaration rules
- [Collection Literals](../features/data-structures/collection-literals.md) - Collection syntax

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
