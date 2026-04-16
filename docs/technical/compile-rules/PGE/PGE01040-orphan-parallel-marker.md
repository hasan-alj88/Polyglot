---
audience: developer
rule: "1.40"
code: PGE01040
name: Orphan Parallel Marker
severity: error
---

# Rule 1.40 — Orphan Parallel Marker
`PGE01040`

<!-- @u:syntax/blocks -->
<!-- @c:compile-rules/PGE/PGE01024-incompatible-operation-marker -->
<!-- @c:compile-rules/PGE/PGE03005-b-has-no-collectible-output -->

**Statement:** The `[=]` and `[b]` execution markers mean "run in parallel with the next `[=]` or `[b]` sibling at the same indentation level." If the next sibling at the same level does not use `[=]` or `[b]`, the parallel marker has no pairing partner and PGE01040 fires. Markers `[*]`, `[-]`, and all other block element markers are inherently sequential — they cannot be paired with `[=]`/`[b]`.
**Rationale:** `[=]` and `[b]` express parallelism by forming pairs (or groups) of sibling lines that execute concurrently. A lone `[=]` with no parallel partner is ambiguous — the developer intended parallelism but has nothing to parallelize against. Catching this at compile time prevents confusion about what "parallel" means when there is only one operation.
**Detection:** The compiler scans sibling lines at the same indentation level. For each `[=]` or `[b]` line, it checks the next sibling. If the next sibling exists but uses a different marker (e.g., `[-]`, `[*]`, `[?]`), or if there is no next sibling, PGE01040 fires.

## Pairing Rules

| Current Line | Next Sibling | Result |
|-------------|-------------|--------|
| `[=]` | `[=]` | Valid — parallel pair |
| `[=]` | `[b]` | Valid — parallel pair (mixed markers OK) |
| `[b]` | `[=]` | Valid — parallel pair |
| `[b]` | `[b]` | Valid — parallel pair |
| `[=]` | `[-]` | **PGE01040** — next sibling is sequential |
| `[=]` | `[*]` | **PGE01040** — collector is sequential |
| `[=]` | (none) | **PGE01040** — no sibling to pair with |
| `[b]` | `[-]` | **PGE01040** — next sibling is sequential |
| `[-]` | `[-]` | Valid — both sequential (no parallelism claim) |
| `[-]` | `[=]` | Valid — sequential line followed by start of parallel group |

**Note:** The rule checks forward only. A `[-]` line followed by `[=]` is valid — the `[-]` is sequential, and the `[=]` starts a new parallel group that must have its own forward partner.

## Diagnostic Format

`Orphan parallel marker '[=]' at line N — next sibling uses '[marker]' which is not parallel-compatible; use '[-]' for sequential execution or add a parallel sibling`

---

## Examples

### VALID

```polyglot
[ ] ✓ two [=] siblings — parallel pair
[=] -Fetch.A
   (-) <id << $id
   (-) >resultA >> $resultA

[=] -Fetch.B
   (-) <id << $id
   (-) >resultB >> $resultB
```

```polyglot
[ ] ✓ three [=] siblings — parallel group
[=] -Fetch.A
   (-) <id << $id
   (-) >a >> $a

[=] -Fetch.B
   (-) <id << $id
   (-) >b >> $b

[=] -Fetch.C
   (-) <id << $id
   (-) >c >> $c
```

```polyglot
[ ] ✓ [=] + [b] mixed — both are parallel markers
[=] -Fetch.Data
   (-) <id << $id
   (-) >data >> $data

[b] -Log.Access
   (-) <id << $id
```

```polyglot
[ ] ✓ [-] collectors in expand scope — sequential, no parallel claim
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

```polyglot
[ ] ✓ [=] collector siblings — parallel pair (independent collectors)
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

### INVALID

```polyglot
[ ] ✗ PGE01040 — lone [=] collector, next sibling is [-]
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

```polyglot
[ ] ✗ PGE01040 — [=] pipeline with no parallel sibling
[=] -Fetch.Data
   (-) <id << $id
   (-) >data >> $data

[-] -Process
   (-) <data << $data
   (-) >result >> $result
```

```polyglot
[ ] ✗ PGE01040 — [=] followed by [*] collector
[=] -Fetch.A
   (-) <id << $id
   (-) >a >> $a

[*] *All
   (*) << $a
```

```polyglot
[ ] ✗ PGE01040 — lone [b] with no parallel sibling
[b] -Log.Access
   (-) <event << $event
```

### See Also

- [[compile-rules/PGE/PGE01024-incompatible-operation-marker|PGE01024]] — operation used with incompatible marker
- [[compile-rules/PGE/PGE03005-b-has-no-collectible-output|PGE03005]] — `[b]` has no collectible output
- [[compile-rules/PGE/PGE03012-parallel-label-isolation|PGE03012]] — parallel label isolation
