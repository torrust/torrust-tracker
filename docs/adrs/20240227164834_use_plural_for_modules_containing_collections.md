# Use plural for modules containing collections of types

## Description

In Rust, the naming conventions for module names (mod names) generally lean
towards using the singular form, rather than plurals. This practice aligns with
Rust's emphasis on clarity and precision in code organization. The idea is that
a module name should represent a single concept or functionality, which often
means using a singular noun to describe what the module contains or does.

However, it's important to note that conventions can vary depending on the
context or the specific project. Some projects may choose to use plural forms
for module names if they feel it more accurately represents the contents of the
module. For example, a module that contains multiple implementations of a
similar concept or utility functions related to a specific theme might be named
in the plural to reflect the diversity of its contents.

This could have some pros anc cons. For example, for a module containing types of
requests you could refer to a concrete request with `request::Announce` or
`requests::Announce`. If you read a code line `request::Announce` is probably
better. However, if you read the filed or folder name `requests`gives you a
better idea of what the modules contains.

## Agreement

We agree on use plural in cases where the modules contain some types with the
same type of responsibility. For example:

- `src/servers`.
- `src/servers/http/v1/requests`.
- `src/servers/http/v1/responses`.
- `src/servers/http/v1/services`.
- Etcetera.

We will change them progressively.
