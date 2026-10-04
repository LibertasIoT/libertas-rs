# Data-type evolution quick rules

Choose initial representations under the shared
[user-facing and non-user-facing type guidance](../reference/app-contract.md#user-facing-and-non-user-facing-types).

Protocol and persistent-data Unions are published wire contracts. Keep each
Union's native name versionless and unchanged for the App's lifetime. Append
new variants only; never reorder, remove, renumber, rename, reinterpret, or
change the payload of an existing variant. Preserve TypeScript discriminator
values and Rust declaration order.

Nested Tables and Unions must evolve compatibly as well. Treat a later variant
as optional capability and check peer support before sending it. Peers may
group a capability at different Union depths; only the
[capability prefix](../reference/protocols.md#capability-prefixes) is
translated, so nested optional variants inside a payload still only extend the
tail. Installing a new version never changes the version a task runs, so
older and newer versions coexist; which one reads data the other wrote
follows the linked directional rules
([task version selection](../reference/app-contract.md#task-version-selection)).

The complete rationale, language schematics, and review checklist are in the
[Libertas App data-type design guidelines](../reference/data-type-evolution.md).
