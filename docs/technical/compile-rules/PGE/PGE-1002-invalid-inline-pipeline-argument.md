---
rule: "10.2"
code: PGE-1002
name: Invalid Inline Pipeline Argument
severity: error
---

### Rule 10.2 — Invalid Inline Pipeline Argument
`PGE-1002`

**Statement:** When an inline pipeline call `=Foo"arg"` targets a pipeline that declares `.inlineFormat#array:RawString` metadata, the inline string argument must match at least one of the declared regex patterns. If no pattern matches, the call is a compile error.
**Rationale:** Inline pipeline arguments have format requirements specific to the target pipeline (e.g., `=T.Daily` expects a time string, `=T.Webhook` expects a URL path). The `.inlineFormat` metadata makes these requirements machine-checkable. Catching format errors at compile time prevents runtime failures from malformed arguments. The `.inlineExamples` metadata provides human-readable guidance in the error message.
**Detection:** The compiler reads `.inlineFormat` from the target pipeline's `[%]` metadata. If present, it tests the inline string against each regex pattern. If none match, the call is rejected. The error message includes `.inlineExamples` values if declared. If `.inlineFormat` is absent, no validation occurs (see PGW-1002).

**See also:** PGE-1001 (undefined metadata field access), PGE-806 (inline pipeline in chain context), PGW-1002 (missing inline format metadata)

---

#### Metadata Declaration

Pipelines accepting inline arguments declare format expectations in `[%]`:

| Field | Type | Purpose |
|-------|------|---------|
| `.inlineFormat` | `#array:RawString` | Regex patterns — argument must match at least one |
| `.inlineExamples` | `#array:RawString` | Valid example strings shown in error messages |

Example stdlib trigger declaration:
```polyglot
{=} =T.Daily
   [%] .description << "Triggers daily at specified time"
   [%] .inlineFormat#array:RawString << ["^[0-9]{2}:[0-9]{2}$", "^[0-9]{2}:[0-9]{2}:[0-9]{2}$"]
   [%] .inlineExamples#array:RawString << ["14:30", "08:00:00"]
```

---

**VALID:**
```polyglot
[ ] ✓ inline argument matches declared format
{=} =DailyReport
   [t] =T.Daily"14:30"                 [ ] ✓ matches ^[0-9]{2}:[0-9]{2}$
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✓ inline argument matches second pattern
{=} =PreciseReport
   [t] =T.Daily"08:00:00"              [ ] ✓ matches ^[0-9]{2}:[0-9]{2}:[0-9]{2}$
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✓ pipeline with no .inlineFormat — no validation (see PGW-1002)
{=} =FlexibleCall
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =CustomPipeline"anything goes"
      [=] >result >> >out
```

**INVALID:**
```polyglot
[ ] ✗ PGE-1002 — inline argument does not match any format pattern
{=} =BadTrigger
   [t] =T.Daily"not-a-time"            [ ] ✗ PGE-1002 — no pattern matches
   [Q] =Q.Default                      [ ]   Valid examples: "14:30", "08:00:00"
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE-1002 — whitespace-only path for webhook
{=} =BadWebhook
   [t] =T.Webhook" "                   [ ] ✗ PGE-1002 — no pattern matches
   [Q] =Q.Default                      [ ]   Valid examples: "/api/hook", "/ingest/v2"
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE-1002 — wrong format for folder trigger
{=} =BadFolder
   [t] =T.Folder.NewFiles"not a path"  [ ] ✗ PGE-1002 — no pattern matches
   [Q] =Q.Default                      [ ]   Valid examples: "/data/inbox", "./uploads"
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

**Open point:** None.
