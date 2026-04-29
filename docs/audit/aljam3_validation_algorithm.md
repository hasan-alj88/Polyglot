# Aljam3 Compiler Validation Algorithm

The Aljam3 compiler utilizes a modular, multi-pass validation engine based on a Trait-driven architecture to enforce its rules while supporting robust cross-referencing. 

## Architectural Overview

The validation process is orchestrated in the `compiler::validator::validate` function. The core algorithm relies on a flat token stream rather than a heavy recursive AST:

1. **Context Initialization**: The token stream (`&[Spanned<Aljam3Token>]`) and raw source code lines are packaged into an `AnalysisContext`. 
2. **Sequential Algorithm Execution**: The compiler iterates over a list of registered rules (`get_all_rules()`), which are discrete algorithmic structures implementing the `Rule` trait.
3. **Report Generation**: Each rule algorithm populates a shared `ValidationReport`, pushing errors (like `PGE` error codes) with line numbers and messages.

## The Role of `AnalysisContext`

The true power of the validation engine—and the reason it handles cross-referencing so effectively—lies in the `AnalysisContext`. As it initializes, it computes a `token_contexts` stack for every token in the AST:

- **Scope Tracking**: It detects `ScopeIn` and `ScopeOut` markers, keeping a stack of definitions (e.g., entering a `Pipeline` definition, then a `Trigger` definition).
- **Flat AST with Deep Context**: Instead of building a complex hierarchical tree, the compiler retains a flattened token stream where each token is paired with its full structural lineage. 

### Why This Works for Cross-Referencing

Cross-referencing rules (such as checking if a variable pulled from the state machine was properly declared, or checking if a pipeline correctly targets an existing registry) require algorithms to look both backwards and forwards, traversing scopes.

Because of the `AnalysisContext`:
1. **O(1) Scope Resolution**: Any algorithm analyzing token $N$ knows exactly which `Pipeline` or `Package` it belongs to by looking at `ctx.token_contexts[N]`.
2. **Decoupled Passes**: Rule algorithms (like `VariableStateAlgorithm` or `CycleDetection`) can perform forward/backward scans across the entire token stream (`ctx.tokens`) without needing to recursively navigate a tree. If a variable is declared on line 10 and referenced on line 50, the `VariableStateAlgorithm` can simply build a state machine as it iterates through the flat token stream, knowing exactly when a variable transitions from `Declared` to `Final`.
3. **Separation of Concerns**: Structural algorithms (like `FileStructureAlgorithm`) run first to guarantee basic integrity. Then, deeper semantic algorithms (like `PipelineSemanticsAlgorithm` or `VariableStateAlgorithm`) can safely rely on that integrity when they perform cross-referencing validation, building internal HashMaps of definitions and checking references against them.

## Implemented Rule Algorithms

The `get_all_rules()` function registers several algorithmic sets:
- **Algorithm 1.1**: `FileStructureAlgorithm` (PGE01001, etc.) - Validates packages and global blocks.
- **Algorithm 1.2**: `InvalidTokensAlgorithm` (PGE01041-01047, etc.) - Validates structural markers and sigils.
- **Algorithm 1.3**: `DefinitionSemanticsAlgorithm` (PGE02008, etc.) - Validates declaration uniqueness.
- **Algorithm 2**: `IOSemanticsAlgorithm` (PGE01050-01055, etc.) - Validates input/output operators.
- **Algorithm 3**: `PipelineSemanticsAlgorithm` (PGE01008-01011, etc.) - Validates pipeline and trigger blocks.
- **Algorithm 3.5**: `QueueSemanticsAlgorithm` - Validates queue configurations.
- **Algorithm 4**: `PipelineAlgorithms` - Graph & Cycle Detection.
- **Algorithm 5**: `VariableStateAlgorithm` - Validates the Variable Lifecycle FSM.
