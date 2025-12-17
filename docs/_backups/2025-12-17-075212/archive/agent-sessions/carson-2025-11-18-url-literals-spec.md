# URL Literals - Type System & Syntax Specification

**Session Date**: 2025-11-18
**Facilitator**: Carson (Elite Brainstorming Specialist)
**Participant**: hhj (Product Owner)
**Status**: APPROVED - Production Ready

---

## Executive Summary

Complete specification for URL literals in Polyglot, introducing the `pg\url` distinct type with four literal variants for different encoding scenarios, parsed query parameters, and reserved field access.

**Key Decisions**:
1. `pg\url` as distinct type (like `pg\path`)
2. Four URL literal variants: `url`, `urlencoded`, `urlraw`, `urltemplate`
3. Parsed query parameters via `.query` (returns `pg\serial`)
4. Reserved fields for URL components
5. Runtime protocol validation (not compile-time)

---

## Table of Contents

1. [Type System](#type-system)
2. [URL Literal Variants](#url-literal-variants)
3. [Reserved Fields](#reserved-fields)
4. [Query Parameter Parsing](#query-parameter-parsing)
5. [Protocol Handling](#protocol-handling)
6. [Validation Rules](#validation-rules)
7. [Standard Library Integration](#standard-library-integration)
8. [Complete Examples](#complete-examples)
9. [Design Decisions](#design-decisions)

---

## Type System

### `pg\url` - Distinct Type

**Purpose**: Dedicated type for URLs with structure, validation, and component access

**Rationale**:
- URLs have well-defined structure (protocol, domain, path, query, fragment)
- Type safety prevents mixing regular strings with URLs
- Enables reserved field access like `pg\path`
- Consistent with Polyglot's type philosophy

**Declaration**:
```polyglot
[i] .api_endpoint: pg\url
[r] .website: pg\url << url"https://google.com"
```

**Serialization**: Fully serializable (like all `pg\` types)

---

## URL Literal Variants

### 1. `url"..."` - Standard URL

**Purpose**: Basic URL literal with actual URL syntax (as-is)

**Syntax**:
```polyglot
url"https://api.example.com/v2/users"
url"http://localhost:8080/api"
url"ftp://files.example.com/data/file.zip"
url"ws://socket.example.com:3000"
```

**Behavior**:
- Uses actual URL syntax (forward slashes, colons, standard format)
- No automatic encoding
- Expects valid URL format
- Any protocol supported

**Use Cases**:
- Static, well-formed URLs
- URLs without special characters
- Standard API endpoints

**Example**:
```polyglot
[r] .github_api: pg\url << url"https://api.github.com/repos/user/project"
[r] .local_server: pg\url << url"http://localhost:3000/api/v1"
```

---

### 2. `urlencoded"..."` - Auto-Encoding URL

**Purpose**: Automatically encodes special characters (spaces, unicode, etc.)

**Syntax**:
```polyglot
urlencoded"https://example.com/search?q=hello world"
→ "https://example.com/search?q=hello%20world"

urlencoded"https://example.com/file?name=my document.pdf"
→ "https://example.com/file?name=my%20document.pdf"
```

**Behavior**:
- Automatically percent-encodes special characters
- Spaces → `%20`
- Non-ASCII characters → percent-encoded
- Reserved characters in query strings encoded appropriately

**Use Cases**:
- User input in URLs
- Search queries
- Filenames with spaces
- International characters

**Example**:
```polyglot
[r] .user_query: pg\string << "machine learning"
[r] .search_url: pg\url << urlencoded"https://search.example.com?q={.user_query}"
// Result: "https://search.example.com?q=machine%20learning"
```

---

### 3. `urlraw"..."` - Raw Pre-Encoded URL

**Purpose**: URL is already encoded, don't modify it

**Syntax**:
```polyglot
urlraw"https://example.com/search?q=hello%20world"
// No modification - already encoded
```

**Behavior**:
- No encoding or decoding
- Assumes URL is already properly formatted
- Pass-through literal

**Use Cases**:
- URLs from external sources already encoded
- Copy-pasted URLs from browsers
- Pre-encoded API responses

**Example**:
```polyglot
[r] .external_url: pg\string << "https://api.example.com/data?filter=%7B%22status%22%3A%22active%22%7D"
[r] .typed_url: pg\url << urlraw"{.external_url}"
// No double-encoding
```

---

### 4. `urltemplate"..."` - Template URL with Variable Substitution

**Purpose**: URL with variable placeholders, auto-encoded after substitution

**Syntax**:
```polyglot
urltemplate"https://api.example.com/users/{.user_id}/posts/{.post_id}"
```

**Behavior**:
- Substitutes `{.variable}` placeholders
- Encodes substituted values automatically
- Static parts remain as-is

**Use Cases**:
- Dynamic API endpoints
- REST APIs with path parameters
- User-specific URLs

**Example**:
```polyglot
[r] .user_id: pg\int << 12345
[r] .post_id: pg\int << 67890

[r] .api_url: pg\url << urltemplate"https://api.example.com/users/{.user_id}/posts/{.post_id}"
// Result: "https://api.example.com/users/12345/posts/67890"

// With encoding
[r] .tag: pg\string << "machine learning"
[r] .search: pg\url << urltemplate"https://blog.example.com/tags/{.tag}"
// Result: "https://blog.example.com/tags/machine%20learning"
```

---

## Reserved Fields

### Component Access

**All `pg\url` values have reserved fields for accessing URL components:**

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `.protocol` | `pg\string` | URL scheme | `"https"` |
| `.domain` | `pg\string` | Host/domain name | `"api.example.com"` |
| `.port` | `pg\int` | Port number | `8080` |
| `.path` | `pg\string` | URL path | `"/v2/users"` |
| `.query` | `pg\serial` | Parsed query parameters | See below |
| `.anchor` | `pg\string` | Fragment identifier | `"results"` |
| `.full` | `pg\string` | Complete URL string | Full URL |

---

### Field Access Examples

```polyglot
[r] .api: pg\url << url"https://api.example.com:8080/v2/users?limit=10&offset=20#results"

// Component access
[r] .scheme: pg\string << .api.protocol      // "https"
[r] .host: pg\string << .api.domain          // "api.example.com"
[r] .port_num: pg\int << .api.port           // 8080
[r] .url_path: pg\string << .api.path        // "/v2/users"
[r] .fragment: pg\string << .api.anchor      // "results"
[r] .complete: pg\string << .api.full        // Full URL
```

---

### Default Values

**Missing components return defaults:**

```polyglot
[r] .simple: pg\url << url"https://example.com/path"

.simple.port     → 443 (default HTTPS port)
.simple.anchor   → "" (empty string)
.simple.query    → Empty pg\serial (no parameters)
```

**Port defaults by protocol:**
- `http` → 80
- `https` → 443
- `ftp` → 21
- `ws` → 80
- `wss` → 443

---

## Query Parameter Parsing

### Parsed Query Structure

**`.query` field returns `pg\serial` with parsed key-value pairs:**

```polyglot
[r] .api: pg\url << url"https://api.example.com?limit=10&offset=20&sort=asc&filter=active"

// Access individual query parameters
.api.query.limit   → "10"
.api.query.offset  → "20"
.api.query.sort    → "asc"
.api.query.filter  → "active"
```

---

### Array Parameters

**Repeated keys become arrays:**

```polyglot
[r] .search: pg\url << url"https://api.example.com?tag=python&tag=rust&tag=go"

.search.query.tag → ["python", "rust", "go"]  // Array of strings
```

---

### Empty Query String

```polyglot
[r] .simple: pg\url << url"https://example.com/path"

.simple.query → Empty pg\serial (no keys)
```

---

### Raw Query String

**Use `.full` to get unparsed query:**

```polyglot
[r] .api: pg\url << url"https://api.example.com?limit=10&offset=20"

.api.full → "https://api.example.com?limit=10&offset=20"  // Raw
```

---

## Protocol Handling

### Runtime Validation Only

**Compile-time**: Only checks URL format structure
**Runtime**: Validates protocol support and format

```polyglot
// All valid at compile-time (proper syntax)
url"https://example.com"       // Standard HTTPS
url"http://localhost:8080"     // HTTP with port
url"ftp://files.example.com"   // FTP
url"ws://socket.example.com"   // WebSocket
url"wss://secure.example.com"  // WebSocket Secure
url"file:///local/path"        // File protocol
url"custom://myapp/action"     // Custom protocol
```

**No protocol whitelist** - if URL parser can handle it, it's valid

---

### Protocol-Specific Behavior

**Standard library may provide protocol-specific utilities:**

```polyglot
// HTTP/HTTPS
[r] |U.HTTP.Get
[<] .url: pg\url << url"https://api.example.com"

// WebSocket
[r] |U.WebSocket.Connect
[<] .url: pg\url << url"ws://socket.example.com"

// FTP
[r] |U.FTP.Download
[<] .url: pg\url << url"ftp://files.example.com/data.zip"
```

---

## Validation Rules

### Compile-Time Validation

**Checks performed at compile-time:**

1. **Proper URL format structure**
   ```polyglot
   url"https://example.com"     // ✓ Valid
   url"not a url"               // ✗ Compile error
   url"://missing-protocol"     // ✗ Compile error
   ```

2. **Matching quotes**
   ```polyglot
   url"https://example.com"     // ✓ Valid
   url"https://example.com      // ✗ Compile error (unclosed)
   ```

3. **Valid characters in protocol**
   ```polyglot
   url"https://example.com"     // ✓ Valid
   url"ht!tps://example.com"    // ✗ Compile error (invalid char)
   ```

---

### Runtime Validation

**Checks performed at runtime:**

1. **Protocol support** (depends on runtime capabilities)
2. **DNS resolution** (for network protocols)
3. **Port validity** (1-65535)
4. **URL component parsing**

---

### Validation Errors

```polyglot
[r] .api: pg\url << url"https://invalid..domain..com"

// Runtime error: Invalid domain format
// Error type: !pg.URL.InvalidFormat
```

---

## Standard Library Integration

### HTTP Operations

```polyglot
[r] |U.HTTP.Get
[<] .url: pg\url << url"https://api.example.com/users"
[>] .response: pg\string >> .result

[r] |U.HTTP.Post
[<] .url: pg\url << url"https://api.example.com/users"
[<] .body: pg\string << JSON"request_body.json"
[>] .response: pg\string >> .result
```

---

### URL Building

```polyglot
[r] |U.URL.Build
[<] .base: pg\url << url"https://api.example.com"
[<] .path: pg\string << "/v2/users"
[<] .query: pg\serial << {limit: 10, offset: 20}
[>] .url: pg\url >> .built_url
// Result: "https://api.example.com/v2/users?limit=10&offset=20"
```

---

### URL Validation

```polyglot
[r] |U.URL.Validate
[<] .url: pg\url << url"https://example.com"
[>] .is_valid: pg\bool >> .valid
[>] .error: !pg.URL.Error >> .validation_error
```

---

### URL Encoding/Decoding

```polyglot
[r] |U.URL.Encode
[<] .text: pg\string << "hello world"
[>] .encoded: pg\string >> .result
// Result: "hello%20world"

[r] |U.URL.Decode
[<] .text: pg\string << "hello%20world"
[>] .decoded: pg\string >> .result
// Result: "hello world"
```

---

## Complete Examples

### Example 1: REST API Client

```polyglot
[|] FetchUserData
[i] .user_id: pg\int
[o] .user_data: pg\string

// Build API URL with template
[r] .api_url: pg\url << urltemplate"https://api.example.com/users/{.user_id}"

// Make HTTP request
[r] |U.HTTP.Get
[<] .url: pg\url << .api_url
[<] .headers: pg\serial << {Authorization: "Bearer token123"}
[>] .response: pg\string >> .user_data

[!] !pg.HTTP.RequestFailed
[r] |LogError
[<] .message: pg\string << "Failed to fetch user {.user_id}"
[X]
```

---

### Example 2: Search URL with Encoding

```polyglot
[|] BuildSearchURL
[i] .query: pg\string
[i] .filters: pg\serial
[o] .search_url: pg\url

// User query with special characters
[r] .encoded_url: pg\url << urlencoded"https://search.example.com?q={.query}"

// Add filters from serial
[r] |U.URL.Build
[<] .base: pg\url << .encoded_url
[<] .query_params: pg\serial << .filters
[>] .url: pg\url >> .search_url
[X]
```

---

### Example 3: URL Component Extraction

```polyglot
[|] ParseAPIEndpoint
[i] .endpoint: pg\url
[o] .host: pg\string
[o] .api_version: pg\string

// Extract domain
[r] .host << .endpoint.domain

// Extract version from path
[r] .path_parts: pg\array{pg\string} << |U.String.Split
[<] .text: pg\string << .endpoint.path
[<] .delimiter: pg\string << "/"

[r] .api_version << .path_parts[1]  // e.g., "v2" from "/v2/users"
[X]
```

---

### Example 4: WebSocket Connection

```polyglot
[|] ConnectWebSocket
[i] .server: pg\string
[i] .port: pg\int
[o] .connection: pg\db

// Build WebSocket URL
[r] .ws_url: pg\url << urltemplate"wss://{.server}:{.port}/socket"

// Connect
[r] |U.WebSocket.Connect
[<] .url: pg\url << .ws_url
[<] .timeout_ms: pg\int << 5000
[>] .conn: pg\db >> .connection

[!] !pg.WebSocket.ConnectionFailed
[r] |U.Log.Error
[<] .message: pg\string << "Failed to connect to {.ws_url.full}"
[X]
```

---

### Example 5: File URL Handling

```polyglot
[|] LoadLocalFile
[i] .file_path: pg\path
[o] .contents: pg\string

// Convert path to file URL
[r] .file_url: pg\url << urltemplate"file:///{.file_path.full}"

// Load file
[r] |U.File.Load
[<] .url: pg\url << .file_url
[>] .data: pg\string >> .contents

[!] !pg.FileSystem.NotFound
[r] |U.Log.Warning
[<] .message: pg\string << "File not found: {.file_path.full}"
[X]
```

---

### Example 6: Query Parameter Manipulation

```polyglot
[|] AddPaginationParams
[i] .base_url: pg\url
[i] .page: pg\int
[i] .page_size: pg\int
[o] .paginated_url: pg\url

// Extract existing query parameters
[r] .existing_params: pg\serial << .base_url.query

// Add pagination
[r] .existing_params.page << .page
[r] .existing_params.page_size << .page_size

// Rebuild URL
[r] |U.URL.Build
[<] .protocol: pg\string << .base_url.protocol
[<] .domain: pg\string << .base_url.domain
[<] .path: pg\string << .base_url.path
[<] .query: pg\serial << .existing_params
[>] .url: pg\url >> .paginated_url
[X]
```

---

## Design Decisions

### Decision 1: Distinct Type vs String

**Decision**: `pg\url` as distinct type (not `pg\string`)

**Rationale**:
- URLs have well-defined structure deserving dedicated type
- Type safety prevents mixing plain strings with URLs
- Enables reserved field access (like `pg\path`)
- Consistent with Polyglot's type philosophy
- Allows URL-specific validation and operations

**Alternatives Considered**:
- `pg\string` with validation: Rejected (loses type safety)
- Subtype of `pg\string`: Rejected (unnecessary complexity)

---

### Decision 2: Four Literal Variants

**Decision**: Provide four URL literal syntaxes: `url`, `urlencoded`, `urlraw`, `urltemplate`

**Rationale**:
- Different use cases require different encoding behavior
- Explicit is better than implicit (Polyglot philosophy)
- Prevents encoding bugs (double-encoding, missing encoding)
- Clear intent in source code

**Alternatives Considered**:
- Single `url"..."` with auto-detection: Rejected (too magical)
- Flags/modifiers: Rejected (less readable)

---

### Decision 3: Parsed Query Parameters

**Decision**: `.query` returns `pg\serial` with parsed key-value pairs

**Rationale**:
- Convenient access to individual parameters
- Type-safe parameter extraction
- Consistent with Polyglot's structured data approach
- Raw query still accessible via `.full`

**Alternatives Considered**:
- Raw string only: Rejected (requires manual parsing)
- Both `.query` (parsed) and `.query_raw` (string): Redundant (use `.full`)

---

### Decision 4: Runtime Protocol Validation

**Decision**: Protocol validation happens at runtime, not compile-time

**Rationale**:
- Allows custom/future protocols
- Flexibility for edge cases
- Compile-time would require maintaining protocol whitelist
- Runtime can provide better error messages with context

**Alternatives Considered**:
- Compile-time whitelist: Rejected (too restrictive)
- No validation: Rejected (loses safety benefits)

---

### Decision 5: Reserved Field Names

**Decision**: `.protocol`, `.domain`, `.port`, `.path`, `.query`, `.anchor`, `.full`

**Rationale**:
- Clear, unambiguous names
- Consistent with web standards terminology
- `.protocol` over `.scheme` (more common term)
- `.domain` over `.host` (clearer for users)
- `.anchor` over `.fragment` (more intuitive)

**Alternatives Considered**:
- RFC 3986 exact terminology: Rejected (less user-friendly)
- Abbreviated names: Rejected (less clear)

---

### Decision 6: Standard URL Syntax Inside Literals

**Decision**: Use actual URL syntax inside literals (forward slashes, standard format)

**Rationale**:
- Familiar to all developers
- Copy-paste from browser/documentation works
- Standard tooling support (linters, validators)
- Clear distinction from path literals (backslashes)

**Alternatives Considered**:
- Polyglot-style backslashes: Rejected (confusing, non-standard)
- Custom DSL: Rejected (unnecessary learning curve)

---

### Decision 7: Automatic Port Defaults

**Decision**: Missing port returns protocol default (80, 443, etc.)

**Rationale**:
- Matches web standard behavior
- Convenient for users
- Explicit port still accessible if specified

**Alternatives Considered**:
- Return 0 or null: Rejected (requires nil checks)
- Always require explicit port: Rejected (verbose)

---

## Implementation Notes

### Compiler Responsibilities

1. **Syntax validation**: Check URL format structure
2. **Literal type assignment**: Mark as `pg\url` type
3. **Reserved field compilation**: Generate accessor code
4. **Encoding transformation**: Apply encoding rules per variant

---

### Runtime Responsibilities

1. **URL parsing**: Parse into components
2. **Query parameter parsing**: Build `pg\serial` structure
3. **Protocol validation**: Verify protocol support
4. **DNS resolution**: Resolve domains (for network protocols)
5. **Error handling**: Generate appropriate error types

---

### Standard Library Requirements

**Minimum required utilities:**
- `|U.HTTP.Get`, `|U.HTTP.Post` (HTTP operations)
- `|U.URL.Build` (construct URLs from components)
- `|U.URL.Validate` (explicit validation)
- `|U.URL.Encode`, `|U.URL.Decode` (encoding utilities)
- `|U.WebSocket.Connect` (WebSocket support)

---

## Migration Guide

### For New Code

Use appropriate literal variant:
```polyglot
// Static URLs
url"https://api.example.com/v2/users"

// User input
urlencoded"https://search.example.com?q={.user_query}"

// External URLs
urlraw"{.external_url}"

// Dynamic endpoints
urltemplate"https://api.example.com/users/{.user_id}"
```

---

### For Existing Code Using pg\string

**Before:**
```polyglot
[r] .endpoint: pg\string << "https://api.example.com"
```

**After:**
```polyglot
[r] .endpoint: pg\url << url"https://api.example.com"
```

**Note**: Existing string-based URLs continue working, but `pg\url` provides type safety and component access.

---

## Summary

**Complete URL system for Polyglot:**
- ✅ `pg\url` distinct type
- ✅ Four literal variants: `url`, `urlencoded`, `urlraw`, `urltemplate`
- ✅ Seven reserved fields for component access
- ✅ Parsed query parameters (`pg\serial`)
- ✅ Runtime protocol validation
- ✅ Standard library integration points
- ✅ Complete examples and migration guide

**Status**: Production-ready specification, ready for implementation

---

**Session Facilitator**: Carson (Elite Brainstorming Specialist)
**Date**: 2025-11-18
**Approval**: hhj (Product Owner)
**Next Steps**:
1. Update `language/type-system.md` with `pg\url` type
2. Document URL literals in language specification
3. Add to lexer token definitions (Story 1.2)
4. Implement in standard library

---

**End of URL Literals Specification**
