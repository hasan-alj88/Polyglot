# Lesson 002: Pipeline Triggers

**Date**: 2026-04-27
**Context**: Executing a Aljam3 pipeline.

## Lesson Summary

A pipeline block (defined with `{-}`) will never start execution unless it has a designated trigger.

### Trigger Syntax
Triggers are defined using the `[T]` block. Without this block, the pipeline is inert.

Example of defining a daily trigger at 3 AM:
```aljam3
{-} -CollectTemperatures
   [T] -T.Daily"3AM"
```
