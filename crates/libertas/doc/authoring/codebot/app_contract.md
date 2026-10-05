# Libertas App contract

This is the compact codebot projection of the full
[App authoring contract](../reference/app-contract.md). Follow the linked
contract for edge cases; never infer schema behavior from this summary.

## Required design principle

Apps must use the smallest design that remains logically complete. Preserve all
required behavior and give each responsibility a clear owner; reuse platform
contracts and normal startup instead of inventing duplicate mechanisms. This
is mandatory for CodeBot-authored Apps. See the
[design requirement](../reference/app-contract.md#required-design-principle).

After saving schema-affecting source changes, use Studio’s `compile_schema`
tool when available and follow the [compiler workflow and result contract](../reference/app-contract.md#schema-compilation-with-codex).
Deploy only when the user requests it. The MCP deployment tools require an
exact Hub choice and report the result through a job; follow the
[MCP deployment contract](../reference/deployment.md#mcp-deployment).

## AI access in every design step

Consider AI document import and AI Search at every design stage (types,
pages, rules, messages, tests), not after the design is done
([owner](../reference/app-contract.md#ai-access-in-every-design-step)):

- A page whose values come from a document declares
  [document import](../reference/llm-document-transformation.md); every field
  printed on that document is an import target, also inside conditional
  sections. Values only the person's own records hold are not; their
  descriptions say where to find them.
- [AI Search](../reference/protocols.md#ai-navigation-and-ai-search) matches
  the destination a link's Request carries, usually an Enumeration field filled
  by `CopyFrom` from the row or response. Document every destination member's
  name and description with the identifiers and common names people type or
  say, and bind it through `CopyFrom`; extend `AINavigationPrompt` when needed.

## Application model

- Public functions are task entry points; their arguments are configuration.
- Source declarations generate schema and localized documents. Never edit
  generated JSON or assign schema IDs.
- An owned identity is `developer-package::Name`; importing or re-exporting a
  foreign type never transfers ownership.

## Project and discovery rules

- An ordinary App publishes each non-ignored public function and only the
  reachable type closure.
- A `TypesOnly` package has no executable functions and publishes every
  non-ignored, package-owned named type exported from the entry module.
- Every published named type is an exact-name public root export. An included
  declaration may not reach an ignored type.

## Type model

Use the closest native Libertas type; never replace typed objects, dates,
bytes, Enumerations, Unions, or Endpoints with ad-hoc Strings. TypeScript
numbers need explicit integer/float width when `f64` is not intended; Rust
number width comes from its native type. `DateOnly` is integer `yyyyMMdd`,
`TimeOnly` is seconds after midnight, and `DateTime` is epoch seconds.

Choose types by occurrence: user input, displayed output, or internal data;
public/exported does not mean editable. Ordinary optional text uses String with
empty Default and minimum zero; meaningful absence uses Nullable. Labeled
Enumerations/Unions make user decisions explicit, including binary choices;
use folding for presentation alone. Follow the
[type-selection and change-review guidance](../reference/app-contract.md#user-facing-and-non-user-facing-types).

## External types and App dependencies

External declarations resolve through native imports/modules to the owning App
FQN and must be exact-name exports of a Libertas App. `ForeignType` declares a
published local compatible type for one direct official type; it does not
transfer identity. Never author `AppDependencies`: Studio materializes and
versions them. Endpoint `Fixed` contributes a package dependency; `Default`
does not.

## Functions, permissions, and singleton tasks

- `Singleton` allows at most one task for the App and native function name.
- Every function permission and `AccessHost` key needs its exact package-owned,
  nonempty localized reason. Clients show that reason beside, never instead of,
  the raw permission or host key.
- Every App function whose task process performs local or external TCP/UDP
  access, directly or through a dependency, must declare a valid `AccessHost`
  covering every required DNS hostname or an intentionally chosen reserved
  network grant. The Hub blocks undeclared network access: omitting
  `AccessHost` leaves that function with no local or external TCP/UDP access.
  Follow the [network-access contract](../reference/network-access.md).
- `AccessPrivilege` is per protocol input and is not a task-process permission.
- A client Endpoint with `Default` or `Fixed` can make its named peer task a
  creation prerequisite when that Endpoint occurrence is not optional. See the
  [canonical dependency check](../reference/schema.md#62-task-pre-creation-static-dependency-check).

## Task versions and removal

A task runs the App version selected for it: installing or upgrading an App
never changes a task's selection, so several versions can run at once; which
version reads data another wrote follows the
[directional evolution rules](../reference/data-type-evolution.md). A version
change keeps the task's arguments and enabled state, and fails, keeping the
old version, when the arguments cannot initialize under the new one
([selection](../reference/app-contract.md#task-version-selection)). Removing a
task removes its data and sets to null every other task's argument bound to
one of its Endpoints; each such task restarts when its arguments stay valid
and is otherwise disabled, atomically
([removal](../reference/app-contract.md#task-removal)).

## Documentation and localization

Give every authored package, function, type, parameter, field, List container
and item, Enumeration member, Union variant, and inline value a natural label
plus complete description. Follow the
[schema documentation invariants](documentation_invariants.md) and the separate
[function-comment invariants](function_comment_invariants.md).

`[DefaultText]` is localized initial text for a String or server Endpoint;
`[DefaultTaskName]` is a localized initial task name on a function. They apply
only when no persisted value exists; an empty saved value remains saved.
Codebots must provide `DefaultText` for every authored server Endpoint and
`DefaultTaskName` for every authored App function so people receive a sensible,
editable name instead of having to name each new endpoint or task themselves.
Do not put `DefaultText` on a function; `DefaultTaskName` is its supported
property.

`StringResources` is one package-owned catalog. Preserve LMF1 structure in
translation. `FormattedText` and `DataName` are canonical byte blobs, never
literal or Base64 App Strings; see [LMF1](../reference/message-format.md).

For Data API failure handling, follow the compact
[persistence rule](protocols_runtime.md#persistent-data-schemas).

## Wire and compatibility discipline

- Keep protocol and persistent-data Union names versionless and invariant.
- Append variants only at the end; never remove, reorder, or mutate published
  cases. Follow the [data-type design contract](data_type_design.md).
- Presentation metadata does not change the wire type.
- Check optional protocol capability before sending it.

## Completion checklist

Typecheck the library, validate the generated schema, inspect its default
document, confirm each document-sourced page declares import and each
destination member is documented for AI Search, confirm exact-name exports and intentional wire changes, and report
permissions/dependencies. Never hand-edit generated schema, documents,
dependency lists, manifests, or release versions.
