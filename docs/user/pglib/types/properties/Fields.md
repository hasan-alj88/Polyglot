---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Fields"
---

# %##Fields

<!-- @c:syntax/types/schema-properties -->

`%##Fields` declares how a type's children are indexed. It accepts either `#Range` (integer-indexed) or an enum reference (stamp one child per variant).

## Allows (`%##Fields << #Range`)

```
#Scores
├── :0  -> 95
├── :1  -> 87
└── :2  -> 72
         ← children keyed by integer index
```

## Allows (`%##Fields << #DayOfWeek`)

```
#Schedule
├── :Monday     -> "standup"
├── :Tuesday    -> "deep work"
├── :Wednesday  -> "review"
├── :Thursday   -> "deep work"
├── :Friday     -> "retro"
├── :Saturday   -> "off"
└── :Sunday     -> "off"
                 ← children keyed by enum variants
```

## Disallows

```
#Scores [%##Fields << #Range]
├── :0      -> 95
├── :name   -> "Alice"    ✗ string key — #Range requires integer indices
└── :two    -> 72         ✗ word key — not an integer

#Schedule [%##Fields << #DayOfWeek]
├── :Monday  -> "standup"
├── :Funday  -> "play"    ✗ not a #DayOfWeek variant
└── :0       -> "nap"     ✗ integer key — enum fields require variant names
```

## Values

| Value | Effect | Used By |
|-------|--------|---------|
| `#Range` | Integer-indexed children (`:0`, `:1`, `:2`, ...) | `##Array`, `##Dataframe` (L1) |
| enum ref | Stamps one `:` child per variant | `##Record`, `##Dataframe` (L2) |

`#FieldsDescriptor` is the governing enum with `.Range` and `.Enum` variants. `#Range` is an alias for `#FieldsDescriptor.Range`.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Array|##Array]] -- uses `%##Fields << #Range`
- [[schemas/Record|##Record]] -- uses `%##Fields << <#Fields` (enum ref)
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
