---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/packages/README.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Package System

**Creating, publishing, and using Polyglot packages**

---

## Overview

This directory contains documentation for Polyglot's package system, which allows you to share and reuse code through packages. Learn how to create packages, publish them to registries, and import them into your projects.

---

## Package Documentation

### 📦 [00 - Package Overview](./00-overview.md)
Introduction to the Polyglot package system.

**Topics:**
- What are packages?
- Package structure
- Package manifest (`package.toml`)
- Versioning and dependencies
- Package lifecycle

**Covers:**
- Package anatomy
- Naming conventions
- Version resolution
- Dependency graphs

---

### 🌐 [01 - Package Registries](./01-registries.md)
Working with package registries.

**Topics:**
- Public registry (registry.polyglot.dev)
- Private registries
- Registry authentication
- Mirror configuration
- Registry API

**Operations:**
- Searching packages
- Publishing packages
- Downloading packages
- Managing versions

---

### 🛠 [02 - Creating Packages](./02-creating-packages.md)
Step-by-step guide to creating your own packages.

**Topics:**
- Package initialization
- Directory structure
- Writing `package.toml`
- Documenting your package
- Testing packages
- Building packages

**Example Structure:**
```
my-package/
├── package.toml
├── README.md
├── src/
│   ├── pipelines/
│   ├── enums/
│   └── utilities/
├── tests/
└── docs/
```

---

### 📥 [03 - Importing Packages](./03-importing-packages.md)
Using packages in your Polyglot projects.

**Topics:**
- Adding dependencies
- Import syntax (`[<]` marker)
- Version constraints
- Dependency resolution
- Lock files
- Updating dependencies

**Import Examples:**
```polyglot
[<] "http-utils" as HTTP
[<] "json-parser@1.2.0"
[<] "./local-package"
```

---

## Package Manifest

### package.toml

Example package manifest:

```toml
[package]
name = "my-package"
version = "1.0.0"
authors = ["Your Name <email@example.com>"]
description = "A useful package"
license = "MIT"
repository = "https://github.com/user/my-package"

[dependencies]
http-utils = "^1.0"
json-parser = "~1.2.0"

[dev-dependencies]
test-framework = "^0.5"

[build]
include = ["src/**/*.pg", "README.md"]
exclude = ["tests/**", "examples/**"]
```

---

## Version Constraints

### Constraint Syntax

- `1.2.3` - Exact version
- `^1.2.3` - Compatible with 1.2.3 (>=1.2.3, <2.0.0)
- `~1.2.3` - Approximately 1.2.3 (>=1.2.3, <1.3.0)
- `>=1.2.3` - Greater or equal
- `<2.0.0` - Less than
- `*` - Any version (not recommended)

### Version Resolution

Polyglot uses a deterministic dependency resolver that:
1. Satisfies all constraints
2. Prefers newer versions
3. Minimizes dependency tree depth
4. Creates `package.lock` for reproducible builds

---

## Package Types

### Library Packages

Reusable code components:
- Pipelines
- Enums
- Utilities
- Wrappers

### Application Packages

Executable applications:
- Entry point pipelines
- Configuration
- Assets
- Deployment specs

### Meta Packages

Collections of related packages:
- No source code
- Just dependencies
- Convenient bundles

---

## Publishing Workflow

### 1. Prepare Package

```bash
# Run tests
$ polyglot test

# Build package
$ polyglot build

# Validate package
$ polyglot package validate
```

### 2. Authenticate

```bash
$ polyglot registry login
Enter username: your-username
Enter password: ********
Logged in successfully!
```

### 3. Publish

```bash
$ polyglot publish
Publishing my-package v1.0.0...
Uploading package (125 KB)...
Success! Package published to registry.
```

### 4. Verify

```bash
$ polyglot search my-package
Found 1 package:
- my-package (1.0.0) - A useful package
```

---

## Best Practices

### Package Design

1. **Single responsibility** - One clear purpose per package
2. **Semantic versioning** - Follow SemVer strictly
3. **Comprehensive docs** - README and inline documentation
4. **Thorough testing** - High test coverage
5. **Minimal dependencies** - Only necessary deps

### Versioning

1. **Breaking changes** → Major version bump
2. **New features** → Minor version bump
3. **Bug fixes** → Patch version bump
4. **Document changes** - Keep a CHANGELOG.md

### Security

1. **Audit dependencies** - Check for vulnerabilities
2. **Pin versions** - Use lock files
3. **Verify signatures** - Use registry signatures
4. **Review updates** - Don't auto-update blindly

---

## Common Workflows

### Installing a Package

```bash
$ polyglot add http-utils
Added http-utils@^1.0.0 to dependencies
```

### Updating Packages

```bash
$ polyglot update
Updating dependencies...
- http-utils: 1.0.0 → 1.1.0
Updated 1 package
```

### Removing a Package

```bash
$ polyglot remove http-utils
Removed http-utils from dependencies
```

---

## Related Documentation

- [Language Documentation](../language/) - Using imported code
- [CLI Documentation](../cli/) - Package commands
- [Examples](../examples/) - Package usage examples

---

## Troubleshooting

**Dependency conflict:**
- Check version constraints
- Update dependencies
- Use `polyglot tree` to visualize

**Package not found:**
- Verify registry configuration
- Check package name spelling
- Ensure registry is accessible

**Build fails:**
- Check for breaking changes
- Review dependency updates
- Clear cache: `polyglot clean`

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Documentation Team
