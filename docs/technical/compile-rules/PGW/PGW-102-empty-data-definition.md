---
rule: "1.2w"
code: PGW-102
name: Empty Data Definition
severity: warning
---

### Rule 1.2w — Empty Data Definition
`PGW-102`

**Statement:** A `{#}` data definition that contains no `[.]` fixed field or `[:]` flexible field declarations produces a warning. The definition is syntactically valid but defines a type with no structure — likely a stub or incomplete definition.
**Rationale:** A data type with no fields cannot carry data. This is almost always an authoring oversight — the developer created the type but forgot to add fields. This is a warning rather than an error because the definition is structurally valid and may be intentional as a marker type.
**Detection:** The compiler checks each `{#}` block for at least one `[.]` or `[:]` field declaration. Comment lines (`[ ]`) and metadata lines (`[%]`) do not count as fields.

**See also:**
- [PGW-101 — Empty Execution Body](PGW-101-empty-execution-body.md) — same pattern for empty pipeline bodies

**VALID:**
```polyglot
[ ] ✓ data definition with fields
{#} #UserRecord
   [.] .name;string
   [.] .email;string
   [.] .role;string
```

```polyglot
[ ] ✓ data definition with flexible fields
{#} #Config
   [:] :setting;string
```

```polyglot
[ ] ✓ enum definition — fixed fields with no type are enum variants
{#} #Status
   [.] .Active
   [.] .Inactive
   [.] .Pending
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-102 — no field declarations
{#} #UserRecord
```

```polyglot
[ ] ⚠ PGW-102 — comment-only is still empty
{#} #Placeholder
   [ ] TODO: add fields later
```

```polyglot
[ ] ⚠ PGW-102 — metadata-only is still empty
{#} #Tagged
   [%] .description << "a type with no fields"
```

**Diagnostic:** "Empty data definition `#Name` — no `[.]` or `[:]` field declarations"

**Open point:** None.
