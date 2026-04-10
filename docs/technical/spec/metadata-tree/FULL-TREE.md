---
audience: [architect, designer]
type: spec
updated: 2026-04-09
---

# Full Metadata Tree

Complete `%` tree showing all branches, definition templates, and runtime instances.

**Notation:**
- `(alias)` — this node is commonly known by alias
- `((alias|Canonical))` — alias resolves to Canonical at this location
- `.field` — fixed field (Polyglot-defined)
- `:field` — flexible field (user/runtime-defined)
- `#live.*` — type modifier: runtime-managed, read-only (PGE02006). `live` is a property of the type, not a separate branch
- `→ [[link]]` — see linked file for full specification

---

```
%
+-- definition                                  COMPILE-TIME TEMPLATES
|   |                                           → [[definition-templates]]
|   |
|   +-- .#                                      Struct definitions
|   |   |                                       → [[pglib/types/types|Built-in Types]]
|   |   |
|   |   +-- :Boolean                            %definition.#:Boolean  (bool)
|   |   +-- :String                             %definition.#:String  (string)
|   |   |   +-- %Native.Class
|   |   |       +-- .Rust                       "PgString"
|   |   |       +-- .Validate                   #True
|   |   +-- :Array                              %definition.#:Array  (array)
|   |   +-- :Map                                %definition.#:Map  *(retired #275 — use ##Record)*
|   |   +-- :Serial                             %definition.#:Serial  (serial)
|   |   +-- :Error                              %definition.#:Error
|   |   +-- :Job                                %definition.#:Job
|   |   +-- :Dataframe                          %definition.#:Dataframe
|   |   +-- :path                               %definition.#:path
|   |   +-- :Queue                              %definition.#:Queue  → [[pglib/types/structs]]
|   |   |
|   |   |   Enum types (all {#}, ##Enum)        → [[pglib/types/enums]]
|   |   +-- :PipelineStatus                     %definition.#:PipelineStatus  (pipelinestatus)
|   |   +-- :VarState                           %definition.#:VarState  (varstate)
|   |   +-- :QueueStrategy                      %definition.#:QueueStrategy  (queuestrategy)
|   |   +-- :QueueState                         %definition.#:QueueState  (queuestate)
|   |   +-- :RetriggerStrategy                  %definition.#:RetriggerStrategy  (retriggerstrategy)
|   |   +-- :KillPropagation                    %definition.#:KillPropagation  (killpropagation)
|   |   +-- :ResourceTag                        %definition.#:ResourceTag  (resourcetag)
|   |   +-- :FileAccess                         %definition.#:FileAccess  (fileaccess)
|   |   +-- :FieldKind                          %definition.#:FieldKind  (fieldkind)
|   |   +-- :OS                                 %definition.#:OS  (os)
|   |   +-- :NativeKind                         %definition.#:NativeKind  → [[pglib/types/NativeKind]]
|   |   +-- :WrapperStatus                      %definition.#:WrapperStatus
|   |   |
|   |   |   Permission enums                    → [[permissions]]
|   |   +-- :PermissionIntent                   %definition.#:PermissionIntent  (Ceiling | Grant)
|   |   +-- :PermissionCategory                 %definition.#:PermissionCategory  (File | Web | Database | ...)
|   |   +-- :IODirection                        %definition.#:IODirection  (Inbound | Outbound | Both)
|   |   +-- :AccessLevel                        %definition.#:AccessLevel  (Allow | Deny)
|   |   +-- :GrantAuthority                     %definition.#:GrantAuthority  (Package | Pipeline)
|   |   +-- :OSTarget                           %definition.#:OSTarget  (Any | Linux | Windows | MacOS)
|   |   +-- :Protocol                           %definition.#:Protocol  (File | TCP | UDP | HTTPS | ...)
|   |   +-- :HandleKind                         %definition.#:HandleKind  (Path | ConnectionString | ...)
|   |   +-- :AuditLevel                         %definition.#:AuditLevel  (None | OnUse | OnDeny | All)
|   |   +-- :AlertLevel                         %definition.#:AlertLevel  (None | OnDeny | OnEscalation)
|   |   |
|   |   |   Runtime types                       → [[pglib/types/rt]]
|   |   +-- :Code                               %definition.#:Code  (per-language .Output)
|   |   +-- :PyEnv                              %definition.#:PyEnv
|   |   +-- :RsEnv                              %definition.#:RsEnv
|   |   |
|   |   +-- :UserRecord (example)               %definition.#:UserRecord
|   |   +-- :(any user-defined)
|   |
|   +-- .=                                      Pipeline definitions
|   |   +-- :(name)                             %definition.=:{name}
|   |       +-- .<                               input port template
|   |       +-- .>                               output port template
|   |       +-- .status, .errors, ...            #live fields (type modifier)
|   |
|   +-- .T                                      Trigger definitions
|   |   +-- :(name)                             %definition.T:{name}
|   |       +-- .<                               input port template
|   |       +-- .>                               output port template (.>IsTriggered mandatory)
|   |
|   +-- .W                                      Wrapper definitions
|   |   +-- :(name)                             %definition.W:{name}
|   |       +-- .<                               input template
|   |       +-- .>                               output template
|   |
|   +-- .Q                                      Queue definitions
|   |   +-- :(name)                             %definition.Q:{name}
|   |       +-- .strategy, .host, ...            → [[branches#Queue Branch]]
|   |
|   +-- .##                                     Schema definitions
|   |   |                                       → [[definition-templates#Schema Definition Templates]]
|   |   |
|   |   |   Depth schemas
|   |   +-- :Leaf                               %definition.##:Leaf           %##Depth.Max -> 0
|   |   +-- :Scalar                             %definition.##:Scalar         %##Depth.Max -> 1
|   |   +-- :Flat                               %definition.##:Flat           %##Depth.Max -> 1
|   |   |
|   |   |   Value schemas
|   |   +-- :Inf                                %definition.##:Inf            composable .Inf variant
|   |   |
|   |   |   Structure schemas
|   |   +-- :Sorted                             %definition.##:Sorted         %##Sorted -> #True, %##Ordered -> #True
|   |   |
|   |   |   Classification schemas
|   |   +-- :Enum                               %definition.##:Enum           ##Flat + %##Active -> .One + %###Kind -> .Enum
|   |   |
|   |   |   Parameterized schemas
|   |   +-- :Fields                             %definition.##:Fields         <#Type(##Enum) → stamps variants as [.] fields
|   |   +-- :Nullable                           %definition.##:Nullable       <#ValueType → .Ok.Value OR .None
|   |   +-- :Result                             %definition.##:Result         <#OkType, <#ErrType → .Ok OR .Err
|   |   +-- :String                             %definition.##:String         <regex → .string + .regex
|   |   +-- :Record                             %definition.##:Record         <#Fields(##Enum), <#ValueType → flat enum-keyed
|   |   +-- :Array                              %definition.##:Array          <#ValueType, <Dim → contiguous ordered
|   |   +-- :Dataframe                          %definition.##:Dataframe      <#Columns, <#CellType → array of records
|   |   |
|   |   |   Retired schemas (#275)
|   |   +-- :Deep                               *(retired — use %##Depth.Max << #Inf directly)*
|   |   +-- :Contiguous                         *(retired — use %##Gap << #False, %##Ordered << #True)*
|   |   +-- :Sparse                             *(retired — use %##Gap << #True)*
|   |   +-- :Rectangular                        *(retired — use %##Propagate << #True)*
|   |   +-- :Map                                *(retired — use ##Record)*
|   |   +-- :Set                                *(retired — use ##Array + %###Unique << #True)*
|   |
|   +-- .###                                    Field type definitions
|       |                                       → [[definition-templates#Field Type Definition Templates]]
|       +-- :Value                              %definition.###:Value          leaf holds typed data
|       +-- :Enum                               %definition.###:Enum           leaf is variant selector
|       +-- :ScalarValue                        %definition.###:ScalarValue    regex-validated string
|       +-- :ScalarEnum                         %definition.###:ScalarEnum     variant selector in scalar
|       +-- :None                               %definition.###:None           nullable (empty string "")
|
+-- #   Structs (instances)                     RUNTIME INSTANCES
|   |                                           → [[object-types#%# Branch]]
|   |
|   +-- :Boolean:0                              %#:Boolean:0
|   |   +-- .True  (or .False)                   enum: ONE active field per instance → [[enum-rules]]
|   |
|   +-- :String                                 %#:String  → [[string-subtypes]]
|   |   +-- :int                                %#:String:int  ((#int|#Int))
|   |   +-- :uint                               %#:String:uint  ((#uint|#UnsignedInt))
|   |   +-- :float                              %#:String:float  ((#float|#Float))
|   |   +-- :sci                                %#:String:sci
|   |   +-- :eng                                %#:String:eng
|   |   +-- :dim                                %#:String:dim
|   |   +-- :emailAddress                       %#:String:emailAddress  (user-defined)
|   |   +-- :(any)                               extensible — users define new subtypes
|   |
|   +-- :PipelineStatus:0                       %#:PipelineStatus:0  (enum — ONE active field)
|   +-- :VarState:0                             %#:VarState:0
|   +-- :QueueStrategy:0                        %#:QueueStrategy:0
|   +-- :QueueState:0                           %#:QueueState:0
|   +-- :WrapperStatus:0                        %#:WrapperStatus:0
|   +-- :NativeKind:0                           %#:NativeKind:0
|   +-- :KillPropagation:0                      %#:KillPropagation:0
|   +-- :OS:0                                   %#:OS:0
|   +-- :PermissionIntent:0                     %#:PermissionIntent:0
|   +-- :PermissionCategory:0                   %#:PermissionCategory:0
|   +-- :(all other enums follow same pattern)
|   |
|   +-- :Array:0                                %#:Array:0
|   |   +-- :0, :1, :2, ...                     flexible children (elements)
|   |
|   +-- :Map:0                                  %#:Map:0  *(retired #275 — use ##Record)*
|   |   +-- :key1, :key2, ...                   flexible children (key-value)
|   |
|   +-- :Serial:0                               %#:Serial:0  (arbitrary depth)
|   +-- :Dataframe:0                            %#:Dataframe:0
|   +-- :Error:0                                %#:Error:0  (.Name, .ErrorAlias, .Message, .Info)
|   +-- :Job:0                                  %#:Job:0  (.PID, .status, .hierarchy, .parent)
|   +-- :path:0                                 %#:path:0
|   +-- :Queue:0                                %#:Queue:0  → [[pglib/types/structs#Queue]]
|   +-- :Code:0                                 %#:Code:0  (.stdout, .stderr, .return)
|   |
|   +-- :UserRecord:0 (example)                 %#:UserRecord:0  (user-defined {#})
|   +-- :(any user-defined):N
|   |
|   +-- (all {#} instances carry these fields)
|       +-- .lastModified#live.string
|       +-- .files#live.array:path
|       +-- .errors#live.array:error
|       +-- .usageCount#live.int
|
+-- =   Pipelines (instances)                   → [[branches#Pipeline Branch]]
|   |
|   +-- :(name):N                               %-:{name}:N
|       +-- .<                                   input ports  → [[io-ports]]
|       |   +-- .(port)#type                     (from (-) IO declaration)
|       +-- .>                                   output ports
|       |   +-- .(port)#type
|       +-- .jobs                                job instances (UID-keyed)
|       |   +-- :(uid)#Job                       → [[branches#Job Positional Addressing]]
|       |       +-- .PID, .status, .hierarchy, .parent
|       +-- .r.0, .p.0, .b.0                    positional job paths (compiler-internal)
|       +-- ._                                   pipeline-level permission grants
|       +-- .status#live.#PipelineStatus         → [[metadata#Pipeline]]
|       +-- .errors#live.array:error
|       +-- .isSuccess#live.#Boolean
|       +-- .instanceCount#live.int
|       +-- .lastRun#live.string
|       +-- .duration#live.string
|       +-- .triggerCount#live.int
|
+-- T   Triggers (instances)                    → [[branches#Trigger Branch]]
|   |
|   +-- :(name):N                               %T:{name}:N
|       +-- .<                                   input ports
|       +-- .>                                   output ports (.>IsTriggered#bool mandatory)
|       +-- .status#live
|       +-- .lastFired#live
|       +-- .fireCount#live
|
+-- =   Expanders (instances)
|   |
|   +-- :(name):N                               %=:{name}:N
|   |   +-- .<                                   input (collection)
|   |   +-- .>                                   output (individual items)
|   |
|   +-- Examples: ForEach.Array, ForEach.Map, ForEach.Serial, ForEach.Dataframe, ForEach.Level (.=)
|
+-- *   Collectors (instances)
|   |
|   +-- :(name):N                               %*:{name}:N
|   |
|   +-- Data:       Into.Array, Into.Map, Into.Serial, Agg.Sum, Agg.Count, Agg.Min, Agg.Max, Agg.Avg
|   +-- Collect-all: All
|   +-- Race:       First, Nth
|
+-- $   Variables (instances)                   → [[instance-lifecycle]]
|   |
|   +-- :(name):N                               %$:{name}:N
|       +-- (value subtree per type)
|       +-- .state#live.#VarState                → [[metadata#Variable]]
|       +-- .sourceError#live.error
|
+-- W   Wrappers (instances)                    → [[branches#Wrapper Branch]]
|   |
|   +-- :(name):N                               %W:{name}:N
|       +-- .<                                   wrapper inputs
|       +-- .>                                   wrapper outputs
|       +-- .setup                               [\] setup phase
|       +-- .cleanup                              [/] cleanup phase
|       +-- .status#live.#WrapperStatus          → [[metadata#Wrapper]]
|       +-- .errors#live.array:error
|       +-- .setupDuration#live.string
|       +-- .cleanupDuration#live.string
|
+-- Q   Queues (instances)                      → [[branches#Queue Branch]]
|   |
|   +-- :(name):N                               %Q:{name}:N
|       +-- .strategy#QueueStrategy
|       +-- .host#String
|       +-- .maxInstances#UnsignedInt
|       +-- .maxConcurrent#UnsignedInt
|       +-- .resourceTags#array:ResourceTag
|       +-- .killPropagation#KillPropagation
|       +-- .maxWaitTime#String
|       +-- .description#String
|       +-- .controls (.pause, .resume, .kill)
|       +-- .pendingCount#live.int               → [[metadata#Queue]]
|       +-- .activeCount#live.int
|       +-- .totalProcessed#live.int
|
+-- !   Errors                                  FIXED NAMESPACES (no instances)
|   |                                           → [[pglib/errors/errors]]
|   |
|   +-- .File                                    .NotFound, .ReadError, .WriteError, .ParseError
|   +-- .No                                      .Input, .Output
|   +-- .Timeout                                 .Connection, .Read
|   +-- .Math                                    .DivideByZero
|   +-- .Validation                              .Schema, .Type, .Regex
|   +-- .Field                                   .NotFound, .PathError
|   +-- .Alias                                   .Clash
|   +-- .Permission                              .File.Denied, .Web.Denied, .Database.Denied, .System.Denied,
|   |                                            .Crypto.Denied, .IPC.Denied, .Device.Denied, .Memory.Denied
|   +-- .RT                                      .CompileError, .RuntimeError, .Timeout, .EnvironmentError
|   |
|   +-- .Error                                   USER-EXTENSIBLE (flexible : children)
|       +-- :(namespace)                         %!.Error:{namespace}
|           +-- :(branch)
|               +-- .leaf#Error                  %!.Error:{ns}:{branch}.{leaf}
|
+-- @   Packages                                → [[path-grammar#Package Path]]
|   |
|   +-- :Local:NNN::Name                        %@:Local:{id}::{name}
|   |   +-- ._                                   package permission ceiling
|   +-- :Community:org:pkg::Name                 %@:Community:{org}:{pkg}::{name}
|   +-- :Registry:vendor:lib::Name               %@:Registry:{vendor}:{lib}::{name}
|
+-- _   Permissions                             NO INSTANCES (compile-time only)
    |                                           → [[branches#Permission Branch]], [[permissions]]
    |
    +-- :(name)                                  %_:{name}
        +-- .intent#PermissionIntent
        +-- .target                              __PermissionTarget
        |   +-- .category#PermissionCategory
        |   +-- .capability                      per-category enum
        |   +-- .scope                           __PermissionScope
        |       +-- .pattern#GlobPattern
        |       +-- .direction#IODirection
        +-- .grant                               __PermissionGrant
        |   +-- .level#AccessLevel
        |   +-- .authority#GrantAuthority
        |   +-- .intent#PermissionIntent
        +-- .resource                            __ResourceDescriptor
        |   +-- .os#OSTarget
        |   +-- .protocol#Protocol
        |   +-- .handle#HandleKind
        +-- .audit                               __PermissionAudit
            +-- .log#AuditLevel
            +-- .alert#AlertLevel

    Identifier tiers:  _  = permission object
                        __ = permission descriptor (schema)
                       ___ = constraint descriptor
```

