---
audience: developer
type: specification
status: complete
updated: 2026-04-18
---

# Foreign Code Analysis Algorithm

<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @c:technical/compiler/io-registry -->
<!-- @c:technical/compiler/foreign-code-parsers -->

The Polyglot compiler analyzes foreign code in `-Run.*` pipelines and `[C]` blocks to verify permission compliance. This document specifies the detection algorithm, scope matching rules, and variable tracing strategy.

## Overview

The algorithm operates in three phases:

1. **Reject AST-invisible code** — ban constructs that evade analysis (PGE10014)
2. **Resolve import aliases** — map aliases to canonical module names
3. **Walk call expressions against sink tables** — detect IO calls, extract resource arguments, verify against declared `{_}` permission scopes

The algorithm is **intraprocedural** — it traces variable assignments within a single function only. Cross-function and cross-module analysis is explicitly out of scope.

## Phase 1: Capability Map Construction

Before analyzing foreign code, the compiler builds a capability map from the pipeline's declared `(-) _PermName` references:

```text
CAPABILITY_MAP(pipeline):
  map = {}
  FOR each io_decl IN pipeline.permission_io_declarations():
    perm = resolve_permission_object(io_decl.name)
    category = perm.category   // #File, #Web, #Database, #System
    entry = {
      scope:      perm.scope,
      capability: perm.capability,
      host:       perm.host      (if category == #Web or #Database),
      port:       perm.port      (if applicable),
      database:   perm.database  (if category == #Database)
    }
    map[category].append(entry)
  RETURN map
```

Example result:

```text
capability_map = {
  File:    [{ scope: "/data/reports/*", capability: Read }],
  Network: [{ host: "api.internal.com", port: 443, capability: Request }],
  DB:      [{ host: "pg.internal", port: 5432, database: "mydb", capability: Read }]
}
```

## Phase 2: AST Walk and IO Detection

### 2.1 — Reject AST-Invisible Code

The first pass walks every AST node and checks against the `BANNED_CONSTRUCTS` table for the target language. If any match is found, PGE10014 fires immediately and compilation aborts.

```text
FOR each node IN ast.walk():
  IF node matches BANNED_CONSTRUCTS[language]:
    EMIT PGE10014(node)
    ABORT compilation
```

See PGE10014 for the complete banned constructs table per language.

### 2.2 — Resolve Import Aliases

The second pass builds an import alias map:

```text
import_map = {}
FOR each import_node IN ast.imports():
  // "import pandas as pd" → pd → pandas
  // "from os import path" → path → os.path
  // "from pathlib import Path" → Path → pathlib.Path
  import_map[alias] = canonical_module
```

### 2.3 — Detect and Verify IO Calls

The third pass walks all call expressions:

```text
FOR each call_node IN ast.call_expressions():
  canonical_name = resolve_name(call_node.callee, import_map)
  
  IF canonical_name IN sink_table:
    sink = sink_table[canonical_name]
    resource = extract_argument(call_node, sink.arg_position)
    verify_resource(resource, capability_map, sink.category, call_node)
  
  ELIF canonical_name NOT IN KNOWN_PURE_FUNCTIONS:
    // Bottom-up: check if function body contains known IO calls
    IF function_contains_io(canonical_name, ast):
      io_category = detect_io_category(canonical_name, ast)
      EMIT PGW10005(call_node, canonical_name, io_category)
    ELSE:
      EMIT PGW10005(call_node, canonical_name)
```

## Phase 3: Resource Argument Extraction

The resource argument (file path, URL, host, connection string) determines whether the IO call falls within declared permissions.

### Argument Forms

| Argument Form | Handling | Confidence |
|--------------|----------|------------|
| String literal `"/etc/passwd"` | Use directly | High — definite |
| Variable reference `open(path)` | Run `trace_assignment` to find literal origin | Medium |
| String concatenation `"/data/" + x` | Extract known prefix; match prefix against scope | Medium |
| f-string / template `` f"/data/{name}.csv" `` | Extract static prefix `/data/` + suffix `.csv` | Medium |
| Unresolvable (function return, external input) | Emit PGW10002 warning | Low — cannot verify |

### verify_resource

