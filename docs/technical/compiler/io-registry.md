---
audience: developer
type: specification
status: complete
updated: 2026-04-18
---

# IO Registry (Sink Tables)

<!-- @c:technical/algorithms/foreign-code-analysis -->
<!-- @u:concepts/permissions/foreign-code -->
<!-- @c:technical/compiler/ast-invisible-registry -->

The IO registry maps foreign function names to permission categories with the parameter position that holds the resource argument. It is the core data structure behind the compiler's foreign code analysis.

## Overview

The registry is a separate configuration file (`io-registry.toml`) that ships with the compiler but is versioned independently. It contains:

- **Sink tables** — per-language, per-category function→resource mappings
- **Known-pure functions** — builtins that are silently skipped during analysis
- **Deferred categories** — future expansion areas

**Companion registry:** [[ast-invisible-registry|AST-Invisible Functions Registry]] is the dual to this file — it lists foreign constructs that **cannot** be analyzed (eval/exec/dynamic codegen). Together they cover the foreign-code analysis surface: this file maps analyzable IO to permission categories; the AST-invisible registry bans the unanalyzable.

## Registry Structure

The registry uses TOML format, organized by language and category:

```toml
[meta]
version = "2026.04"
compiler_min = "0.1.0"

[python.file]
"builtins.open" = { arg = 0, capability = "mode_dependent", notes = "mode arg determines Read/Write" }
"pathlib.Path.read_text" = { arg = "constructor_0", capability = "Read" }
"pathlib.Path.write_text" = { arg = "constructor_0", capability = "Write" }
"os.remove" = { arg = 0, capability = "Delete" }
"shutil.copy" = { args = [0, 1], capability = ["Read", "Write"] }
"pandas.read_csv" = { arg = 0, capability = "Read" }
"pandas.read_excel" = { arg = 0, capability = "Read" }
"pandas.DataFrame.to_csv" = { arg = 0, capability = "Write" }

[python.network]
"requests.get" = { arg = 0, capability = "Request" }
"requests.post" = { arg = 0, capability = "Request" }
# ...

[python.pure]
builtins = ["len", "str", "int", "float", "bool", "list", "dict", "set", "tuple",
            "sorted", "enumerate", "range", "zip", "map", "filter", "isinstance",
            "type", "hasattr", "getattr", "print", "format", "repr", "abs", "min",
            "max", "sum", "round", "chr", "ord", "hex", "bin", "oct"]
string_methods = ["upper", "lower", "strip", "split", "join", "replace", "startswith",
                  "endswith", "find", "count", "format", "encode", "decode"]
```

## File Sink Tables

### Python

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `builtins.open(0)` | arg 0 = path | #File (mode arg determines #Read/#Write) |
| `pathlib.Path(0).read_text()` | constructor arg 0 | #File.#Read |
| `pathlib.Path(0).write_text()` | constructor arg 0 | #File.#Write |
| `os.remove(0)` | arg 0 | #File.#Delete |
| `shutil.copy(0, 1)` | args 0, 1 | #File.#Read + #File.#Write |
| `pandas.read_csv(0)` | arg 0 | #File.#Read |
| `pandas.read_excel(0)` | arg 0 | #File.#Read |
| `pandas.DataFrame.to_csv(0)` | arg 0 | #File.#Write |

### Rust

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `File::open(0)` | arg 0 | #File.#Read |
| `File::create(0)` | arg 0 | #File.#Write |
| `fs::read(0)` | arg 0 | #File.#Read |
| `fs::write(0, _)` | arg 0 | #File.#Write |
| `fs::remove_file(0)` | arg 0 | #File.#Delete |

### C/C++

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `fopen(0, 1)` | arg 0 = path, arg 1 = mode | #File |
| `open(0, 1)` | arg 0 | #File |
| `remove(0)` | arg 0 | #File.#Delete |
| `stat(0, _)` | arg 0 | #File.#Read |

### JavaScript

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `fs.readFileSync(0)` | arg 0 | #File.#Read |
| `fs.writeFileSync(0, _)` | arg 0 | #File.#Write |
| `fs.promises.readFile(0)` | arg 0 | #File.#Read |
| `Deno.readTextFile(0)` | arg 0 | #File.#Read |

## Network Sink Tables

### Python

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `requests.get(0)` | arg 0 = URL | #Web.#Request |
| `requests.post(0)` | arg 0 = URL | #Web.#Request |
| `httpx.get(0)` | arg 0 = URL | #Web.#Request |
| `urllib.request.urlopen(0)` | arg 0 = URL | #Web.#Request |
| `socket.connect(0)` | arg 0 = (host, port) tuple | #Web.#Socket |
| `aiohttp.ClientSession.get(0)` | arg 0 = URL | #Web.#Request |

