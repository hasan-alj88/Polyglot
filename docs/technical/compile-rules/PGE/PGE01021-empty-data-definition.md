---
audience: developer
rule: "1.21"
code: PGE01021
name: Empty Data Definition
severity: error
supersedes: PGW01002
---

# Rule 1.21 — Empty Data Definition
`PGE01021`

<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `{#}` data definition must contain at least one `[.]` fixed field, `[:]` flexible field, or `[#]` schema/generic composition. A `{#}` block with no structural content is a compile error.
**Rationale:** A data type with no fields cannot carry data and serves no purpose. Previously this was a warning (PGW01002) but has been upgraded to an error because the EBNF now requires at least one body line, making empty definitions grammatically invalid.
**Detection:** The compiler checks that each `{#}` block contains at least one field declaration or schema composition line. Comment-only and metadata-only blocks still trigger this error.

**Supersedes:** PGW01002 — Empty Data Definition (warning). The grammar now rejects empty `{#}` at parse time.

**VALID:**
```aljam3
[ ] ✓ data definition with fields
{#} #UserRecord
   [.] .name#string
   [.] .email#string
```

```aljam3
[ ] ✓ enum definition — enum variants count as fields
{#} #Status
   [.] .Active
   [.] .Inactive
```

```aljam3
[ ] ✓ schema composition provides structure
{#} #MyArray
   [#] ##Array
      (#) <#ValueType << #int
```

**INVALID:**
```aljam3
[ ] ✗ PGE01021 — no fields
{#} #EmptyRecord
```

```aljam3
[ ] ✗ PGE01021 — comment-only is still empty
{#} #Placeholder
   [ ] TODO: add fields later
```

```aljam3
[ ] ✗ PGE01021 — metadata-only is still empty
{#} #Tagged
   [%] .description << "a type with no fields"
```

**Diagnostic:** "Empty data definition `#Name` — requires at least one field or schema composition"
