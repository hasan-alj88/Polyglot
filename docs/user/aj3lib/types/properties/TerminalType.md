---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##TerminalType"
---

# %##TerminalType

<!-- @c:syntax/types/schema-properties -->

`%##TerminalType` constrains all terminal branches to instances of a specific `{#}` type. Unlike `%###Type` (which constrains scalar leaf values), `%##TerminalType` works with any type regardless of depth — the terminal branch is itself a subtree that must conform to the referenced type's structure.

A "terminal branch" is a branch that has no further user-defined children — it is the end of the tree before the type's own fields begin.

## Allows

```
!Error [%##TerminalType << #Error]
├── :Validation
│   ├── .Empty  -> #Error instance      ← terminal conforms to #Error
│   │   ├── .Name -> "Validation.Empty"
│   │   ├── .MessageTemplate -> ...
│   │   ├── .Info -> #Record
│   │   ├── .Stderr -> null
│   │   ├── .StackTrace -> null
│   │   └── .ExitCode -> null
│   └── .TooLong -> #Error instance     ← terminal conforms to #Error
│       ├── .Name -> "Validation.TooLong"
│       └── ...
└── :Auth
    └── .Expired -> #Error instance     ← terminal conforms to #Error
        ├── .Name -> "Auth.Expired"
        └── ...
```

## Disallows

```
!Error [%##TerminalType << #Error]
├── :Validation
│   ├── .Empty  -> #Error instance      ✓ conforms
│   └── .TooLong -> "just a string"     ✗ not an #Error instance
└── :Auth
    └── .Expired -> #SomeOtherType      ✗ wrong type — must be #Error
```

## Values

| Value | Meaning |
|-------|---------|
| specific `#Type` | All terminal branches must be instances of that type |
| `#` | Any type (no constraint) |
| absent | No terminal type enforcement — each branch declares its own type |

## Relationship to `%###Type`

| Property | Scope | Target depth | Use case |
|----------|-------|-------------|----------|
| `%###Type` | Leaf values | Depth 0 (scalar) | `#int`, `#string`, `#bool` |
| `%##TerminalType` | Terminal branches | Any depth | `#Error`, `#Record`, any `{#}` type |

`%###Type` and `%##TerminalType` are mutually exclusive on the same level — a terminal cannot be both a scalar leaf and a multi-level type instance. The compiler raises PGE11004 if both are set at the same scope.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[properties/Type|%###Type]] -- scalar leaf type constraint
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
- [[aj3lib/errors/errors|Error System]] -- `{!}` uses `%##TerminalType << #Error`