### Rust

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `reqwest::get(0)` | arg 0 = URL | #Web.#Request |
| `TcpStream::connect(0)` | arg 0 = addr | #Web.#Socket |
| `hyper::Client::get(0)` | arg 0 = URI | #Web.#Request |

### C/C++

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `connect(_, 1)` | arg 1 = sockaddr (parse) | #Web.#Socket |
| `getaddrinfo(0, 1, _, _)` | arg 0 = host | #Web |

### JavaScript

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `fetch(0)` | arg 0 = URL | #Web.#Request |
| `http.request(0)` | arg 0 = URL/options | #Web.#Request |
| `net.connect(0)` | arg 0 = options | #Web.#Socket |
| `axios.get(0)` | arg 0 = URL | #Web.#Request |

## Database Sink Tables

### Python

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `psycopg2.connect(dsn=0)` | arg 0 or kwargs | #Database |
| `pymysql.connect(host=)` | kwargs: host, port, db | #Database |
| `sqlite3.connect(0)` | arg 0 = path | #Database (also #File) |
| `sqlalchemy.create_engine(0)` | arg 0 = connection URL | #Database |
| `motor.AsyncIOMotorClient(0)` | arg 0 = MongoDB URL | #Database |

### Rust

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `PgConnection::establish(0)` | arg 0 = URL | #Database |
| `sqlx::PgPool::connect(0)` | arg 0 = URL | #Database |
| `diesel::connection::establish(0)` | arg 0 = URL | #Database |

### C/C++

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `mysql_real_connect(_, 1, ...)` | args 1-4 = host, user, pass, db | #Database |
| `PQconnectdb(0)` | arg 0 = connection string | #Database |

### JavaScript

| Function | Resource Arg | Category |
|----------|-------------|----------|
| `new pg.Client(0)` | arg 0 = connection config | #Database |
| `mysql.createConnection(0)` | arg 0 = config | #Database |
| `mongoose.connect(0)` | arg 0 = MongoDB URL | #Database |
| `prisma.$connect()` | schema file | #Database |

## Known-Pure Functions

Builtins and standard library functions that are silently skipped during IO analysis. These produce no PGW10005 warning.

| Language | Functions |
|----------|----------|
| Python | `len`, `str`, `int`, `float`, `bool`, `list`, `dict`, `set`, `tuple`, `sorted`, `enumerate`, `range`, `zip`, `map`, `filter`, `isinstance`, `type`, `hasattr`, `print`, `format`, `repr`, `abs`, `min`, `max`, `sum`, `round`; string methods (`.upper()`, `.split()`, `.join()`, etc.); math module functions |
| Rust | `println!`, `format!`, `vec!`, `String::from`, `to_string`, `clone`, `iter`, `collect`, `map`, `filter`, `unwrap`, `expect` |
| C/C++ | `printf`, `sprintf`, `strlen`, `strcmp`, `memcpy`, `memset`, `malloc`, `free`, `sizeof` |
| JavaScript | `console.log`, `JSON.parse`, `JSON.stringify`, `parseInt`, `parseFloat`, `Array.from`, `Object.keys`, `Object.values`, `.map()`, `.filter()`, `.reduce()` |

## Deferred Categories

These categories need sink tables but are not in scope for initial implementation:

| Category | Example Sinks | Priority |
|----------|--------------|----------|
| #System.#Process | `subprocess.run`, `os.system`, `child_process.exec` | High |
| #System.#Env | `os.environ`, `process.env`, `std::env::var` | Medium |
| #System.#Shell | `os.system`, `popen`, shell pipes | High |
| #Crypto | `hashlib.*`, `ring::*`, `crypto.*` | Low |
| #IPC | `multiprocessing.*`, Unix sockets, shared memory | Low |
| #Device | Platform-specific APIs | Low |
| #Memory | `mmap`, shared memory, `ctypes.addressof` | Low |

## Package-Level Extension

Developers can declare additional IO mappings for packages the registry does not cover. Extensions are package-scoped and travel with the code:

```toml
# In package's io-registry-ext.toml
[python.file]
"custom_lib.save_report" = { arg = 0, capability = "Write" }
"custom_lib.load_data" = { arg = 0, capability = "Read" }
```

Extensions follow the same format as the built-in registry and are merged at compile time.

## Versioning and Updates

- **Ships with compiler** — each compiler release includes a registry version
- **Registry patches** — can be updated independently of compiler releases
- **Community submissions** — like DefinitelyTyped for TypeScript, community can submit registry entries for popular packages
- **Version format** — `YYYY.MM` (e.g., `2026.04`)

## Related

- [[algorithms/foreign-code-analysis]] — detection algorithm that consumes this registry
- [[compiler/ast-invisible-registry]] — companion registry for *banned* constructs (PGE10014) — functions that evade AST analysis entirely, vs this registry which maps *analyzable* IO calls to permission categories
- PGW10005 — fires when a function is not in this registry or the known-pure list
