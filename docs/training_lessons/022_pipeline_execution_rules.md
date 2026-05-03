# Lesson 022: Pipeline Sub-job Execution Rules

**Date**: 2026-05-01
**Context**: Defining the Aljam3 Execution Engine behavior regarding structural cascading, data dependencies, parallel blocks, and synchronization barriers.

## Lesson Summary

The Aljam3 Execution Engine operates on an elegant **hybrid execution model** that perfectly merges Structural Control Flow with Data-Driven evaluation. A job transitions through states primarily governed by two distinct requirements: its placement in the code (Structure) and the readiness of its inputs (Data).

### 1. The Dual-Condition Enqueue Rule
A sub-job (or block) transitions to **Enqueued** ONLY when BOTH of the following conditions are satisfied:
1. **Structural Condition:** The previous sequential block has been Enqueued.
2. **Data Condition:** All of the job's required input variables (e.g., `<email << $userEmail`) are in a **Final state**.

*(Note: If the Structural condition is met but the Data condition is not, the job waits implicitly. It does not formally become Enqueued until the Data condition is also met).*

### 2. The `[=]` Independent Evaluation Rule
Jobs grouped by the parallel marker `[=]` belong to the same structural block, but they evaluate their Data Conditions *independently*. 

Example:
```aljam3
   [-] @Auth-API.Users.GetProfile
      (-) <id << <userId
      (-) >email#string >> $userEmail

   [=] @Cloud-API.Storage.CreateBucket
      (-) <owner << <userId

   [=] @Mail-API.Account.Create
      (-) <email << $userEmail
```
* **`@Cloud-API.Storage.CreateBucket`**: Its input (`<userId`) is likely already Final. Because it has no data dependency on `Auth-API`, it meets both Structural and Data conditions immediately and Enqueues, running **in parallel** with `Auth-API`.
* **`@Mail-API.Account.Create`**: Meets the Structural condition immediately (same block as Cloud), but its Data condition (`$userEmail`) is not met yet. It waits until `Auth-API` finishes before it Enqueues and runs.

### 3. The `[*] *All` Synchronization Barrier
The `[*] *All` block acts as a deliberate **Data Synchronization** construct. 
* By explicitly requiring specific operations (e.g., `(*) << $EmailCreationOp%status`) to be Final, it halts.
* **Crucially**: It does *not* structurally block the jobs beneath it. The structural progression cascades past it immediately. It only delays subsequent sub-jobs if those sub-jobs actually *depend* on the variables being synchronized (or if they depend on an output produced by the `[*]` block itself).

### 4. Post-Barrier Cascading
If a subsequent job does not depend on the variables synchronized by the `[*]` block, it will execute as soon as its own Data Conditions are met, completely ignoring the barrier.

Example:
```aljam3
   [*] *All
      (*) << $EmailCreationOp%status
      (*) << $StorageCreationOp%status
   [ ]
   [-] @Mail-API.Email.Send
      (-) <recipient << $userEmail
```
Because `[-] @Mail-API.Email.Send` only takes `$userEmail` (provided previously by `Auth-API`), it has no data dependency on anything happening in the `[=]` block or the `[*]` block. Therefore, it will Enqueue and run as soon as `$userEmail` is final, running concurrently with the account creation processes.

### 5. Final State Queries & Assignment
Conditional queries (`[?]`, `[&]`) and Variable Assignments (`[-] >onboardingStatus << ...`) act as final structural blocks. They trigger and evaluate safely only when the variables they explicitly inspect reach a Final state, securely closing the pipeline logic.
