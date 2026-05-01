---
audience: developer
rule: "7.3"
code: PGE07003
name: Duplicate Fallback Assignment
severity: error
---

# Rule 7.3 — Duplicate Fallback Assignment
`PGE07003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Each output port may have at most one generic `!<` fallback and at most one `!Error.Name>` fallback per specific error. Duplicate generic fallbacks or duplicate error-specific fallbacks for the same error on the same output are compile errors.
**Rationale:** Multiple fallbacks for the same condition create ambiguity — which value should the runtime use? A single, deterministic fallback per error condition keeps the behavior predictable and explicit. Aljam3's compiler enforces determinism by construction — in a system where pipelines run concurrently, ambiguous fallback resolution would make pipeline behaviour non-reproducible.
**Detection:** The compiler collects all `(>)`/`(<)` fallback lines scoped under each `(-)` output. If two or more generic `!>` or two or more `!Error.Name>` with the same error name are found, PGE07003 fires.

**VALID:**
```aljam3
[ ] ✓ one generic + distinct error-specific fallbacks
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !> "generic"
      (>) !File.NotFound> "missing"
      (>) !File.ReadError> "error"
```

```aljam3
[ ] ✓ error-specific only, no generic
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !File.NotFound> "missing"
      (>) !File.ReadError> "error"
```

```aljam3
[ ] ✓ generic only
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !> "fallback"
```

**INVALID:**
```aljam3
[ ] ✗ PGE07003 — duplicate generic fallback on same output
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !> "first"
      (>) !> "second"              [ ] ✗ PGE07003
```

```aljam3
[ ] ✗ PGE07003 — duplicate error-specific fallback for same error
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !File.NotFound> "a"
      (>) !File.NotFound> "b"      [ ] ✗ PGE07003
```

**See also:**
- [PGE07001 — Error Block Scoping](PGE07001-error-block-scoping.md) — `[!]` block scoping rules
- [PGE07002 — Chain Error Scoping](PGE07002-chain-error-scoping.md) — chain-specific error rules
- [[user/syntax/io|IO]] — references PGE07003 in fallback duplicate rules
- [[user/concepts/errors|Errors]] — references PGE07003 in error fallback operators

