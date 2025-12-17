---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "join-first"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Join.First"
summary: "API reference: *Join.First"
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
# *Join.First

**Take first completed iteration result**

**Category:** Pack Operators > Collect
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Join.First
[*] <item
[*] >first
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope

**Outputs:**
- `>first` - First completed result in main scope

---

## Description

Collects only the **first completed** iteration result and discards all others. In sequential loops, this is the first iteration. In parallel loops, this is the **fastest** to complete.

**Key characteristics:**
- **Race condition** - In parallel loops, first to finish wins
- **Discards others** - All subsequent iterations are ignored
- **Type inference** - Output type matches item type

**Use when:**
- Only need first result
- Racing multiple operations
- Quick failure/success detection
- Performance optimization (stop after first)

---

## Examples

### Basic Usage - Sequential

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|Process <input << $element

   [v] *Join.First
   [*] <item << $result
   [*] >first >> $first_result
```

**Input:** `$items = [1, 2, 3, 4, 5]`
**Output:** `$first_result = Process(1)`

**Only the first iteration result is collected.**

---

### Parallel - Race to First Result

```polyglot
[p] ~ForEach.Array
[~] <array << $servers
[~] >item >> $server

   [r] $response << \|HTTP.Get <url << $server.url

   [v] *Join.First
   [*] <item << $response
   [*] >first >> $fastest_response
[v]
```

**All servers are queried in parallel, first response wins.**

---

### Find First Match

```polyglot
[r] ~ForEach.Array
[~] <array << $candidates
[~] >item >> $candidate

   [r] $matches :pg.bool << \|CheckCondition <input << $candidate

   [y] $matches == #True
      [v] *Join.First
      [*] <item << $candidate
      [*] >first >> $first_match
```

**Stops and returns first candidate that matches condition.**

---

### First Successful API Call

```polyglot
[p] ~ForEach.Array
[~] <array << $backup_urls
[~] >item >> $url

   [z] $data << \|API.Fetch <url << $url
      [!] !Network.*
         // Skip failed requests

   [v] *Join.First
   [*] <item << $data
   [*] >first >> $successful_data
[v]
```

**Returns data from first successful API call.**

---

### Early Exit Pattern

```polyglot
[r] ~ForEach.Array
[~] <array << $large_dataset
[~] >item >> $record

   [r] $is_target :pg.bool << \|IsTarget <record << $record

   [y] $is_target == #True
      [v] *Join.First
      [*] <item << $record
      [*] >first >> $target_record
      // Stops iteration after first match
```

---

## No Result Handling

**If no iteration produces a result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $element

   [v] *Join.First
   [*] <item << $element
   [*] >first >> $result
```

**Input:** `$empty = []`
**Behavior:** No value assigned to `$result` (variable undefined)

**Handle with default:**
```polyglot
[r] $result << <~ $default_value ~> $first_result
```

---

## Type Inference

**Output type matches item type:**

| Item Type | First Type |
|-----------|------------|
| `:pg.int` | `:pg.int` |
| `:pg.string` | `:pg.string` |
| `:pg.serial` | `:pg.serial` |

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [v] *Join.First
   [*] <item << $element
   [*] >first >> $first_item
```

**Result:** First element in array order.

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|SlowOperation <input << $element

   [v] *Join.First
   [*] <item << $result
   [*] >first >> $fastest
[v]
```

**Result:** First operation to complete (non-deterministic).

---

## Performance Optimization

**In parallel mode, remaining iterations may be cancelled after first completes:**

```polyglot
[p] ~ForEach.Array
[~] <array << $many_items
[~] >item >> $element

   [r] $result << \|ExpensiveOperation <input << $element

   [v] *Join.First
   [*] <item << $result
   [*] >first >> $quick_result
[v]
```

**Optimization:** Runtime may cancel remaining iterations for efficiency.

---

## Common Patterns

### Pattern 1: First Available
```polyglot
[p] ~ForEach.Array
[~] <array << $sources
[~] >item >> $source
   [r] $data << \|FetchData <source << $source
   [v] *Join.First
   [*] <item << $data
   [*] >first >> $first_available
[v]
```

### Pattern 2: Early Exit Search
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [y] $item.id == $target_id
      [v] *Join.First
      [*] <item << $item
      [*] >first >> $found_item
```

### Pattern 3: Fastest Mirror
```polyglot
[p] ~ForEach.Array
[~] <array << $mirror_urls
[~] >item >> $url
   [r] $content << \|HTTP.Get <url << $url
   [v] *Join.First
   [*] <item << $content
   [*] >first >> $fastest_content
[v]
```

### Pattern 4: First Valid Result
```polyglot
[r] ~ForEach.Array
[~] <array << $attempts
[~] >item >> $attempt
   [z] $result << \|TryOperation <input << $attempt
      [!] *!
         // Skip errors

   [v] *Join.First
   [*] <item << $result
   [*] >first >> $first_success
```

---

## Performance

**Time Complexity:**
- Sequential: O(1) - only first iteration
- Parallel: O(min(t₁, t₂, ..., tₙ)) - fastest operation

**Space Complexity:** O(1) - single result

**Early termination:** May stop remaining iterations after first completes.

---

## Comparison with Other Operators

| Operator | Collects | Result | Use Case |
|----------|----------|--------|----------|
| **\*Join.First** | First | Single item | First result |
| **\*Join.Last** | Last | Single item | Final result |
| **\*Join.Nth** | Nth | Single item | Specific position |
| **\*Into.Array** | All | Array | All results |

**When to use \*Join.First:**
- Only need first result
- Racing operations
- Early exit on success
- Performance critical

**When to use \*Into.Array:**
- Need all results
- Order matters
- Complete collection required

---

## Related Operators

- [*Join.Last](./join-last.md) - Take last result
- [*Join.Nth](./join-nth.md) - Take Nth result
- [*Into.Array](../collection-building/into/into-array.md) - Collect all results

---

## See Also

- [Loop System](../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