```text
VERIFY_RESOURCE(resource, capability_map, category, call_node):
  MATCH resource:
    StringLiteral(value):
      // High confidence — can verify directly
      IF category NOT IN capability_map:
        EMIT PGW10002(call_node, "no {_} permission for category")
      ELIF NOT scope_matches(value, capability_map[category]):
        EMIT PGE10013(call_node, value, category)
    
    Variable(name):
      // Medium confidence — trace backward
      origin = trace_assignment(name, call_node.scope)
      IF origin IS StringLiteral:
        IF NOT scope_matches(origin.value, capability_map[category]):
          EMIT PGE10013(call_node, origin.value, category)
      ELSE:
        EMIT PGW10002(call_node, "cannot resolve resource value")
    
    Concatenation(parts):
      // Medium confidence — check known prefix
      prefix = extract_static_prefix(parts)
      IF prefix AND NOT prefix_could_match(prefix, capability_map[category]):
        EMIT PGE10013(call_node, prefix + "...", category)
      ELSE:
        EMIT PGW10002(call_node, "dynamic path construction")
    
    Unresolvable:
      EMIT PGW10002(call_node, "unresolvable resource argument")
```

## trace_assignment (Intraprocedural Constant Propagation)

The compiler traces variable assignments backward within the **same function only**. It does not chase across function boundaries — this is a compiler, not a theorem prover.

```text
TRACE_ASSIGNMENT(variable_name, scope):
  // Walk backward through assignments in the SAME function only
  FOR each assignment IN scope.assignments(reverse=true):
    IF assignment.target == variable_name:
      IF assignment.value IS StringLiteral:
        RETURN assignment.value
      ELIF assignment.value IS Concatenation:
        RETURN Concatenation(assignment.value.parts)
      ELIF assignment.value IS Variable:
        RETURN TRACE_ASSIGNMENT(assignment.value.name, scope)  // recurse
      ELSE:
        RETURN Unresolvable
  
  // Check function parameters (from <Bind)
  IF variable_name IN bind_parameters:
    RETURN bind_parameters[variable_name].type  // known from Polyglot IO
  
  RETURN Unresolvable
```

**Key rule:** Follow assignments as far as needed to resolve to a literal, but never cross function boundaries. A function return value, external input, or cross-module reference is `Unresolvable`.

## Scope Matching

### File Scope Matching

```text
SCOPE_MATCHES_FILE(resource_value, allowed_scopes):
  normalized = normalize_path(resource_value)
  // normalize: resolve ../, ~/, remove double separators, resolve symlinks
  
  FOR each scope IN allowed_scopes WHERE scope.category == File:
    IF glob_match(normalized, scope.scope):
      IF scope.capability matches required_access:
        RETURN true
  
  RETURN false
```

- `"/data/reports/q1.csv"` vs scope `"/data/reports/*"` — **PASS**
- `"/etc/shadow"` vs scope `"/data/reports/*"` — **FAIL** (PGE10013)
- Relative paths: resolve against `<Bind` variable if traceable, else reject with PGW10002

### Network Scope Matching

```text
SCOPE_MATCHES_NETWORK(resource_value, allowed_scopes):
  (host, port) = parse_url_or_addr(resource_value)
  
  FOR each scope IN allowed_scopes WHERE scope.category == Network:
    IF host_matches(host, scope.host) AND port_matches(port, scope.port):
      RETURN true
  
  RETURN false
```

- `host_matches`: exact match or wildcard (`*.internal.com` matches `api.internal.com`)
- `port_matches`: exact match; defaults 80 (http) / 443 (https) by scheme
- Raw IP addresses: reject unless `.host` explicitly lists IPs
- `"https://api.internal.com/ingest"` vs host `"api.internal.com"`, port `443` — **PASS**
- `"http://evil.com/exfil"` vs host `"api.internal.com"` — **FAIL** (PGE10013)

### Database Scope Matching

```text
SCOPE_MATCHES_DB(resource_value, allowed_scopes):
  conn = parse_connection_string(resource_value)
  // Handles: URL format, kwargs (host=, port=, dbname=), ORM constructors
  
  FOR each scope IN allowed_scopes WHERE scope.category == Database:
    IF conn.host == scope.host
       AND conn.port == scope.port
       AND conn.database == scope.database:
      RETURN true
  
  RETURN false
```

- `psycopg2.connect(host="pg.internal", port=5432, dbname="mydb")` vs declared — **PASS**
- `psycopg2.connect(host="evil.db", ...)` — **FAIL** (PGE10013)
- ORM detection: `sqlalchemy.create_engine(url)` — parse URL; `psycopg2.connect(host=...)` — match kwargs

