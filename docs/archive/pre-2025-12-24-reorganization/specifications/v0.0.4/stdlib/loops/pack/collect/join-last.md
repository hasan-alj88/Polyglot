---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "join-last"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Join.Last"
summary: "API reference: *Join.Last"
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
# *Join.Last

**Take last completed iteration result**

**Category:** Pack Operators > Collect
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Join.Last
[*] <item
[*] >last
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope

**Outputs:**
- `>last` - Last completed result in main scope

---

## Description

Collects only the **last completed** iteration result and discards all others. In sequential loops, this is the final iteration. In parallel loops, this is the **slowest** to complete.

**Key characteristics:**
- **Final result** - In sequential, last in order
- **Slowest wins** - In parallel, last to finish
- **Discards others** - All previous iterations are ignored
- **Type inference** - Output type matches item type

**Use when:**
- Only need final result
- Accumulation result (though *Math.Sum is better)
- Last state matters
- Final iteration has special meaning

---

## Examples

### Basic Usage - Sequential

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|Process <input << $element

   [v] *Join.Last
   [*] <item << $result
   [*] >last >> $last_result
```

**Input:** `$items = [1, 2, 3, 4, 5]`
**Output:** `$last_result = Process(5)`

**Only the last iteration result is collected.**

---

### Get Final State

```polyglot
[r] ~ForEach.Array
[~] <array << $operations
[~] >item >> $operation

   [r] $state << \|ApplyOperation <state << $current_state <op << $operation

   [v] *Join.Last
   [*] <item << $state
   [*] >last >> $final_state
```

**Returns state after all operations applied.**

---

### Last Valid Result

```polyglot
[r] ~ForEach.Array
[~] <array << $attempts
[~] >item >> $attempt

   [z] $result << \|TryOperation <input << $attempt
      [!] *!
         // Skip errors

   [v] *Join.Last
   [*] <item << $result
   [*] >last >> $last_success
```

**Returns last successful result.**

---

### Parallel - Slowest to Complete

```polyglot
[p] ~ForEach.Array
[~] <array << $servers
[~] >item >> $server

   [r] $response << \|HTTP.Get <url << $server.url

   [v] *Join.Last
   [*] <item << $response
   [*] >last >> $slowest_response
[v]
```

**All servers queried in parallel, slowest response is returned.**

---

### Final Iteration Value

```polyglot
[r] ~Iter.Range
[~] <from << 0
[~] <to << 10
[~] >index >> $i

   [r] $squared :pg.int << \|U.Math.Multiply"{$i, $i}"

   [v] *Join.Last
   [*] <item << $squared
   [*] >last >> $last_square
```

**Output:** `$last_square = 81` (9²)

---

## No Result Handling

**If no iteration produces a result:**

```polyglot
[r] ~ForEach.Array
[~] <array << $empty
[~] >item >> $element

   [v] *Join.Last
   [*] <item << $element
   [*] >last >> $result
```

**Input:** `$empty = []`
**Behavior:** No value assigned to `$result` (variable undefined)

**Handle with default:**
```polyglot
[r] $result << <~ $default_value ~> $last_result
```

---

## Type Inference

**Output type matches item type:**

| Item Type | Last Type |
|-----------|-----------|
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

   [v] *Join.Last
   [*] <item << $element
   [*] >last >> $last_item
```

**Result:** Last element in array order.

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|SlowOperation <input << $element

   [v] *Join.Last
   [*] <item << $result
   [*] >last >> $slowest
[v]
```

**Result:** Last operation to complete (non-deterministic which operation).

---

## Common Patterns

### Pattern 1: Final State
```polyglot
[r] ~ForEach.Array
[~] <array << $updates
[~] >item >> $update
   [r] $new_state << \|ApplyUpdate <state << $state <update << $update
   [v] *Join.Last
   [*] <item << $new_state
   [*] >last >> $final_state
```

### Pattern 2: Last Modified
```polyglot
[r] ~ForEach.Array
[~] <array << $files
[~] >item >> $file
   [r] $timestamp :pg.int << $file.modified
   [v] *Join.Last
   [*] <item << $timestamp
   [*] >last >> $latest_modification
```

### Pattern 3: Completion Marker
```polyglot
[r] ~ForEach.Array
[~] <array << $tasks
[~] >item >> $task
   [r] $completed << \|ExecuteTask <task << $task
   [v] *Join.Last
   [*] <item << $completed
   [*] >last >> $all_done_marker
```

### Pattern 4: Last Successful Attempt
```polyglot
[r] ~ForEach.Array
[~] <array << $retry_attempts
[~] >item >> $attempt
   [z] $result << \|TryConnect <attempt << $attempt
      [!] *!
         // Skip failures
   [v] *Join.Last
   [*] <item << $result
   [*] >last >> $final_attempt
```

---

## Performance

**Time Complexity:**
- Sequential: O(n) - all iterations must complete
- Parallel: O(max(t₁, t₂, ..., tₙ)) - slowest operation

**Space Complexity:** O(1) - single result

**Note:** All iterations run, but only last result is kept.

---

## Use Cases vs Anti-Patterns

### Good Use Cases

**Final state:**
```polyglot
[r] ~ForEach.Array
[~] <array << $steps
[~] >item >> $step
   [r] $state << \|ApplyStep <state << $state <step << $step
   [v] *Join.Last
   [*] <item << $state
   [*] >last >> $final_state
```

### Anti-Patterns

**Summing values** (use *Math.Sum instead):
```polyglot
// WRONG - Don't do this
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $sum << \|U.Math.Add"{$running_sum, $num}"
   [v] *Join.Last
   [*] <item << $sum
   [*] >last >> $total

// CORRECT - Use this instead
[r] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [v] *Math.Sum
   [*] <item << $num
   [*] >sum >> $total
```

---

## Comparison with Other Operators

| Operator | Collects | Result | Use Case |
|----------|----------|--------|----------|
| **\*Join.Last** | Last | Single item | Final result |
| **\*Join.First** | First | Single item | First result |
| **\*Join.Nth** | Nth | Single item | Specific position |
| **\*Into.Array** | All | Array | All results |

**When to use \*Join.Last:**
- Only need final result
- Final state matters
- Last iteration has special meaning
- Completion marker

**When to use \*Into.Array:**
- Need all results
- Order matters
- Complete collection required

---

## Related Operators

- [*Join.First](./join-first.md) - Take first result
- [*Join.Nth](./join-nth.md) - Take Nth result
- [*Into.Array](../collection-building/into/into-array.md) - Collect all results

---

## See Also

- [Loop System](../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
