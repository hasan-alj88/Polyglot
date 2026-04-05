---
audience: developer
rule: "1.23"
code: PGE01023
name: Parameterless Macro
severity: error
---

### Rule 1.23 — Parameterless Macro
`PGE01023`

**Statement:** A `{M}` type macro must declare at least one parameter (`[#] <param` or `[#] <#TypeParam`). A macro with no parameters is a compile error.
**Rationale:** Macros exist to generate type definitions parameterized by inputs. A macro with no parameters always produces the same output — it should be a plain `{#}` data definition instead. The EBNF requires at least one parameter line.
**Detection:** The compiler checks that each `{M}` block contains at least one `[#]` parameter declaration (value or type parameter).

**VALID:**
```polyglot
[ ] ✓ macro with value parameter
{M} #Array
   [#] <ElementType#RawString
   {#} #Array
      [.] .elements;array.{$ElementType}
```

```polyglot
[ ] ✓ macro with type parameter
{M} #TypedContainer
   [#] <#ContentType
   {#} #TypedContainer
      [.] .content;{$ContentType}
```

**INVALID:**
```polyglot
[ ] ✗ PGE01023 — no parameters, should be a plain {#}
{M} #Singleton
   {#} #Singleton
      [.] .instance#string << "only"
```

**Diagnostic:** "Macro `#Name` has no parameters — use `{#}` for constant type definitions"
