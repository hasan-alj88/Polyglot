---
audience: design
type: reference
updated: 2026-04-17
---

<!-- @edge-cases/INDEX -->

## 12. Collection Operations (S12)

### EC-12.1: Expand with sequential vs parallel

<!-- @u:collections:Expand Operators -->
**EBNF:** `expand_line ::= ( "[-]" | "[=]" ) expand_invocation`

**What it tests:** `[-]` = sequential mini-pipelines, `[=]` = parallel. See [[concepts/collections/expand#Expand Operators]].

```aljam3
[ ] Sequential — order matters
[-] =ForEach.Array
   (=) <Array << $orderedItems
   (=) >item >> $item

[ ] Parallel — order doesn't matter
[=] =ForEach.Array
   (=) <Array << $independentItems
   (=) >item >> $item
```

### EC-12.2: ForEach.Array.Enumerate — index + item

**EBNF:** `expand_operator ::= "ForEach.Array.Enumerate"`

**What it tests:** Enumerate provides both `>index` and `>item`.

```aljam3
[-] =ForEach.Array.Enumerate
   (=) <Array << $items
   (=) >index >> $idx
   (=) >item >> $val
```

### EC-12.3: ForEach.Serial — key/item pairs

**EBNF:** `expand_operator ::= "ForEach.Serial"`

**What it tests:** Serial iteration with `>key` and `>item`.

```aljam3
[-] =ForEach.Serial
   (=) <Serial << $config
   (=) >key >> $k
   (=) >item >> $v
```

### EC-12.4: ForEach.Level — `.=` level iteration marker

**EBNF:** `level_input` production in §12.1 — `<level << #SomeData.SubField.=`

**What it tests:** The `.=` level iteration marker on the input path — analogous to `.*` wildcard, `.=` means "expand siblings at this level." See [[concepts/collections/expand#ForEach.Level]].

```aljam3
[-] =ForEach.Level
   (=) <level << #UserData.Preferences.=
   (=) >key >> $prefKey
   (=) >item >> $prefValue
```

### EC-12.5: Collector invocation with execution marker + (*) IO

<!-- @u:io:Collection Operators -->
**EBNF:** `collect_line ::= ( "[-]" | "[=]" ) collect_invocation NEWLINE { indent collect_io_line NEWLINE }` where `collect_io_line ::= "(*)" ...`

**What it tests:** `[-]`/`[=]` execution marker for invocation, `(*)` for IO — consistent with expand (`[-]`/`[=]` + `(=)`). See [[io#Collection Operators]].

```aljam3
[-] *Into.Array
   (*) <item << $value
   (*) >Array >> $collected
```

### EC-12.6: Direct output port write from collector

**EBNF:** `assign_target ::= output_param` — collector output writes to `>pipelineOutput`.

**What it tests:** `>> >pipelineOutput` syntax. See [[io#Direct Output Port Writing]].

```aljam3
[-] *Agg.Count
   (*) <item << $service
   (*) >count >> >successCount
[ ] Target >successCount is now Final — no other push allowed
```

### EC-12.7: Multiple collectors in same expand scope

**What it tests:** Two `*` collectors operating within one `=ForEach` body.

```aljam3
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [-] *Into.Array
      (*) <item << $item
      (*) >Array >> >results

   [-] *Agg.Sum
      (*) <number << $item.value
      (*) >sum >> >total
```

### EC-12.8: All *Agg operators

**What it tests:** Every aggregate collector variant.

```aljam3
[-] *Agg.Sum
   (*) <number << $n
   (*) >sum >> $s

[-] *Agg.Count
   (*) <item << $x
   (*) >count >> $c

[-] *Agg.Average
   (*) <number << $n
   (*) >average >> $avg

[-] *Agg.Max
   (*) <number << $n
   (*) >max >> $mx

[-] *Agg.Min
   (*) <number << $n
   (*) >min >> $mn

[-] *Agg.Concatenate
   (*) <string << $s
   (*) >result >> $concat
```

### EC-12.9: `*All` — collect all with `(*) <<` only

**EBNF:** `sync_operator ::= "All"` ; `wait_input ::= "(*)" "<<" variable_ref`

**What it tests:** `*All` with `(*) <<`-only lines outside expand scope. Variables remain accessible after.

```aljam3
[=] -Fetch.A
   (-) <id << $id
   (-) >resultA >> $resultA

[=] -Fetch.B
   (-) <id << $id
   (-) >resultB >> $resultB

[*] *All
   (*) << $resultA
   (*) << $resultB

[ ] $resultA and $resultB are accessible here
[-] -Process
   (-) <a << $resultA
   (-) <b << $resultB
```

### EC-12.10: `*First` — race collector with `(*) <<` inputs and `(*) >>` output

**EBNF:** `race_operator ::= "First"` ; `collect_output ::= "(*)" ">>" variable_ref`

**What it tests:** `*First` cancels losing `(*) <<` inputs; only `(*) >>` output survives. All `(*) <<` inputs same type.

```aljam3
[=] -Search.A
   (-) <q << $query
   (-) >result >> $rA

[=] -Search.B
   (-) <q << $query
   (-) >result >> $rB

[*] *First
   (*) << $rA
   (*) << $rB
   (*) >> $fastest

[ ] Only $fastest is accessible here — $rA and $rB are cancelled
```

### EC-12.11: `*Nth` — generic race with `<n#int` IO

**EBNF:** `race_operator ::= "Nth"` ; `collect_io_line ::= "(*)" "<n#int" assignment_op value_expr`

**What it tests:** `*Nth` takes `<n#int` position parameter. `*First`/`*Second` are sugar for n=1/n=2.

```aljam3
[=] -Search.A
   (-) <q << $query
   (-) >result >> $rA

[=] -Search.B
   (-) <q << $query
   (-) >result >> $rB

[=] -Search.C
   (-) <q << $query
   (-) >result >> $rC

[*] *Nth
   (*) <n#int << 2
   (*) << $rA
   (*) << $rB
   (*) << $rC
   (*) >> $second
```

### EC-12.12: Multi-wave parallel pattern with multiple `*All` barriers

**What it tests:** `[*] *All` used twice in a pipeline body to form sequential parallel waves.

```aljam3
[=] -Fetch.Profile
   (-) <id << $id
   (-) >profile >> $profile

[=] -Fetch.Prefs
   (-) <id << $id
   (-) >prefs >> $prefs

[*] *All
   (*) << $profile
   (*) << $prefs

[=] -Enrich.A
   (-) <profile << $profile
   (-) >enriched >> $enriched

[=] -Enrich.B
   (-) <prefs << $prefs
   (-) >recs >> $recs

[*] *All
   (*) << $enriched
   (*) << $recs

[-] -Assemble
   (-) <enriched << $enriched
   (-) <recs << $recs
```

### EC-12.13: `(*) <<` vs `(*) >>` — wait input keeps variable, collect output cancels inputs

**What it tests:** Contrast: `(*) <<` alone on `*All` leaves vars accessible; `(*) <<`+`(*) >>` on `*First` cancels `(*) <<` vars.

```aljam3
[ ] *All: (*) << only — $a and $b accessible after
[*] *All
   (*) << $a
   (*) << $b
[-] -UseAB
   (-) <x << $a
   (-) <y << $b

[ ] *First: (*) <<+(*) >> — $a and $b cancelled; only $winner accessible
[*] *First
   (*) << $a
   (*) << $b
   (*) >> $winner
[-] -UseWinner
   (-) <x << $winner
```

### EC-12.14: Collector outside expand/parallel scope

<!-- @u:collections -->
**EBNF ref:** `exec_line` includes `collect_line` at any scope
**What it tests:** `*Into.Array` at pipeline top level with no `=ForEach` or `[=]` context. PGE03010 fires. See [[concepts/collections/expand|expand]].

```aljam3
[ ] ✗ PGE03010 — collector has no parallel source
[-] *Into.Array
   (*) <item << $someValue
   (*) >Array >> $results
```

### EC-12.15: `[=]` on sequential collector — incompatible marker

**EBNF ref:** `collect_line ::= ( "[-]" | "[=]" ) collect_invocation`
**What it tests:** `*Into.Array` with `[=]` marker — PGE01024 fires because `*Into.Array` only declares `[-]` compatibility. See [[concepts/collections/expand|expand]].

```aljam3
[ ] ✗ PGE01024 — *Into.Array is not [=]-compatible
[=] *Into.Array
   (*) <item << $val
   (*) >Array >> $arr
```

### EC-12.16: `(=)` expand IO outside expand context

**EBNF ref:** `(=)` is a block element for expand IO
**What it tests:** Orphaned `(=)` markers with no enclosing `=ForEach`. PGE03011 fires.

```aljam3
[ ] ✗ PGE03011 — (=) outside expand scope
(=) <Array << $items
(=) >item >> $item
```

### EC-12.17: Orphan parallel marker on collector (X.34)

**EBNF ref:** `collect_line ::= ( "[-]" | "[=]" ) collect_invocation`
**What it tests:** `[=]` and `[b]` must pair with the next `[=]` or `[b]` sibling. A `[=]` line whose next sibling is not `[=]`/`[b]` fires PGE01040. See also EC-12.15 for PGE01024 (marker compatibility).

```aljam3
[ ] ✓ two [=] collector siblings — parallel pair
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [=] *Into.Array
      (*) <item << $item
      (*) >Array >> >results

   [=] *Agg.Sum
      (*) <number << $item.value
      (*) >sum >> >total
```

```aljam3
[ ] ✗ PGE01040 — [=] collector followed by [-] sibling
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [=] *Into.Array
      (*) <item << $item
      (*) >Array >> >results

   [-] *Agg.Sum
      (*) <number << $item.value
      (*) >sum >> >total
```

```aljam3
[ ] ✓ [-] collectors — both sequential, no parallel claim
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [-] *Into.Array
      (*) <item << $item
      (*) >Array >> >results

   [-] *Agg.Sum
      (*) <number << $item.value
      (*) >sum >> >total
```

```aljam3
[ ] ✗ PGE01040 — [=] pipeline followed by [*] collector
[=] -Fetch.A
   (-) <id << $id
   (-) >a >> $a

[*] *All
   (*) << $a
```

### EC-12.18: Reassemble with incompatible IO signatures (X.46)

<!-- @u:ebnf/12-collections -->
**EBNF ref:** `reassemble_line` (§12.3), reassemble IO signature table (§12.3)

**What it tests:** Reassemble operators combine an expander and collector. The `(=)` expander input is typed to a specific collection schema (e.g., `<Collection.Array` requires `##Array`). The compiler desugars `=*` into `=ForEach.*` + `*` and validates IO wiring using standard type checking. Incompatible IO signatures are caught by `PGE04001` (type mismatch), not by a separate reassemble rule.

**Decision:** Accept — existing type system covers this via schema-typed inputs and desugar + IO wiring validation.

```aljam3
[ ] ✓ Compatible — Serial yields >key + >value, matches Into.Map collector
[-] =*Into.Map
   (=) <Serial << $data
   (*) >Map >> $result

[ ] ✓ Compatible — Array yields >item, matches Agg.Sum <number (if item is numeric)
[-] =*Agg.Sum
   (=) <Array << $numbers
   (*) >sum >> $total
```

```aljam3
[ ] ✗ PGE04001 — Array yields >item (single element), Into.Map needs >key + >value
[ ] Compiler desugars to =ForEach.Array + *Into.Map, IO wiring fails
[-] =*Into.Map
   (=) <Array << $items
   (*) >Map >> $result

[ ] ✗ PGE04001 — Dataframe yields >row, Agg.Sum needs <number
[ ] Schema %##Active << #ActiveKind.One determines per-iteration output shape
[-] =*Agg.Sum
   (=) <Dataframe << $df
   (*) >sum >> $total
```
