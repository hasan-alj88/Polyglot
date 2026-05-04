---
audience: automation-builder
type: specification
updated: 2026-05-04
status: draft
metadata_definition: "%definition.##:ErrorsTree"
metadata_instance: "%##:ErrorsTree:N"
---

# `##ErrorsTree` — Predefined System Errors Schema

<!-- @c:schemas -->

In Aljam3, the `{!}` Data Tree acts as the global registry for all predefined system errors. The `##ErrorsTree` schema guarantees exhaustive error classification, making it impossible for untracked exception types to propagate at runtime.

## Structural Definition

The terminals of the `##ErrorsTree` schema are strictly instances of `##Error`.

```aljam3
[ ] The Error Data Type
{#} ##Error
   [.] .Code#Int
   [.] .Message#String
   [.] .Fatal#Boolean

[ ] The ErrorsTree Schema
{#} ##ErrorsTree
   [.] .File
      [.] .NotFound##Error
      [.] .AccessDenied##Error
   [.] .Database
      [.] .Timeout##Error
      [.] .SchemaMismatch##Error
   [.] .Runtime
      [.] .CompilationFailed##Error
      [.] .ProcessCrashed##Error
      [.] .MissingDependency##Error
```

## Interaction with Expanders and Collectors

Every streaming Expander (`=File`, `=DB`, `=ForEach`) and Collector (`*File`, `*DB`) formally declares the subset of the `{!}` tree that it might yield during execution.

For example, `=File.CSV.Rows` universally yields:
* `(=) >error`: `!File.*` or `!Format.*` (which map to `{!}<File` and `{!}<Format`).

If the compiler asserts `!NoErrors`, it guarantees that none of the branches in the `{!}` tree will be triggered at runtime, completely eliminating the need for boilerplate `try/catch` logic.

## Immutability Triggers

Certain errors in the `{!}` tree, such as `!Database.SchemaMismatch`, trigger **Immutable Pipeline Safety**. If a resource's structure changes drastically after compilation, the Aljam3 runtime gracefully disables the pipeline and yields the corresponding fatal `##Error`, waiting for a human to recompile the project.

## Related
- [[jm3lib/types/schemas/Environments|##Environments]]
- [[jm3lib/types/schemas/Permissions|##Permissions]]
