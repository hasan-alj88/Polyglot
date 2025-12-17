# Polyglot CLI Documentation

**Command-line interface tools and development workflow**

---

## Overview

The Polyglot CLI provides tools for compiling, registering, activating, and testing Polyglot code. This directory documents the complete development workflow using command-line tools.

---

## CLI Workflow

### 📋 [00 - Workflow Overview](./00-workflow.md)
High-level overview of the Polyglot development workflow.

**Topics:**
- Development lifecycle
- Tool chain overview
- Workflow stages
- Best practices

**Covers:**
1. Write code
2. Compile
3. Register
4. Activate
5. Test
6. Deploy

---

### ⚙️ [01 - Compile](./01-compile.md)
Compiling Polyglot source code into executable artifacts.

**Topics:**
- `polyglot compile` command
- Compilation options and flags
- Output artifacts
- Error reporting
- Optimization levels
- Debug vs release builds

**Command Reference:**
```bash
polyglot compile [options] <source-file>
```

**Common Options:**
- `--output <dir>` - Output directory
- `--optimize` - Enable optimizations
- `--debug` - Include debug symbols
- `--check` - Syntax check only (no code generation)

---

### 📦 [02 - Register](./02-register.md)
Registering pipelines and components with the Polyglot runtime.

**Topics:**
- `polyglot register` command
- Registration process
- Component discovery
- Dependency resolution
- Version management
- Registry structure

**Command Reference:**
```bash
polyglot register [options] <artifact>
```

**Common Options:**
- `--force` - Force re-registration
- `--dry-run` - Simulate registration
- `--verbose` - Detailed output

---

### 🚀 [03 - Activate](./03-activate.md)
Activating registered pipelines for execution.

**Topics:**
- `polyglot activate` command
- Pipeline activation
- Trigger setup
- Queue configuration
- Monitoring setup
- Deactivation process

**Command Reference:**
```bash
polyglot activate [options] <pipeline-name>
```

**Common Options:**
- `--trigger <type>` - Override trigger type
- `--queue <config>` - Queue configuration
- `--env <file>` - Environment variables

---

### ✅ [04 - Test](./04-test.md)
Running tests and validating Polyglot code.

**Topics:**
- `polyglot test` command
- Test discovery
- Test execution
- Coverage reporting
- Integration tests
- Mock and stub utilities

**Command Reference:**
```bash
polyglot test [options] [test-pattern]
```

**Common Options:**
- `--verbose` - Detailed test output
- `--coverage` - Generate coverage report
- `--watch` - Watch mode for continuous testing
- `--parallel` - Run tests in parallel

---

## Complete Workflow Example

### 1. Create a Pipeline

```polyglot
{|} |HelloWorld
[|] <name :pg.string
[|] >greeting :pg.string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $message :pg.string << |U.String.Concat"{\"Hello, \", $name, \"!\"}"
   [|] >greeting >> $message
{x}
```

### 2. Compile

```bash
$ polyglot compile hello-world.pg
Compiling hello-world.pg...
Generated: build/hello-world.pgc
Success!
```

### 3. Register

```bash
$ polyglot register build/hello-world.pgc
Registering pipelines...
- |HelloWorld registered
Success!
```

### 4. Activate

```bash
$ polyglot activate HelloWorld
Activating |HelloWorld...
Trigger: Call
Status: Active
Success!
```

### 5. Test

```bash
$ polyglot test HelloWorld
Running tests for |HelloWorld...
✓ Test: Basic greeting
✓ Test: Special characters
2 passed, 0 failed
```

---

## Global Options

These options work with all CLI commands:

- `--help` - Show command help
- `--version` - Show version information
- `--config <file>` - Use custom config file
- `--verbose` - Enable verbose output
- `--quiet` - Suppress non-error output
- `--color [always|never|auto]` - Control color output

---

## Configuration Files

### polyglot.toml

Project configuration file:

```toml
[project]
name = "my-project"
version = "1.0.0"

[build]
output_dir = "build"
optimize = true

[test]
parallel = true
coverage = true

[registry]
url = "https://registry.polyglot.dev"
```

### .polyglotignore

Exclude files from compilation:

```
# Ignore test files
*_test.pg

# Ignore examples
examples/

# Ignore build artifacts
build/
dist/
```

---

## Environment Variables

- `POLYGLOT_HOME` - Installation directory
- `POLYGLOT_CONFIG` - Config file path
- `POLYGLOT_REGISTRY` - Registry URL
- `POLYGLOT_DEBUG` - Enable debug mode
- `POLYGLOT_LOG_LEVEL` - Logging level (error|warn|info|debug|trace)

---

## Exit Codes

Standard exit codes used by CLI tools:

- `0` - Success
- `1` - General error
- `2` - Command-line usage error
- `3` - Compilation error
- `4` - Registration error
- `5` - Activation error
- `6` - Test failure

---

## Troubleshooting

### Common Issues

**Compilation Fails:**
- Check syntax errors in source file
- Verify all dependencies are available
- Run with `--verbose` for detailed errors

**Registration Fails:**
- Ensure artifact is valid compiled output
- Check registry is accessible
- Verify pipeline names are unique

**Activation Fails:**
- Confirm pipeline is registered
- Check trigger configuration
- Verify queue system is running

**Tests Fail:**
- Run with `--verbose` for details
- Check test data and mocks
- Verify environment setup

---

## Related Documentation

- [Language Documentation](../language/) - Language features
- [Examples](../examples/) - Example projects
- [Architecture](../architecture/) - Runtime architecture
- [Guides](../guides/) - Development guides

---

## IDE Integration

CLI tools can be integrated with IDEs:

- **VS Code** - Extension available
- **IntelliJ** - Plugin support
- **Vim/Neovim** - LSP integration
- **Emacs** - Mode available

See IDE-specific documentation for setup instructions.

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Documentation Team
