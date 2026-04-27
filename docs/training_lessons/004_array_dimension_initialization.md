# Lesson 004: Array Dimension Initialization

**Date**: 2026-04-27
**Context**: Initializing multidimensional arrays.

## Lesson Summary

When declaring and initializing a multidimensional array, the explicit type assigned on the right-hand side constructor must include the dimension specifier matching the left-hand side.

### Correct Usage
If you are declaring a `3D` array, the initialization type must explicitly be `#Array.Type:3D`.
```polyglot
[-] $worldTemp#array:float:3D << #Array.Float:3D
   (#) << $northTemp
   (#) << $southTemp
```

### Incorrect Usage
Omitting the `:3D` dimension specifier on the right side will cause an error or fallback to a default 1D array type mismatch.
```polyglot
[-] $worldTemp#array:float:3D << #Array.Float
```
