---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 19. Collections — Gaps (S19)

### EC-19.1: `*Into.Serial` — uses `<key` and `<value` (not `<item`)

<!-- @u:collections:Collect Operators -->
**EBNF:** `collect_operator ::= "Into.Serial"` with IO `<key`, `<value`, `>Serial`

**What it tests:** `*Into.Serial` takes separate key and value inputs, not a single `<item`. See [[concepts/collections/collect#Collect Operators]].

```polyglot
[-] =ForEach.Array
   (=) <Array << $pairs
   (=) >item >> $pair

   [-] *Into.Serial
      (*) <key << $pair:key
      (*) <value << $pair:value
      (*) >Serial >> $result
```

### EC-19.2: `*Into.Level` — collects siblings at a specific level

**EBNF:** `collect_operator ::= "Into.Level"` with IO `<key`, `<value`, `>Serial`

**What it tests:** Level-targeted collect, parallel to `=ForEach.Level`.

```polyglot
[-] =ForEach.Level
   (=) <level << #UserData.Preferences.=
   (=) >key >> $k
   (=) >item >> $v

   [-] *Into.Level
      (*) <key << $k
      (*) <value << $v
      (*) >Serial >> >preferencesOut
```

### EC-19.3: `*Second` — sugar for `*Nth` n=2

**EBNF:** `race_operator ::= "Second"` — syntactic sugar, equivalent to `*Nth` with `<n << 2`

**What it tests:** `*Second` used like `*First` but captures 2nd-to-finish. Same `(*) <<`/`(*) >>` semantics.

```polyglot
[=] -Search.Fast
   (-) <q << $query
   (-) >result >> $rFast

[=] -Search.Accurate
   (-) <q << $query
   (-) >result >> $rAccurate

[=] -Search.Deep
   (-) <q << $query
   (-) >result >> $rDeep

[*] *Second
   (*) << $rFast
   (*) << $rAccurate
   (*) << $rDeep
   (*) >> $backup

[ ] Only $backup is accessible — others cancelled
```

### EC-19.4: `[b]` collector inside expand — fire-and-forget per item

<!-- @u:blocks:Execution -->
**What it tests:** `[b]` execution marker on a collector invocation — fires without waiting for result. See [[blocks#Execution]], [[concepts/collections/collect#Collect Operators]].

```polyglot
[=] =ForEach.Array
   (=) <Array << $events
   (=) >item >> $event

   [ ] Fire metric per item — no output needed
   [b] -Metrics.Emit
      (-) <event << $event
```
