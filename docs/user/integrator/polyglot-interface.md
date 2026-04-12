---
audience: integrator
type: guide
updated: 2026-04-12
---

# The `polyglot-interface` Library

<!-- @c:vision#Ways of Integration -->
<!-- @c:glossary#Polyglot Service -->
<!-- @c:glossary#Cross-Language Integration -->

Not all users need to write Polyglot Code to use the Polyglot Service. For each supported programming language, a library called `polyglot-interface` (or the idiomatic variant for that ecosystem) provides direct code-to-code integration. This was the original vision for the Polyglot project: abstract away FFIs and the setup needed to interface one codebase with another.

The [[glossary#Polyglot Service|c:Polyglot Service]] runs as a standalone service (server or local). The `polyglot-interface` library is the interface between your application and that service. It handles all communication and data translation between programming languages, so you write code in your preferred language and interact with code written in other languages seamlessly.

## What the Library Provides

1. **Pipeline execution** — Call a Polyglot pipeline with a given input and receive the output.
2. **Environment management** — Define a runtime environment and run code in a different programming language, with results returned in your native language's types.

## Python Example

```python
import polyglot_interface as pg

# Create Rust environment
# pointing to the target Rust codebase (pre-built artifact or existing source)
rust_env = pg.create_environment(
    language="rust",
    code_base_path="/path/to/rust/code/base"
)

# Run a Rust function in the Rust environment
rust_output = await rust_env.run_function(
    function_name="rust_function",
    kwargs={"arg1": "value1", "arg2": "value2"},
    output_cast_type={"out": list[int]}
)
```

Behind the scenes, this creates a `{;}` environment definition and invokes `-Env.Run.Function` to execute the call asynchronously, then casts the result to the requested type.

The same pattern applies to other languages — Rust, JavaScript, Go — with idiomatic naming for each ecosystem's package manager.

## What `create_environment` Does

`create_environment` prepares the runtime and dependencies for code-to-code integration. It does **not** compile or build the target codebase — it expects a pre-built artifact or existing source. The `code_base_path` parameter locates the codebase so the environment can be configured accordingly.

## Codebase Location

In the Polyglot source tree, the `polyglot-interface` library implementations live under `lib/<language>/integrator/`. See `lib/README.md` for the full structure.

## Related

- [[vision#Ways of Integration|c:Ways of Integration]] — high-level integration strategy
- [[vision#Evolution|c:Evolution]] — Phase 1 (orchestration) and Phase 2 (variable-level) roadmap
