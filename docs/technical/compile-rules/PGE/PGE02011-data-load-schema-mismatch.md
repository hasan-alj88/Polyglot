---
rule: "2.11"
code: PGE02011
name: Data Load Schema Mismatch
severity: error
---

### Rule 2.11 — Data Load Schema Mismatch
`PGE02011`

**Statement:** The `[#]` data load marker requires its source to be a pipeline call or data reference whose output schema tree matches the target variable's type. Plain literals are not valid `[#]` sources — `[#]` is for deserialization of structured data, not literal assignment. The compiler compares the source and target schema trees; a mismatch is a compile error (PGE04001 type mismatch applies to the schema comparison).
**Rationale:** `[#]` exists to load serialized data (JSON, YAML, TOML, config files) through deserialization pipelines. Using it with a plain literal like `42` or `"Bob"` bypasses the deserialization intent and is misleading — a regular `[r]` assignment should be used instead. Schema tree matching ensures the deserialized data structure is compatible with the target type.
**Detection:** The compiler checks that the RHS of a `[#]` assignment is a pipeline call (`=Pipeline.Name`) or data reference (`#DataType`), not a literal value. It then compares the output schema tree of the source against the target type's schema tree.

**VALID:**
```polyglot
[ ] ✓ data load from deserialization pipeline — schemas match
[#] $config#Config << =File.Serial.Read.JSON"/config.json"

[ ] ✓ data load from schema pipeline
[#] $schema << =#.JSON.Parse
   [=] <raw << $jsonString
   [=] >parsed >> $config
```

**INVALID:**
```polyglot
[ ] ✗ PGE02011 — plain literal is not a valid [#] source
[#] $x#int << 42

[ ] ✗ PGE02011 — string literal is not a valid [#] source
[#] $name#string << "Bob"
```

**Diagnostic:** "Data load `[#]` requires a pipeline call or data reference, not a literal — use `[r]` for literal assignment"
