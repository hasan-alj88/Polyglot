# Lesson 018: IO Markers

**Date**: 2026-04-27
**Context**: Providing parameters to, and getting results from, actions and pipelines.

## Lesson Summary

The Aljam3 compiler strictly enforces IO Scope Mismatches (PGE01061). You must match the IO marker property prefix (`<` or `>`) with its corresponding block context IO marker `(<)` or `(>)`. You cannot use the generic `(-)` for inputs and outputs.

### Correct Usage
```aljam3
   [-] -File.CSV.Log
      (<) <file.Path << "data.csv"
      (>) >status >> $logStatus
```

### Incorrect Usage
```aljam3
   [-] -File.CSV.Log
      (-) <file.Path << "data.csv"
      (-) >status >> $logStatus
```
