---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-22
---

<!-- @c:vision -->
<!-- @c:philosophy/behavioral-contract -->
<!-- @c:audit/reference/glossary -->
<!-- @u:concepts/permissions -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @u:syntax/packages -->
# Accountability

> Aljam3 exists to orchestrate code across language boundaries. That power demands a clear answer to one question: **who is responsible when something goes wrong?** This page defines Aljam3's answer. See [[vision]] for the broader project context.

## The Accountability Chain

Every piece of code that runs through the [[glossary#Aljam3 Service|Aljam3 Service]] passes through five gates — the **Accountability Chain**. No gate can be skipped.

1. **Author** — A person writes the code. The author's identity is recorded. Authorship is not anonymous.
2. **Inspector** — A human reads the code and takes personal responsibility for approving it. The inspector is not the compiler — it is a named person who decides "this should run."
3. **Compiler** — The Aljam3 compiler validates the code exhaustively: type safety, permission coverage, error handling, concurrency correctness. The compiler catches what humans miss, but it does not replace human judgement.
4. **Permission Grant** — The compiled output includes explicit `{_}` permission objects that declare what the [[glossary#Pipeline|pipeline]] is allowed to do. No permission, no execution. Grants are bound to the compiled content — any change invalidates them.
5. **Execution** — Only after all four preceding gates are satisfied does the Aljam3 Service execute the pipeline.

The compiler is powerful, but it answers "is this code correct?" — not "should this code exist?" That second question requires a human.

## Human Inspection Is Non-Negotiable

Correctness is not accountability. A compiler can verify that code is syntactically valid, type-safe, and free of race conditions. It cannot verify intent. It cannot judge whether a pipeline *should* delete a database, *should* send an email to every customer, or *should* grant network access to a foreign script.

**A named human must inspect and take responsibility for what the code does.** This is not about trust in tools — the compiler is trustworthy. It is about requiring a person who stands behind the decision to run this code. When something goes wrong, "the compiler approved it" is not an acceptable answer. "I reviewed it and approved it" is.

This principle applies regardless of who wrote the code — whether a junior developer, a senior architect, or an AI system. The inspector role exists because accountability requires a person, not a process.

## Why No Runtime-Generated Code?

All code that Aljam3 executes must be static and analysable at compile time. No dynamically generated code is permitted to run through the Aljam3 platform at runtime. This is enforced for three reasons:

### Security

Dynamically generated code bypasses the entire Accountability Chain. It has no author on record, no human inspector, no compile-time analysis, and no permission grant. It is unverified intent running with whatever privileges the host process holds. This is not a theoretical risk — it is the mechanism behind code injection, privilege escalation, and supply chain attacks.

The compiler's foreign code analysis (see [[technical/compiler/ast-invisible-registry]]) explicitly bans constructs like `eval()`, `exec()`, `Function()`, and their equivalents across all supported languages. These constructs generate code at runtime, making them invisible to compile-time analysis.

### Accountability and Auditability

All code must be reviewable by a human auditor who takes responsibility for its behaviour. Runtime-generated code cannot be reviewed before it runs — by definition, it does not exist until execution time. An auditor cannot inspect what has not yet been written.

This is not a limitation of current tooling. Even if future tools could predict what dynamic code *might* generate, the audit trail would depend on the prediction being correct — introducing exactly the kind of unverifiable assumption that Aljam3's design eliminates.

### Black Box Accountability

Compiled binaries referenced by `-Run.*` pipelines are opaque to the Aljam3 compiler — it cannot analyse their internal behaviour. Aljam3 addresses this by making `author` and `auditor` metadata fields **mandatory** for compiled binary inputs. A named person must be recorded as the author of the binary, and a named person must be recorded as having audited it.

This closes the "nobody's fault" loophole. If a compiled binary misbehaves, the metadata identifies who wrote it and who approved it. The auditor's name on the record means a real person accepted responsibility for the binary's behaviour — sloppy audits have consequences because a name is attached.

## AI Policy

AI systems are increasingly capable of writing code. Aljam3's position is clear: AI is a tool, not an authority. Three rules apply, with no exceptions:

1. **AI-generated code is untrusted by default.** Code produced by an AI system receives no special treatment — it enters the Accountability Chain at the Author stage and must pass through human inspection, compilation, and permission granting like any other code. The compiler does not distinguish between human-written and AI-written code.

2. **Auditing and inspection are always done by a human.** The Inspector gate in the Accountability Chain requires a named person. An AI system cannot fill this role. The purpose of inspection is not to find bugs — the compiler does that — but to take personal responsibility for the decision to run the code. Responsibility requires a person.

3. **Even if AI verification becomes reliable, a human must give the green light.** This is a design principle, not a technology limitation. Future AI systems may become excellent at code review. Aljam3 will still require a human inspector, because the principle is about accountability — who answers when things go wrong — not about capability. A system that removes human oversight removes human accountability.

## Compile-Time File Binding

Aljam3 enforces the Accountability Chain through **compile-time file binding** — **content-bound permissions**. Any pipeline input that references a source file — `<code.file` in `-Run.*` pipelines, configuration files, data files — has its permission grant bound to the file's content hash at compilation time.

If the file changes after compilation:

- The content hash no longer matches the compiled grant
- The Aljam3 Service **revokes** the associated permissions
- The pipeline **refuses to execute** until the developer recompiles with the updated file
- A file change watcher trigger notifies the developer that recompilation is required

This mechanism ensures that no code or input runs without having passed through the full Accountability Chain. A changed file means a changed intent — and changed intent requires fresh inspection, fresh compilation, and fresh permission grants.

For technical details on content-bound permissions, see [[user/concepts/permissions/enforcement]] and [[technical/spec/behavior-contract]].

---

## Related Philosophy

- [[philosophy/core-philosophy]] — Mind-shift, values, and evolution
- [[philosophy/language-design]] — Design principles and safety model
- [[philosophy/cybersecurity]] — Zero trust and black box monitoring
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling
