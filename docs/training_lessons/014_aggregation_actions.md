# Lesson 014: Aggregation Actions

**Date**: 2026-04-27
**Context**: Performing operations on collections or data.

## Lesson Summary

To perform aggregations (like finding the maximum value in an array), do not use inline method calls like `$array.Max()`. Instead, use the explicit aggregation action block `[-] =*Agg...` (e.g., `[-] =*Agg.Flat.Max`).

This block takes input using `(=) <<` and outputs the result using `(*) [type] >>`.

### Correct Usage
```aljam3
   [-] =*Agg.Flat.Max
      (=) << $worldTemp
      (*) #float >> $maxTemp
```
