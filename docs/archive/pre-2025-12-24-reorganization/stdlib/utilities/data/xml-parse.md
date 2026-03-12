---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "xml-parse"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.XML.Parse"
summary: "API reference: |U.Data.XML.Parse"
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: low

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# |U.Data.XML.Parse

**Parse XML string**

**Category:** Utilities > Data > XML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.XML.Parse <xml >result
```

**Inline:**
```polyglot
\|U.Data.XML.Parse"{$xml}"
```

---

## Parameters

**Inputs:**
- `<xml` :pg.string - XML string to parse

**Outputs:**
- `>result` :pg.serial - Parsed XML data

---

## Description

Parses an XML-formatted string into a `:pg.serial` structure.

**Produces error:**
- `!Data.XML.ParseError` - Invalid XML syntax

---

## XML to Serial Mapping

**Element → Serial field:**
```xml
<server>
  <host>localhost</host>
  <port>8080</port>
</server>
```

Maps to:
```polyglot
$data."server.host" = "localhost"
$data."server.port" = "8080"
```

**Attributes → Field with `@` prefix:**
```xml
<server host="localhost" port="8080"/>
```

Maps to:
```polyglot
$data."server.@host" = "localhost"
$data."server.@port" = "8080"
```

**Text content → Field named `#text`:**
```xml
<message>Hello World</message>
```

Maps to:
```polyglot
$data."message.#text" = "Hello World"
```

**Multiple elements → Array:**
```xml
<users>
  <user>Alice</user>
  <user>Bob</user>
</users>
```

Maps to:
```polyglot
$data."users.user" :pg.array.pg.string = ["Alice", "Bob"]
```

---

## Examples

### Basic Usage

```polyglot
[r] $xml_string :pg.string << "<person><name>Alice</name><age>30</age></person>"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$xml_string}"
```

**Output:** Serial with `person.name="Alice"`, `person.age="30"`

---

### Parse with Attributes

```polyglot
[r] $xml :pg.string << "<server host=\\\"localhost\\\" port=\\\"8080\\\"/>"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$xml}"
[r] $host :pg.string << $data."server.@host"
[r] $port :pg.string << $data."server.@port"
```

---

### Parse Nested Elements

```polyglot
[r] $xml :pg.string << "<config><database><host>localhost</host><port>5432</port></database></config>"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$xml}"
[r] $db_host :pg.string << $data."config.database.host.#text"
[r] $db_port :pg.string << $data."config.database.port.#text"
```

---

### Handle Parse Errors

```polyglot
[r] $data :pg.serial << \|U.Data.XML.Parse"{$untrusted_xml}"

[!] !Data.XML.ParseError
   [r] !Validation.InvalidXML << "Invalid XML format in input"
```

---

### Parse Array of Elements

```polyglot
[r] $xml :pg.string << "<users><user><name>Alice</name></user><user><name>Bob</name></user></users>"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$xml}"
[r] $users :pg.array.pg.serial << $data."users.user"

[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user
   [r] $name :pg.string << $user."name.#text"
   // Process each user...
```

---

## Common Patterns

### Pattern 1: Parse SOAP Response

```polyglot
[r] $soap_response :pg.string << $api_response."body"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$soap_response}"

[!] !Data.XML.ParseError
   [r] !API.InvalidResponse << "Invalid SOAP XML"
   [v] [r] [^]

[r] $result :pg.string << $data."soap:Envelope.soap:Body.Response.Result.#text"
```

### Pattern 2: Parse RSS Feed

```polyglot
[r] $rss_xml :pg.string << $feed_content
[r] $data :pg.serial << \|U.Data.XML.Parse"{$rss_xml}"

[r] $items :pg.array.pg.serial << $data."rss.channel.item"

[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $title :pg.string << $item."title.#text"
   [r] $link :pg.string << $item."link.#text"
   [r] $description :pg.string << $item."description.#text"
   // Process feed item...
```

### Pattern 3: Parse SVG

```polyglot
[r] $svg_xml :pg.string << $svg_content
[r] $data :pg.serial << \|U.Data.XML.Parse"{$svg_xml}"

[r] $width :pg.string << $data."svg.@width"
[r] $height :pg.string << $data."svg.@height"
[r] $viewbox :pg.string << $data."svg.@viewBox"
```

### Pattern 4: Parse Configuration with Mixed Content

```polyglot
[r] $xml :pg.string << "<config><setting name=\\\"timeout\\\">30</setting><setting name=\\\"retries\\\">3</setting></config>"
[r] $data :pg.serial << \|U.Data.XML.Parse"{$xml}"

[r] $settings :pg.array.pg.serial << $data."config.setting"

[r] ~ForEach.Array
[~] <array << $settings
[~] >item >> $setting
   [r] $name :pg.string << $setting."@name"
   [r] $value :pg.string << $setting."#text"

   [f] $name == "timeout"
      [r] $timeout :pg.int << \|U.Math.Add"{$value, 0}"  // Convert string to int
```

---

## XML Features Supported

**Elements:**
- Nested elements
- Empty elements (`<tag/>`)
- Elements with text content
- Multiple elements with same name (arrays)

**Attributes:**
- Element attributes
- Mapped to fields with `@` prefix

**Special handling:**
- Text content mapped to `#text` field
- Namespaces preserved in element names
- CDATA sections treated as text
- Comments ignored

**Not supported:**
- Processing instructions (ignored)
- DOCTYPE declarations (must be removed)
- Entity references beyond standard XML entities (`&lt;`, `&gt;`, `&amp;`, `&quot;`, `&apos;`)

---

## Field Access Patterns

**Simple element:**
```polyglot
$data."root.element.#text"
```

**Attribute:**
```polyglot
$data."root.element.@attribute"
```

**Nested path:**
```polyglot
$data."root.level1.level2.#text"
```

**Array of elements:**
```polyglot
$data."root.items.item"  // Returns :pg.array.pg.serial
```

---

## Related Pipelines

- [|U.Data.JSON.Parse](./json-parse.md) - Parse JSON string
- [|U.Data.YAML.Parse](./yaml-parse.md) - Parse YAML string
- [|U.String.Replace](../../string/replace.md) - For XML preprocessing

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
