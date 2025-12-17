---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/packages.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Packages

Polyglot's package system enables code reuse and distribution through a three-tier registry: Local, Community, and Company.

## Package Basics

A **package** is a collection of pipelines, wrappers, enumerations, and macros that can be imported and used in other Polyglot programs.

### Package Structure

```
my_package/
├── package.yaml        # Package metadata
├── pipelines/          # Pipeline definitions
│   ├── process.pg
│   └── transform.pg
├── wrappers/           # Foreign function wrappers
│   ├── python/
│   │   └── utils.py
│   └── rust/
│       └── lib.rs
├── enums/              # Enumeration definitions
│   └── status.pg
└── README.md           # Documentation
```

### Package Metadata

`package.yaml`:

```yaml
name: my_package
version: 1.2.3
author: "Your Name <your.email@example.com>"
description: "Data processing utilities for Polyglot"
license: MIT

dependencies:
  data_utils: "^2.0.0"
  validators: "~1.5"

exports:
  pipelines:
    - process_data
    - transform_data
  wrappers:
    - py\clean_data
    - rs\fast_processor
  enumerations:
    - Status
```

## Three-Tier Registry System

Polyglot uses a hierarchical registry system:

```
┌──────────────────────────────────┐
│  Company Registry (Private      │  ← Company-specific packages
│  registry.company.com            │
└──────────────────────────────────┘
         ↓ Fallback
┌──────────────────────────────────┐
│  Community Registry (Public     │  ← Public shared packages
│  registry.polyglot-lang.org      │
└──────────────────────────────────┘
         ↓ Fallback
┌──────────────────────────────────┐
│  Local Registry (Machine        │  ← Local development packages
│  ~/.polyglot/packages            │
└──────────────────────────────────┘
```

### Registry Resolution Order

When importing `@package/module`:

