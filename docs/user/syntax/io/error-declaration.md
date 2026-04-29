---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Error Declaration

<!-- @u:errors:Declaring Pipeline Errors -->
<!-- @u:variable-lifecycle#Failed -->
Pipelines that can raise errors declare them in the IO section using `(-) !ErrorName`:

```aljam3
(-) <name#string
(-) >validated#string
(-) !Validation.Empty
(-) !Validation.TooLong
```

Error declarations use the same `(-)` marker as inputs (`<`) and outputs (`>`). The `!` prefix identifies them as error declarations. See [[errors#Declaring Pipeline Errors]] for compiler enforcement rules.
