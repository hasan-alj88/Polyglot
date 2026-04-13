---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##Alias"
---

# %##Alias

<!-- @c:syntax/types/schema-properties -->

`%##Alias` declares a lowercase shorthand name for a type or schema variant. Aliases follow the `#Boolean.True` -> `#True` pattern — the alias becomes a top-level name that resolves to the fully qualified path.

## Allows

```
{#} #Bound
   [.] .Inf
      [#] %##Alias << "inf"

#Inf  ← resolves to #Bound.Inf via alias
```

```
{#} #Boolean
   [.] .True
      [#] %##Alias << "true"
   [.] .False
      [#] %##Alias << "false"

#True   ← resolves to #Boolean.True
#False  ← resolves to #Boolean.False
```

## Disallows

```
{#} #Bound
   [.] .Inf
      [#] %##Alias << "INF"      ✗ uppercase — aliases must be lowercase

{#} #Bound
   [.] .Inf
      [#] %##Alias << "infinity"  ✗ if another type already claims "infinity"
                                     aliases must be unique across the namespace
```

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
