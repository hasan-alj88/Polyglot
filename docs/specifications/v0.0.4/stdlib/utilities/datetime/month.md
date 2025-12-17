---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: month
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.DT.Month"
summary: "API reference: |U.DT.Month"
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
# |U.DT.Month

**Extract month component**

**Category:** Utilities > DateTime
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.DT.Month <datetime >result
```

**Inline:**
```polyglot
\|U.DT.Month"{$datetime}"
```

---

## Parameters

**Inputs:**
- `<datetime` :pg.datetime - Source datetime

**Outputs:**
- `>result` :pg.int - Month (1-12)

---

## Description

Extracts the month component from a datetime value as an integer from 1 to 12.

**Values:**
- 1 = January
- 2 = February
- ...
- 12 = December

---

## Examples

### Basic Usage

```polyglot
[r] $dt :pg.datetime << \|U.DT.Parse"{\"2025-12-15\", \"YYYY-MM-DD\"}"
[r] $month :pg.int << \|U.DT.Month"{$dt}"
```

**Output:** `$month = 12`

---

### Current Month

```polyglot
[r] $now :pg.datetime << \|U.DT.Now
[r] $current_month :pg.int << \|U.DT.Month"{$now}"
```

---

### Filter Records by Month

```polyglot
[r] ~ForEach.Array
[~] <array << $transactions
[~] >item >> $transaction
   [r] $dt :pg.datetime << $transaction."date"
   [r] $month :pg.int << \|U.DT.Month"{$dt}"

   [y] $month == 12
      // December transactions
      [v] *Into.Array
      [*] <item << $transaction
      [*] >array >> $december_transactions
```

---

### Check Quarter

```polyglot
[r] $month :pg.int << \|U.DT.Month"{$date}"

[y] $month >= 1 & $month <= 3
   [r] $quarter :pg.int << 1
[&] $month >= 4 & $month <= 6
   [r] $quarter :pg.int << 2
[&] $month >= 7 & $month <= 9
   [r] $quarter :pg.int << 3
[^]
   [r] $quarter :pg.int << 4
```

---

## Common Patterns

### Pattern 1: Check if Same Month

```polyglot
[r] $month1 :pg.int << \|U.DT.Month"{$date1}"
[r] $month2 :pg.int << \|U.DT.Month"{$date2}"
[r] $year1 :pg.int << \|U.DT.Year"{$date1}"
[r] $year2 :pg.int << \|U.DT.Year"{$date2}"

[y] $month1 == $month2 & $year1 == $year2
   // Same month and year
```

### Pattern 2: Group by Month

```polyglot
[r] ~ForEach.Array
[~] <array << $sales
[~] >item >> $sale
   [r] $month :pg.int << \|U.DT.Month"{$sale.\"date\"}"
   [r] $amount :pg.float << $sale."amount"

   [v] *Into.Serial
   [*] <path :pg.string << \|U.String.Concat"{\"month_\", $month}"
   [*] <item << $amount
   [*] >serial >> $monthly_totals
```

### Pattern 3: Season Detection

```polyglot
[r] $month :pg.int << \|U.DT.Month"{$date}"

[y] $month == 12 | $month == 1 | $month == 2
   [r] $season :pg.string << "Winter"
[&] $month >= 3 & $month <= 5
   [r] $season :pg.string << "Spring"
[&] $month >= 6 & $month <= 8
   [r] $season :pg.string << "Summer"
[^]
   [r] $season :pg.string << "Fall"
```

---

## Related Pipelines

- [|U.DT.Year](./year.md) - Extract year component
- [|U.DT.Day](./day.md) - Extract day component
- [|U.DT.Format](./format.md) - Format complete datetime

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
