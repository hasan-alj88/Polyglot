---
audience: design
type: spec
updated: 2026-04-16
---

# Deployment Architecture

<!-- @c:technical/plan/queue-manager/infrastructure -->
<!-- @c:technical/plan/queue-manager/process-isolation -->

The Aljam3 Service deploys as a set of cooperating daemons managed by a single system package. The Runner requires direct Linux kernel access (PID namespaces, cgroups v2, CRIU) — container runtimes like Docker conflict with these primitives. Aljam3 IS the isolation layer for its jobs.

## Sections

- [[packaging]] — How to build the .deb and .rpm packages: nfpm configuration, binary sources, systemd units, postinst/prerm scripts, APT repository hosting
- [[installation]] — How users install, configure, and verify the Aljam3 Service: prerequisites, install modes (all-in-one vs distributed), TLS certificate setup
- [[lifecycle]] — Upgrade, rollback, data migration, version compatibility, and uninstall procedures

## Component Inventory

| Component | Binary | Source | Role |
|-----------|--------|--------|------|
| Compiler | `aljam3` | Rust (cargo build) | Compiles .jm3 files to Behavior Contracts |
| Trigger Monitor | `aljam3-tm` | Rust (cargo build) | Long-running daemon — monitors triggers, manages job hierarchy |
| Runner | `aljam3-runner` | Rust (cargo build) | Long-running daemon — executes jobs in PID namespaces with cgroup limits |
| Management CLI | `aljam3-ctl` | Rust (cargo build) | Setup, join, status, certificate management |
| Queue Handler | Lua scripts | Hand-written | Loaded into Redis — atomic dispatch logic |
| Redis | `redis-server` | Vendor binary (redis.io) | State store — queues, counters, sets |
| NATS | `nats-server` | Vendor binary (nats.io) | Messaging backbone — inter-service signals |

## Why Not Docker

The Runner creates PID namespaces, manages cgroups, uses pidfd operations, and performs CRIU checkpoint/restore for cross-host job migration. These operations require direct kernel access that conflicts with container sandboxes:

| Kernel Operation | Docker Conflict |
|-----------------|-----------------|
| `clone(CLONE_NEWPID)` | Double namespace nesting — unnecessary overhead |
| cgroup management | Competing cgroup controllers (Docker's vs Aljam3's) |
| CRIU dump/restore | Requires `--privileged` — defeats container isolation |
| `pidfd_open()` + `pidfd_send_signal()` | Requires host PID namespace visibility |

Infrastructure dependencies (Redis, NATS) can run in containers — they do not require kernel-level access.

## Why Not Snap or Flatpak

| Format | Blocking Issue |
|--------|---------------|
| Snap (strict) | AppArmor sandbox blocks PID namespaces, cgroups, CRIU |
| Snap (classic) | Defeats Snap's value; Snap Store gates `--classic` review |
| Flatpak | bubblewrap sandbox fundamentally prevents all kernel ops; no escape hatch |

deb and RPM provide full host access, systemd integration, and standard package management. This matches every major Linux infrastructure project (Docker, Kubernetes, GitLab, PostgreSQL).

---

See also: [[infrastructure]], [[process-isolation]], [[redis-containers]], [[nats-namespace]]
