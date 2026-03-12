---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "zip-arrays"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~Zip.Arrays
summary: "API reference: ~Zip.Arrays"
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# ~Zip.Arrays

**Combine multiple arrays element-wise**

**Category:** Unpack Operators > Zip
**Since:** v0.0.1

---

## Signature

### Full Syntax

```polyglot
[execution_marker] ~Zip.Arrays
[~] <arrays.0 :pg.array.*
[~] <arrays.1 :pg.array.*
[~] <arrays.N :pg.array.*
[~] >items.0 :*
[~] >items.1 :*
[~] >items.N :*
```

### Shortcut Syntax

```polyglot
[execution_marker] ~Zip.Arrays
[~] <<< $array0
[~] <<< $array1
[~] <<< $arrayN
[~] >>> $item0
[~] >>> $item1
[~] >>> $itemN
```

---

## Parameters

**Inputs (variadic):**
- `<arrays.0`, `<arrays.1`, ... - Arrays to zip together
- OR `<<<` - Shortcut for `<arrays.N <<` (implied indexing)

**Outputs (variadic):**
- `>items.0`, `>items.1`, ... - Corresponding elements from each array
- OR `>>>` - Shortcut for `>items.N >>` (implied indexing)

**Type:** Input signature `:pg.array.serial`, output signature `:pg.array.serial`

---

## Description

Zips multiple arrays together element-wise, creating iterations where each iteration receives the Nth element from each input array. Stops when the **shortest array is exhausted**.

**Key characteristics:**
- **Variadic inputs** - Zip any number of arrays (2 or more)
- **Element-wise pairing** - Nth iteration gets Nth element from each array
- **Shortest array wins** - Iteration stops at shortest array length
- **Shortcut syntax** - Use `<<<` and `>>>` for cleaner code

**Use when:**
- Combining parallel arrays
- Processing corresponding elements
- Building paired/grouped data

---

## Examples

### Basic Usage - Two Arrays (Shortcut Syntax)

```polyglot
[p] ~Zip.Arrays
[~] <<< $names
[~] <<< $ages
[~] >>> $name
[~] >>> $age

   [r] $person :pg.string << \|U.String.Concat"{$name, \" is \", $age, \" years old\"}"

   [v] *Into.Array
   [*] <item << $person
   [*] >array >> $descriptions
[v]
```

**Input:**
- `$names = ["Alice", "Bob", "Charlie"]`
- `$ages = [30, 25, 35]`

**Output:**
```
$descriptions = [
  "Alice is 30 years old",
  "Bob is 25 years old",
  "Charlie is 35 years old"
]
```

---

### Basic Usage - Full Syntax

```polyglot
[p] ~Zip.Arrays
[~] <arrays.0 << $names
[~] <arrays.1 << $ages
[~] >items.0 >> $name
[~] >items.1 >> $age

   [r] $person :pg.string << \|U.String.Concat"{$name, \" is \", $age, \" years old\"}"

   [v] *Into.Array
   [*] <item << $person
   [*] >array >> $descriptions
[v]
```

**Same result as shortcut syntax above.**

---

### Three Arrays - Build Records

```polyglot
[p] ~Zip.Arrays
[~] <<< $names
[~] <<< $emails
[~] <<< $departments
[~] >>> $name
[~] >>> $email
[~] >>> $dept

   [r] $employee :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $employee.name << $name
   [r] $employee.email << $email
   [r] $employee.department << $dept

   [v] *Into.Array
   [*] <item << $employee
   [*] >array >> $employees
[v]
```

**Output:**
```json
[
  {"name": "Alice", "email": "alice@example.com", "department": "Engineering"},
  {"name": "Bob", "email": "bob@example.com", "department": "Sales"},
  ...
]
```

---

### Build Key-Value Pairs

```polyglot
[p] ~Zip.Arrays
[~] <<< $keys
[~] <<< $values
[~] >>> $key
[~] >>> $value

   [r] $path :pg.string << $key

   [v] *Into.Serial
   [*] <path << $path
   [*] <item << $value
   [*] >serial >> $config
[v]
```

**Input:**
- `$keys = ["database.host", "database.port", "cache.enabled"]`
- `$values = ["localhost", 5432, true]`

**Output:**
```json
{
  "database": {
    "host": "localhost",
    "port": 5432
  },
  "cache": {
    "enabled": true
  }
}
```

---

### Shortest Array Wins

```polyglot
[p] ~Zip.Arrays
[~] <<< $short_array
[~] <<< $long_array
[~] >>> $short_item
[~] >>> $long_item

   [v] *Into.Array
   [*] <item << $short_item
   [*] >array >> $result
[v]
```

**Input:**
- `$short_array = [1, 2, 3]`
- `$long_array = [10, 20, 30, 40, 50]`

**Output:**
- `$result = [1, 2, 3]`

**Only 3 iterations occur** (shortest array length).

---

### Four Arrays - Multi-Column Processing

```polyglot
[p] ~Zip.Arrays
[~] <<< $col1
[~] <<< $col2
[~] <<< $col3
[~] <<< $col4
[~] >>> $val1
[~] >>> $val2
[~] >>> $val3
[~] >>> $val4

   [r] $row :pg.string << \|U.String.Concat"{$val1, \",\", $val2, \",\", $val3, \",\", $val4}"

   [v] *String.Lines
   [*] <line << $row
   [*] >lines >> $csv_content
[v]
```

