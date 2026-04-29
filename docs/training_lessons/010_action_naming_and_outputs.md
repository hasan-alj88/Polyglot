# Lesson 010: Action Naming and Output Referencing

**Date**: 2026-04-27
**Context**: Executing actions and resolving complex outputs.

## Lesson Summary

When executing an action (like calling an API) that generates structured results or enum responses, you should explicitly name the action execution context and use typed fallback patterns, rather than relying solely on arbitrary error blocks (`[!] *!`).

### Naming an Action
Assign a label to the action's execution block so you can reference its internal state or outputs directly using `>`.
```aljam3
   [-] @Mail-API.Email.SendAdminReport
      (-) $EmailOperation
```

### Structured Outputs and Fallbacks
Instead of catching generic errors with `[!] *!`, define the expected typed output enum and provide a fallback enum if it fails.
```aljam3
      (-) >status >> @Mail#Status.Delivered
         (>) >! @Mail#Status.FailedToSend
```

### Downstream Referencing
You can then extract the specific output from the named action using the `>` accessor:
```aljam3
   [-] >finalStatus >> $EmailOperation>status
```
