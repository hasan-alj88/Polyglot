---
audience: architect
type: spec
updated: 2026-04-14
---

# The Compiler Floor

<!-- @c:glossary#RawString -->
<!-- @c:glossary#Polyglot Service -->
<!-- @c:glossary#Trigger Monitor -->
Related: [[behavior-contract]], [[native-dispatch]]

## The Dictionary Problem

Every compiler faces the same problem as an English-English dictionary: some words must be self-evident, or all definitions become circular. In formal logic these are **axioms** — truths that require no proof because everything else derives from them.

<!-- @c:vision#Core Philosophy -->
In Polyglot, these axioms are called **native operations**. They are the irreducible floor — the atomic instructions and atomic async functions that everything else compiles down to. [[glossary#RawString|c:RawString]] is the type-system floor (the only compiler intrinsic). Native operations are the execution floor — the primitives that the [[glossary#Polyglot Service|c:Polyglot Service]] knows how to perform without further decomposition.

Polyglot Code exists to shield users from the complexity beneath this floor.

## Two Kinds of Native

Every native operation is one of two kinds:

### Atomic Instruction

A directive in the [[behavior-contract|c:Behavior Contract]] that configures the Polyglot Service's behavior. Instructions tell the [[glossary#Trigger Monitor|c:Trigger Monitor]] what to listen for and how to orchestrate.

All `-T.*` triggers are instructions. They tell the Trigger Monitor to set up listening behavior — the Trigger Monitor has all trigger listeners implemented internally and activates them when the signal map says to start.

Examples:

| Instruction | Effect |
|---|---|
| `-T.Call` | Register this pipeline as callable |
| `-T.Daily` | Activate the daily time-check listener |
| `-T.Webhook` | Activate the webhook listener on this endpoint |
| `-Q.Default` | Use FIFO dispatch |
| `-W.Polyglot` | Use standard wrapper behavior (built into [[glossary#Runner|c:Runner]]) |
| `-DoNothing` | Compiler-inlined no-op |

### Atomic Async Function

A real Rust function — a [[glossary#Job|c:job]] triggered by the Trigger Monitor. When the signal map says "execute this job," the Trigger Monitor sends the trigger signal through NATS, and the [[glossary#Runner|c:Runner]] (or [[glossary#Queue Handler|c:Queue Handler]]) invokes the async function.

"Async function" means specifically a job triggered *by* the Trigger Monitor. Things that *configure* the Trigger Monitor's listening behavior (triggers) are instructions, not async functions.

Examples:

| Async Function | Subsystem |
|---|---|
| `-File.Text.Read` | Runner: invoke Rust to read file |
| `-Math.Add` | Runner: invoke Rust for arithmetic |
| `-W.DB.Connection` | Runner: invoke Rust for connection lifecycle |
| `-Q.Pause.Hard` | Queue Handler: invoke Rust to implement pause |

### Categorization by Subsystem

| Pattern | Category | Rationale |
|---|---|---|
| All triggers (`-T.*`) | **Instruction** | Configure Trigger Monitor listeners |
| Queue strategy (`-Q.Default`) | **Instruction** | Dispatch rules for Queue Handler |
| Queue actions (`-Q.Pause.*`, `-Q.Resume`) | **Async Function** | Active commands invoking Queue Handler code |
| `-W.Polyglot` | **Instruction** | Built-in Runner behavior, no external code |
| Other wrappers (`-W.DB.*`, `-W.File.*`) | **Async Function** | Runner invokes Rust for resource lifecycle |
| Execution (`-File.*`, `-Math.*`, `-DB.*`) | **Async Function** | Runner invokes Rust for real work |
| Intrinsics (`-DoNothing`, `-#.JSON.Parse`) | **Instruction** | Compiler-inlined, no runtime dispatch |

## The `{N}` Block — Self-Documenting Natives

<!-- @u:technical/ebnf/definition-blocks/09-06-native -->
`{N}` definitions in pglib files are the self-documentation of native operations — designed before any Rust code is written. They serve two purposes:

1. **Make signal logic clear on paper** — what signals exist, what each native does
2. **Give the compiler vocabulary** to validate user code against real natives

<!-- @u:technical/compile-rules/PGE/PGE01028-base-derived-mutual-exclusion -->
The `{N}` block declares:

- Which kind: `#NativeKind` — Trigger, Queue, Wrapper, Execution, Intrinsic
- Host function name: `.Rust << "FunctionName"`
- IO contract: `(-)` input, output, and error declarations

Implementation status is tracked in the YAML frontmatter of the `.md` documentation file that accompanies each `{N}` definition — not in the `.pg` file itself.

### Adding a New Native Operation

Adding a native operation (e.g., `-T.MQTT`) requires all three steps:

1. Write the Rust async function (or Trigger Monitor listener implementation)
2. Add the `{N}` definition to pglib (self-documentation)
3. Update the compiler to recognize it

## The Floor Is Versioned

The native instruction set and async function set are fixed per Polyglot version. They change if and only if the Polyglot version changes — the same relationship as a CPU instruction set tied to its architecture revision. Users write infinite programs; the instructions those programs compile to are fixed.
