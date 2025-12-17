---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: now
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Now"
summary: "API reference: |U.DT.Now"
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
# |U.DT.Now

**Get current timestamp**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Now >result
```

**Inline:**
```polyglot
\|U.DT.Now
```

---

## Parameters

**Outputs:**
- `>result` :pg.datetime - Current timestamp

---

## Description

Returns the current date and time as a `:pg.datetime` value.

**Timezone:** Returns UTC timestamp.

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
```

**Output:** Current UTC timestamp

---

### Log Timestamp

```polyglot
[r] $timestamp :pg.datetime << \|U.DT.Now
[r] $formatted :pg.string << \|U.DT.Format"{$timestamp, \"YYYY-MM-DD HH:mm:ss\"}"
[r] $log_entry :pg.string << \|U.String.Concat"{$formatted, \" - Event occurred\"}"
```

---

### Calculate Expiration

```polyglot
[r] $created_at :pg.datetime << \|U.DT.Now
[r] $expires_at :pg.datetime << \|U.DT.AddDays"{$created_at, 30}"
```

---

### Measure Duration

```polyglot
[r] $start_time :pg.datetime << \|U.DT.Now

// ... perform operations ...

[r] $end_time :pg.datetime << \|U.DT.Now
[r] $duration_seconds :pg.int << \|U.DT.Diff"{$start_time, $end_time, \"seconds\"}"
```

---

## Common Patterns

### Pattern 1: Timestamping Records

```polyglot
[r] $record :pg.serial << #Serial
   id << $id
   created_at << \|U.DT.Now
   updated_at << \|U.DT.Now
```

### Pattern 2: Check Business Hours

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $hour >= 9 & $hour < 17
   // Within business hours (9 AM - 5 PM)
[^]
   // Outside business hours
```

### Pattern 3: Age Calculation

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $age_years :pg.int << \|U.DT.Diff"{$birthdate, $now, \"years\"}"
```

---

## Related Pipelines

- [|U.DT.Format](./format.md) - Format datetime to string
- [|U.DT.AddDays](./add-days.md) - Add days to datetime
- [|U.DT.Diff](./diff.md) - Calculate time difference

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
