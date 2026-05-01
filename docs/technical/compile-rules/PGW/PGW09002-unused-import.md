---
audience: developer
rule: "9.2"
code: PGW09002
name: Unused Import
severity: warning
---

# Rule 9.2 — Unused Import
`PGW09002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** An `[@]` import alias declared in the `{@}` package block that is never referenced by any `@alias-Pipeline` or `@alias#Data` expression in the file is flagged with a warning. The import is dead code.
**Rationale:** Unused imports add clutter and may indicate incomplete refactoring — a pipeline call was removed but the import was left behind. Flagging unused imports keeps the dependency graph clean and makes it clear which external packages are actually used.
**Detection:** The compiler collects all `[@]` alias declarations from the `{@}` block, then scans all expressions in the file for `@alias` references. Any alias with zero references is flagged.

**See also:** PGE09001 (undefined import alias — the inverse: using an alias that was never declared)

**VALID:**
```aljam3
[ ] ✓ all imports are referenced
{@} @MyPackage
   [@] @auth << "com.example.auth"
   [@] @data << "com.example.data"

{-} -Process
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   [ ]
   [-] @auth-Validate                   [ ] ✓ @auth used
      (-) <token << $input
      (-) >ok >> $valid
   [-] @data-Store                      [ ] ✓ @data used
      (-) <item << $valid
      (-) >result >> >out
```

```aljam3
[ ] ✓ no imports — leaf package using only aj3lib
{@} @LeafPackage

{-} -Simple
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   [ ]
   [-] >out << $input
```

**WARNING:**
```aljam3
[ ] ⚠ PGW09002 — @data imported but never referenced
{@} @MyPackage
   [@] @auth << "com.example.auth"
   [@] @data << "com.example.data"      [ ] ⚠ PGW09002 — @data never used

{-} -Process
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   [ ]
   [-] @auth-Validate
      (-) <token << $input
      (-) >ok >> >out
```

```aljam3
[ ] ⚠ PGW09002 — all imports unused
{@} @Abandoned
   [@] @old << "com.example.legacy"     [ ] ⚠ PGW09002 — @old never used
   [@] @also << "com.example.other"     [ ] ⚠ PGW09002 — @also never used

{-} -NoExternalCalls
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   [ ]
   [-] >out << $input
```

**Open point:** None.
