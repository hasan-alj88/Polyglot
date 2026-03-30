---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 3. Identifiers (S3)

### EC-3.1: Package address — all components present

<!-- @packages -->
<!-- @identifiers -->
**EBNF:** `package_address ::= registry_type flex_sep registry_id fixed_sep package_name { fixed_sep sub_package } [ flex_sep version ]`

**What it tests:** Full address with subpackage and 4-segment version. See [[packages]], [[identifiers]].

```polyglot
{@} @Local:999.MyPackage.Sub:v1.2.3.2
```

### EC-3.2: Community registry type

**EBNF:** `registry_type ::= "Local" | "Community"`

**What it tests:** `Community` registry with username-style ID.

```polyglot
[@] @Slack << @Community:tools.SlackAdmin:v1.3.0
```

### EC-3.3: Package address — minimal (no subpackage, no version)

**What it tests:** Whether version and subpackage are truly optional.

```polyglot
{@} @Local:001.Minimal
```

### EC-3.4: Cross-package enum reference

<!-- @types:Enum Fields -->
**EBNF:** `cross_pkg_enum ::= '@' name '#' dotted_name`

**What it tests:** Referencing an enum value from an imported package: `@alias#DataName.EnumField`. See [[syntax/types/structs#Enum Fields vs Value Fields]].

```polyglot
[?] $status =? @HR#EmployeeStatus.Active
```

### EC-3.5: Cross-package pipeline reference

**EBNF:** `cross_pkg_pipeline ::= '@' name pipeline_id`

**What it tests:** Calling an imported pipeline. See [[packages#Usage]].

```polyglot
[r] @Mail=Mailbox.Provision
[=] <email << $email
```

### EC-3.6: Flexible-field variable paths

<!-- @identifiers:Serialized Identifiers -->
**EBNF:** `field_path ::= name { field_separator name }` with `flex_sep ::= ':'`

**What it tests:** Variables with `:` flexible field separators. See [[identifiers#Serialized Identifiers]].

```polyglot
[r] $config:timeout:value#int << 30
[r] $user:name#string << "Alice"
```

### EC-3.7: Sibling homogeneity violation (INVALID)

<!-- @identifiers:Serialization Rules -->
**EBNF:** Semantic rule — all siblings must use same separator. See [[identifiers#Serialization Rules]].

**What it tests:** Mixing `.` and `:` at same level is rejected.

```polyglot
[ ] INVALID — mixed separators at same sibling level
[r] $point.x << 10
[r] $point:y << 20
```
