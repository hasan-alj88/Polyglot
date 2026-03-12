---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "add-seconds"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.AddSeconds"
summary: "API reference: |U.DT.AddSeconds"
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
# |U.DT.AddSeconds

**Add or subtract seconds**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.AddSeconds <datetime <seconds >result
```

**Inline:**
```polyglot
\|U.DT.AddSeconds"{$datetime, $seconds}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime
- `<seconds` :pg.int - Number of seconds to add (negative to subtract)

**Outputs:**
- `>result` :pg.datetime - Modified datetime

---

## Description

Adds or subtracts the specified number of seconds to/from a datetime value.

**Use negative values to subtract seconds.**

---

## Examples

### Basic Usage

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $later :pg.datetime << \|U.DT.AddSeconds"{$now, 30}"
```

---

### Add One Minute (60 seconds)

```polyglot
[r] $one_minute_later :pg.datetime << \|U.DT.AddSeconds"{$now, 60}"
```

---

### Subtract Seconds

```polyglot
[r] $ten_sec_ago :pg.datetime << \|U.DT.AddSeconds"{$now, -10}"
```

---

### Request Timeout

```polyglot
[r] $request_start :pg.datetime << \|U.DT.Now
[r] $timeout_at :pg.datetime << \|U.DT.AddSeconds"{$request_start, 30}"
```

---

### Performance Measurement

```polyglot
[r] $start :pg.datetime << \|U.DT.Now

// ... perform operation ...

[r] $end :pg.datetime << \|U.DT.Now
[r] $elapsed :pg.int << \|U.DT.Diff"{$start, $end, \"seconds\"}"
```

---

## Common Patterns

### Pattern 1: API Rate Limit Reset

```polyglot
[r] $limit_reached :pg.datetime << \|U.DT.Now
[r] $reset_at :pg.datetime << \|U.DT.AddSeconds"{$limit_reached, 60}"
```

### Pattern 2: Session Keepalive

```polyglot
[r] $last_activity :pg.datetime << \|U.DT.Now
[r] $session_expires :pg.datetime << \|U.DT.AddSeconds"{$last_activity, 3600}"
// 1-hour session timeout
```

### Pattern 3: Retry After Delay

```polyglot
[r] $failed_at :pg.datetime << \|U.DT.Now
[r] $retry_after :pg.datetime << \|U.DT.AddSeconds"{$failed_at, 10}"

// Wait until retry_after before retrying
```

### Pattern 4: Check Time Window

```polyglot
[r] $event_time :pg.datetime << \|U.DT.Parse"{$event_str, \"YYYY-MM-DD HH:mm:ss\"}"
[r] $grace_end :pg.datetime << \|U.DT.AddSeconds"{$event_time, 30}"
[r] $now :pg.datetime << \|U.DT.Now

[r] $diff :pg.int << \|U.DT.Diff"{$now, $grace_end, \"seconds\"}"

[f] $diff <= 0
   // Within 30-second grace period
```

---

## Minute, Hour, and Day Boundaries

**Automatically handles:**
- Minute boundaries (e.g., 59 sec + 2 sec = 01 sec next minute)
- Hour boundaries
- Day boundaries
- Month boundaries
- Year boundaries

---

## Related Pipelines

- [|U.DT.AddMinutes](./add-minutes.md) - Add minutes
- [|U.DT.AddHours](./add-hours.md) - Add hours
- [|U.DT.Diff](./diff.md) - Calculate difference in seconds

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
