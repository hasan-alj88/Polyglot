---
audience: [automation-builder, integrator, architect, designer]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
# Extensibility

> Polyglot is designed to grow safely. The same compile-time guarantees that protect core pipelines extend to every package in the ecosystem. This page explains how extensibility works without compromising safety. See [[vision]] for the broader project context.

## pglib: The Standard Library

Polyglot ships with `pglib` — a standard library of battle-tested operations that cover common automation needs. File operations, data transformations, date/time handling, collection operators, trigger types, queue strategies — these are not user-defined pipelines. They are compiler-known operations backed by native implementations, validated by the same exhaustive checks as any other pipeline.

pglib exists because automation has common patterns. Reading a file, parsing JSON, iterating over a collection, handling a cron trigger — every automation project needs these. Rather than forcing every developer to rewrite them (and reintroduce the bugs that come with reimplementation), Polyglot provides them as first-class language features with full compiler support.

## Community Packages

The ecosystem grows through packages. Developers publish reusable pipelines, types, and definitions that other projects can import. A package might provide Slack integration, database connectors, or domain-specific data transformations.

The critical property: **packages receive the same compile-time validation as core code.** When a project imports a package, the compiler validates the package's types against the project's types, verifies permission compatibility, checks error handling completeness, and confirms that the package's pipelines satisfy the same exhaustive coverage rules as locally defined pipelines. There are no "trusted" packages that skip validation. Every package is subject to the same compiler scrutiny.

## The Permission Ceiling

Packages cannot exceed the permissions granted by the importing pipeline. This is the **permission ceiling** — a structural guarantee that prevents packages from escalating access.

When a pipeline imports a package with `[@]`, the package's pipelines inherit the importing pipeline's `{_}` permission grants as an upper bound. If the importing pipeline has no file-write permission, no package it imports can write files — regardless of what permissions the package declares internally. The compiler enforces this ceiling at import time, not at runtime.

This means developers can import third-party packages without auditing every line of their source code for permission abuse. The ceiling ensures that a package designed for data transformation cannot secretly open a network connection, and a package designed for file operations cannot exceed the specific file paths the importing pipeline grants. The permission model is not just access control — it is a trust boundary enforced by the compiler.

## No Resource Hogging

Polyglot treats compute resources with the same discipline it applies to file access, network connections, and external APIs. Permissions are not just about *what* a job can access — they also govern *how much* of the host's resources a job can consume.

Any resource at 100% utilisation is system death. A job that consumes all available RAM starves every other process on the host — not just other Polyglot jobs, but the operating system, monitoring agents, and any other software sharing the machine. The same applies to CPU, GPU, disk I/O, and network bandwidth.

Polyglot enforces **default breathing margins** on every resource category. Jobs are prevented from exceeding a configurable ceiling that reserves capacity for the host's other processes. These margins exist by default — developers must explicitly raise them if they have a justified reason, and raising them requires explicit `{_}` permission grants with resource category declarations. The compiler validates that resource grants do not exceed the host's declared capacity, and the runtime enforces the limits through OS-level mechanisms (cgroups on Linux).

Exceeding a resource limit is not silent. It produces an error that requires handling — the same philosophy that governs every other error in Polyglot. A job that hits its RAM ceiling triggers a `!Resource.Exceeded` error with the same mandatory handling rules as any other error. There is no "the job just slows down" or "the OS kills it without notice." Resource exhaustion is a first-class error condition.

See [[cybersecurity]] for the zero-trust model that underpins the permission system, and [[error-philosophy]] for why resource errors demand explicit handling.
