---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: year
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Year"
summary: "API reference: |U.DT.Year"
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
complexity: low

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
# |U.DT.Year

**Extract year component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Year <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Year"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Year (e.g., 2025)

---

## Description

Extracts the year component from a datetime value as a 4-digit integer.

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $year :pg.int << \|U.DT.Year"{$dt}"
```

**Output:** `$year = 2025`

---

### Current Year

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_year :pg.int << \|U.DT.Year"{$now}"
```

---

### Group Records by Year

```polyglot
[r] ~ForEach.Array
[~] <array << $records
[~] >item >> $record
   [r] $dt :pg.datetime << $record."created_at"
   [r] $year :pg.int << \|U.DT.Year"{$dt}"

   [y] $year == 2025
      [v] *Into.Array
      [*] <item << $record
      [*] >array >> $records_2025
```

---

### Copyright Notice

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $year :pg.int << \|U.DT.Year"{$now}"
[r] $year_str :pg.string << \|U.String.Concat"{$year}"
[r] $copyright :pg.string << \|U.String.Concat"{\"© \", $year_str, \" Company Name\"}"
```

**Output:** `"© 2025 Company Name"`

---

## Common Patterns

### Pattern 1: Filter by Year

```polyglot
[r] $target_year :pg.int << 2024

[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event
   [r] $event_year :pg.int << \|U.DT.Year"{$event.\"date\"}"

   [y] $event_year == $target_year
      [v] *Into.Array
      [*] <item << $event
      [*] >array >> $filtered
```

### Pattern 2: Age Verification

```polyglot
[r] $birth_year :pg.int << \|U.DT.Year"{$birthdate}"
[r] $current_year :pg.int << \|U.DT.Year"{$now}"
[r] $age_approx :pg.int << \|U.Math.Subtract"{$current_year, $birth_year}"

[y] $age_approx >= 18
   // User is at least 18
```

### Pattern 3: Year Range Check

```polyglot
[r] $year :pg.int << \|U.DT.Year"{$date}"

[y] $year >= 2020 & $year <= 2025
   // Date within 2020-2025 range
```

---

## Related Pipelines

- [|U.DT.Month](./month.md) - Extract month component
- [|U.DT.Day](./day.md) - Extract day component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
