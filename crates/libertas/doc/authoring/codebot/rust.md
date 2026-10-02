# Rust App quick guide

The installed public
[`libertas-rs`](https://github.com/LibertasIoT/libertas-rs) rustdoc owns exact
runtime APIs. The [Rust manual](../reference/languages/rust.md) owns full language
behavior; use the [annotation inventory](schema_annotations.md) for every
schema macro.

## Project and schema

- A Rust App is a library rooted at `src/lib.rs`, normally with
  `crate-type = ["rlib"]`. Never add `src/main.rs`. Preserve existing
  dependency pins and run `cargo check --lib`.
- Put the package label/description and package directives in leading
  `//!` rustdoc. `libertas_default_locale`, `libertas_types_only`, and
  `libertas_string_resources` use the exact rustdoc forms in the inventory.
  `libertas_ignored` is likewise an exact outer-rustdoc line.
- Public App functions use `#[libertas_export]`. Schema structs/enums are
  public, derive `LibertasExport` and `LibertasAvroDecode`, and add
  `LibertasAvroEncode` when runtime code encodes them.
- Document a typed function parameter directly on that parameter. Put its
  natural label/description before custom properties or parser-directive
  rustdoc.
- Named structs use named fields. `Option<T>` is nullable, `Box<T>` keeps
  the inner schema, `Vec<u8>` is Bytes, and other `Vec<T>` values are Lists.
  Native numeric types determine wire width; `libertas_number` supplies only
  constraints.
- Native `#![no_std]` is supported only when the App and every dependency
  satisfy the ordinary Rust `no_std` contract.

Rust enum declaration order is its wire index. Published Unions append variants
only. Use public `VariantIndex` / `variant_index!` when code needs a
compile-time variant index; do not construct a payload merely to discover it.

For ordinary optional input text, use `String`, `#[libertas_default("")]`, and
`#[libertas_size(min = 0)]` with an appropriate maximum. Keep `Option<T>` for meaningful absence;
use labeled enum choices for user decisions. See the
[worked type examples](../reference/languages/rust.md#optional-types).
For AI-imported optional text, also declare
`#[libertas_llm_doc_field(allow_empty = true)]` and instruct the model to return
`""` when absent; the bare String marker rejects blanks. Follow the
[AI Import example and contract](../reference/languages/rust.md#general-field-attributes).

## Runtime and build

Create `FormattedText` and every human-renderable data name with
`libertas_formatted_text(resource_key, arguments)`; pass its `Vec<u8>`
directly without String/Base64 conversion. Use SDK APIs and types exactly as
documented, especially Endpoint and Matter callbacks, ownership, and lifetimes.
Every FormattedText node requires [static ReadOnly](../reference/presentation.md#static-read-only-placement-and-request-values),
including output and dormant declarations.

Declare native compiler packages only with supported
[Cargo metadata](../reference/languages/rust.md#declaring-native-build-packages);
runtime shared-library packages are derived from the built artifact. A new
Matter App uses the manual's
[Matter SDK dependency](../reference/languages/rust.md#matter-sdk-dependency).
