# Polyglot Syntax Corrections: Memory Lesson 1

**Date:** 2026-04-25
**Scope:** Pipeline Definitions, Parallel Execution, and Environment Syntax

## Corrections Learned

1. **Parallel Execution (`[=]`)**
   - **Incorrect (Old Assumption):** Using `[=]` as a dedicated block that wraps sequential `[-]` calls.
   - **Correct:** `[=]` replaces the `[-]` marker for parallel execution. It is used directly on the operation line: `[=] -API.Vantage.GetPrice`.

2. **Triggers, Queues, and Wrappers**
   - **Incorrect:** Nesting pipelines under a `{T}` definition block.
   - **Correct:** `[T]`, `[Q]`, and `[W]` are placed inline directly within the `{-}` pipeline block just below the input/output declarations, e.g.:
     ```polyglot
     {-} -FetchAndNotifyStocks
        [T] -T.Daily"3AM"
        [Q] -Q.Default
        [W] -W.DB;StocksDB
     ```

3. **Environments and Imports**
   - **Package Declaration:** The package header at the top of the file uses namespace/domain framing: `{@} @Namespace:Domain<Path` (e.g., `{@} @Company:MyCompany<Jobs.Stocks`).
   - **Environment Mapping:** Instead of `{;}` local blocks, you can import and alias external environments directly in the package header using `[@]`: `[@] ;StocksDB << @StocksDBop;StocksDB`.
   - **Imports Mapping:** Package imports are mapped under the `{@}` header using `[@]`: e.g., `[@] @StocksDBop << @MyCompany.DB`.
   - **Compound Identifiers:** You can chain package, pipeline, and environment on invocation: `[=] @StocksDBop-DB.Traders.GetAllMails;StocksDB`.

4. **Formatting and Comments**
   - Use `[ ]` for standalone comments/spacer lines between operations, instead of using `( )` everywhere. `( )` is strictly for inline IO comments.

5. **String Interpolation**
   - Use `{$VariableName}` inside strings instead of appending with `+`: `"The price is {$Price}$"`.

**Action taken:** This memory file ensures the Polyglot Coder persona correctly applies these pipeline execution markers going forward.
