tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Into Pack Operators
summary: "API reference: Into Pack Operators"
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
 BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Into Pack Operators
summary: API reference: Into Pack Operators
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
# Into Pack Operators

**Collect iteration items into collections**

---

## Operators

- [*Into.Array](./into-array.md) - Collect into array
- [*Into.Set](./into-set.md) - Collect into set (unique values)
- [*Into.Serial](./into-serial.md) - Collect into serial data

---

## Common Pattern

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Transform <input << $element

   [v] *Into.Array              // Collect into array
   [*] <item << $processed
   [*] >array >> $results
```

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
