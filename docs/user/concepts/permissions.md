---
audience: user
type: specification
updated: 2026-03-25
status: complete
---

# Permissions

<!-- @identifiers -->
<!-- @blocks -->
Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the pipeline must explicitly declare permissions using `[_]` block elements with `_` prefixed [[identifiers]]. The `[_]` marker is registered in [[blocks#Permissions]].

This follows the Cisco ACL model: if you don't explicitly allow it, it's denied.

## Implicit Deny

A pipeline with no `[_]` declarations is **pure computation** — it can transform data, run conditionals, and call other pipelines, but cannot touch the outside world. Any IO call without a matching permission is a compile error.

```polyglot
{=} PureComputation
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [ ] no [_] lines — this pipeline cannot do IO
   [r] $result;int << =Math.Add $a $b
```

## Permission Identifiers

Permission identifiers use the `_` prefix with `.` fixed-field navigation. The top level names a category, the second level names a capability:

```
_Category.capability
```

Examples: `_File.read`, `_Web.request`, `_Database.connect`, `_System.env`

All subfields use `.` (fixed schema) because permission categories and their capabilities are predefined by Polyglot — not user-extensible.

## Inline Form

For single-argument permissions, use inline syntax — the argument follows the identifier directly in quotes:

```polyglot
[_] _File.read"/var/log/*"
[_] _File.write"/tmp/reports/*"
[_] _System.env"LOG_LEVEL"
[_] _Device.camera"true"
```

This mirrors inline pipeline syntax (`=Path"/tmp"`). The quoted argument is the scope or value of the permission grant.

## IO Form

When multiple parameters are needed, use IO form with input-only `[_]` lines:

```polyglot
[_] _Web.request
   [_] <url;string << "https://api.example.com/*"
   [_] <method;string << "GET"
[_] _Database.connect
   [_] <host;string << "localhost"
   [_] <port;int << "5432"
   [_] <database;string << "mydb"
   [_] <auth;string << "certificate"
```

IO form permissions are **input-only** — `[_] <param` with `<` input direction only. Permissions declare what is allowed, not what is returned. There is no `>` output in permission IO.

## Permission Categories

Eight predefined categories cover all IO capabilities:

| Category | Subfields | Form |
|----------|-----------|------|
| `_File` | `.read`, `.write`, `.execute`, `.delete` | Inline |
| `_Web` | `.request`, `.socket` | IO |
| `_Database` | `.connect`, `.read`, `.write` | `.connect` IO, others inline |
| `_System` | `.env`, `.process`, `.signal` | `.process` IO, others inline |
| `_Crypto` | `.key`, `.sign`, `.encrypt` | Inline |
| `_IPC` | `.send`, `.receive`, `.subscribe` | IO |
| `_Device` | `.camera`, `.microphone`, `.location`, `.bluetooth` | Inline |
| `_Memory` | `.allocate`, `.shared` | Inline |

The **Form** column indicates whether the capability typically uses inline (single argument) or IO (multiple parameters) syntax. IO-form capabilities have multiple parameters that must be specified together.

## Hierarchical Scoping

Permissions operate at two levels: **package ceiling** and **definition request**.

### Package Ceiling

`[_]` lines in `{@}` set the maximum permissions any definition in the package may request. The package ceiling **allows but does not grant** — no definition inherits permissions automatically.

```polyglot
{@} LogAnalyzer
   [_] _File.read"/var/log/*"
   [_] _Web.request
      [_] <url;string << "https://api.example.com/*"
      [_] <method;string << "GET"
```

### Definition Request

Each `{=}` pipeline or `{M}` macro must explicitly request the permissions it needs. Requests can only **narrow** what the package ceiling allows — never widen.

```polyglot
{=} ProcessLogs
   [_] _File.read"/var/log/app/*"
   [ ] narrower than ceiling — granted
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   ...

{=} ComputeStats
   [ ] no [_] lines — pure computation, zero IO
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   ...

{=} SneakyPipeline
   [_] _File.read"/etc/shadow"
   [ ] DENIED — outside package ceiling (compile error)
   ...
```

### No Inheritance

Permissions are never inherited. Every definition must restate the permissions it requires, even if the package ceiling allows them. This makes each definition's IO footprint explicit and auditable.

## Compile-Time Enforcement

All permission checks are **static analysis** — resolved at compile time, not runtime. The compiler verifies:

1. **Usage matches declaration** — a pipeline calling `=File.Text.Read` must have `[_] _File.read` declared
2. **Request within ceiling** — every `[_]` in a `{=}`/`{M}` must fall within the `{@}` package ceiling
3. **Pure computation enforced** — any IO call in a pipeline with no `[_]` lines is a compile error

No runtime permission checks exist. If it compiles, the permissions are satisfied.

## Foreign Code

<!-- @blocks#Foreign Code -->
Pipelines using `[c]` foreign code blocks ([[blocks#Foreign Code]]) interact with permissions as follows:

- The pipeline must declare `[_]` permissions for the IO the foreign code will perform
- The **compiler issues a warning** (not an error) that foreign code cannot be statically verified against declared permissions
- The **programmer takes responsibility** for ensuring the foreign code stays within declared permissions
- The **foreign runtime** (Python, Node, etc.) handles its own enforcement mechanisms if any

```polyglot
{=} AnalyzeData
   [_] _File.read"/data/*.csv"
   [ ] compiler warning: [c] block cannot be statically verified
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Python
   [c] #Code:Python:3:14
   [c] import pandas as pd
   [c] df = pd.read_csv("/data/report.csv")
   [c] result = df.describe()
```
