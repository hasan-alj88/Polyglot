---
audience: developer
rule: "1.23"
code: PGE01023
name: "Retired — Parameterless Macro"
severity: retired
redirect: "#272"
---

### Rule 1.23 — Parameterless Macro (RETIRED)
`PGE01023`

**Status:** Retired as of Issue #272 — Parameterized ## schemas replace {M} macros.

**Original purpose:** Enforced that `{M}` type macros declared at least one parameter. Macros with no parameters should have been plain `{#}` definitions.

**Why retired:** The `{M}` block type has been removed entirely. Parameterized types now use generic `{#}` definitions with `[#] <#param` type inputs and `[#] <param` value inputs. The concept of a "parameterless macro" no longer applies.

**See also:** [[09-definition-blocks|EBNF §9.2]] — `{#}` data definitions with generic parameters.