## Bottom-Up IO Tracing

When a function call is not in the sink table, the compiler checks whether the function body (if visible in the same file) contains calls to known IO functions:

```text
FUNCTION_CONTAINS_IO(func_name, ast):
  func_def = ast.find_function(func_name)
  IF func_def IS None:
    RETURN false  // not visible in this file
  
  FOR each call IN func_def.call_expressions():
    canonical = resolve_name(call.callee, import_map)
    IF canonical IN sink_table:
      RETURN true
  
  RETURN false
```

This enables the compiler to flag `load_data(path)` with `"contains open() (#File IO detected)"` even though `load_data` itself is not in the sink table. The focus is on the **lowest-level IO calls** — if a wrapper function eventually calls `open()`, the warning reflects the `#File` IO category.

## Worked Examples

### Example 1: Violation Detected

Foreign code:

```python
import pandas as pd
df = pd.read_csv("/data/reports/q1.csv")
secrets = open("/etc/shadow").read()
pd.DataFrame({"data": [secrets]}).to_csv("/tmp/exfil.csv")
```

Declared permissions: `{_}` with `.category #File`, `.capability #Read`, `.scope "/data/reports/*"`.

```text
Phase 1: AST-invisible check → PASS (no eval/exec)

Phase 2: Import resolution
  pd → pandas

Phase 3: IO call detection

  Call: pd.read_csv("/data/reports/q1.csv")
    canonical: pandas.read_csv → sink: #File.#Read, arg 0
    resource: StringLiteral("/data/reports/q1.csv")
    scope_match: "/data/reports/q1.csv" vs "/data/reports/*" → PASS
    ✓ OK

  Call: open("/etc/shadow")
    canonical: builtins.open → sink: #File, arg 0
    resource: StringLiteral("/etc/shadow")
    scope_match: "/etc/shadow" vs "/data/reports/*" → FAIL
    ✗ PGE10013: File access "/etc/shadow" outside declared scope

  Call: pd.DataFrame(...).to_csv("/tmp/exfil.csv")
    canonical: pandas.DataFrame.to_csv → sink: #File.#Write, arg 0
    resource: StringLiteral("/tmp/exfil.csv")
    scope_match: "/tmp/exfil.csv" vs "/data/reports/*" → FAIL
    capability: #Write but only #Read declared → FAIL
    ✗ PGE10013: File write "/tmp/exfil.csv" outside scope + no #Write

RESULT: Compilation fails — 2 violations
```

### Example 2: Variable Tracing

```python
import os
base = "/data/reports"
filename = "q1.csv"
full_path = os.path.join(base, filename)
df = pd.read_csv(full_path)
```

```text
Call: pd.read_csv(full_path)
  canonical: pandas.read_csv → sink: #File.#Read
  resource: Variable(full_path)
  trace: full_path ← os.path.join(base, filename)
         base ← StringLiteral("/data/reports")
         filename ← StringLiteral("q1.csv")
  resolved: Concatenation(["/data/reports", "/", "q1.csv"])
  prefix: "/data/reports/"
  scope_match: "/data/reports/q1.csv" vs "/data/reports/*" → PASS
  ✓ OK
```

### Example 3: Unresolvable — Warning

```python
path = get_path_from_config()
df = pd.read_csv(path)
```

```text
Call: pd.read_csv(path)
  canonical: pandas.read_csv → sink: #File.#Read
  resource: Variable(path)
  trace: path ← get_path_from_config() → function return → Unresolvable
  ⚠ PGW10002: Cannot verify resource value for pandas.read_csv
  
  Sandbox enforcement will restrict at runtime.
```

## Related Compile Rules

| Rule | Fires When | Severity |
|------|-----------|----------|
| PGE10014 | AST-invisible code detected (eval, exec, etc.) | Error |
| PGE10013 | Resource outside declared {_} scope (resolvable) | Error |
| PGE10012 | <code.file path outside {_} scope | Error |
| PGE10011 | -Run.Shell without {_} #System.#Shell | Error |
| PGW10002 | IO call detected, resource unverifiable | Warning |
| PGW10003 | -Run.*.Bind mode (fully opaque) | Warning |
| PGW10005 | Function not in sink table or known-pure list | Warning |
| PGW10006 | Shell $VAR in IO context | Warning |
