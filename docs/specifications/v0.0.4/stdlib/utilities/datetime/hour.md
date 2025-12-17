---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: hour
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Hour"
summary: "API reference: |U.DT.Hour"
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
# |U.DT.Hour

**Extract hour component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Hour <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Hour"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Hour (0-23)

---

## Description

Extracts the hour component from a datetime value as an integer from 0 to 23.

**24-hour format:**
- 0 = Midnight (12 AM)
- 12 = Noon (12 PM)
- 23 = 11 PM

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15 14:30:00\", \"YYYY-MM-DD HH:mm:ss\"}"
[r] $hour :pg.int << \|U.DT.Hour"{$dt}"
```

**Output:** `$hour = 14` (2 PM)

---

### Current Hour

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_hour :pg.int << \|U.DT.Hour"{$now}"
```

---

### Check Business Hours

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $hour >= 9 & $hour < 17
   // Within business hours (9 AM - 5 PM)
[^]
   // Outside business hours
```

---

### Filter by Time of Day

```polyglot
[r] ~ForEach.Array
[~] <array << $logs
[~] >item >> $log
   [r] $hour :pg.int << \|U.DT.Hour"{$log.\"timestamp\"}"

   [y] $hour >= 0 & $hour < 6
      // Overnight logs (midnight to 6 AM)
      [v] *Into.Array
      [*] <item << $log
      [*] >array >> $overnight_logs
```

---

## Common Patterns

### Pattern 1: Time-Based Greeting

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $hour >= 5 & $hour < 12
   [r] $greeting :pg.string << "Good morning"
[&] $hour >= 12 & $hour < 17
   [r] $greeting :pg.string << "Good afternoon"
[&] $hour >= 17 & $hour < 21
   [r] $greeting :pg.string << "Good evening"
[^]
   [r] $greeting :pg.string << "Good night"
```

### Pattern 2: Peak Hours Detection

```polyglot
[r] $hour :pg.int << \|U.DT.Hour"{$request_time}"

[y] ($hour >= 8 & $hour < 10) | ($hour >= 17 & $hour < 19)
   // Peak traffic hours (8-10 AM or 5-7 PM)
   [r] $is_peak :pg.bool << true
[^]
   [r] $is_peak :pg.bool << false
```

### Pattern 3: Scheduled Tasks

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $hour :pg.int << \|U.DT.Hour"{$now}"

[y] $hour == 2
   // Run maintenance at 2 AM
   [r] !RunMaintenance
```

---

## Convert to 12-Hour Format

```polyglot
[r] $hour24 :pg.int << \|U.DT.Hour"{$datetime}"

[y] $hour24 == 0
   [r] $hour12 :pg.int << 12
   [r] $period :pg.string << "AM"
[&] $hour24 < 12
   [r] $hour12 :pg.int << $hour24
   [r] $period :pg.string << "AM"
[&] $hour24 == 12
   [r] $hour12 :pg.int << 12
   [r] $period :pg.string << "PM"
[^]
   [r] $hour12 :pg.int << \|U.Math.Subtract"{$hour24, 12}"
   [r] $period :pg.string << "PM"
```

---

## Related Pipelines

- [|U.DT.Minute](./minute.md) - Extract minute component
- [|U.DT.Second](./second.md) - Extract second component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
