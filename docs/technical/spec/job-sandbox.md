---
audience: architect
type: spec
updated: 2026-04-18
status: complete
---

# Job Sandbox Specification

<!-- @c:concepts/permissions/enforcement#Foreign Code Sandbox -->
<!-- @c:technical/spec/behavior-contract#Permission Manifest -->
<!-- @c:technical/spec/compiler-floor#Two Kinds of Native -->
<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:technical/compiler/compliance-report -->
Related: [[enforcement]], [[behavior-contract]], [[compiler-floor]], [[foreign-code]], [[compliance-report]], [[resource-controls]]

## Why OS-Level Sandboxing

Polyglot's compile-time permission model (`{_}` grants, ceilings, AST analysis) covers Polyglot-native code completely — if it compiles, the permissions are satisfied. But foreign code in `-Run.*` pipelines can make **raw syscalls**. A Python script can call `os.system()`, a Go binary can invoke `syscall.Open()`, a Rust binary can call `libc::connect()` directly. No runtime API layer intercepts these calls.

This makes OS-level sandboxing **mandatory**, not optional. The compiler emits a [[behavior-contract#Permission Manifest|Permission Manifest]] as part of the Behavior Contract. The [[glossary#Runner|Runner]] reads this manifest and configures OS-level restrictions before spawning the job process. The sandbox catches violations that AST analysis cannot detect — unresolvable variable paths, IO registry gaps, or new library functions not yet in the [[technical/compiler/ast-invisible-registry|AST-invisible registry]].

**Polyglot's position** is between Deno and gVisor:
- **Deno** enforces permissions at the runtime API layer — works because Deno controls the full JS API surface. Insufficient for arbitrary foreign code.
- **gVisor** reimplements the Linux syscall interface in user space — 2-10x overhead. Practical for medium-trust cloud workloads.
- **Polyglot** uses Landlock + seccomp-bpf + Linux namespaces + cgroups v2 — lighter than gVisor, stronger than Deno's runtime-only checks.

The principle remains: **compilation is a license to launch**. The sandbox narrows that license to exactly what was declared.

## Sandbox Setup Sequence

When the Runner spawns a foreign code job, it applies restrictions in this order:

```text
1.  fork()
2.  unshare(CLONE_NEWUSER | CLONE_NEWNS | CLONE_NEWNET | CLONE_NEWPID | CLONE_NEWIPC)
3.  Configure user namespace (map UID/GID)
4.  Mount minimal rootfs, bind-mount declared paths from {_} .path fields
5.  Configure network namespace:
    - If {_} #Web or #Database permissions exist: veth pair + nftables for declared hosts/ports
    - If no network permissions: empty namespace (all network fails)
6.  Move process to job-specific cgroup (resource limits)
7.  landlock_create_ruleset()
    - Add path rules for each {_} #File .path
    - Add port rules for each {_} #Web/#Database .port (Landlock ABI v4+)
    - landlock_restrict_self()
8.  Install seccomp-bpf filter:
    - Block: ptrace, mount, kexec, bpf, userfaultfd, personality, reboot
    - Block fork/exec unless {_} #System.#Process declared
    - Block socket creation unless {_} #Web or #Database declared
    - Allow all other syscalls (filesystem mediated by Landlock, network by namespace)
9.  prctl(PR_SET_NO_NEW_PRIVS, 1) — prevent privilege escalation
10. exec(foreign_code_interpreter, script)
```

All of this is achievable **without root** — user namespaces enable unprivileged creation of all other namespace types. See [[#Kernel Requirements]] for version requirements.

## Permission Category to Sandbox Mapping

Each `{_}` permission category maps to specific OS-level enforcement mechanisms:

| {\_} Category | {\_} Fields Used | Sandbox Mechanism | What It Restricts |
|---|---|---|---|
| #File.#Read | `.path` | Landlock `LANDLOCK_ACCESS_FS_READ_FILE` + mount namespace bind-mount (read-only) | Only declared paths readable |
| #File.#Write | `.path` | Landlock `LANDLOCK_ACCESS_FS_WRITE_FILE` + mount namespace bind-mount (read-write) | Only declared paths writable |
| #File.#Execute | `.path` | Landlock `LANDLOCK_ACCESS_FS_EXECUTE` + seccomp (allow execve) | Only declared binaries executable |
| #File.#Delete | `.path` | Landlock `LANDLOCK_ACCESS_FS_REMOVE_FILE` | Only declared paths deletable |
| #File.#Create | `.path` | Landlock `LANDLOCK_ACCESS_FS_MAKE_REG` + parent directory | Only under declared directories |
| #Web.#Request | `.host`, `.port` | Network namespace + nftables (allow outbound to host:port) | Only declared endpoints reachable |
| #Web.#Socket | `.host`, `.port` | Network namespace + nftables (allow TCP/UDP to host:port) | Only declared hosts/ports |
| #Web.#Listen | `.port` | Network namespace + nftables (allow inbound on port) | Only declared ports listenable |
| #Database | `.host`, `.port` | Network namespace + nftables (allow outbound to host:port) | Same as #Web — DB is TCP |
| #System.#Process | (capability flag) | seccomp allows fork/clone/execve | Without this, no child processes |
| #System.#Shell | (capability flag) | seccomp allows execve + Landlock for /bin/sh path | Without this, no shell invocation |
| #System.#Env | `.vars` (future) | Runtime prunes environment before exec | Only declared env vars visible |
| #Crypto | (capability flag) | No OS restriction needed (crypto is pure computation) | — |

**Layered enforcement:** Landlock provides filesystem and port-level restrictions. Network namespaces provide host-level isolation. seccomp-bpf provides syscall-level filtering. The combination is defense-in-depth — each layer catches what the others miss.

## Opaque Code: \_Unsafe.SandboxOnly

When a binary or code cannot have its AST fully analyzed (compiled binaries, obfuscated code, languages without tree-sitter support), the compiler cannot verify all permission compliance. The `_Unsafe.SandboxOnly` permission acknowledges this and activates maximum containment.

### Syntax

`[!] _Unsafe.SandboxOnly` appears in the error handler position of a `-Run.*` call:

```polyglot
{-} -ProcessData
   [.] %Authors << "jane.doe@company.com"
   [.] %Description << "Legacy Go binary for report generation — no source available"
   [.] %Version << "1.2.0"
   (-) _FileGrant
   (-) _NetworkGrant
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

### Behavior

1. **ALL isolation layers activate** — not just the standard sandbox, but every available defense mechanism, even those normally skipped for performance:
   - Standard: Landlock + seccomp + namespaces + cgroups
   - Additional: USER\_NOTIF supervisor (per-syscall inspection), stricter seccomp baseline (deny-by-default instead of allow-with-blocks), mount namespace with read-only rootfs, IPC namespace, PID namespace with PID 1 reaping
   - The performance cost is accepted in exchange for maximum containment

2. **AST analysis still runs** — the compiler analyzes whatever it can (imports, visible call sites, known patterns). But instead of emitting PGE10014 errors for AST-invisible constructs (eval, exec, dynamic imports), it **suppresses those errors to warnings**. The analysis results feed into the [[compliance-report|compliance report]] as "best-effort" findings.

3. **Mandatory metadata for accountability** — any pipeline using `_Unsafe.SandboxOnly` must have the following metadata fields filled:
   - `%Authors` — who wrote or approved this code
   - `%Description` — what the code does and why it needs opaque execution
   - `%Version` — version tracking for audit trail

### Compile Rules

- **PGE10015**: `-Run.*.CLI` or any opaque binary without `[!] _Unsafe.SandboxOnly` is a compile error — forces developer acknowledgment
- **PGE10016**: Pipeline using `_Unsafe.SandboxOnly` with missing `%Authors`, `%Description`, or `%Version` is a compile error — accountability is mandatory
- **PGW10007**: `_Unsafe.SandboxOnly` present — informational warning noting maximum sandbox enforcement active and suppressed AST-invisible errors listed

See the individual compile rule documents for full specifications.

### Design Rationale

This is not "less secure" — it is actually *more* sandboxed than normal pipelines. The trade-off is performance (all layers active) for trust (cannot verify the code). The mandatory metadata creates an audit trail: who approved this opaque code, what it does, and which version is running.

Without `[!] _Unsafe.SandboxOnly`, the compiler refuses to compile pipelines where it cannot analyze the code. The developer must either:
1. Provide source code (switch from `.CLI` to `.Function`/`.Script`) so AST analysis can run, OR
2. Add `[!] _Unsafe.SandboxOnly` with mandatory metadata to acknowledge opaque execution

## -Run.\* Mode Sandbox Variations

All `-Run.*` modes receive the same sandbox. The difference is compile-time confidence:

| Mode | Sandbox Strictness | Compile-Time Confidence |
|---|---|---|
| `-Run.*.Function` | Full sandbox | High — code visible to AST analysis AND sandboxed |
| `-Run.*.Script` | Full sandbox | High — code visible to AST analysis AND sandboxed |
| `-Run.*.CLI` | Full sandbox + PGE10015 | Low — binary is opaque, sandbox is primary enforcement |
| `-Run.*.Bind` | Full sandbox + PGW10003 | Low — code initiates calls, sandbox is primary enforcement |
| `-Run.Shell` | Full sandbox + mandatory #System.#Shell | Medium — shell is inherently dangerous, strictest baseline |

Function and Script modes have AST analysis providing high compile-time confidence. CLI and Bind modes rely primarily on the sandbox. All modes get identical OS-level restrictions.

## Runtime Violation Behavior

When the kernel blocks an operation (EACCES from Landlock, EPERM from seccomp):

1. The foreign code receives a permission error from the OS (e.g., Python's `PermissionError: [Errno 13]`)
2. The foreign code's error handling determines what happens next:
   - If the code catches and continues — job completes with degraded output
   - If the code crashes — job enters Failed state
3. The Queue Handler receives the job's exit status and routes accordingly (retry, error handling, etc.)

**No special Polyglot runtime intervention** — the kernel enforces, the process fails naturally, standard job lifecycle handles the failure. This keeps the runtime simple.

Resource limit violations (cgroup limits exceeded) follow a similar pattern — the kernel enforces limits, the process receives signals or throttling, and the Queue Handler manages the outcome. See the future resource categories specification for limit-exceeded behavior details.

## Sandbox Inspection

The `polyglot inspect -sandbox` command shows the effective sandbox configuration derived from the Permission Manifest:

```text
$ polyglot inspect -sandbox -ProcessData

Pipeline: -ProcessData
Sandbox Configuration:

  Filesystem (Landlock):
    READ:    /data/reports/*
    WRITE:   /output/results/*
    EXECUTE: /usr/bin/python3

  Network (namespace + nftables):
    OUTBOUND: api.internal.com:443
    OUTBOUND: pg.internal:5432

  Syscall Filter (seccomp):
    BLOCKED: ptrace, mount, kexec, bpf, userfaultfd
    ALLOWED: fork, clone (has #System.#Process)

  Resources (cgroups v2):
    RAM:       512MB (from _JobResources)
    CPU:       1.0 core (from _CPULimit)
    Processes: 10 (QH default)
    Duration:  300s (QH default)

  Opaque: No (AST analysis active)

  Compliance warnings: 2
    PGW10002: line 45 — cannot resolve resource in pd.read_csv(path)
    PGW10005: line 52 — unrecognized function custom_lib.process()
```

This is a development and debugging tool — it reads the compiled Permission Manifest and shows exactly what OS restrictions will be applied without running the pipeline.

## Implementation Approach

### Rust-Native (Recommended)

bubblewrap (`bwrap`) handles namespace setup well but lacks Landlock, cgroups v2, advanced seccomp (USER\_NOTIF), and GPU support — all of which Polyglot needs. Using bubblewrap would mean layering additional Rust code on top.

Rust-native implementation provides:
- Single codebase, no external dependency
- Full control over all five sandbox layers
- Structured error handling and audit logging at each setup step
- Zero runtime dependencies (compiled into the Runner)

### Key Crates

| Crate | Purpose | Source |
|---|---|---|
| `landlock` | Safe Landlock API | Maintained by the Landlock author |
| `seccompiler` | seccomp-bpf filter builder | From AWS Firecracker project |
| `nix` | POSIX API wrappers (unshare, clone, mount, pivot\_root) | Community maintained |
| `rustix` | Alternative to nix — lower-level, no libc dependency | Community maintained |

The Runner component (which already manages wrapper lifecycles and process spawning) is the natural place to apply sandbox restrictions.

## Supervisor Architecture (Optional Enhancement)

For cases where Landlock + namespace is not granular enough (e.g., host-level network filtering beyond port-level), a seccomp USER\_NOTIF supervisor provides per-syscall inspection:

```text
Runner Process
  +-- Job Process (sandboxed)
  |     \-- Foreign code executing
  \-- Supervisor Thread
        \-- seccomp USER_NOTIF listener
              +-- Intercepts: connect() -> inspects sockaddr -> matches {_} .host
              +-- Intercepts: open() -> inspects path -> matches {_} .path (backup)
              \-- Returns: ALLOW or ERRNO(EACCES)
```

seccomp USER\_NOTIF (kernel 5.0+) enables a supervisor to intercept specific syscalls, inspect their arguments (including pointer dereference via `/proc/pid/mem`), and approve or deny. This adds IPC latency per intercepted call (microseconds) but enables:

- **Host-level network filtering** — Landlock only does ports, not hosts
- **Path inspection backup** — for edge cases where mount namespace bind-mounts are insufficient
- **Audit logging** — of all IO operations for compliance

**Recommendation:** Start without USER\_NOTIF. Landlock + namespaces + nftables cover the majority of cases. Add USER\_NOTIF as an optional high-security mode for pipelines that need host-level granularity. When `_Unsafe.SandboxOnly` is active, USER\_NOTIF is always enabled.

## Kernel Requirements

| Feature | Minimum Kernel | Notes |
|---|---|---|
| User namespaces | 3.8+ | Enables unprivileged sandbox setup |
| seccomp-bpf | 3.17+ | Syscall filtering |
| Landlock (filesystem) | 5.13+ | Path-based filesystem restrictions |
| seccomp USER\_NOTIF | 5.0+ | Per-syscall supervisor inspection (optional) |
| Landlock (network) | 6.4+ | Port-based network restrictions |
| cgroups v2 | 4.5+ (unified) | Resource limits (CPU, memory, IO, PIDs) |

**Target: kernel 6.4+** for full Landlock support (filesystem + network ports). On older kernels, the Runner falls back to namespace-only network isolation (functional but coarser). The Runner logs a warning if Landlock network is unavailable.

All sandbox setup operates through **user namespaces** — no root privileges required. The Runner needs no special capabilities (no `CAP_SYS_ADMIN`, no setuid).

## Future Work

- **Resource categories in {\_}** — `#RAM`, `#CPU`, `#GPU`, `#IO`, `#Processes`, `#Duration` as top-level `{_}` permission categories, with `#LimitAction` enum for limit-exceeded behavior. See the tracked sub-issue.
- **OpenTelemetry logging** — structured OTel logs for all permission and sandbox events (violations, setup, resource limits). See the tracked sub-issue.
- **GPU device restrictions** — cgroups v2 device controller + vendor-specific APIs for GPU memory limits.
- **IPC restrictions** — seccomp filters for `shmget`/`msgget` when `#IPC` permissions are formalized.