## Alias Resolution Summary

| User writes | Alias resolves to | Schema | Tree path |
|-------------|-------------------|--------|-----------|
| `#int` | `#Int` | `##Int` | `%#:String:int` |
| `#uint` | `#UnsignedInt` | `##UnsignedInt` | `%#:String:uint` |
| `#float` | `#Float` | `##Float` | `%#:String:float` |
| `#string` | `#String` | `#String` | `%#:String` |
| `#bool` | `#Boolean` | `#Boolean` | `%#:Boolean` |
| `#array` | `#Array` | `#Array` | `%#:Array` |
| `#map` | `#Map` | `#Map` | `%#:Map` *(retired #275)* |
| `#serial` | `#Serial` | `#Serial` | `%#:Serial` |

## Path Grammar Quick Reference

→ [[path-grammar]] for formal EBNF

| Pattern | Example |
|---------|---------|
| `%type:ref:instance.field` | `%-:ProcessData:0.status` |
| `%definition.type:ref` | `%definition.#:Boolean` |
| `%definition.##:schema` | `%definition.##:Leaf` |
| `%definition.###:fieldtype` | `%definition.###:Value` |
| `%!.namespace.leaf` | `%!.File.NotFound` |
| `%!.Error:user:path.leaf` | `%!.Error:MyApp:Auth.Expired` |
| `%@:registry:id::name` | `%@:Local:999::MyPkg` |
| `%_:name.field` | `%_:DataCeiling.intent` |
| Shorthand: `-Name%field` | resolves to `%-:Name:<current>.field` |

## Related

- [[path-grammar|Path Grammar]] — formal EBNF
- [[branches|Branch Specifications]] — detailed branch docs
- [[object-types|Object Type Branches]] — branch table
- [[definition-templates|Definition Templates]] — `%definition` details
- [[instance-lifecycle|Instance Lifecycle]] — creation, numbering, release
- [[metadata|user/concepts/metadata]] — user-facing `%` accessors and `live` fields
- [[data-is-trees|user/concepts/data-is-trees]] — conceptual overview
