---
name: pg_lesson_run_script_output_bind
description: Script execution output flows through >Bind port, not >output
type: feedback
---

**Rule:** When using `-Run.*.Script`, the script's output flows through the `>Bind` port, not a separate `>output` port. Wire as `(-) >Bind#Code:Python.Output >> >output`.
**Why:** I used `(-) >output#Code:Python.Output >> >output` referencing a separate `>output` port. The corrected form uses `>Bind` as the output source.
**How to apply:** For `-Run.*.Script` calls, use `>Bind` as the output port name with the appropriate type annotation.
**Spec file:** docs/user/pglib/pipelines/Run/Script.md
**Spec updated:** no
