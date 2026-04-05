---
audience: pg-coder
type: specification
updated: 2026-03-20
status: draft
---

# *All / *First / *Nth — Collect-All & Race Collectors

Used outside expand scopes to collect variables from parallel `[p]` pipeline calls. Uses `[*] <<` (wait input) and `[*] >>` (collect output).

No `[@]` import needed.

```polyglot
*All
   [ ] Wait for ALL listed variables to be Final.
   [ ] Uses [*] << only — no [*] >>. All variables stay accessible after.
   [ ] No type constraint on inputs.
   [ ] Example:
   [ ]   [*] *All
   [ ]      [*] << $profile
   [ ]      [*] << $history

*First
   [ ] Sugar for *Nth with n=1.
   [ ] Takes first arriving value; all other inputs are cancelled.
   [ ] All [*] << inputs must be the same type. [*] >> output required.
   [*] << $candidate...
   [*] >> $winner

*Second
   [ ] Sugar for *Nth with n=2.
   [*] << $candidate...
   [*] >> $winner

*Nth
   [ ] Generic race form — wait for the Nth arrival.
   [ ] *First and *Second are named aliases for *Nth n=1/n=2.
   [*] <n#int            (which arrival to capture)
   [*] << $candidate...
   [*] >> $winner
```

### Discarding Parallel Output

```polyglot
$*
   [ ] Inline discard — no variable created. Output immediately released.

*Ignore
   [ ] Explicit collector discard — variable exists but is released.
   [*] << $unneededVar
```

See also: [collections.md §Collect-All & Race Collectors](../concepts/collections.md)
