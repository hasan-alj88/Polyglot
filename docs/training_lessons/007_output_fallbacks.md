# Lesson 007: Output Fallbacks

**Date**: 2026-04-27
**Context**: Providing fallback values for outputs.

## Lesson Summary

When receiving an output from an action, you can specify a fallback value that will be used if the output fails, is empty, or encounters an error.

### The Fallback Syntax
Use the `(>) >! [FallbackValue]` syntax immediately below the standard output mapping `(-) >output >> $variable`.

```aljam3
   [=] @Weather-API.Sensor.GetRegion
      (-) <regionId << "North"
      (-) >matrix >> $northTemp#array:float:2D
         (>) >! $Array.Float:2D"Empty"
```

In this example, if the `GetRegion` API call fails to return a matrix, the output will default to `$Array.Float:2D"Empty"`.
