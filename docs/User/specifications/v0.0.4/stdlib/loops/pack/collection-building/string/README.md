---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: String Pack Operators
summary: API reference: String Pack Operators
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
# String Pack Operators

**Aggregate strings from iterations**

---

## Operators

- [*String.Concat](./string-concat.md) - Concatenate strings
- [*String.Lines](./string-lines.md) - Join strings with newlines

---

## Common Pattern

```polyglot
[r] ~ForEach.Array
[~] <array << $words
[~] >item >> $word

   [v] *String.Concat
   [*] <string << $word
   [*] >concatenated >> $sentence
```

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
