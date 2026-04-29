# Lesson 011: Pipeline Mandatory Blocks

**Date**: 2026-04-27
**Context**: Defining pipelines.

## Lesson Summary

A pipeline block `{-}` requires three mandatory configuration blocks to compile successfully. Without them, the pipeline context is incomplete.

1. **Trigger `[T]`**: Defines when the pipeline runs (e.g., `-T.Hourly`, `-T.Daily"10AM"`). Note that `-T.Hourly` is equivalent to `-T.Every"1H"`.
2. **Queue `[Q]`**: Defines the queue configuration (e.g., `-Q.Default`).
3. **Worker/Workflow `[W]`**: Defines the worker assignment (e.g., `-W.Aljam3`).

### Correct Usage
```aljam3
{-} -AlertPipeline
   [T] -T.Hourly
   [Q] -Q.Default
   [W] -W.Aljam3
```
