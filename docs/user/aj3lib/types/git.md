---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.#:Git"
metadata_instance: "%#:Git:N"
---

# #Git — Git Event Type Tree

<!-- @c:types -->

Aj3lib struct providing typed payloads for Git event triggers (`-T.Git.*`). All subtypes are nested under a single `{#} #Git` definition. Enum subtypes carry `[#] ##Enum` schema classification.

See [[aj3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]] for the trigger family that produces these types.

## Definition

```aljam3
{#} #Git
   [%] .description << "Git event type tree for CI/CD triggers"

   [ ] ─── Identity ───
   [.] .Author
      [.] .name#string
      [.] .email#email

   [ ] ─── File change stats ───
   [.] .FileStatus
      [#] ##Enum
      [#] ##Scalar
      [#] ###ScalarEnum
      [.] .Added
      [.] .Modified
      [.] .Deleted
      [.] .Renamed
      [.] .Copied

   [.] .DiffStat
      [.] .path#path
      [.] .status#Git.FileStatus
      [.] .insertions#int
      [.] .deletions#int

   [ ] ─── Core commit record ───
   [.] .Commit
      [.] .sha#string
      [.] .author#Git.Author
      [.] .committer#Git.Author
      [.] .message#string
      [.] .timestamp#DateTime
      [.] .parents#array.string
      [.] .files#array.Git.DiffStat

   [ ] ─── Ref kind ───
   [.] .RefKind
      [#] ##Enum
      [#] ##Scalar
      [#] ###ScalarEnum
      [.] .Branch
      [.] .Tag
      [.] .Remote

   [.] .Ref
      [.] .name#string
      [.] .sha#string
      [.] .kind#Git.RefKind

   [ ] ─── Push event payload ───
   [.] .Push
      [.] .branch#string
      [.] .remote#string
      [.] .before#string
      [.] .after#string
      [.] .forced#bool
      [.] .commits#array.Git.Commit

   [ ] ─── Tag event payload ───
   [.] .Tag
      [.] .name#string
      [.] .sha#string
      [.] .annotated#bool
      [.] .message#string
      [.] .tagger#Git.Author

   [ ] ─── Pull Request (remote webhooks only) ───
   [.] .PRAction
      [#] ##Enum
      [#] ##Scalar
      [#] ###ScalarEnum
      [.] .Opened
      [.] .Synchronize
      [.] .Merged
      [.] .Closed

   [.] .PR
      [.] .number#int
      [.] .title#string
      [.] .body#string
      [.] .source#string
      [.] .target#string
      [.] .author#Git.Author
      [.] .action#Git.PRAction
      [.] .commits#array.Git.Commit

   [ ] ─── Raw local hook passthrough ───
   [.] .HookName
      [#] ##Enum
      [#] ##Scalar
      [#] ###ScalarEnum
      [.] .PostCommit
      [.] .PrePush
      [.] .PostReceive
      [.] .PostMerge
      [.] .PostCheckout
      [.] .PreRebase

   [.] .Hook
      [.] .name#Git.HookName
      [.] .args#array.string

   [ ] ─── Repository configuration ───
   [.] .Repo
      [.] .path#path
      [.] .remote#string
      [.] .defaultBranch#string
```

## Subtypes

### Struct Subtypes

| Subtype | Fields | Description |
|---------|--------|-------------|
| `.Author` | `.name#string`, `.email#email` | Git identity (author or committer) |
| `.DiffStat` | `.path#path`, `.status`, `.insertions#int`, `.deletions#int` | Per-file change statistics |
| `.Commit` | `.sha`, `.author`, `.committer`, `.message`, `.timestamp`, `.parents`, `.files` | Complete commit record |
| `.Ref` | `.name#string`, `.sha#string`, `.kind#Git.RefKind` | Branch, tag, or remote reference |
| `.Push` | `.branch`, `.remote`, `.before`, `.after`, `.forced`, `.commits` | Push event payload |
| `.Tag` | `.name`, `.sha`, `.annotated`, `.message`, `.tagger` | Tag creation event payload |
| `.PR` | `.number`, `.title`, `.body`, `.source`, `.target`, `.author`, `.action`, `.commits` | Pull request event payload (remote only) |
| `.Hook` | `.name#Git.HookName`, `.args#array.string` | Raw local git hook passthrough |
| `.Repo` | `.path#path`, `.remote#string`, `.defaultBranch#string` | Repository configuration |

### Enum Subtypes

| Subtype | Values | Description |
|---------|--------|-------------|
| `.FileStatus` | Added, Modified, Deleted, Renamed, Copied | File change classification |
| `.RefKind` | Branch, Tag, Remote | Reference type |
| `.PRAction` | Opened, Synchronize, Merged, Closed | Pull request event action |
| `.HookName` | PostCommit, PrePush, PostReceive, PostMerge, PostCheckout, PreRebase | Git hook names |

All enum subtypes use `[#] ##Enum`, `[#] ##Scalar`, `[#] ###ScalarEnum` schema properties.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Git` | Compile-time type template |
| Instance | `%#:Git:N` | Runtime instance (N = instance number) |

## Related

- [[aj3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]] — triggers that produce `#Git` payloads
- [[aj3lib/pipelines/T/Git.Hook|-T.Git.Hook]] — local hook transport trigger
- [[aj3lib/pipelines/T/Git.Push|-T.Git.Push]] — push event trigger
- [[aj3lib/pipelines/T/Git.PR|-T.Git.PR]] — pull request event trigger
- [[aj3lib/pipelines/T/Git.Tag|-T.Git.Tag]] — tag creation trigger
- [[syntax/types/INDEX|types]] — full type system specification
