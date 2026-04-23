---
description: How to synchronize project gaps/inconsistencies with GitHub Issues
---

# GitHub Backlog Sync Workflow

This workflow is used by the **Product Owner agent** to ensure the local audit logs and GitHub repository are in sync.

1. **Scan Local Tracking Files**: 
   - Read `docs/audit/tracking/inconsistencies/INDEX.md` and `docs/audit/tracking/inconsistencies/*.md`
   - Read `docs/audit/tracking/coverage-gaps/INDEX.md` and `docs/audit/tracking/coverage-gaps/*.md`
   - Read `docs/audit/tracking/progress.md`

2. **Identify Unqueued Tasks**:
   - Filter items that do not have a GitHub Issue number recorded.
   - For `progress.md`, find areas marked "Not Started".

3. **Format GitHub Issues**:
   - Title: `[Audit: AREA] Task Description`
   - Description: Include a link to the local file, the Audience, and the specific failure from the `checklist`.
   - Labels: `audit`, `docs`, and the Audience label (e.g., `audiences/developer`).

4. **Update Local Tracking**:
   - Once the issue is created (or simulated), update the local file with the Issue ID.

5. **Handoff to Scrum Master**:
   - Provide the list of new Issue IDs for the Scrum Master to plan into the next "Sprint".
