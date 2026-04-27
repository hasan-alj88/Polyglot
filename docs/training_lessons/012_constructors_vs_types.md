# Lesson 012: Constructors vs Types

**Date**: 2026-04-27
**Context**: Using types and initializing values.

## Lesson Summary

In Polyglot, type definitions and constraints use the `#` prefix, while constructors for initializing values use the `$` prefix.

### Correct Usage
When providing a default or fallback value, use the constructor format `$Name"Value"`:
```polyglot
   (-) >matrix >> $southTemp#array:float:3D
      (>) >! $Array.Float:3D"Empty"
```

### Incorrect Usage
Using `#` when attempting to construct a value is invalid:
```polyglot
   (-) >matrix >> $southTemp#array:float:3D
      (>) >! #Array.Float:3D"Empty"
```
