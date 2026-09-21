# TypeScript App quick guide

The installed
[`@libertas-ai/types`](https://www.npmjs.com/package/@libertas-ai/types)
`index.d.ts` owns exact runtime APIs. The
[TypeScript manual](../reference/languages/typescript.md) owns full language behavior; use
the [annotation inventory](schema_annotations.md) for every JSDoc tag.

## Project and schema

- Use an ESM TypeScript project and Studio's selected entry module, normally
  `src/index.ts`. SDK declarations are ambient globals unless the installed
  package explicitly exports a module binding. Preserve dependency pins and run
  the project's build/typecheck or at least `tsc --noEmit`. For a new project,
  use the manual's [strict configuration baseline](../reference/languages/typescript.md#project-configuration-baseline).
- Put package label/description and package directives in leading `///`
  comments. A `TypesOnly` package publishes owned exact-name entry exports;
  `@LibertasExport` is a compiler/parser helper, not a root selector.
- Every schema-visible function and named type is exported from the entry graph.
  Native imports and qualified names must resolve in actual TypeScript scope;
  Studio does not search packages by terminal name.
- Put a declaration's natural label, description, and `[Property]` blocks
  before its first `@Libertas...` tag. `@param` text overrides inline
  parameter text. For Union variants, discriminator-member documentation wins
  over branch text, which wins over payload-type text; keep every source
  semantically identical.
- An unannotated `number` is `f64`. Every integer declares
  `@LibertasNumber(type=u8|i8|u16|i16|u32|i32|u64|i64|u128|i128)`; floats
  use `f32` or `f64`.
- Preserve stable numeric discriminators and append published Union variants
  only.

For a const Enumeration, dedicate one file to one documented exported numeric
`const` per statement and put `@LibertasConstEnum` in its file-leading
JSDoc. Import it as a namespace, then export a documented
`type Name = typeof Namespace` alias from the App entry module. Prefer an
ordinary documented numeric enum otherwise.

## Runtime

Create `FormattedText` and every human-renderable data name with
`libertasFormattedText(resourceKey, arguments)`; pass its `Uint8Array`
directly without String/Base64 conversion. Follow the installed declaration's
JSDoc for Endpoint, Matter, timer, notification, data, and logging APIs.
Every FormattedText node requires [static ReadOnly](../reference/presentation.md#static-read-only-placement-and-request-values),
including output and dormant declarations.
