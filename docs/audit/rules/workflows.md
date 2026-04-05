---
audience: ai
type: audit-rules
scope: documentation-workflows
updated: 2026-03-30
---

# Documentation Workflows

<!-- @audit/README -->
<!-- @rules/checklist -->
Three workflows for maintaining documentation quality. All route through GitHub Issues (label: `docs`) for tracking. See [[audit/README]] for authority chain and [[rules/checklist]] for quality checks.

## Fix Workflow

**When:** Resolving a known inconsistency from [[tracking/inconsistencies]].

1. Pick an IC item by severity (Critical → Low)
2. Find or create a GitHub issue (labels: `docs` + priority label)
3. Fix the affected file(s)
4. Run [[rules/checklist]] against each fixed file
5. Move the IC item to the Resolved table with date and GitHub issue number
6. Update [[tracking/progress]] if the area is now fully clean
7. Close the GitHub issue (or check the box on the umbrella issue)

## Sweep Workflow

**When:** Proactively auditing an area marked "Not Started" in [[tracking/progress]].

1. Pick an area from [[tracking/progress]] with status "Not Started"
2. Read every file in that area
3. Run each file against [[rules/checklist]] (contradiction, structure, audience, completeness)
4. For each issue found: log as IC item in [[tracking/inconsistencies]] AND create/update a GitHub issue
5. Update area status in [[tracking/progress]] ("In Progress" or "Done")
6. Record any audit decisions in [[tracking/decisions]]

## Gate Workflow

**When:** Before finalizing any new or modified documentation.

1. Run [[rules/checklist]] against the changed file
2. Run the 5-step audit process from [[tracking/coverage-gaps]]
3. If issues found: log them in [[tracking/inconsistencies]] and create a GitHub issue before merging
