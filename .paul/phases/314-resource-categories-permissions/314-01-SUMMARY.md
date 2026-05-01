---
phase: 314-resource-categories-permissions
plan: 01
status: complete
started: 2026-04-18
completed: 2026-04-18
---

# Plan 314-01 Summary: Resource categories as {_} permissions

## What Was Done

- Added 6 resource limit categories (#RAM, #CPU, #GPU, #IO, #Processes, #Duration) to #PermissionCategory enum (8 -> 14 variants)
- Created 6 capability enum types with per-category resource operations (Limit, Weight, Device, Iops)
- Created #LimitAction enum (Kill, Throttle, Retry) for limit-exceeded behavior
- Created #LimitConfig struct (.action#LimitAction + .gracePeriod#Duration)
- Updated capability-enums.md with 14 rows in both tables (category/capabilities + resource fields)
- Added 5 resource locator fields to permission-schema.md (.max, .weight, .device, .maxBps, .maxIops)
- Mapped all 6 resource categories to cgroups v2 controllers in job-sandbox.md (9 new rows)
- Added Resource Limits section to enforcement.md with 8 rows in Permission Category Mapping table
- Added Resource Limit Defaults section to queue.md with LimitAction fields and pipeline-level override example
- Removed resource categories from job-sandbox.md Future Work (now implemented)

## Files Created

- docs/user/jm3lib/types/RAMCapability.md
- docs/user/jm3lib/types/CPUCapability.md
- docs/user/jm3lib/types/GPUCapability.md
- docs/user/jm3lib/types/IOCapability.md
- docs/user/jm3lib/types/ProcessCapability.md
- docs/user/jm3lib/types/DurationCapability.md
- docs/user/jm3lib/types/LimitAction.md
- docs/user/jm3lib/types/LimitConfig.md

## Files Modified

- docs/user/jm3lib/types/PermissionCategory.md
- docs/user/concepts/permissions/capability-enums.md
- docs/user/concepts/permissions/permission-schema.md
- docs/user/concepts/permissions/enforcement.md
- docs/technical/spec/job-sandbox.md
- docs/user/concepts/pipelines/queue.md

## Decisions Made

- #Throttle only valid for CPU and IO — memory cannot be throttled, only killed (compiler validates this constraint)
- GPU device identifier deferred as capability flag (no specific GPU addressing scheme yet)
- Resource limits use existing flat __ResourceLocator structure with category-dependent validation
- QH applies sensible defaults (RAM 512MB, CPU 1.0, Processes 20, Duration 300s) when pipelines omit {_} resource permissions
- No separate __ResourceLocator schemas per category — flat structure follows established pattern

## Deviations

None.

## Open Items

- GPU device identifier specifics deferred (capability flag for now)
- {;} environment interaction with resource permissions deferred
- No compile rules added (PGE10003 handles unknown categories; resource-specific validation is future work)