1. **Company Registry:** Check company registry first (if configured
2. **Community Registry:** Check public registry
3. **Local Registry:** Check local machine

### Configuring Registries

`~/.polyglot/config.yaml`:

```yaml
registries:
  # Company registry (optional
  company:
    url: "https://registry.company.com"
    auth_token: "${POLYGLOT_COMPANY_TOKEN"

  # Community registry (default
  community:
    url: "https://registry.polyglot-lang.org"

  # Local registry
  local:
    path: "~/.polyglot/packages"
```

## Creating Packages

### Step 1: Initialize Package

```bash
# Create package structure
polyglot package init my_package

# Or with template
polyglot package init my_package --template data-processing
```

This creates:

```
my_package/
├── package.yaml
├── pipelines/
├── wrappers/
├── enums/
├── tests/
└── README.md
```

### Step 2: Write Package Code

`pipelines/clean_data.pg`:

```polyglot
[|Pipeline] .clean_data
[i] .raw_data: py\DataFrame
[o] .clean_data: py\DataFrame

// Remove nulls
.no_nulls << py\pandas.DataFrame.dropna(.raw_data

// Remove duplicates
.clean_data << py\pandas.DataFrame.drop_duplicates(.no_nulls
```

`wrappers/python/validator.py`:

```python
def validate_schema(df, schema:
    """Validate DataFrame against schema."""
    for column, dtype in schema.items(:
        if column not in df.columns:
            raise ValueError(f"Missing column: {column"
        if df[column].dtype != dtype:
            raise TypeError(f"Column {column has wrong type"
    return True
```

Register wrapper in `wrappers/validator.pg`:

```polyglot
[r] py\validate_schema
[i] .dataframe: py\DataFrame
[i] .schema: py\dict
[o] .is_valid: Boolean
[<] <module: "validator"
[<] <function: "validate_schema"
```

### Step 3: Update Package Metadata

`package.yaml`:

```yaml
name: data_cleaning
version: 1.0.0
author: "Data Team <data@company.com>"
description: "Data cleaning and validation utilities"
license: MIT

exports:
  pipelines:
    - clean_data
  wrappers:
    - py\validate_schema
```

### Step 4: Test Package

```bash
# Run package tests
polyglot package test my_package

# Validate package structure
polyglot package validate my_package
```

### Step 5: Publish Package

**To Local Registry:**

```bash
polyglot package publish my_package --local
```

**To Community Registry:**

```bash
# Login to community registry
polyglot login

# Publish package
polyglot package publish my_package
```

**To Company Registry:**

```bash
# Login to company registry
polyglot login --registry company

# Publish package
polyglot package publish my_package --registry company
```

## Importing Packages

### Basic Import

```polyglot
// Import entire package
@data_cleaning

// Use exported pipeline
.cleaned << @data_cleaning/clean_data(.raw_data

// Use exported wrapper
.valid << @data_cleaning/py\validate_schema(.df, .schema
```

### Selective Import

```polyglot
// Import specific items
@data_cleaning: .clean_data, py\validate_schema

// Use imported items directly
.cleaned << .clean_data(.raw_data
.valid << py\validate_schema(.df, .schema
```

### Versioned Import

```polyglot
// Specific version
@data_cleaning@1.2.3

// Version range
@data_cleaning@^1.0.0  // 1.x.x (semver
@data_cleaning@~1.2.0  // 1.2.x

// Latest version (default
@data_cleaning
```

### Aliased Import

```polyglot
// Import with alias
@data_cleaning as @dc

// Use alias
.cleaned << @dc/clean_data(.raw_data
```

## Package Dependencies

### Declaring Dependencies

`package.yaml`:

```yaml
dependencies:
  # Exact version
  data_utils: "1.2.3"

  # Version range (semver
  validators: "^2.0.0"      # 2.x.x
  transformers: "~1.5.0"    # 1.5.x

  # From specific registry
  company_auth:
    version: "^3.0.0"
    registry: company

  # From git (development
  experimental:
    git: "https://github.com/user/experimental.git"
    branch: "main"
```

### Dependency Resolution

```bash
# Install all dependencies
polyglot package install

# Update dependencies
polyglot package update

# Show dependency tree
polyglot package tree
```

Output:

```
my_package@1.0.0
├── data_utils@1.2.5
│   └── validators@2.1.0
├── transformers@1.5.3
└── company_auth@3.2.1
    └── jwt_lib@4.0.0
```

## Versioning

Polyglot follows [Semantic Versioning (SemVer](https://semver.org/:

- **MAJOR.MINOR.PATCH** (e.g., `1.2.3`
- **MAJOR:** Breaking changes
- **MINOR:** New features (backward compatible
- **PATCH:** Bug fixes (backward compatible

### Version Constraints

| Constraint | Matches | Example |
|------------|---------|---------|
| `1.2.3` | Exact version | Only `1.2.3` |
| `^1.2.3` | Compatible minor | `1.2.3` to `1.x.x` |
| `~1.2.3` | Compatible patch | `1.2.3` to `1.2.x` |
| `>1.2.3` | Greater than | `1.2.4`, `1.3.0`, `2.0.0`, etc. |
| `>=1.2.3` | Greater or equal | `1.2.3`, `1.2.4`, etc. |
| `<2.0.0` | Less than | `1.x.x` |
| `*` | Any version | Latest |

### Publishing New Versions

```bash
# Bump patch version (1.2.3 → 1.2.4
polyglot package version patch

# Bump minor version (1.2.3 → 1.3.0
polyglot package version minor

# Bump major version (1.2.3 → 2.0.0
polyglot package version major

# Publish new version
polyglot package publish
```

## Package CLI Commands

### Package Management

```bash
# Initialize new package
polyglot package init <name>

# Validate package structure
polyglot package validate <path>

# Test package
polyglot package test <path>

# Build package (create .tar.gz
polyglot package build <path>

# Publish package
polyglot package publish <path> [--registry <name>]
```

### Dependency Management

```bash
# Install dependencies
polyglot package install

# Add dependency
polyglot package add <name>@<version>

# Remove dependency
polyglot package remove <name>

# Update dependencies
polyglot package update

# Show dependency tree
polyglot package tree

# Show outdated dependencies
polyglot package outdated
```

### Registry Management

```bash
# Login to registry
polyglot login [--registry <name>]

# Logout from registry
polyglot logout [--registry <name>]

# Search packages
polyglot search <query>

# Show package info
polyglot package info <name>

# List installed packages
polyglot package list
```

## Example: Complete Package

### Data Processing Package

**Structure:**

```
data_processing/
├── package.yaml
├── pipelines/
│   ├── extract.pg
│   ├── transform.pg
│   └── load.pg
├── wrappers/
│   ├── python/
│   │   └── extractors.py
│   └── rust/
│       └── transformers.rs
└── tests/
    └── test_pipelines.pg
```

**package.yaml:**

```yaml
name: data_processing
version: 2.1.0
author: "Data Team <data@company.com>"
description: "ETL pipelines for data processing"
license: Apache-2.0

dependencies:
  data_validators: "^1.0.0"

exports:
  pipelines:
    - extract_from_api
    - transform_data
    - load_to_warehouse
  wrappers:
    - py\fetch_api
    - rs\transform_records
```

**pipelines/extract.pg:**

```polyglot
[|Pipeline] .extract_from_api
[i] .api_url: String
[i] .auth_token: String
[o] .data: py\DataFrame

// Fetch data
.response << py\fetch_api(.api_url, .auth_token

// Parse JSON
.json_data << .response >> json(

// Convert to DataFrame
.data << py\pandas.DataFrame(.json_data
```

**wrappers/python/extractors.py:**

```python
import requests

def fetch_api(url, token:
    """Fetch data from API with authentication."""
    headers = {"Authorization": f"Bearer {token"
    response = requests.get(url, headers=headers
    response.raise_for_status(
    return response
```

**Usage in another project:**

```polyglot
// Import the package
@data_processing@^2.0.0

[|Pipeline] .daily_etl
[t] .trigger: #T.Daily(.hour: 2, .minute: 0
[i] .api_url: String
[i] .auth_token: String
[i] .warehouse_conn: String

// Use package pipeline
.extracted << @data_processing/extract_from_api(.api_url, .auth_token
.transformed << @data_processing/transform_data(.extracted
.loaded << @data_processing/load_to_warehouse(.transformed, .warehouse_conn
```

## Best Practices

### Package Design

1. **Single Responsibility:** Each package should have a clear, focused purpose
2. **Stable API:** Minimize breaking changes between versions
3. **Document Everything:** Comprehensive README and examples
4. **Test Thoroughly:** Include tests for all exported items
5. **Version Carefully:** Follow SemVer strictly

### Dependency Management

1. **Pin Versions:** Use exact versions for production deployments
2. **Minimize Dependencies:** Only depend on what you actually use
3. **Keep Updated:** Regularly update to patch security issues
4. **Lock Files:** Commit `package-lock.yaml` for reproducible builds

### Publishing

1. **Changelog:** Maintain CHANGELOG.md with version history
2. **README:** Clear documentation with usage examples
3. **License:** Always include LICENSE file
4. **Tests:** Run full test suite before publishing
5. **Semantic Versioning:** Use major.minor.patch correctly

## Next Steps

- **Create Your First Package:** Follow [Package Tutorial](examples/creating-packages.md
- **Explore Community Packages:** Browse [Package Registry](https://registry.polyglot-lang.org
- **Learn Advanced Packaging:** Read [Package Development Guide](advanced/package-development.md
- **Contribute:** Share your packages with the community!
