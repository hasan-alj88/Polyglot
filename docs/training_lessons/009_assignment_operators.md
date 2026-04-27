# Lesson 009: Assignment Operators

**Date**: 2026-04-27
**Context**: Assigning values to variables.

## Lesson Summary

Polyglot differentiates between "final" assignments and "mutable" (or pushable) assignments to optimize state tracking and mutation predictability.

1. **Final Assignment (`<<`, `>>`)**: Use this when a value is finalized and will not be pushed to or modified again.
2. **Mutable/Pushable Assignment (`<~`, `~>`)**: This is the default assignment operator when the variable is expected to be pushed to, appended, or modified further down the pipeline.

### Example
```polyglot
[ ] If a variable will not be mutated later, use <<
[-] $emailSubject << "Daily Campaign Report for {<campaignDate}"

[ ] If a variable will be appended to later, use <~
[-] $logList <~ "Start of log"
```
