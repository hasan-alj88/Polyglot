---
audience: developer
rule: "7.2w"
code: PGW07002
name: Caller Overrides Pipeline Fallback
severity: warning
---

# Rule 7.2w — Caller Overrides Pipeline Fallback
`PGW07002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** When a caller uses `(>) !>` to provide a fallback on an output that the called pipeline already defines a fallback for (inside its `[!] >>` raise block), and that fallback has a non-empty `(>) %FallbackMessage`, the compiler emits a warning displaying the author's message. The caller is overriding an intentional pipeline-defined fallback without acknowledging the author's reasoning.
**Rationale:** Pipeline authors set fallback values for specific reasons — data consistency, safe defaults, protocol compliance. Callers overriding these fallbacks should be aware of what they are replacing. The warning surfaces the author's reasoning so the caller can make an informed decision.
**Detection:** The compiler compares the caller's `(>) !>` fallback targets against the called pipeline's raise-block fallbacks. If a match exists and the pipeline's fallback has `%FallbackMessage` with a non-empty string, PGW07002 fires with the message content.

**See also:**
- [PGW07003 — Missing Fallback Message](PGW07003-missing-fallback-message.md) — author-side warning for missing `%FallbackMessage`

**VALID:**
```aljam3
[ ] ✓ No override — caller does not provide (>) !> on >status
{-} -Caller
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >result#string
   [ ]
   [-] -ValidateUser
      (-) <name << $name
      (-) >status >> $status
   [-] >result << $status
```

```aljam3
[ ] ✓ Override allowed — author suppressed with %FallbackMessage << ""
{-} -CallerOverrideOk
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >result#string
   [ ]
   [-] -ValidateUser
      (-) <name << $name
      (-) >status >> $status
         (>) !> "unknown"
   [-] >result << $status
```

**WARNING:**
```aljam3
[ ] ⚠ PGW07002 — caller overrides pipeline-defined fallback on >status
[ ]   Author: "Pipeline returns invalid status on empty input"
{-} -CallerOverrideBad
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >result#string
   [ ]
   [-] -ValidateUser
      (-) <name << $name
      (-) >status >> $status
         (>) !> "unknown"                     [ ] ⚠ PGW07002 — overrides author fallback
   [-] >result << $status
```

Where `-ValidateUser` contains:
```aljam3
[!] >> !Validation.Empty
   (!) .Message << "Name is required"
   (!) >status << "invalid"
      (>) %FallbackMessage << "Pipeline returns invalid status on empty input"
```

**Open point:** None.
