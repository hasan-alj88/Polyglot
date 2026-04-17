---
audience: developer
type: spec
updated: 2026-04-16
status: spec
---

# Parallel Processing Overflow (PPTD)

<!-- @c:queue-manager/INDEX -->

## Overview

When parallel jobs produce intermediate data that exceeds available RAM, the system uses a three-tier overflow chain to prevent out-of-memory failures. The default path is always in-memory — overflow machinery activates only when thresholds are exceeded.

```
RAM (default) → overflow → PPTD (distributable) → overflow → batch collection
```

**PPTD** (Parallel Processing Temporary Directory) is a system-managed temporary directory created per parallel scope. Each parallel job writes intermediate results to its own subdirectory within the PPTD, avoiding file conflicts between concurrent jobs.

## Overflow Chain

### Tier 1: RAM (Default)

All collector intermediate data stays in memory. This is the fast path — no disk IO, no temporary files. If total intermediate data fits within `-Q.Overflow.RamLimit`, processing completes entirely in RAM.

### Tier 2: PPTD (Disk Spill)

When RAM usage exceeds `-Q.Overflow.RamLimit`, the system spills intermediate data to one or more PPTDs:

- Each parallel job gets its own subdirectory within the PPTD
- Multiple PPTDs can distribute the storage load (up to `-Q.Overflow.MaxPptdCount`)
- When one PPTD approaches `-Q.Overflow.PptdMaxSize`, the next PPTD is allocated
- PPTDs are **system-managed**: auto-created at scope entry, auto-deleted at scope exit

### Tier 3: Batch Collection

When all PPTDs approach capacity (usage ratio exceeds `-Q.Overflow.BatchThreshold`):

1. Parallel processing **pauses**
2. The collector runs a partial collection pass, merging completed job outputs
3. Collected data is freed from PPTDs
4. Processing **resumes** with freed space
5. Repeat until all jobs complete

This ensures processing can complete even when total intermediate data exceeds available disk space — as long as the final collected result fits.

## Queue Configuration

Configure overflow behavior via `-Q.Overflow.*` parameters in the `[Q]` queue block:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `-Q.Overflow.RamLimit` | `#int` | System-defined | Max RAM (bytes) for intermediate data before spilling to PPTD |
| `-Q.Overflow.PptdPaths` | `#Array.String` | System temp dir | Directory paths for PPTD allocation |
| `-Q.Overflow.PptdMaxSize` | `#int` | System-defined | Max disk (bytes) per PPTD before allocating next or triggering batch collection |
| `-Q.Overflow.BatchThreshold` | `#float` | `0.85` | PPTD usage ratio that triggers batch collection (0.0–1.0) |
| `-Q.Overflow.MaxPptdCount` | `#int` | `3` | Maximum number of PPTDs to distribute across |

### Example

```polyglot
[Q] =Q.Default
   [Q] -Q.Overflow.RamLimit << 536870912
   [Q] -Q.Overflow.PptdPaths << ["/tmp/pg-overflow", "/data/pg-overflow"]
   [Q] -Q.Overflow.MaxPptdCount << 5
   [Q] -Q.Overflow.BatchThreshold << 0.9
```

## Permission

Collectors require `_File.TempWrite` permission for PPTD paths — but **only when overflow actually spills to disk**. If processing completes in RAM, no file permission is needed.

```polyglot
{_} _CollectorOverflow
   (-) _File.TempWrite
   (-) .paths << -Q.Overflow.PptdPaths
```

## Error

| Error | When |
|-------|------|
| `!Storage.Space` | All overflow options exhausted — RAM full, all PPTDs full, batch collection cannot free enough space |

This error is compiler-enforced: any pipeline using collectors with potential overflow must handle `!Storage.Space` in an `[!]` block.

## Applicability

PPTD overflow is designed for text and CSV collection operators (`*Into.Text.*`, `*Into.CSV.*`) but the mechanism is reusable by any collector that processes large intermediate data. The overflow chain is transparent to the collector's merge logic — the same algorithm runs whether data is in RAM or on disk.

## Related

- [[queue-manager/INDEX]] — queue system overview
- [[queue-manager/infrastructure]] — runtime infrastructure
