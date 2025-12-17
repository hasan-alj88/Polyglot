---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "foreach-serialarray"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: ~ForEach.SerialArray
summary: "API reference: ~ForEach.SerialArray"
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
# ~ForEach.SerialArray

**Iterate array at specific path in serial data**

**Category:** Unpack Operators > ForEach
**Since:** v0.0.1

---

## Signature

```polyglot
[execution_marker] ~ForEach.SerialArray
[~] <serial :pg.serial
[~] <path :pg.string
[~] >item :pg.serial
```

---

## Parameters

**Inputs:**
- `<serial` :pg.serial - Serial data containing the array
- `<path` :pg.string - Path to the array (e.g., `"users"` or `"data.items"`)

**Outputs:**
- `>item` :pg.serial - Each array element (always `:pg.serial`)

---

## Description

Iterates over an array located at a specific path within `:pg.serial` data. Unlike `~ForEach.Serial` which iterates all fields, this operator targets a **specific array** by path.

**Key characteristics:**
- **Path-based access** - Specify which array to iterate
- **Serial elements** - Each element is treated as `:pg.serial`
- **Nested access** - Can target deeply nested arrays

**Use when:**
- Array is embedded in serial data
- Know the array's path
- Need to iterate specific array
- Working with JSON/YAML structures

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "users"
[~] >item >> $user

   [r] $name :pg.string << $user.name
   [r] $email :pg.string << $user.email

   [v] *Into.Array
   [*] <item << $email
   [*] >array >> $emails
```

**Input:**
```json
{
  "users": [
    {"name": "Alice", "email": "alice@example.com"},
    {"name": "Bob", "email": "bob@example.com"}
  ]
}
```

**Output:**
```
$emails = ["alice@example.com", "bob@example.com"]
```

---

### Nested Array Path

```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $api_response
[~] <path << "data.results"
[~] >item >> $result

   [r] $id :pg.int << $result.id

   [v] *Into.Array
   [*] <item << $id
   [*] >array >> $result_ids
```

**Input:**
```json
{
  "status": "success",
  "data": {
    "results": [
      {"id": 1, "value": "foo"},
      {"id": 2, "value": "bar"}
    ]
  }
}
```

**Output:**
```
$result_ids = [1, 2]
```

---

### Process Multiple Arrays

```polyglot
// Process first array
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "orders"
[~] >item >> $order
   [r] $order_total << $order.total
   [v] *Math.Sum
   [*] <item << $order_total
   [*] >sum >> $orders_total

// Process second array
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "refunds"
[~] >item >> $refund
   [r] $refund_amount << $refund.amount
   [v] *Math.Sum
   [*] <item << $refund_amount
   [*] >sum >> $refunds_total
```

---

### Transform Array Elements

```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $config
[~] <path << "servers"
[~] >item >> $server

   [r] $hostname :pg.string << $server.host
   [r] $port :pg.int << $server.port
   [r] $url :pg.string << \|U.String.Concat"{\"http://\", $hostname, \":\", $port}"

   [v] *Into.Array
   [*] <item << $url
   [*] >array >> $server_urls
```

**Input:**
```json
{
  "servers": [
    {"host": "server1.example.com", "port": 8080},
    {"host": "server2.example.com", "port": 8081}
  ]
}
```

**Output:**
```
$server_urls = ["http://server1.example.com:8080", "http://server2.example.com:8081"]
```

---

### Filter Array Elements

```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "products"
[~] >item >> $product

   [r] $price :pg.float << $product.price

   [y] $price > 100.0
      [v] *Into.Array
      [*] <item << $product
      [*] >array >> $expensive_products
