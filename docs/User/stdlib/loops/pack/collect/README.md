tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Collect Pack Operators
summary: "API reference: Collect Pack Operators"
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
topic: Collect Pack Operators
summary: API reference: Collect Pack Operators
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
# Collect Pack Operators

**Select specific items from iterations**

---

## Operators Tree

```
Collect
├── *Join.First
│   ├── <item
│   └── >first
├── *Join.Last
│   ├── <item
│   └── >last
├── *Join.Nth
│   ├── <item
│   ├── <n :pg.uint
│   └── >nth
└── *Collect.Errors
    ├── <error :pg.error
    └── >errors :pg.array.pg.error
```

---

## Join Operators

Select specific iteration results.

- [*Join.First](./join-first.md) - Take first completed result
- [*Join.Last](./join-last.md) - Take last completed result
- [*Join.Nth](./join-nth.md) - Take Nth iteration result

---

## Error Collection

- [*Collect.Errors](./collect-errors.md) - Collect all errors from iterations

---

## Common Pattern

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $result << \|Process <input << $element

   [v] *Join.First              // Take first result
   [*] <item << $result
   [*] >first >> $fastest
```

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
