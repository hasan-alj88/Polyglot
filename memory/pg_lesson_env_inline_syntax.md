---
name: pg_lesson_env_inline_syntax
description: Environment wiring uses inline ;EnvName on [W] and [-] lines, not verbose IO
type: feedback
---

**Rule:** Environment wiring uses inline `;EnvName` syntax appended to the pipeline reference:
- Wrapper: `[W] -W.Env;PythonUV` (not `[W] -W.Env` + `(-) <env#; << ;PythonUV`)
- Run call: `[-] -Run.Python.Script;PythonUV` (not `(-) <env#PyEnv << $pyenv`)
**Why:** I used the verbose multi-line IO form from the spec examples, but the corrected form uses concise inline `;EnvName` appended directly to the pipeline name. No separate `<env` IO line needed.
**How to apply:** When wiring environments to `-W.Env` or `-Run.*` calls, append `;EnvName` inline to the pipeline reference. Do not use separate `(-) <env` IO lines.
**Spec file:** docs/user/aj3lib/pipelines/W/Env.md, docs/user/aj3lib/pipelines/Run/Script.md
**Spec updated:** yes — W/Env.md and Run/Script.md updated with inline syntax and corrected examples
