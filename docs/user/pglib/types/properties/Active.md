---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Active"
---

# %##Active

<!-- @c:syntax/types/schema-properties -->

`%##Active` declares which branches must be present in a type instance. It governs whether all, exactly one, or a subset of children are active.

## Allows (`%##Active << #All`)

```
#Person [%##Active << #All]
├── .name   -> "Alice"     ✓ present
├── .age    -> 30          ✓ present
└── .email  -> "a@b.com"   ✓ present
                            ← all fields required
```

## Allows (`%##Active << #One`)

```
#Boolean [%##Active << #One]
├── .True                   ✓ active
└── .False                  ○ inactive
                            ← exactly one branch active
```

## Allows (`%##Active << #Partial`)

```
#Permissions [%##Active << #Partial]
├── .Read    ✓ active
├── .Write   ✓ active
├── .Execute ○ inactive
└── .Delete  ○ inactive
              ← any non-zero subset
```

## Disallows

```
#Person [%##Active << #All]
├── .name   -> "Alice"
└── .age    -> 30
                            ✗ .email missing — #All requires every branch

#Boolean [%##Active << #One]
├── .True                   ✓ active
└── .False                  ✓ active
                            ✗ two branches active — #One requires exactly one

#Permissions [%##Active << #Partial]
├── .Read    ○ inactive
├── .Write   ○ inactive
├── .Execute ○ inactive
└── .Delete  ○ inactive
              ✗ zero branches active — #Partial requires at least one
```

## Values

| Value | Meaning |
|-------|---------|
| `#ActiveKind.All` / `#All` | Every branch must be present |
| `#ActiveKind.One` / `#One` | Exactly one branch active at a time |
| `#ActiveKind.Partial` / `#Partial` | Any non-zero subset of branches |

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Enum|##Enum]] -- uses `%##Active << #One`
- [[schemas/Record|##Record]] -- uses `%##Active << #All`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
