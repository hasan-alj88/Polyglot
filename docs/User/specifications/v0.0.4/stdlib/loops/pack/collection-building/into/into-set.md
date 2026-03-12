---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "into-set"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Into.Set"
summary: "API reference: *Into.Set"
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
# *Into.Set

**Collect unique iteration items into a set**

**Category:** Collection Building > Into
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Into.Set
[*] <item
[*] >set
```

---

## Parameters

**Inputs:**
- `<item` - Item from iteration scope to collect

**Outputs:**
- `>set` :pg.set.T - Collected set (unique values) in main scope (T = type of item)

---

## Description

Collects each iteration's `<item` value into a set in the main scope. **Duplicates are automatically removed** - only unique values are stored.

**Key Features:**
- Automatic deduplication
- Unordered collection
- Faster membership testing than arrays

---

## Examples

### Basic Usage

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [v] *Into.Set
   [*] <item << $element
   [*] >set >> $unique_items
```

**Input:** `$items = ["a", "b", "a", "c", "b"]`
**Output:** `$unique_items = {"a", "b", "c"}` (duplicates removed)

---

### Extract Unique Tags

```polyglot
[p] ~ForEach.Array
[~] <array << $articles
[~] >item >> $article

   [r] $tag :pg.string << $article.tag

   [v] *Into.Set
   [*] <item << $tag
   [*] >set >> $all_tags
```

**Input:** Articles with tags `["tech", "news", "tech", "sports", "news"]`
**Output:** `$all_tags = {"tech", "news", "sports"}`

---

### Unique User IDs

```polyglot
[p] ~ForEach.Array
[~] <array << $orders
[~] >item >> $order

   [r] $user_id :pg.string << $order.user_id

   [v] *Into.Set
   [*] <item << $user_id
   [*] >set >> $unique_users
```

---

### Deduplication with Transformation

```polyglot
[p] ~ForEach.Array
[~] <array << $emails
[~] >item >> $email

   [r] $lowercase :pg.string << \|U.String.Lower"{$email}"

   [v] *Into.Set
   [*] <item << $lowercase
   [*] >set >> $unique_emails
```

**Input:** `["Alice@Example.com", "bob@test.com", "alice@example.com"]`
**Output:** `{"alice@example.com", "bob@test.com"}` (case-insensitive dedup)

---

## Comparison with *Into.Array

| Feature | *Into.Array | *Into.Set |
|---------|-------------|-----------|
| **Order** | Preserved | Unordered |
| **Duplicates** | Allowed | Removed |
| **Access** | By index | Membership test |
| **Use Case** | Preserve all values | Unique values only |

**When to use *Into.Set:**
- Need unique values
- Don't care about order
- Want fast membership testing

**When to use *Into.Array:**
- Need all values (including duplicates)
- Order matters
- Need indexed access

---

## Type Inference

Output set type is inferred from `<item` type:

| Item Type | Output Set Type |
|-----------|-----------------|
| `:pg.string` | `:pg.set.pg.string` |
| `:pg.int` | `:pg.set.pg.int` |
| Custom `#Enum` | `:pg.set.EnumName` |

---

## Common Patterns

### Pattern 1: Unique Values from Array

```polyglot
[p] ~ForEach.Array
[~] <array << $items_with_duplicates
[~] >item >> $item

   [v] *Into.Set
   [*] <item << $item
   [*] >set >> $unique_items
```

### Pattern 2: Extract All Categories

```polyglot
[p] ~ForEach.Array
[~] <array << $products
[~] >item >> $product

   [r] $category :pg.string << $product.category

   [v] *Into.Set
   [*] <item << $category
   [*] >set >> $all_categories
```

### Pattern 3: Set Operations After Collection

```polyglot
[p] ~ForEach.Array
[~] <array << $group1
[~] >item >> $item

   [v] *Into.Set
   [*] <item << $item
   [*] >set >> $set1

[p] ~ForEach.Array
[~] <array << $group2
[~] >item >> $item

   [v] *Into.Set
   [*] <item << $item
   [*] >set >> $set2

// Union of sets
[r] $union :pg.set.pg.string << \|Set.Union <set1 << $set1 <set2 << $set2
```

---

## Performance

**Time Complexity:**
- Insertion: O(1) average per item
- Total: O(n) where n = number of unique items

**Space Complexity:** O(k) where k = number of unique items (k ≤ n)

**Deduplication:**
- Automatic during collection
- No post-processing needed
- Memory efficient (stores each unique value once)

---

## Related Operators

- [*Into.Array](./into-array.md) - Collect all values (with duplicates)
- [*Into.Serial](./into-serial.md) - Collect into serial data

---

## See Also

- [Loop System](../../../../User/language/advanced/loop-system.md)
- [Pack Operators Overview](../../README.md)
- [Set Utilities](../../../utilities/README.md#set-operations)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
