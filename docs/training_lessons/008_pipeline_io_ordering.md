# Lesson 008: Pipeline IO Ordering

**Date**: 2026-04-27
**Context**: Declaring pipeline variables and inputs.

## Lesson Summary

In Polyglot pipelines (`{-}`), all Input/Output (`IO`) declarations must occur **before** the trigger definitions (`[T]`). 

If you define pipeline variables or arguments after the trigger, the compiler will raise an error because the trigger needs to know the exact data context it is executing within before it resolves.

### Example
**Correct:**
```polyglot
{-} -RunDailyCampaign
   [ ] IO declared first
   (-) <campaignDate#DT << $DT"today"
   [ ] Trigger declared after IO
   [T] -T.Daily"8AM"
```

**Incorrect:**
```polyglot
{-} -RunDailyCampaign
   [T] -T.Daily"8AM"
   [ ] IO incorrectly declared after trigger
   (-) <campaignDate#DT << $DT"today"
```
