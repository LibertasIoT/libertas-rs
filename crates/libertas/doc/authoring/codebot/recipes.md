# App recipe index

Use the canonical [Libertas App recipes](../examples/app-recipes.md) only when a task
needs a complete copyable starting shape. The language guides and the
[annotation inventory](schema_annotations.md) remain authoritative for syntax.

Choose the smallest matching recipe:

- [TypeScript Endpoint server](../examples/app-recipes.md#typescript-endpoint-server)
- [Rust Endpoint server](../examples/app-recipes.md#rust-endpoint-server)
- [TypesOnly shared package](../examples/app-recipes.md#typesonly-shared-package)
- [Function permission and reason](../examples/app-recipes.md#function-permission-and-reason)
- [Persistent data](../examples/app-recipes.md#persistent-data)

Never copy a schematic marked non-copyable. Do not invent annotations, omit
schema-visible documentation, hard-code generated `AppDependencies`, or change
an existing Union's name, variant order, discriminator, or payload meaning.
