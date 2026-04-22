---
audience: design
type: spec
updated: 2026-04-03
---

# Enum Instance Rules

<!-- @source:metadata-tree/INDEX -->

## Active-Field-Only Invariant

An enum instance collapses to exactly ONE active field. The definition (`%definition.#:{EnumName}`) lists all valid branches, but a specific instance has only the active one:

```polyglot
%definition.#:Boolean       <- schema: .True, .False (both listed)
%#:Boolean:0.True           <- instance 0: .True is active
                              .False does NOT exist for this instance
```

## Architecture Safeguards

The runtime enforces exactly one active enum field per instance:

| Operation | Behavior |
|-----------|----------|
| Push new field | Atomically clears previous field, sets new one |
| Read active field | Returns the field's value/path |
| Read non-active field | Returns no path (field does not exist on this instance) |
| Multi-set same instance in same scope | Compiler rejects (PGE prevents) |

See also: [[definition-templates|Definition Templates]], [[object-types|Object Type Branches]]
