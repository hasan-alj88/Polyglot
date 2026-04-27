# Lesson 005: String Interpolation

**Date**: 2026-04-27
**Context**: Constructing dynamic strings in data operations.

## Lesson Summary

String interpolation is performed by enclosing the variable access with `{}` inside the string literal. The variable access includes the standard `<` tree child accessor or identifier syntax.

### Syntax Example
```polyglot
   [-] $file <~ "~/Data/{<today}/worldTemp.csv"
```
In this example, the value of `$today` (or `<today` depending on scope access) is injected into the string path.
