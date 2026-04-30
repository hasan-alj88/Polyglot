---
audience: integrator
type: guide
updated: 2026-04-19
---

# The `aljam3-interface` Library

<!-- @c:vision#Ways of Integration -->
<!-- @c:glossary#Aljam3 Service -->
<!-- @c:glossary#Cross-Language Integration -->
<!-- @c:spec/aljam3-sdk -->

Not all users need to write Aljam3 Code to use the Aljam3 Service. For each supported programming language, a library called `aljam3-interface` (or the idiomatic variant for that ecosystem) provides direct code-to-code integration. The SDK achieves this through universal string serialization — all data crosses language boundaries as typed JSON envelopes through the Aljam3 Service's NoSQL store, eliminating the need for per-language FFI setup. For performance-critical pairwise integration, see `-Run.Bridge` ([[spec/aljam3-sdk#Design Decisions]]).

The [[glossary#Aljam3 Service|c:Aljam3 Service]] runs as a standalone service (server or local). The `aljam3-interface` library is the interface between your application and that service. It handles all communication and data translation between programming languages, so you write code in your preferred language and interact with code written in other languages seamlessly.

## What the Library Provides

1. **Pipeline execution** — Call a Aljam3 pipeline with a given input and receive the output.
2. **Environment management** — Define a runtime environment and run code in a different programming language, with results returned in your native language's types.

## Python Example

```python
import aljam3_interface as pg

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

## Bridge Integration

<!-- @c:aj3lib/pipelines/Run/Bridge.Function -->
<!-- @c:aj3lib/pipelines/Run/Bridge.Script -->

For performance-critical pairwise integration, Aljam3 provides `-Run.Bridge` pipelines that convert data directly between two language type systems -- bypassing the universal string serialization used by the SDK.

Bridge pipelines are written in Aljam3 code (`.aj3` files) and require two `-W.Env` wrappers, one per language. The SDK caller invokes Bridge pipelines the same way as any other pipeline -- through `call()`:

```python
# Python calling a Bridge pipeline that runs Rust code
result = await pg.call("CrossLangProcess", {
    "inputData": [1.5, 2.7, 3.14, 4.0]
})
```

```go
// Go calling a Bridge pipeline that runs Python ML code
result, err := pg.Call(ctx, "GoToPythonML", map[string]interface{}{
    "features":  []float64{1.0, 2.0, 3.0, 4.0, 5.0},
    "modelName": "linear_demo",
})
```

The SDK caller does not manage language environments or type conversion directly -- the Bridge handles that inside the Aljam3 Service.

### SDK vs Bridge

| Aspect | SDK (`call`/`pull`/`push`) | Bridge (`-Run.Bridge.*`) |
|--------|----------------------------|--------------------------|
| Serialization | Universal string (JSON envelopes) | Pairwise native type conversion |
| Setup cost | Zero -- works for all supported languages | Per-pair: requires `.aj3` pipeline with two `{;}` environments |
| Performance | JSON serialization overhead | Direct type mapping, minimal overhead |
| Language coverage | All supported languages | Only implemented language pairs |
| Who writes it | Integrator (host-language code only) | Automation builder (Aljam3 code) |

**Use the SDK** when you need universal integration across many languages with minimal setup. **Use Bridge** when a specific language pair is a performance bottleneck and the conversion overhead of JSON serialization is unacceptable.

See [[aj3lib/pipelines/Run/Bridge.Function|-Run.Bridge.Function]] and [[aj3lib/pipelines/Run/Bridge.Script|-Run.Bridge.Script]] for the full Aljam3-side specification.

## Codebase Location

In the Aljam3 source tree, the `aljam3-interface` library implementations live under `lib/<language>/integrator/`. See `lib/README.md` for the full structure.

## Related

- [[vision#Ways of Integration|c:Ways of Integration]] — high-level integration strategy
- [[vision#Evolution|c:Evolution]] — Phase 1 (orchestration) and Phase 2 (variable-level) roadmap
- [[aljam3-sdk]] — SDK specification, type mapping, and serialization algorithm
