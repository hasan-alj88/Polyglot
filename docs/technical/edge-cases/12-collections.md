---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 12. Collection Operations (S12)

### EC-12.1: Expand with sequential vs parallel

<!-- @collections:Expand Operators -->
**EBNF:** `expand_line ::= ( "[r]" | "[p]" ) expand_invocation`

**What it tests:** `[r]` = sequential mini-pipelines, `[p]` = parallel. See [[concepts/collections/expand#Expand Operators]].

```polyglot
[ ] Sequential — order matters
[r] ~ForEach.Array
   [~] <Array << $orderedItems
   [~] >item >> $item

[ ] Parallel — order doesn't matter
[p] ~ForEach.Array
   [~] <Array << $independentItems
   [~] >item >> $item
```

### EC-12.2: ForEach.Array.Enumerate — index + item

**EBNF:** `expand_operator ::= "ForEach.Array.Enumerate"`

**What it tests:** Enumerate provides both `>index` and `>item`.

```polyglot
[r] ~ForEach.Array.Enumerate
   [~] <Array << $items
   [~] >index >> $idx
   [~] >item >> $val
```

### EC-12.3: ForEach.Serial — key/item pairs

**EBNF:** `expand_operator ::= "ForEach.Serial"`

**What it tests:** Serial iteration with `>key` and `>item`.

```polyglot
[r] ~ForEach.Serial
   [~] <Serial << $config
   [~] >key >> $k
   [~] >item >> $v
```

### EC-12.4: ForEach.Level — tilde suffix marks iteration point

**EBNF:** Special input syntax `<level << #SomeData.SubField.~`

**What it tests:** The `~` suffix on the input path. See [[concepts/collections/expand#ForEach.Level]].

```polyglot
[r] ~ForEach.Level
   [~] <level << #UserData.Preferences.~
   [~] >key >> $prefKey
   [~] >item >> $prefValue
```

### EC-12.5: Collector invocation with execution marker + [*] IO

<!-- @io:Collection Operators -->
**EBNF:** `collect_line ::= ( "[r]" | "[p]" ) collect_invocation NEWLINE { indent collect_io_line NEWLINE }` where `collect_io_line ::= "[*]" ...`

**What it tests:** `[r]`/`[p]` execution marker for invocation, `[*]` for IO — consistent with expand (`[r]`/`[p]` + `[~]`). See [[io#Collection Operators]].

```polyglot
[r] *Into.Array
   [*] <item << $value
   [*] >Array >> $collected
```

### EC-12.6: Direct output port write from collector

**EBNF:** `assign_target ::= output_param` — collector output writes to `>pipelineOutput`.

**What it tests:** `>> >pipelineOutput` syntax. See [[io#Direct Output Port Writing]].

```polyglot
[r] *Agg.Count
   [*] <item << $service
   [*] >count >> >successCount
[ ] Target >successCount is now Final — no other push allowed
```

### EC-12.7: Multiple collectors in same expand scope

**What it tests:** Two `*` collectors operating within one `~ForEach` body.

```polyglot
[p] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

   [r] *Into.Array
      [*] <item << $item
      [*] >Array >> >results

   [r] *Agg.Sum
      [*] <number << $item.value
      [*] >sum >> >total
```

### EC-12.8: All *Agg operators

**What it tests:** Every aggregate collector variant.

```polyglot
[r] *Agg.Sum
   [*] <number << $n
   [*] >sum >> $s

[r] *Agg.Count
   [*] <item << $x
   [*] >count >> $c

[r] *Agg.Average
   [*] <number << $n
   [*] >average >> $avg

[r] *Agg.Max
   [*] <number << $n
   [*] >max >> $mx

[r] *Agg.Min
   [*] <number << $n
   [*] >min >> $mn

[r] *Agg.Concatenate
   [*] <string << $s
   [*] >result >> $concat
```

### EC-12.9: `*All` — sync barrier with `[*] <<` only

**EBNF:** `sync_operator ::= "All"` ; `wait_input ::= "[*]" "<<" variable_ref`

**What it tests:** `*All` with `[*] <<`-only lines outside expand scope. Variables remain accessible after.

```polyglot
[p] =Fetch.A
   [=] <id << $id
   [=] >resultA >> $resultA

[p] =Fetch.B
   [=] <id << $id
   [=] >resultB >> $resultB

[*] *All
   [*] << $resultA
   [*] << $resultB

[ ] $resultA and $resultB are accessible here
[r] =Process
   [=] <a << $resultA
   [=] <b << $resultB
```

### EC-12.10: `*First` — race collector with `[*] <<` inputs and `[*] >>` output

**EBNF:** `race_operator ::= "First"` ; `collect_output ::= "[*]" ">>" variable_ref`

**What it tests:** `*First` cancels losing `[*] <<` inputs; only `[*] >>` output survives. All `[*] <<` inputs same type.

```polyglot
[p] =Search.A
   [=] <q << $query
   [=] >result >> $rA

[p] =Search.B
   [=] <q << $query
   [=] >result >> $rB

[*] *First
   [*] << $rA
   [*] << $rB
   [*] >> $fastest

[ ] Only $fastest is accessible here — $rA and $rB are cancelled
```

### EC-12.11: `*Nth` — generic race with `<n#int` IO

**EBNF:** `race_operator ::= "Nth"` ; `collect_io_line ::= "[*]" "<n#int" assignment_op value_expr`

**What it tests:** `*Nth` takes `<n#int` position parameter. `*First`/`*Second` are sugar for n=1/n=2.

```polyglot
[p] =Search.A
   [=] <q << $query
   [=] >result >> $rA

[p] =Search.B
   [=] <q << $query
   [=] >result >> $rB

[p] =Search.C
   [=] <q << $query
   [=] >result >> $rC

[*] *Nth
   [*] <n#int << 2
   [*] << $rA
   [*] << $rB
   [*] << $rC
   [*] >> $second
```

### EC-12.12: Multi-wave parallel pattern with multiple `*All` barriers

**What it tests:** `[*] *All` used twice in a pipeline body to form sequential parallel waves.

```polyglot
[p] =Fetch.Profile
   [=] <id << $id
   [=] >profile >> $profile

[p] =Fetch.Prefs
   [=] <id << $id
   [=] >prefs >> $prefs

[*] *All
   [*] << $profile
   [*] << $prefs

[p] =Enrich.A
   [=] <profile << $profile
   [=] >enriched >> $enriched

[p] =Enrich.B
   [=] <prefs << $prefs
   [=] >recs >> $recs

[*] *All
   [*] << $enriched
   [*] << $recs

[r] =Assemble
   [=] <enriched << $enriched
   [=] <recs << $recs
```

### EC-12.13: `[*] <<` vs `[*] >>` — wait input keeps variable, collect output cancels inputs

**What it tests:** Contrast: `[*] <<` alone on `*All` leaves vars accessible; `[*] <<`+`[*] >>` on `*First` cancels `[*] <<` vars.

```polyglot
[ ] *All: [*] << only — $a and $b accessible after
[*] *All
   [*] << $a
   [*] << $b
[r] =UseAB
   [=] <x << $a
   [=] <y << $b

[ ] *First: [*] <<+[*] >> — $a and $b cancelled; only $winner accessible
[*] *First
   [*] << $a
   [*] << $b
   [*] >> $winner
[r] =UseWinner
   [=] <x << $winner
```

### EC-12.14: Collector outside expand/parallel scope

<!-- @collections -->
**EBNF ref:** `exec_line` includes `collect_line` at any scope
**What it tests:** `*Into.Array` at pipeline top level with no `~ForEach` or `[p]` context. PGE03010 fires. See [[concepts/collections/expand|expand]].

```polyglot
[ ] ✗ PGE03010 — collector has no parallel source
[r] *Into.Array
   [*] <item << $someValue
   [*] >Array >> $results
```

### EC-12.15: `[p]` on sequential collector — incompatible marker

**EBNF ref:** `collect_line ::= ( "[r]" | "[p]" ) collect_invocation`
**What it tests:** `*Into.Array` with `[p]` marker — PGE01024 fires because `*Into.Array` only declares `[r]` compatibility. See [[concepts/collections/expand|expand]].

```polyglot
[ ] ✗ PGE01024 — *Into.Array is not [p]-compatible
[p] *Into.Array
   [*] <item << $val
   [*] >Array >> $arr
```

### EC-12.16: `[~]` expand IO outside expand context

**EBNF ref:** `[~]` is a block element for expand IO
**What it tests:** Orphaned `[~]` markers with no enclosing `~ForEach`. PGE03011 fires.

```polyglot
[ ] ✗ PGE03011 — [~] outside expand scope
[~] <Array << $items
[~] >item >> $item
```
