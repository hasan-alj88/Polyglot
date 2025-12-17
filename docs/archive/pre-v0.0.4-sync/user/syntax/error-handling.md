---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/error-handling.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Error Handling

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Error Type Definition

Errors require **three mandatory fields**: `.message`, `.code`, `.trace`

```polyglot
[!] NetworkTimeout
[<] <message:pg.string << "Network operation timed out"
[<] <code:pg.int << 1001
[<] <trace:pg.string << ""       // Populated at runtime
[<] <retry_after:pg.int << 5     // Custom field (optional
[X]
```

---

## Error Catching

Use `[!]` marker within execution blocks:

```polyglot
[r] |MightFail
[<] .url << .api_endpoint
[>] .data >> .result
[~]
[~][!] !NetworkTimeout             // Catch specific error
[~][<] <error: !NetworkTimeout << .timeout_err
[~][r] |HandleTimeout
[~]
[~][!] *                           // Catch all other errors
[~][o] !UnknownError
[~]
```

---

## Error State Propagation

Errors propagate through variable states automatically:

```
Pending → Faulted
```

Access error via reserved field:

```polyglot
[?] .result.pgvar.state =? #PgVar.States.Faulted
[~][<] <errors:pg.array{! << .result.pgvar.errors
[~]
```

---

## Reserved Error Fields

Every error automatically has:

- `.message:pg.string` - Error message
- `.code:pg.int` - Error code
- `.trace:pg.string` - Stack trace

---

## Error Handling Patterns

### Retry Pattern

```polyglot
[r] .retries:pg.int << 0

[r] |AttemptOperation
[>] .result >> .data
[~]
[~][!] !NetworkTimeout
[~][<] .retries << .retries + 1
[~][?] .retries <? 3
[~][~][r] |AttemptOperation       // Retry
[~][~]
[~]
```

### Fallback Pattern

```polyglot
[r] |PrimaryService
[>] .data >> .result
[~]
[~][!] !ServiceUnavailable
[~][r] |FallbackService           // Use backup
[~][>] .data >> .result
[~]
```

---

**Next:** [Comments →](comments.md
