---
audience: design
type: spec
updated: 2026-04-16
---

# Package Lifecycle

<!-- @c:technical/plan/deployment/INDEX -->
<!-- @c:technical/plan/queue-manager/infrastructure -->
<!-- @c:technical/plan/queue-manager/process-isolation -->

This document specifies how the Aljam3 package handles upgrades, rollbacks, data migration, and version compatibility across its components.

## Upgrade Process

### Service Restart Order

When `apt upgrade aljam3` installs a new version, the postinstall script restarts services in dependency order:

1. **Redis** — state store restarts first (data persisted to disk via RDB/AOF)
2. **NATS** — messaging restarts second (JetStream replays from disk)
3. **Trigger Monitor** — reconnects to Redis and NATS, reloads Behavior Contracts
4. **Runner** — restarts last, after all upstream services are available

Running jobs are handled by the Runner's graceful shutdown:

| Scenario | Behavior |
|----------|----------|
| Jobs in execution body | Runner sends SIGTERM to all jobs, waits 30s grace period, then SIGKILL |
| Jobs in `[\]` setup | Setup completes, then job is terminated before body |
| Jobs in `[/]` cleanup | Cleanup completes before Runner exits |
| CRIU checkpointed jobs | Remain on disk — restored after Runner restarts |

### Zero-Downtime Upgrade (Distributed)

In multi-host deployments, upgrade hosts one at a time:

1. Upgrade infra host (Redis, NATS, TM restart — brief signal gap)
2. Upgrade worker hosts sequentially (each Runner drains jobs before restart)
3. Verify cluster health: `aljam3-ctl status --cluster`

The Queue Handler (Redis Lua scripts) is updated atomically during step 1 — the postinstall script reloads all Lua scripts via `aljam3-ctl load-qh-scripts`.

## Behavior Contract Compatibility

The Behavior Contract is the compiled output of .jm3 files — a JSON signal map that the Trigger Monitor interprets at runtime.

### Contract Version Field

Every Behavior Contract includes a `contract_version` field:

```json
{
  "contract_version": "1.0",
  "pipelines": [ ... ],
  "signals": [ ... ]
}
```

### Compatibility Rules

| TM Version | Contract Version | Result |
|-----------|-----------------|--------|
| Supports v1.x | v1.0 contract | Runs normally |
| Supports v1.x | v1.3 contract | Runs normally (minor versions are backward-compatible) |
| Supports v1.x | v2.0 contract | Rejected — TM logs error, contract not loaded |
| Supports v2.x | v1.0 contract | Runs normally (major versions support previous major) |

**Rule:** A Trigger Monitor supports contracts from its own major version and one major version back. Users must recompile .jm3 files with the new compiler when skipping more than one major version.

## Data Migration

### Redis State

Redis stores runtime state (queues, job status, counters). This state is **ephemeral** — it represents in-flight work, not persistent configuration.

| Upgrade Type | Redis Impact | Action Required |
|-------------|-------------|----------------|
| PATCH | No schema change | None — Redis restarts, state reloaded |
| MINOR | Additive fields only | None — new fields use defaults |
| MAJOR | Schema may change | `aljam3-ctl migrate` runs automatic migration |

`aljam3-ctl migrate` reads the current Redis schema version from `aljam3:meta:schema_version` and applies incremental migration scripts. This runs automatically in the postinstall script for major upgrades.

### NATS JetStream Streams

NATS stores signal history in JetStream streams. Stream names and subject patterns may change between major versions.

| Upgrade Type | NATS Impact | Action Required |
|-------------|-------------|----------------|
| PATCH/MINOR | No stream changes | None |
| MAJOR | Stream names or subjects may change | `aljam3-ctl migrate` creates new streams, drains old ones |

### NoSQL Database

Queue definitions and job hierarchy are stored in NoSQL. The schema is versioned in the database itself.

| Upgrade Type | NoSQL Impact | Action Required |
|-------------|-------------|----------------|
| PATCH/MINOR | No schema change | None |
| MAJOR | Schema may change | `aljam3-ctl migrate` applies schema migration |

## Rollback

### Rollback to Previous Version

```bash
# Debian/Ubuntu — install specific version
sudo apt install aljam3=0.1.0

# Fedora/RHEL
sudo dnf downgrade aljam3-0.1.0
```

### Rollback Constraints

| Component | Rollback Safety |
|-----------|----------------|
| Binaries | Safe — old binaries replace new ones |
| Redis state | Safe for PATCH/MINOR — same schema. MAJOR rollback requires `aljam3-ctl migrate --reverse` |
| NATS streams | Safe — old TM ignores unknown streams |
| Behavior Contracts | Safe if rolling back within same major version. Cross-major requires recompile |
| QH Lua scripts | Safe — postinstall reloads scripts matching the installed version |
| Configuration | `config\|noreplace` in nfpm preserves user edits — no overwrite on rollback |

### Rollback Procedure

1. Stop all services: `sudo aljam3-ctl stop`
2. Install previous version: `sudo apt install aljam3=<version>`
3. If major version rollback: `sudo aljam3-ctl migrate --reverse --target <version>`
4. Start services: `sudo aljam3-ctl start`
5. Verify: `aljam3-ctl status`

## Multi-Version Clusters

In distributed deployments, infra host and workers may temporarily run different versions during rolling upgrades.

### Compatibility Matrix

| Infra Version | Worker Version | Supported |
|--------------|---------------|-----------|
| 1.2.0 | 1.2.0 | Yes — identical |
| 1.2.0 | 1.1.0 | Yes — same major, NATS signals compatible |
| 1.2.0 | 1.3.0 | Yes — same major, worker ahead is fine |
| 2.0.0 | 1.x.x | Yes — one major version back supported |
| 2.0.0 | 0.x.x | No — two major versions apart |

**Rule:** All hosts in a cluster must be within one major version of each other. The infra host should be upgraded first — it defines the signal protocol that workers must understand.

## Vendor Dependency Updates

Redis and NATS are bundled binaries with pinned versions. Updates follow this policy:

| Vendor Change | Aljam3 Version Bump | Process |
|--------------|----------------------|---------|
| Security patch (Redis/NATS) | PATCH | Rebuild package with updated vendor binary |
| Minor feature (Redis/NATS) | MINOR | Test compatibility, update `vendor/VERSIONS`, rebuild |
| Major version (Redis/NATS) | MINOR or MAJOR | Full integration test, may require QH Lua script changes |

Vendor binaries are tested against the QH Lua scripts and NATS signal protocol before each release. The CI pipeline includes integration tests that verify the full signal chain (TM → NATS → QH/Redis → Runner) with the exact bundled versions.

---

See also: [[packaging]], [[installation]], [[infrastructure]]
