# Package Management and Namespaces

[← Back to README](../README.md)

## Table of Contents
- [Overview](#overview)
- [Namespace Declaration](#namespace-declaration)
- [Import System](#import-system)
- [Naming Conventions](#naming-conventions)
- [File Continuation Pattern](#file-continuation-pattern)
- [Standard Library](#standard-library-implicit-import)
- [Collision Resolution](#namespace-collision-resolution)
- [Best Practices](#best-practices)

## Overview

Polyglot uses a registry-based package management system with descriptive naming to avoid namespace collisions. The system is inspired by Java/Maven's hierarchical structure.

**Key Features:**
- Hierarchical namespace paths using `>`
- Import aliasing for convenience
- File continuation for large libraries
- Automatic standard library imports
- Compile-time collision detection

## Namespace Declaration

Every Polyglot file begins with a namespace declaration:

```polyglot
[@] Organization>ProjectName>ModuleName
```

**Syntax:**
- `[@]` declares the namespace for the current file
- Use `>` to separate hierarchy levels
- Names are case-sensitive
- Only alphanumeric characters and dots are allowed

**Example:**
```polyglot
[@] io.github.myuser>DataProcessing>AdvancedAnalytics
```

This namespace becomes the identifier other files use to import from this file.

## Import System

### Basic Import with Aliasing

```polyglot
[@] com.example>MyProject>Module
[D] @ShortAlias = @io.github.bigcorp>EnterpriseAnalytics
[D] @Viz = @io.gitlab.datavis>Plotting>InteractiveCharts
[D] @DB = @io.github.dbtools>PostgreSQL>ConnectionPool
[X]  \\ End import block

\\ Code follows after [X]
[|] MyPipeline
[t] |T.Call
[r] @ShortAlias|ProcessData  \\ Use imported pipeline
[x]
```

**Elements:**
- `[D]` - Import statement (Define)
- `@AliasName = @Full>Namespace>Path` - Assigns alias to namespace
- `[X]` - Terminates import block
- `@AliasName|PipelineName` - References imported pipeline

### Multi-line Imports

For very long namespace paths:

```polyglot
[@] com.example>project
[D] @ShortenAlias = @io.github.organization>VeryLongProjectName
[^] >VeryLongModuleName>SubModule>DeepModule
[X]
```

The `[^]` continues the previous line.

### Using Imported Pipelines

**Syntax:** `@LibraryReference|PipelineName`

```polyglot
[D] @Analytics = @io.github.mltools>Analysis
[X]

[|] ProcessData
[i] traning_dataset: pg\path

[t] |T.Call
[r] @Analytics|TrainModel
[<] .datatset: pg\path = dataset_file 
[>] .model: pg\path = model_file



[|] ProcessData
[i] traning_dataset: pg\path

[t] |T.Call
[r] @Analytics|TrainModel
[<] .datatset: pg\path = dataset_file 
[>] .model: pg\path = model_file

[r] @Analytics|Predict
[<] .model: pg\path = model 
[<] data
[>] predictions

[o] >> predictions
[x]
```

### Importing Enums

**Syntax:** `@LibraryReference#EnumName.Value`

```polyglot
[D] @DataLib << @com.example>data>types
[X]

[|] CheckQuality
[i] quality: pg\Enum
[t] |T.Call

[?] quality ?> @DataLib#DataQuality.Excellent
[~][r] |ProcessExcellent

[o] >> quality
[x]
```

### Local Scope Imports

Imports can be scoped to a single pipeline:

```polyglot
[|] MyPipeline
[@] << @SpecialLib>Tools  \\ Only available in this pipeline
[t] |T.Call
[r] @SpecialLib|SpecialOperation
[x]
```

## Naming Conventions

### Hierarchical Structure

The following Java / Maven conventions are adapted for Polyglot:

**For organizations with domains:**
```polyglot
[@] com.MyCompany>Project>Module
[@] org.NonProfit>Initiative>Component
```

**For open source on GitHub:**
```polyglot
[@] io.github.username>ProjectName>Module
```

**For GitLab:**
```polyglot
[@] io.gitlab.username>ProjectName>Module
```

**For personal projects:**
```polyglot
[@] dev.username>ProjectName>Module
```

**For internal company projects:**
```polyglot
[@] internal.companyname>TeamName>ProjectName>Module
```

### Rules

- Use `>` to separate hierarchy levels
- Namespaces are case-sensitive
- Only alphanumeric characters and dots in names (regex: `^([a-zA-Z.]+)(>[a-zA-Z.]+)*$`)
- Descriptive names are encouraged; short cryptic names are discouraged
- Minimum 2 levels recommended, no maximum

### Examples

```polyglot
\\ Good naming
[@] io.github.alice>DataPipeline>ETL
[@] com.acme>Analytics>MachineLearning>NeuralNetworks

\\ Discouraged (too short/cryptic)
[@] alice>dp
[@] x>y>z
```

## File Continuation Pattern

For large libraries split across multiple files:

**File 1: `data_processing.pg`**
```polyglot
[@] io.github.myuser>biglib>data#1
[D] @Numerical << @dev.Hasan>NumericalMethods
[D] @DataFrames << @dev.Hasan>DataFrames
[X]

[|] Pipeline1
[t] |T.Call
[r] @Numerical|Compute
[x]

[|] Pipeline2
[t] |T.Call
[r] @DataFrames|Transform
[x]
```

**File 2: `data_analysis.pg`** (same folder)
```polyglot
[@] io.github.myuser>biglib>data#2
\\ @Numerical and @DataFrames are inherited from #1
[X]

[|] Pipeline3
[t] |T.Call
[r] @Numerical|Analyze  \\ Can use imports from #1
[x]
```

**File 3: `data_export.pg`** (same folder)
```polyglot
[@] io.github.myuser>biglib>data#3
[X]

[|] Pipeline4
[t] |T.Call
[r] |Pipeline1  \\ Can call pipelines from #1 and #2
[r] |Pipeline3
[x]
```

### Continuation Rules

- Files with `#1`, `#2`, `#n` suffixes are continuations
- Must be in the same directory
- All continuations share namespace and imports from `#1`
- Pipelines from earlier files are visible in later files
- Cannot split a single pipeline across files
- Missing continuation numbers cause compile error

### Importing Continued Namespaces

Other files import the base namespace:

```polyglot
[D] @BigLib << @io.github.myuser>biglib>data
[X]

[|] UseBigLib
[t] |T.Call
[r] @BigLib|Pipeline1  \\ From #1
[r] @BigLib|Pipeline3  \\ From #2
[r] @BigLib|Pipeline4  \\ From #3
[x]
```

## Standard Library Implicit Import

The Polyglot standard library is **always automatically imported**:

```polyglot
[@] com.example>myapp>module
[X]

[|] SimplePipeline
[t] |T.Call  \\ Standard lib always available
[w] |W.Python3.10  \\ No import needed
[r] |U.System.Resource.Cpu.Get.Usage >> cpu
[o] >> cpu
[x]
```

**Standard library names are reserved** and cannot be used for user-defined namespaces.

All standard library pipelines are accessible without any `[@]` or `[D]` declarations:
- `|T.*` - Triggers
- `|W.*` - Wrappers
- `|With.*` - Validation wrappers
- `|U.*` - Utilities
- `|Q.*` - Queue management

## Namespace Collision Resolution

### Compile-Time Detection

When multiple packages define the same pipeline name, the compiler detects collisions:

```polyglot
[@] myapp>processor
[D] @DataLib1 << @company1>datatools
[D] @DataLib2 << @company2>datatools  \\ Different source, OK
[X]

[|] ProcessData
[t] |T.Call

\\ ❌ COMPILE ERROR: Ambiguous reference
[r] |ProcessData << data >> result

\\ Compiler error message:
\\ Error: Ambiguous pipeline reference 'ProcessData'
\\ Found in: @DataLib1, @DataLib2
\\ Solution: Use @DataLib1|ProcessData or @DataLib2|ProcessData

[x]
```

### Resolution: Use Explicit References

```polyglot
[@] myapp>processor
[D] @DataLib1 << @company1>datatools
[D] @DataLib2 << @company2>datatools
[X]

[|] ProcessData
[t] |T.Call

\\ ✅ Explicit reference - no ambiguity
[r] @DataLib1|ProcessData << data >> result1
[r] @DataLib2|ProcessData << data >> result2

[o] >> result1
[o] >> result2
[x]
```

### Local Pipeline Priority

Local pipelines take precedence over imported ones:

```polyglot
[@] myapp>processor
[D] @External << @other>library
[X]

\\ Local definition
[|] ProcessData
[t] |T.Call
[r] |LocalOperation
[x]

[|] Main
[t] |T.Call
[r] |ProcessData  \\ Calls local ProcessData
[r] @External|ProcessData  \\ Calls imported one
[x]
```

## Best Practices

### 1. Use Descriptive Aliases

```polyglot
\\ ❌ Bad: Cryptic aliases
[D] @A << @io.github.bigcorp>analytics
[D] @D << @io.github.data>processing

\\ ✅ Good: Clear aliases
[D] @Analytics << @io.github.bigcorp>analytics
[D] @DataProc << @io.github.data>processing
```

### 2. Group Related Imports

```polyglot
\\ Analytics libraries
[D] @ML << @io.github.mltools>models
[D] @Stats << @io.github.statslib>analysis
[D] @Viz << @io.github.charts>visualization

\\ Database libraries
[D] @PostgreSQL << @io.github.db>postgresql
[D] @Redis << @io.github.cache>redis

\\ Utility libraries
[D] @Utils << @io.github.common>utilities
```

### 3. Avoid Overly Long Namespaces

```polyglot
\\ ❌ Too deep
[@] io.github.org>BigProject>BackendServices>DataLayer>Models>UserManagement>Authentication

\\ ✅ Reasonable depth
[@] io.github.org>BigProject>Auth>Models
```

### 4. Use Consistent Naming Patterns

```polyglot
\\ Consistent pattern for your organization
[@] com.acme>analytics>etl
[@] com.acme>analytics>ml
[@] com.acme>analytics>reporting
[@] com.acme>web>frontend
[@] com.acme>web>backend
```

### 5. Document Public APIs

```polyglot
[@] io.github.myuser>publiclib>api
[X]

\\ Public API - stable interface
[|] PublicOperation
[i] input: py\dict
[t] |T.Call
[r] |InternalHelper << input >> result
[o] >> result
[x]

\\ Internal helper - may change
[|] InternalHelper
[i] input: py\dict
[t] |T.Call
[r] |ProcessInternal << input >> result
[o] >> result
[x]
```

## Package Registry (Future)

While not yet implemented, Polyglot envisions a central registry similar to crates.io or PyPI.

### Planned Features

**Registry Server:**
- Central repository at `registry.polyglot.io`
- Web interface for browsing packages
- Search functionality
- Package statistics and ratings

**Versioning:**
- Semantic versioning (SemVer)
- Version constraints in imports
- Automatic dependency resolution

**Security:**
- Package signing and verification
- Security scanning for vulnerabilities
- Trusted publisher verification

**Documentation:**
- Auto-generated API docs
- README and changelog hosting
- Example code snippets

### Future Import Syntax

```polyglot
\\ Version-specific imports (planned)
[D] @DataLib << @io.github.datatools>processing@1.2.3
[D] @ML << @io.github.ml>models@^2.0.0  \\ Compatible with 2.x.x
[D] @Utils << @io.github.utils>common@~1.4.5  \\ Compatible with 1.4.x
```

### For Now: Alternative Sources

Until the central registry is ready:

**Local File Paths:**
```polyglot
[D] @LocalLib << ./libs/mylib
[D] @SharedLib << ../shared/common
```

**Git URLs:**
```polyglot
[D] @GitLib << git+https://github.com/user/repo.git
[D] @GitTag << git+https://github.com/user/repo.git@v1.0.0
```

**Archives:**
```polyglot
[D] @ZipLib << https://example.com/library.zip
[D] @TarLib << https://example.com/library.tar.gz
```

## Complete Example

```polyglot
[@] io.github.alice>DataPipeline>ETL
[D] @Python << @io.github.pyutils>helpers
[D] @Rust << @io.github.rustlib>performance
[D] @DB << @io.github.dbtools>postgresql
[D] @Analytics << @io.github.mltools>analysis
[X]

[|] CompleteETLPipeline
[i] source_path: pg\string
[i] target_table: pg\string
[t] |T.FileCreated << source_path
[t] |T.Schedule.Daily << "02:00"

[Q] |Q.Priority << 1
[Q] |Q.CpuAvailable << 70.0

[w] |W.Python3.11
[w] |W.PostgreSQL << "postgres://localhost/mydb"

[\] |U.Log << "Starting ETL pipeline"

[r] @Python|ReadCSV << source_path >> raw_data
[r] @Python|ValidateData << raw_data >> validated_data
[r] @Rust|TransformData << validated_data >> transformed_data
[r] @Analytics|EnrichData << transformed_data >> enriched_data
[r] @DB|BulkInsert << enriched_data << target_table >> row_count

[/] |U.Log << "ETL pipeline completed: {row_count} rows"

[o] >> row_count
[x]
```

This example demonstrates:
- Clear namespace declaration
- Multiple imports with descriptive aliases
- Using imported pipelines from different languages
- Standard library usage (no import needed)
- Complete the pipeline with triggers, queue config, and wrappers

---

[Next: Standard Library →](05-standard-library.md)