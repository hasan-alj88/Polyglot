# Example 3: URL Parser

**Use Case:** Parse URLs in `"protocol://host:port/path"` format

## Complete Implementation

```polyglot
{|} |ParseURL
[t] |T.Call
[<] <formatted_string:pg.string
[>] >protocol :pg.string
[>] >host :pg.string
[>] >port :pg.int
[>] >path :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.url:pg.string << $formatted_string
(|) >protocol :pg.string >> >protocol
(|) >host :pg.string >> >host
(|) >port :pg.int >> >port
(|) >path :pg.string >> >path
(|) <code:pg.string << |U.String.Python""
[+] +"from urllib.parse import urlparse"
[+] -"def parse(url:str)->dict:"
[+] -"   parsed = urlparse(url)"
[+] -"   return dict("
[+] -"       protocol=parsed.scheme or 'http',"
[+] -"       host=parsed.hostname or 'localhost',"
[+] -"       port=parsed.port or 80,"
[+] -"       path=parsed.path or '/'"
[+] -"   )"
{x}


{|} |FetchFromURL
[%] %Pipeline.Inline
   [%] |ParseURL
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >protocol :pg.string >> <protocol
   (|) >host :pg.string >> <host
   (|) >port :pg.int >> <port
   (|) >path :pg.string >> <path

[<] <protocol :pg.string
[<] <host :pg.string
[<] <port :pg.int
[<] <path :pg.string
[>] >content :pg.string

// Construct full URL and fetch
[r] $full_url << |U.String.Concat
(|) <parts :pg.string[] << [$protocol, "://", $host, ":", $port, $path]

[r] |HTTP.GET
(|) <url :pg.string << $full_url
(|) >response >> >content
{x}
```

## Usage

```polyglot
// Full URL
[r] |FetchFromURL "https://api.example.com:443/users"
(|) >content >> $users

// Minimal URL (defaults applied)
[r] |FetchFromURL "example.com/data"
// → protocol="http", host="example.com", port=80, path="/data"
```

---
