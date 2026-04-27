# Lesson 001: Package Declarations

**Date**: 2026-04-27
**Context**: Defining the package scope and importing dependencies in a Polyglot file.

## Lesson Summary

A file must declare its package scope explicitly at the top. The syntax format is:
`{@} @Scope:Identifier<Package.Hierarchy:Version`

There are three main scopes:
1. **Local**: Used for local tests and specific port assignments.
   `{@} @Local:55555<QA:Test.ArraysAndPredicates:1.0.0`
2. **Community**: Used for public packages published by users.
   `{@} @Community:coder123<Weather.Database:1.0.0`
3. **Company**: Used for official organization packages.
   `{@} @Company:RegisteredName<Package.Hierarchy:Version`

### Importing Dependencies
Dependencies are imported with the syntax:
`[@] @Alias << @Scope:Identifier<Package.Hierarchy:Version`
Example:
`[@] @Weather << @Community:coder123<Weather.Database:1.0.0`

### Accessing Dependencies
When using an API or calling an action from a dependency, prefix the action with the alias:
`[=] @Weather-API.Sensor.GetRegion`
