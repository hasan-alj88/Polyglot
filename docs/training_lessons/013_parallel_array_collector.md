# Lesson 013: Parallel Array Collector

**Date**: 2026-04-27
**Context**: Collecting outputs from parallel execution directly into an array.

## Lesson Summary

If the objective is to collect variables generated from parallel actions and store them directly into an array, use the `[*] *Into:Array` collector instead of `[*] *All`. 

This avoids needing a separate array declaration and manual assignment.

### Correct Usage
```polyglot
   [*] *Into:Array
      (*) << $northTemp
      (*) << $southTemp
      (*) >> $worldTemp#array:float:3D
```
