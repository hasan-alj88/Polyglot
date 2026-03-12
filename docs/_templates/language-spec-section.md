---
id: CHANGE-ME
audience: developer
type: language-spec
status: draft
updated: YYYY-MM-DD
---

# Language Feature Name

## Summary

<!-- One paragraph: what this feature is and why it exists -->

## Formal Grammar (EBNF)

```ebnf
<!-- EBNF grammar rules for this feature -->
feature_rule  = prefix , identifier , body ;
prefix        = "[" , marker_char , "]" ;
```

## Railroad Diagram

```
feature_rule:
  ┌─────────┐   ┌────────────┐   ┌──────┐
──┤ prefix  ├───┤ identifier ├───┤ body ├──
  └─────────┘   └────────────┘   └──────┘
```

## Semantics

<!-- What the construct means at runtime -->
<!-- How it behaves in different contexts -->

## Type Rules

<!-- Type constraints, inference rules if applicable -->
<!-- What types are accepted/produced -->

## Examples

### Valid Usage

```polyglot
<!-- Valid example with annotation explaining what it does -->
```

```polyglot
<!-- Another valid example showing a different pattern -->
```

### Invalid Usage

```polyglot
<!-- Invalid example with explanation of WHY it fails -->
```

## Edge Cases

<!-- Boundary conditions, unusual inputs -->
<!-- Interaction with other language features -->

## Constraints

<!-- Rules and restrictions -->
<!-- What is NOT allowed and why -->

## See Also

- [Related grammar rule](path#section)
- [User guide for this feature](../User/path)
- [Design history](path) -- why this design was chosen
