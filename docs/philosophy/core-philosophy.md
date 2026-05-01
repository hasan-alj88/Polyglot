---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-22
---

<!-- @c:vision -->
<!-- @c:philosophy/symbology -->
<!-- @c:audit/reference/glossary -->
<!-- @u:concepts/pipelines/INDEX -->
<!-- @u:concepts/permissions/implicit-deny -->
<!-- @u:concepts/data-is-trees -->
<!-- @u:technical/spec/behavior-contract -->
# Core Philosophy

> This page expands the core philosophy summarized in [[vision]]. It is the authoritative source for Aljam3's mind-shift, values, and evolution strategy.

## Mind-Shift: Think with Intent, Abstract Away the How

The overarching theme of Aljam3 is **think with intent, abstract away the how**. Developers express *what* they need — a variable from C++ in a Python script, parallel tasks that combine results, a trigger that fires on a condition — and the Aljam3 platform handles the implementation details. This is not laziness; it is leverage. The "how" is built from battle-tested legacy code that has already solved those technical problems. Aljam3's job is to let developers focus on outcomes, not mechanisms.

This requires a fundamental shift in how developers think about their code:

- **There Is No Main Function** — Aljam3 pipelines are not synchronous programs with a `main()` entry point. Every [[glossary#Pipeline|pipeline]] is trigger-activated: something happens, the trigger fires, the pipeline executes. There is no "run this program" — there is "when this trigger fires, do this work." Developers coming from imperative languages must unlearn the sequential mindset and think in terms of triggers and reactions. Think of it this way: *what if APIs or microservices were a programming language?* Aljam3 manages reactions rather than actions. You are not writing a sequence of instructions — you are defining how the system should respond to signals and conditions, creating a network of interactions and dependencies that can adapt and evolve over time.

- **Compiles Intent, Not to Binary** — The Aljam3 compiler does not produce a binary executable. It compiles developer intent into instructions for the [[glossary#Aljam3 Service|Aljam3 Service]] — a complete specification of how to react and behave across all possible scenarios. The compiled output is a behavioural contract: "when X happens, do Y; if Y fails, do Z; if A and B race, resolve with C." This is closer to compiling a state machine than compiling a program. Think of it as a building permit: before construction can start, the permit must account for every disaster scenario and every building code requirement. The Aljam3 compiler is exhaustive — it accounts for every possible path the pipeline may take. Only once all scenarios are covered does it approve the instructions and compile them into a serialized form for the Aljam3 Service to register. Compilation is a license to launch.

- **Say Goodbye to Crashes: No Runtime Exceptions** — Other programming languages accept "Runtime Errors" and "Exceptions" as a normal part of the software lifecycle, allowing developers to write the "happy path" and leaving unexpected failures to blow up the process in production. Aljam3 fundamentally rejects this model. In Aljam3, errors are not "Exceptions" to be caught at runtime; they are "Expected" states to be handled at compile time. 

  Why? Because every unhandled edge case is a future production incident that usually happens at 3 AM. By eliminating the concept of a runtime crash and forcing all errors to be handled as strict compile-time requirements, Aljam3 shifts the burden. It forces the developer to account for every way things will go wrong during normal working hours — with full mental capacity and time — rather than forcing a sleep-deprived IT team to debug a crash and make critical decisions under extreme pressure. See [[error-philosophy]] for details.

- **Compile-Time Meets Async — Data Topology Is Known** — The concept of compile-time and runtime in parallel programming is fundamentally different from traditional programming. In synchronous code, if you don't know the shape of incoming data at compile time, the compiler cannot help you. But in Aljam3's asynchronous world, data topology *can* be determined at compile time — because it flows through the Aljam3 Service, which knows what each pipeline produces before the next one consumes it. Consider integrating Python with Rust: Rust's type system and borrow checker demand known data shapes at compile time. Traditionally, receiving dynamically-typed data from Python into Rust is a problem — Rust cannot compile unknown memory allocations when its types demand it. But in Aljam3, the data flows asynchronously through the platform. When Python produces output, Aljam3 knows the data topology and can dispatch it to Rust with all type constraints satisfied — because the Aljam3 compiler has already verified at *its* compile time that the data shapes align. Rust compiles happily with full type knowledge, and Aljam3 ensures the contract is honoured at its own compilation stage. This is a powerful consequence of the async-centric model: what appears to be a runtime concern in traditional programming becomes a compile-time guarantee in Aljam3.

- **Memory Is Not RAM** — Aljam3's memory model is fundamentally different from traditional programming. There are no local variables living in process memory, no data structures allocated on a heap that you manage with bits and bytes. Instead, Aljam3 state lives in the service infrastructure: objects are hosted in a NoSQL database, and queues are hosted in Redis under the [[glossary#Queue Handler|Queue Handler]]'s control. By living in NoSQL and Redis, Aljam3 frees developers from the memory limitations, bottlenecks, and segmentation faults inherent in process-bound RAM. This means developers work with data at a higher level of abstraction — as objects and documents in a distributed, shared resource — rather than as memory addresses in a single process. Different components of a pipeline can access and modify data because it lives in the service, not in any one runtime's address space.

- **Strings Are the Universal Interface** — Every programming language deals with strings — as code, as data, as serialization format. Strings are the common tongue between Python, Rust, JavaScript, Go, and every other language Aljam3 integrates. By building cross-language communication on this universal foundation, Aljam3 avoids the complexity of bespoke type marshalling between language runtimes. The [[glossary#Data Tree|Data Tree]] — where all leaves are [[glossary#RawString|RawString]] — is the structural embodiment of this principle: all Aljam3 data is ultimately a tree of strings, and strings are how languages already talk to each other.

- **Implicit Deny Permission** — Nothing runs without explicit, intentional permission. Developers must grant permissions for their pipelines to access resources, interact with external systems, or perform privileged operations. Even if the platform *could* allow an action, Aljam3 will not unless the developer has explicitly permitted it. Generosity in granting permissions is discouraged — the default is denial, and every grant is a conscious design decision. Permissions are not just about access-rights security — they are also a safeguard against concurrency issues and data integrity. By defining permissions for each component, the compiler can ensure that parallel jobs do not write to shared resources simultaneously, that data is not corrupted by concurrent access, and that resource conflicts are caught at compile time rather than at runtime.

  - **Permissions as Implicit Triggers** — Permissions function as a compile-time trigger gate. Just as a pipeline cannot fire until its `[T]` trigger conditions are met and its IO inputs are satisfied, a pipeline cannot fire — or even compile — without its required `_Permission` objects being granted. The compiler knows that a pipeline missing its permissions will never run regardless of the state of its other triggers, so it rejects the pipeline entirely at compile time rather than waiting for a runtime failure that is guaranteed to occur. This makes permissions part of the same "all gates must be open" model that governs IO inputs and triggers.

---

## Evolution

- **Aljam3 Optimization Evolution** — Aljam3 features will start slow, but as the project evolves and more languages are added, significant improvements in performance and efficiency will follow. Aljam3 will always have the objective of minimizing its footprint in codebase integration.

  The [[glossary#Integration Evolution|Integration Evolution]] strategy is **divide and conquer**: reduce the cross-language integration problem into smaller, solvable pieces and optimize each one.

  **Today:** Integration uses established mechanisms — pybind, FFI, and similar tools — adhering to the "On the Shoulders of Giants" principle.

  **Tomorrow:** Integration evolves to the variable level. For example, a Python list can be passed as a sized array to Rust (possible because it's an async call with known metadata), processed, and returned as a Python list. By reducing integration to the granularity of individual variables, we divide and conquer the problem progressively.

  This approach ensures the project remains maintainable and scalable while continuously improving the developer experience.

---

## Project Values

- **Collaboration and Community** — The project emphasizes collaboration and community involvement from the start of its design. By fostering a strong developer community, we encourage knowledge sharing, peer review, and collective problem-solving. This collaborative approach enhances quality and accelerates growth.

- **Open Source and Transparency** — Aljam3 is committed to being open source, allowing anyone to contribute, review, and use the code. This transparency promotes trust and encourages a diverse range of contributions, leading to innovative solutions and rapid development.

- **Security** — Automated tasks are vulnerable when behaviour is undefined or unexpected. Aljam3 closes this gap through its core philosophy: exhaustive coverage at compile time and implicit deny at runtime. Undefined behavior is a compile error, not an exploitable vulnerability. As the project evolves, we will define secure interactions between automated tasks and close potential security gaps as we and the community discover them.

  - **No Dynamic Code** — All code that Aljam3 executes must be static and analysable at compile time. No dynamically generated code is permitted to run through the Aljam3 platform at runtime. In the age of AI, where models can write and execute code on the fly, Aljam3 stands firmly against this pattern — not because dynamic code is never useful, but because it is a security risk that bypasses compile-time analysis. Developers may run code at their own risk outside of Aljam3, bearing the consequences of unanalysed execution. What Aljam3 guarantees is that every decision to run code — every permission grant, every file reference, every foreign code invocation — is made at compile time, where it can be inspected, validated, and approved before execution begins.

    To enforce this principle, Aljam3 applies **compile-time file binding**: any pipeline input that references a source file (e.g., `<code.file` in `-Run.*` pipelines, configuration files, data files) has its permission grant bound to the file's content at compilation time. If the file changes after compilation, the Aljam3 Service revokes the associated permissions and refuses to execute until the developer recompiles with the updated file. A file change watcher trigger monitors referenced file paths and notifies the developer when recompilation is required. This ensures that no code or input runs without having passed through the compiler's analysis.

  - **AI Execution Advisory** — AI systems may write Aljam3 code (`.jm3` files) for human review and compilation, but should never autonomously execute Aljam3 CLI commands (`aljam3 compile`, `aljam3 run`, etc.). Developers are advised to revoke any permissions that would allow AI tools to invoke the Aljam3 CLI directly. The distinction is clear: AI as author (human reviews and compiles) is acceptable; AI as autonomous executor (bypassing human oversight) is not. If a reliable technical mechanism for detecting and blocking AI invocation becomes available, Aljam3 will adopt it as an enforced safeguard.

- **Observability by Design** — OpenTelemetry (OTel) tracing is part of the Aljam3 runtime foundation, not a bolt-on afterthought. Every job execution, signal dispatch, and permission check is traceable from the moment the Aljam3 Service starts processing. This observability enables audit trails that connect directly to the Security and Accountability principles — when something happens in the system, there is always a record of what triggered it, who authorized it, and how it resolved.

- **Privacy** — The ability to run Aljam3 services locally on users' machines, without the need for cloud-based services, provides greater control over data and privacy. Local execution also reduces latency and improves performance, as users access services directly from their machines without relying on internet connectivity. Users can tailor services to their specific needs and preferences.

---

## Related Philosophy

- [[philosophy/language-design]] — Language design principles and safety model
- [[philosophy/symbology]] — Symbol design rationale
- [[philosophy/accountability]] — Human inspection and no dynamic code
- [[philosophy/cybersecurity]] — Zero trust and black box monitoring
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling
