# Lesson 015: Boolean Predicate Blocks

**Date**: 2026-04-27
**Context**: Conditionals and branching logic.

## Lesson Summary

Conditionals in Aljam3 are represented as structural blocks rather than inline variables. 

### Syntax
- The conditional block is opened with `[?]`.
- Evaluators use specific boolean operators, like `>?` for greater-than, and `=?` for equals.
- Child actions are nested inside the condition block.

### Exhaustiveness
All conditional branches must be exhaustive. A compile error will occur if paths are left unhandled. You must include a fallback/else block using `[?] *?` (which can map to an empty action like `[-] -Do.Noting`).

### Correct Usage
```aljam3
   [?] $maxTemp >? <threshold 
      [=] @Mail-API.Email.SendAlert
         (-) $SendEmailOp
   [?] $SendEmailOp>status =? @Mail#Status.FailedToSend
      [-] -File.CSV.Log
         (-) <file#Path << "alerts_failed.csv"
   [?] *?
      [-] -Do.Noting
```
