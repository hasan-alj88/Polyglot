# Lesson 019: Error Blocks

**Date**: 2026-04-27
**Context**: Handling errors within action blocks.

## Lesson Summary

In Polyglot, error execution blocks are opened using the `[!]` ActionError marker followed by the specific pipeline or action to execute on failure. Using standalone generic identifiers like `*!` is invalid and will fail validation (PGE01047).

### Correct Usage
```polyglot
   [-] -File.CSV.Log
      (<) <file.Path << "data.csv"
      [!] -Do.Nothing
```
