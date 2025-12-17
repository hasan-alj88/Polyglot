tags# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Collection Building Pack Operators
summary: "API reference: Collection Building Pack Opera"
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
topic: Collection Building Pack Operators
summary: API reference: Collection Building Pack Opera
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
# Collection Building Pack Operators

**Build collections from iteration results**

---

## Operators Tree

**Collection Building**
- **\*Into.\***
  - [**\*Into.Array**](./into/into-array.md)
    - `<item`
    - `>array`
  - [**\*Into.Set**](./into/into-set.md)
    - `<item`
    - `>set`
  - [**\*Into.Serial**](./into/into-serial.md)
    - `<path :pg.string`
    - `<item`
    - `>serial`
- **\*String.\***
  - [**\*String.Concat**](./string/string-concat.md)
    - `<string :pg.string`
    - `>concatenated :pg.string`
  - [**\*String.Lines**](./string/string-lines.md)
    - `<line :pg.string`
    - `>lines :pg.string`

---

## Into Operators

Transform iteration items into collections.

- [*Into.Array](./into/into-array.md) - Collect items into array
- [*Into.Set](./into/into-set.md) - Collect unique items into set
- [*Into.Serial](./into/into-serial.md) - Collect items into serial data with field paths

**See:** [Into Package](./into/README.md)

---

## String Operators

Aggregate strings from iterations.

- [*String.Concat](./string/string-concat.md) - Concatenate strings
- [*String.Lines](./string/string-lines.md) - Join strings with newlines

**See:** [String Package](./string/README.md)

---

## Related Documentation

- [Pack Operators Overview](../README.md)
- [Loop System](../../../language/advanced/loop-system.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
