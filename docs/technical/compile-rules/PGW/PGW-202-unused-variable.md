---
rule: "2.2"
code: PGW-202
name: Unused Variable
severity: warning
---

### Rule 2.2 — Unused Variable
`PGW-202`

**Statement:** A variable `$var` that is assigned (reaches Default or Final state) but is never read, pushed to a consumer, or used in a conditional is flagged with a warning. The `$*` discard pattern suppresses this warning.
**Rationale:** An assigned but unconsumed variable is dead code — it occupies resources and its assignment serves no purpose. This is standard lint that helps developers identify forgotten variables or incomplete wiring. The `$*` discard pattern signals intentional discard.
**Detection:** The compiler tracks all variable assignments and reads. Any variable that has at least one assignment but zero reads is flagged. Exceptions: output ports (`>name`) are excluded (see PGW-203), IO inputs (`<name`) are excluded (consumed by the pipeline itself), and `$*` discard patterns are excluded.

**See also:** PGE-201 (lifecycle stages), PGE-202 (declared state is unreadable), PGW-203 (unpushed output port — analogous for output ports)

**VALID:**
```polyglot
[ ] ✓ variable assigned and consumed
{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] =Transform
      [=] <data << $input
      [=] >result >> $cleaned
   [r] >out << $cleaned                 [ ] ✓ $cleaned is consumed
```

```polyglot
[ ] ✓ discard pattern — intentionally unused
{=} =FireAndForget
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] =SideEffect
      [=] <data << $input
      [=] >result >> $*                 [ ] ✓ $* explicit discard — no warning
   [r] >out << "done"
```

```polyglot
[ ] ✓ variable used in conditional
{=} =Route
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <status;Status
   [=] >out;string
   [r] =Lookup
      [=] <key << $status
      [=] >code >> $code
   [?] $code                            [ ] ✓ $code consumed in conditional
      [?] ?[200,299]
         [r] >out << "ok"
      [?] *?
         [r] >out << "error"
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-202 — $unused assigned but never consumed
{=} =Wasteful
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] =Transform
      [=] <data << $input
      [=] >result >> $cleaned
   [r] =OtherWork
      [=] <data << $input
      [=] >result >> $unused            [ ] ⚠ PGW-202 — $unused never read
   [r] >out << $cleaned
```

```polyglot
[ ] ⚠ PGW-202 — $temp assigned but never consumed
{=} =DeadAssignment
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] $temp <~ "placeholder"           [ ] ⚠ PGW-202 — $temp never read
   [r] >out << $input
```

**Open point:** None.
