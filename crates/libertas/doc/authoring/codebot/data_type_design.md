# Data-type evolution quick rules

Protocol and persistent-data Unions are published wire contracts. Keep each
Union's native name versionless and unchanged for the App's lifetime. Append
new variants only; never reorder, remove, renumber, rename, reinterpret, or
change the payload of an existing variant. Preserve TypeScript discriminator
values and Rust declaration order.

Nested Tables and Unions must evolve compatibly as well. Treat a later variant
as optional capability and check peer support before sending it.

The complete rationale, language schematics, and review checklist are in the
[Libertas App data-type design guidelines](../reference/data-type-evolution.md).
