---
rule: "9.2"
code: PGW-902
name: Unused Import
severity: warning
---

### Rule 9.2 — Unused Import
`PGW-902`

**Statement:** An `[@]` import alias declared in the `{@}` package block that is never referenced by any `@alias=Pipeline` or `@alias#Data` expression in the file is flagged with a warning. The import is dead code.
**Rationale:** Unused imports add clutter and may indicate incomplete refactoring — a pipeline call was removed but the import was left behind. Flagging unused imports keeps the dependency graph clean and makes it clear which external packages are actually used.
**Detection:** The compiler collects all `[@]` alias declarations from the `{@}` block, then scans all expressions in the file for `@alias` references. Any alias with zero references is flagged.

**See also:** PGE-901 (undefined import alias — the inverse: using an alias that was never declared)

**VALID:**
```polyglot
[ ] ✓ all imports are referenced
{@} @MyPackage
   [@] @auth << "com.example.auth"
   [@] @data << "com.example.data"

{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] @auth=Validate                   [ ] ✓ @auth used
      [=] <token << $input
      [=] >ok >> $valid
   [r] @data=Store                      [ ] ✓ @data used
      [=] <item << $valid
      [=] >result >> >out
```

```polyglot
[ ] ✓ no imports — leaf package using only stdlib
{@} @LeafPackage

{=} =Simple
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] >out << $input
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-902 — @data imported but never referenced
{@} @MyPackage
   [@] @auth << "com.example.auth"
   [@] @data << "com.example.data"      [ ] ⚠ PGW-902 — @data never used

{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] @auth=Validate
      [=] <token << $input
      [=] >ok >> >out
```

```polyglot
[ ] ⚠ PGW-902 — all imports unused
{@} @Abandoned
   [@] @old << "com.example.legacy"     [ ] ⚠ PGW-902 — @old never used
   [@] @also << "com.example.other"     [ ] ⚠ PGW-902 — @also never used

{=} =NoExternalCalls
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] >out << $input
```

**Open point:** None.
