---
audience: developer
rule: "2.2"
code: PGW02002
name: Unused Variable
severity: warning
---

### Rule 2.2 — Unused Variable
`PGW02002`

**Statement:** A variable `$var` that is assigned (reaches Default or Final state) but is never read, pushed to a consumer, or used in a conditional is flagged with a warning. The `$*` discard pattern suppresses this warning.
**Rationale:** An assigned but unconsumed variable is dead code — it occupies resources and its assignment serves no purpose. This is standard lint that helps developers identify forgotten variables or incomplete wiring. The `$*` discard pattern signals intentional discard.
**Detection:** The compiler tracks all variable assignments and reads. Any variable that has at least one assignment but zero reads is flagged. Exceptions: output ports (`>name`) are excluded (see PGW02003), IO inputs (`<name`) are excluded (consumed by the pipeline itself), and `$*` discard patterns are excluded.

**See also:** PGE02001 (lifecycle stages), PGE02002 (declared state is unreadable), PGW02003 (unpushed output port — analogous for output ports)

**VALID:**
```polyglot
[ ] ✓ variable assigned and consumed
{-} -Process
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] -Transform
      (-) <data << $input
      (-) >result >> $cleaned
   [-] >out << $cleaned                 [ ] ✓ $cleaned is consumed
```

```polyglot
[ ] ✓ discard pattern — intentionally unused
{-} -FireAndForget
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] -SideEffect
      (-) <data << $input
      (-) >result >> $*                 [ ] ✓ $* explicit discard — no warning
   [-] >out << "done"
```

```polyglot
[ ] ✓ variable used in conditional
{-} -Route
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <status#Status
   (-) >out#string
   [-] -Lookup
      (-) <key << $status
      (-) >code >> $code
   [?] $code                            [ ] ✓ $code consumed in conditional
      [?] ?[200,299]
         [-] >out << "ok"
      [?] *?
         [-] >out << "error"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW02002 — $unused assigned but never consumed
{-} -Wasteful
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] -Transform
      (-) <data << $input
      (-) >result >> $cleaned
   [-] -OtherWork
      (-) <data << $input
      (-) >result >> $unused            [ ] ⚠ PGW02002 — $unused never read
   [-] >out << $cleaned
```

```polyglot
[ ] ⚠ PGW02002 — $temp assigned but never consumed
{-} -DeadAssignment
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] $temp <~ "placeholder"           [ ] ⚠ PGW02002 — $temp never read
   [-] >out << $input
```

**Open point:** None.