---

## Shortcut Syntax

**The `<<<` and `>>>` operators provide implied indexing:**

### Input Shortcut `<<<`

**Full syntax:**
```polyglot
[~] <arrays.0 << $array0
[~] <arrays.1 << $array1
[~] <arrays.2 << $array2
```

**Shortcut:**
```polyglot
[~] <<< $array0
[~] <<< $array1
[~] <<< $array2
```

**Indices 0, 1, 2 are implied in order.**

### Output Shortcut `>>>`

**Full syntax:**
```polyglot
[~] >items.0 >> $item0
[~] >items.1 >> $item1
[~] >items.2 >> $item2
```

**Shortcut:**
```polyglot
[~] >>> $item0
[~] >>> $item1
[~] >>> $item2
```

**Indices 0, 1, 2 are implied in order.**

---

## Empty Array Handling

**If any array is empty, no iterations occur:**

```polyglot
[p] ~Zip.Arrays
[~] <<< $empty
[~] <<< $other
[~] >>> $a
[~] >>> $b

   [v] *Into.Array
   [*] <item << $a
   [*] >array >> $result
[v]
```

**Input:**
- `$empty = []`
- `$other = [1, 2, 3]`

**Output:**
- `$result = []`

---

## Type Inference

**Each output type matches corresponding array element type:**

```polyglot
[~] <<< $names              // :pg.array.pg.string
[~] <<< $ages               // :pg.array.pg.int
[~] >>> $name               // :pg.string
[~] >>> $age                // :pg.int
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~Zip.Arrays
[~] <<< $array0
[~] <<< $array1
[~] >>> $item0
[~] >>> $item1
   // Element pairs processed in order
```

**Use when:**
- Order matters
- Operations have dependencies
- Deterministic results required

### Parallel [p]

```polyglot
[p] ~Zip.Arrays
[~] <<< $array0
[~] <<< $array1
[~] >>> $item0
[~] >>> $item1
   // All pairs processed concurrently
[v]
```

**Use when:**
- Independent operations
- Performance critical
- Safe for concurrent execution

---

## Common Patterns

### Pattern 1: Build User Records
```polyglot
[p] ~Zip.Arrays
[~] <<< $usernames
[~] <<< $passwords
[~] <<< $emails
[~] >>> $username
[~] >>> $password
[~] >>> $email
   [r] $user << \|CreateUser <username << $username <password << $password <email << $email
   [v] *Into.Array
   [*] <item << $user
   [*] >array >> $users
[v]
```

### Pattern 2: Coordinate Pairs
```polyglot
[p] ~Zip.Arrays
[~] <<< $x_coords
[~] <<< $y_coords
[~] >>> $x
[~] >>> $y
   [r] $point :pg.serial << \|U.Data.CreateSerial"{}"
   [r] $point.x << $x
   [r] $point.y << $y
   [v] *Into.Array
   [*] <item << $point
   [*] >array >> $points
[v]
```

### Pattern 3: Validate Pairs
```polyglot
[p] ~Zip.Arrays
[~] <<< $expected_values
[~] <<< $actual_values
[~] >>> $expected
[~] >>> $actual
   [f] $expected != $actual
      [r] $error :pg.string << \|U.String.Concat"{\"Expected \", $expected, \" but got \", $actual}"
      [v] *Into.Array
      [*] <item << $error
      [*] >array >> $validation_errors
[v]
```

### Pattern 4: Build Dictionary/Map
```polyglot
[r] ~Zip.Arrays
[~] <<< $keys
[~] <<< $values
[~] >>> $key
[~] >>> $value
   [v] *Into.Serial
   [*] <path << $key
   [*] <item << $value
   [*] >serial >> $dictionary
```

---

## Performance

**Time Complexity:** O(min(n₁, n₂, ..., nₖ)) where nᵢ = length of array i

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(min_length) for concurrent execution

**Stops at shortest array** - Only processes as many elements as shortest array.

---

## Comparison with Other Operators

| Operator | Collection Type | Order | Use Case |
|----------|-----------------|-------|----------|
| **~Zip.Arrays** | Arrays | Deterministic | Parallel arrays |
| **~Zip.Sets** | Sets | Non-deterministic | Unique value pairs |
| **~ForEach.Array** | Single array | Deterministic | Single collection |

**When to use ~Zip.Arrays:**
- Multiple parallel arrays
- Corresponding element processing
- Building paired data

**When to use ~Zip.Sets:**
- Processing unique value pairs
- Order doesn't matter

**When to use ~ForEach.Array:**
- Single array iteration
- No pairing needed

---

## Related Operators

- [~Zip.Sets](./zip-sets.md) - Zip multiple sets
- [~ForEach.Array](../foreach/foreach-array.md) - Single array iteration
- [*Into.Array](../../pack-operators/collection-building/into/into-array.md) - Collect results

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Unpack Operators Overview](../README.md)
- [Variadic Operators](../../../features/core-features/variadic-operators.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
