# Lesson 021: Conditional Logical AND and Operation Status

**Date**: 2026-05-01
**Context**: Checking statuses of multiple operations in boolean predicates.

## Lesson Summary

When evaluating the state of executed operations in a conditional query block, use the metadata accessor `%` to access the operation's status (e.g., `$OperationName%status`). 
Furthermore, you can perform a logical AND condition by chaining an `[&]` block immediately after a `[?]` block.

### Correct Usage
```aljam3
   [?] $EmailCreationOp%status ?= @Mail#Status.Success
   [&] $StorageCreationOp%status ?= @Cloud#Status.Success
      [-] >onboardingStatus#OnboardingStatus << #OnboardingStatus.Completed
   [?] ?*
      [-] >onboardingStatus#OnboardingStatus << #OnboardingStatus.Failed
```