```

---

## Path Syntax

**Path notation uses dots for nesting:**

| Serial Structure | Path | Result |
|------------------|------|--------|
| `{"users": [...]}` | `"users"` | Iterates users array |
| `{"data": {"items": [...]}}` | `"data.items"` | Iterates nested items array |
| `{"response": {"payload": {"records": [...]}}}` | `"response.payload.records"` | Iterates deeply nested records |

**Array index access in paths:**
```polyglot
// Access array inside an array element
[~] <path << "orders.0.items"
// Iterates items array of first order
```

---

## Empty Array Handling

**Empty array produces no iterations:**

```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "items"
[~] >item >> $item

   [v] *Into.Array
   [*] <item << $item
   [*] >array >> $result
```

**Input:** `{"items": []}`
**Output:** `$result = []`

---

## Missing Path Handling

**If path doesn't exist, produces error:**

```polyglot
[z] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "nonexistent"
[~] >item >> $item
   [!] !Serial.PathNotFound
      // Handle missing path
```

---

## Type Handling

**All items are treated as `:pg.serial`:**

```polyglot
[~] >item :pg.serial
```

**Even primitive values in array are wrapped:**

```polyglot
// Array: [1, 2, 3]
[r] ~ForEach.SerialArray
[~] <serial << $data
[~] <path << "numbers"
[~] >item >> $num_serial
   // $num_serial is :pg.serial, access with: $num_serial (auto-converts)
```

---

## Common Patterns

### Pattern 1: Extract Field from Array Elements
```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $api_response
[~] <path << "results"
[~] >item >> $result
   [r] $id :pg.string << $result.id
   [v] *Into.Array
   [*] <item << $id
   [*] >array >> $ids
```

### Pattern 2: Aggregate Array Values
```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $report
[~] <path << "sales"
[~] >item >> $sale
   [r] $amount :pg.float << $sale.amount
   [v] *Math.Sum
   [*] <item << $amount
   [*] >sum >> $total_sales
```

### Pattern 3: Build New Array with Transformation
```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $source
[~] <path << "users"
[~] >item >> $user
   [r] $active :pg.bool << $user.active
   [y] $active == #True
      [r] $new_user :pg.serial << \|U.Data.CreateSerial"{}"
      [r] $new_user.name << $user.name
      [r] $new_user.email << $user.email
      [v] *Into.Array
      [*] <item << $new_user
      [*] >array >> $active_users
```

### Pattern 4: Validate Array Elements
```polyglot
[r] ~ForEach.SerialArray
[~] <serial << $input
[~] <path << "entries"
[~] >item >> $entry
   [z] $validated << \|ValidateEntry <entry << $entry
      [!] !Validation.*
         [r] $id :pg.string << $entry.id
         [v] *Into.Array
         [*] <item << $id
         [*] >array >> $invalid_entry_ids
```

---

## Performance

**Time Complexity:** O(n) where n = array length at path

**Space Complexity:**
- Sequential: O(1) per iteration
- Parallel: O(n) for concurrent execution

**Path lookup:** O(d) where d = path depth

---

## Comparison with Other Operators

| Operator | Path Required | Outputs Path | All Fields | Use Case |
|----------|---------------|--------------|------------|----------|
| **~ForEach.SerialArray** | Yes (input) | No | No | Specific array |
| **~ForEach.Serial** | No | Yes (output) | Yes | All fields |
| **~ForEach.Array** | No | No | N/A | Direct array |

**When to use ~ForEach.SerialArray:**
- Array is embedded in serial data
- Know the array's path
- Don't need path output
- Processing specific array

**When to use ~ForEach.Serial:**
- Don't know structure
- Need field paths
- Process all fields

**When to use ~ForEach.Array:**
- Have direct array reference
- Not embedded in serial

---

## Related Operators

- [~ForEach.Serial](./foreach-serial.md) - Iterate all serial fields
- [~ForEach.Array](./foreach-array.md) - Standard array iteration
- [*Into.Serial](../../pack-operators/collection-building/into/into-serial.md) - Build serial with paths

---

## See Also

- [Serial Load Block](../../../language/advanced/serial-load-block.md)
- [Loop System](../../../language/advanced/loop-system.md)
- [Data Utilities](../../utilities/data/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
