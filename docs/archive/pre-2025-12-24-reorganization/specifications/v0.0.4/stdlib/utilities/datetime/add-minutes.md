---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "add-minutes"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.AddMinutes"
summary: "API reference: |U.DT.AddMinutes"
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
# |U.DT.AddMinutes

**Add or subtract minutes**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.AddMinutes <datetime <minutes >result
```

**Inline:**
```polyglot
\|U.DT.AddMinutes"{$datetime, $minutes}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime
- `<minutes` :pg.int - Number of minutes to add (negative to subtract)

**Outputs:**
- `>result` :pg.datetime - Modified datetime

---

## Description

Adds or subtracts the specified number of minutes to/from a datetime value.

**Use negative values to subtract minutes.**

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $later :pg.datetime << \|U.DT.AddMinutes"{$now, 15}"
```

---

### Add One Hour (60 minutes)

```polyglot
[r] $one_hour_later :pg.datetime << \|U.DT.AddMinutes"{$now, 60}"
```

---

### Subtract Minutes

```polyglot
[r] $fifteen_min_ago :pg.datetime << \|U.DT.AddMinutes"{$now, -15}"
```

---

### Meeting Reminder

```polyglot
[r] $meeting_time :pg.datetime << \|U.DT.Parse"{$meeting_str, \"YYYY-MM-DD HH:mm:ss\"}"
[r] $reminder_time :pg.datetime << \|U.DT.AddMinutes"{$meeting_time, -15}"
```

**Output:** 15 minutes before meeting

---

### Calculate Processing Time

```polyglot
[r] $estimated_minutes :pg.int << 45
[r] $start :pg.datetime << \|U.DT.Now
[r] $eta :pg.datetime << \|U.DT.AddMinutes"{$start, $estimated_minutes}"
```

---

## Common Patterns

### Pattern 1: Short-Lived Token

```polyglot
[r] $issued_at :pg.datetime << \|U.DT.Now
[r] $expires_at :pg.datetime << \|U.DT.AddMinutes"{$issued_at, 30}"
```

### Pattern 2: Rate Limiting Window

```polyglot
[r] $first_request :pg.datetime << \|U.DT.Now
[r] $window_end :pg.datetime << \|U.DT.AddMinutes"{$first_request, 5}"

// Allow N requests within 5-minute window
```

### Pattern 3: Timeout Calculation

```polyglot
[r] $request_start :pg.datetime << \|U.DT.Now
[r] $timeout_at :pg.datetime << \|U.DT.AddMinutes"{$request_start, 2}"

// Check if request exceeds timeout
[r] $now :pg.datetime << \|U.DT.Now
[r] $diff :pg.int << \|U.DT.Diff"{$now, $timeout_at, \"seconds\"}"

[f] $diff > 0
   [r] !Timeout << "Request exceeded 2-minute timeout"
```

---

## Hour and Day Boundaries

**Automatically handles:**
- Hour boundaries (e.g., 59 min + 2 min = 01 min next hour)
- Day boundaries
- Month boundaries
- Year boundaries

---

## Related Pipelines

- [|U.DT.AddHours](./add-hours.md) - Add hours
- [|U.DT.AddSeconds](./add-seconds.md) - Add seconds
- [|U.DT.Diff](./diff.md) - Calculate difference in minutes

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
