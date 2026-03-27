---
rule: "7.2w"
code: PGW-702
name: Caller Overrides Pipeline Fallback
severity: warning
---

### Rule 7.2w — Caller Overrides Pipeline Fallback
`PGW-702`

**Statement:** When a caller uses `[>] <!` to provide a fallback on an output that the called pipeline already defines a fallback for (inside its `[!] >>` raise block), and that fallback has a non-empty `[>] %FallbackMessage`, the compiler emits a warning displaying the author's message. The caller is overriding an intentional pipeline-defined fallback without acknowledging the author's reasoning.
**Rationale:** Pipeline authors set fallback values for specific reasons — data consistency, safe defaults, protocol compliance. Callers overriding these fallbacks should be aware of what they are replacing. The warning surfaces the author's reasoning so the caller can make an informed decision.
**Detection:** The compiler compares the caller's `[>] <!` fallback targets against the called pipeline's raise-block fallbacks. If a match exists and the pipeline's fallback has `%FallbackMessage` with a non-empty string, PGW-702 fires with the message content.

**See also:**
- [PGW-703 — Missing Fallback Message](PGW-703-missing-fallback-message.md) — author-side warning for missing `%FallbackMessage`

**VALID:**
```polyglot
[ ] ✓ No override — caller does not provide [>] <! on >status
{=} =Caller
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] >result#string
   [r] =ValidateUser
      [=] <name << $name
      [=] >status >> $status
   [r] >result << $status
```

```polyglot
[ ] ✓ Override allowed — author suppressed with %FallbackMessage << ""
{=} =CallerOverrideOk
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] >result#string
   [r] =ValidateUser
      [=] <name << $name
      [=] >status >> $status
         [>] <! "unknown"
   [r] >result << $status
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-702 — caller overrides pipeline-defined fallback on >status
[ ]   Author: "Pipeline returns invalid status on empty input"
{=} =CallerOverrideBad
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] >result#string
   [r] =ValidateUser
      [=] <name << $name
      [=] >status >> $status
         [>] <! "unknown"                     [ ] ⚠ PGW-702 — overrides author fallback
   [r] >result << $status
```

Where `=ValidateUser` contains:
```polyglot
[!] >> !Validation.Empty
   [=] .Message << "Name is required"
   [=] >status << "invalid"
      [>] %FallbackMessage << "Pipeline returns invalid status on empty input"
```

**Open point:** None.
