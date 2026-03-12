# Polyglot v0.0.5 Standard Library Reference

This directory contains YAML-based documentation for the Polyglot v0.0.5 standard library components discovered during training and development.

## Files

### [reserved-enums.yaml](./reserved-enums.yaml)
Reserved enum types and schemas provided by the standard library.

**Categories:**
- Database (`-DB-Settings`, `-DB.RDBMS`)
- Email (`-Email-Status`)
- DateTime (`-DT-Now`, `-DT-Business-Week`)
- Primitives (`-Boolean`)
- Control Flow (`-Input-None`, `-Success`)

**Key Concepts:**
- Reserved enums use `-` prefix (not `#`)
- Schemas define required fields for extensions
- Function enums evaluate at runtime
- Extensions use dot notation: `-BaseType.Extension-Field`

### [standard-pipelines.yaml](./standard-pipelines.yaml)
Core pipelines for data processing, database operations, templating, etc.

**Categories:**
- Data Formats (`YAML.Load`, `JSON.Load`, `TOML.Load`)
- Database (`DB.Query`)
- Templates (`Template.Render`)
- Email (`Email.Send`)
- DateTime (`DT.Now`)
- SQL (`SQL` - multi-line query builder)
- Runtime (`RT.Python.File`)
- Validation (`Regex.Match`, `Array.Length`)

**Key Concepts:**
- Inline-capable vs. full I/O wiring
- Chainable pipelines
- Multi-line content with `[+]` continuation

### [standard-wrappers.yaml](./standard-wrappers.yaml)
Wrappers for resource management and runtime contexts.

**Categories:**
- Database (`W.DB.Connect`, `W.DB.Postgresql`)
- Runtime (`W.RT.Python3.9`, `W.RT.Node16`)
- HTTP (`W.HTTP.Client`)
- File (`W.File.Lock`)

**Key Concepts:**
- Automatic resource cleanup
- No nesting - all at same level
- Optional error handlers with `[w][!]`
- Resource lifecycle management

### [standard-triggers.yaml](./standard-triggers.yaml)
Trigger mechanisms for pipeline activation.

**Categories:**
- CLI (`T.Cli`)
- Filesystem (`T.Folder.NewFiles`, `T.File.Modified`)
- Function (`T.Function`, `T.Call`)
- HTTP (`T.HTTP.Endpoint`)
- Scheduled (`T.Cron`, `T.Interval`)
- Message (`T.Queue.Message`)

**Key Concepts:**
- Inline parameters for simple triggers
- I/O wiring for complex triggers
- Each pipeline has exactly one trigger
- Output wiring to pipeline inputs

### [standard-operators.yaml](./standard-operators.yaml)
Pack and unpack operators for collection manipulation.

**Unpack Operators (`~`):**
- `~ForEach.Array` - Iterate arrays
- `~ForEach.Set` - Iterate sets
- `~ForEach.Serial` - Iterate serial fields

**Pack Operators (`*`):**
- `*Into.Array` - Collect into array
- `*Into.Set` - Collect into set
- `*Aggregate.Sum` - Sum values
- `*Aggregate.Count` - Count iterations
- `*String.Concat` - Concatenate strings

**Key Concepts:**
- Space-wrapped operators (` ~ `, ` * `)
- Unpack creates iteration context
- Pack aggregates results
- Multiple packs in same iteration allowed

## Usage Examples

### Complete Pipeline with Multiple Components

```polyglot
%% File processor with database storage
{@} @Local:FileProcessor:1.0.0
[%] %Doc << "Process files and store results"
{x}



{#} #App.Config-FileProcessor
[A] #Config

[s] << |YAML.Load"\\FileDir\\config\settings.yaml"
   [.] .db-host:string << .database.host
   [.] .db-port:int << .database.port
   [.] .folder:path << .monitor.folder

[s][!] !*
{x}



{|} |ProcessFiles
[t] |T.Folder.NewFiles
 |  <folder:path << #Config.folder
 |  >files:array.path >> <new_files

[<] <new_files:array.path
[>] >results:serial

[w] |W.DB.Postgresql
 |  <db << #Config
 |  >db:serial >> $db

[r] $processed:int << 0

[p] ~ForEach.Array
 ~  <array << $new_files
 ~  >item >> $file
 ~  >index >> $idx

   [r] |DB.Query
    |  <db:serial << $db
    |  <query:string << |SQL""
   [+] +"INSERT INTO files (name, processed_at)"
   [+] +"VALUES ('{$file}', {|DT.Now""})"
    |  >results:array.serial >> $result

   [*] *Aggregate.Sum
    *  <inc << 1
    *  >sum >> $processed

[>] >results << {
[+]  .total_processed: $processed,
[+]  .completed_at: |DT.Now""
[+] }
{x}
```

## Maintenance

These files are updated as new standard library components are discovered during:
- Training sessions with code corrections
- Implementation work
- Documentation reviews

**Last Updated:** 2026-01-01
**Version:** 0.0.5
**Status:** Living documentation (updated during training)
