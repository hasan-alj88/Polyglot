# PGE04030: Undefined Field Path

code: PGE04030
description: Field path references must resolve to a valid field on the referenced type schema.
category: Wrappers and External Connections

## Description

**Statement:** Every `$var.field` path reference must resolve to a valid field on the referenced type's schema. Accessing a field that is not structurally declared on the target object raises `PGE04030`.

## Rationale
Enforces schema rigor at compile time, guaranteeing that data manipulations apply strictly to known structure dimensions.
