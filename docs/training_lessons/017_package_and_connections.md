# Lesson 017: Package and Connections

**Date**: 2026-04-27
**Context**: Declaring the package structure and database connections at the start of a Aljam3 file.

## Lesson Summary

Aljam3 does not use generic `[+]` blocks for package and version definitions. Instead, it uses the specific Definition Context `{@}` for the local package, and `[@]` for importing or binding external connections.

### Correct Usage
Notice that the version is defined using dots (`1.1.0`), and connection mapping is explicitly bound.
```aljam3
{@} @Local:55555<Score.Processing:1.1.0
   [@] @DB << @Company:TopHillHighSchool<StudentDB:1.3.5
```
