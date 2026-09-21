# Libertas App-development kit for codebots

These files are compact App-source authoring cards. They intentionally omit
Studio, Hub, client, parser, database, renderer, transport, and Plaza
implementation algorithms. Follow a linked shared reference when an edge
case needs detail.

## Read only what the task needs

Always read:

1. [App contract](app_contract.md).
2. [Documentation rules](documentation_invariants.md) for any schema-visible
   declaration, plus [function comments](function_comment_invariants.md) for a
   public function.
3. [Annotation inventory](schema_annotations.md) before using schema syntax.
4. Exactly one language card: [Rust](rust.md) or
   [TypeScript](typescript.md).

Then open only the applicable card:

- [Protocols/runtime](protocols_runtime.md), [presentation/charts](presentation.md),
  [ValidationRules](validation_rules.md), [Matter](matter.md), or
  [data-type evolution](data_type_design.md);
- [recipes](recipes.md) when a complete starting shape is needed; and
- [delivery](delivery.md) for validation, deployment, translation, or Plaza.

## Authority

The installed public SDK owns runtime APIs. A language parser owns native
source syntax/resolution/extraction. Studio's shared validator owns portable
schema semantics. Shared references own full explanations. These cards
are concise projections; never infer missing behavior from examples.

## Non-negotiable rules

- For App-authoring tasks, edit source and ordinary project metadata such as
  `Cargo.toml` and `package.json`. Do not hand-edit generated schema, localized
  document JSON, dependency locks, target/release manifests, or release versions.
  Translation tasks are the explicit exception: write only their designated
  handoff output under the [translation contract](../reference/translation.md).
- Preserve the App's source language unless the user explicitly requests a
  migration.
- Use only listed public SDK types and annotations; never derive a source
  spelling from emitted schema or a private implementation.
- Rust Apps are libraries rooted at `src/lib.rs`; TypeScript starts at
  Studio's selected ESM entry module.
- Preserve typed Libertas values. Do not replace objects, Enumerations, Bytes,
  dates, protocols, or formatted values with ad-hoc Strings.
- Keep published protocol/data Union names and existing cases unchanged;
  append compatible cases only.
- Document every authored package, function, type, parameter/field, List
  container/item, Enumeration member, Union variant, and inline value in
  natural user language.
- Keep function discovery text complete, actor-explicit, truthful, and
  correctly attributed.
- Let Studio resolve external dependencies and report source-located errors;
  fix the owning source rather than weakening types or patching output.
- If an owning public contract is missing, report that gap instead of guessing
  from private implementation.
- Run the native checker and Studio validation before handoff.

Report changed files/functions/contracts, new permissions/hosts/dependencies,
wire/order impact, checks run, and remaining deployment/publication work.


For authority and missing-contract handling, read the [source policy](sources.md).
For guided learning, start at the [human homepage](../index.md).
