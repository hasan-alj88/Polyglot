---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
---

# -Variable — Variable Manipulation

<!-- @c:pipelines -->
<!-- @c:aj3lib/types/Variable -->
<!-- @c:aj3lib/types/NativeType -->

Variable manipulation pipelines operate on `#Variable` instances -- language-tagged values used by the Bridge system for cross-language data exchange.

No `[@]` import needed.

**PRIMITIVE** -- aj3lib runtime pipelines are direct language runtime integrations. They are implemented by the Aljam3 runtime and cannot be reimplemented in user `.aj3` files.

## Pipelines

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/Variable/Convert\|-Variable.Convert]] | Convert a `#Variable` from one language's type system to another |

## Related

- [[aj3lib/types/Variable|#Variable]] -- language-tagged variable type
- [[aj3lib/types/NativeType|#NativeType]] -- primitive type classifier per host language
- [[aj3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]] -- Bridge pipelines that use `#Variable`
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] -- conversion algorithm internals
