# Lesson 003: Parallel Collectors

**Date**: 2026-04-27
**Context**: Managing parallel execution outputs in a pipeline.

## Lesson Summary

When multiple independent actions are executed in parallel (e.g., fetching data from two different API endpoints simultaneously), their outputs cannot be immediately used downstream. 

If the variables produced by parallel execution (e.g., `$northTemp` and `$southTemp`) are not collected, it will result in a **compile error**.

### The `[*] *All` Collector
To synchronize the outputs, use the `[*] *All` block. This block waits for all specified parallel variables to be populated before execution proceeds.

```aljam3
   [=] @Weather-API.Sensor.GetRegion
      (-) <regionId << "North"
      (-) >matrix >> $northTemp#array:float:2D
   [ ]
   [=] @Weather-API.Sensor.GetRegion
      (-) <regionId << "South"
      (-) >matrix >> $southTemp#array:float:2D
   [ ]
   [*] *All
      (*) << $northTemp
      (*) << $southTemp
```
