# Libertas App contract

This is the common authoring contract for Rust and TypeScript Apps.

## Required design principle

**The goal is the smallest design that remains logically complete.** All App
authors, including CodeBots, must follow this principle. It is a requirement,
not an optional style preference.

Implement every required behavior, condition, and outcome, with a clear owner
for each responsibility. Use the platform's defined behavior and normal App
lifecycle instead of duplicating them with additional mechanisms. Add App logic
when the application's requirements need it; simplicity must not omit necessary
validation, permissions, consistency, or user control.

For ordinary Apps, persistence failure belongs to the Hub runtime. After storage repair
and a third-party `Enable` action, the App follows ordinary startup logic; it must not introduce a separate
storage-failure recovery system. Follow the
[Data API responsibility boundary](protocols.md#persistence-failure-and-the-no-status-api).

## Application model

A Libertas App is self-describing source code. Public functions define task
entry points. Function arguments define task configuration. Reachable public
types define configuration, persistent data, Endpoint protocols, and runtime
presentation. Studio extracts and validates those declarations and generates
the language-neutral schema and localized documents.

A package therefore has these parts: source code; the generated schema; the
localized documents; the optional `display.json` at the project root, the
version-controlled source of its presentation rules; and the compiled
[Display Template](presentation.md#display-template), which Studio builds from
the source presentation attributes and `display.json`. The template is
mandatory, but an App with neither compiles to the valid empty template. It is
not part of the schema, and changing only the template never changes the App
version. Edit `display.json` or source attributes, never the compiled template.

Do not create a second JSON schema, manually assign schema IDs, or treat
generated schema JSON as source.

The schema FQN for an owned public type is:

```text
developer-package::TypeName
```

The App identity contains exactly one hyphen. Both sides are snake_case made of
lowercase ASCII letters, digits, and single underscores; neither side may have
leading, trailing, or consecutive underscores. For example,
`acme-temperature_monitor` is valid and `acme-temperature-monitor` is not. The
developer and package identify ownership. Importing or re-exporting a type from
another App does not transfer ownership.

## Project and discovery rules

An ordinary App uses every non-ignored public/exported function as a root. Its
published type set is the minimum transitive closure needed by:

- function arguments;
- function persistent-data schemas;
- Endpoint protocol schemas;
- inherited and nested type references; and
- schema attributes that reference types, including Bitflags and compatible
  foreign types.

An unreachable public type is not published by an ordinary App.

A `TypesOnly` package publishes no executable functions. It automatically roots
every public package-owned named type exported from the language entry module,
unless the declaration is explicitly ignored. A public helper function in a
`TypesOnly` package must also be ignored. If an included type reaches an ignored
type, generation fails rather than emitting a dangling reference.

Every published named type must be publicly exported from the entry module
under its exact declaration name. Renamed root exports and duplicate flattened
names are invalid.

## Schema compilation with Codex

Studio provides the `compile_schema` tool for saved Rust and TypeScript App
source. After changing public functions, types, attributes, or dependencies,
save the files, compile the affected project, fix the reported source errors,
and repeat until validation succeeds. Edit source declarations rather than
generated schemas. Continue to run language builds and tests separately.

Supply the absolute project directory; an optional `entry` selects the entry
file. The default result contains diagnostics, source locations when known,
the schema revision, and a dependency summary. Set `includeArtifacts: true`
to receive the root schema, default document, and compiled Display Template
plus every transitive App dependency's schema, default document, Display
Template, revision, and dependency edges. Compilation validates `display.json`
with the schema; its errors are ordinary diagnostics naming `display.json` and
the one-based line when known. Each
dependency appears once. Missing, ambiguous, or cyclic App dependencies fail
compilation; a partial bundle is not a successful result.

Compilation uses saved files and installed or locally resolvable dependencies.
It does not read unsaved editor buffers, install missing dependencies, overwrite
source or generated files, or deploy. No Hub connection is required. Success
means source schema validation passed; it does not establish exact deployed
dependency-version bindings or replace Studio's deployment checks.

The tool's instructions tell Codex and Claude Code to follow this
edit-and-validate workflow. See [Studio setup](../guides/studio.md#using-studio-mcp-with-codex-or-claude-code)
for registration and an example prompt. Deployment uses the separate
[MCP deployment contract](deployment.md#mcp-deployment).

## Type model

Use the closest native type. Do not encode typed values as ad-hoc strings.

| Libertas meaning | TypeScript | Rust |
| --- | --- | --- |
| Boolean | `boolean` | `bool` |
| Signed/unsigned integer | `number` plus `@LibertasNumber(type=...)` | concrete `i*`/`u*` |
| Floating point | `number`, optionally `@LibertasNumber(type=...)` | `f32`/`f64` |
| String | `string` | `String` |
| Bytes | `Uint8Array` | `Vec<u8>` |
| List | `T[]` | `Vec<T>` |
| Nullable | optional/nullable supported type | `Option<T>` |
| Table | interface/class/object type | named-field struct |
| Enumeration | TypeScript enum | payload-free enum or enum attribute |
| Union | discriminated union | enum with variants |
| Date/time | `LibertasDateTime`, `LibertasTimeOnly` | SDK equivalents |
| Platform object | Libertas global object type | SDK object type |
| Endpoint | `LibertasEndpoint` | `LibertasEndpoint` |

`DateOnly` is an integer `yyyyMMdd` calendar tuple, not an epoch timestamp.
`TimeOnly` is seconds from midnight. `DateTime` is epoch seconds presented in
the viewer's local timezone.

### User-facing and non-user-facing types

Choose a representation for the value's meaning and its consumers. A public,
schema-reachable declaration is not necessarily user-facing: it may describe
persistent records or internal calculations. Review each occurrence of a type,
including its editability and visibility, rather than classifying it by `pub`
or `export` alone.

| Use | Design question |
| --- | --- |
| User input, including task configuration | What must the person enter or decide? Which answers may start blank, absent, or unselected? |
| User-visible output | What does the App report? How does an unavailable result differ from an empty result or zero? |
| Non-user-facing storage, calculation, or transport | Which states must the App preserve, and which distinctions affect its logic or compatibility? |

Reuse a type when these meanings and constraints agree. Use separate input and
internal types when they differ, and map between them explicitly. For example,
an editor can use blank text while a stored record uses absence. This does not
justify copying every type into a second hierarchy.

For user input, choose among these representations:

| Meaning | Recommended representation |
| --- | --- |
| Ordinary optional text, where blank means not supplied | String with `Default=""`, `SizeMin=0`, and a suitable maximum. |
| Required text | String with `SizeMin` of at least 1; omit a Default when the person must supply the answer. |
| Meaningful absence distinct from a present value | Nullable (`Option<T>` in Rust). For example, an unknown amount differs from an explicit zero. |
| Optional text that must meet a minimum length when supplied | Nullable String with the appropriate size bounds; null remains valid. |
| A decision between named alternatives, including “none” and “provide details” | A labeled Enumeration for choices without payloads, or a labeled Union when a choice carries additional input. |
| Details that only need to expand or collapse | A Table with [folding](presentation.md#folding-structs-and-unions). Folding changes presentation without changing the answer. |

Nullable means that absence is a valid answer. It does not by itself ask the
person to decide between two domain choices. A labeled Union can make a binary
decision easier to operate: its localized choices explain the alternatives,
and the selected branch exposes the relevant inputs. Use meaningful labels
such as “Kept all shares” and “Sold shares”; native `None`/`Some` or
`Nothing`/`Something` names do not explain the decision. Optional
[inline choices](presentation.md#inline-choices) show these choices as radio
controls. For internal presence or absence alone, prefer Nullable instead of
introducing a custom Union.

An `Option<String>` with `SizeMin=1` is valid: null is allowed, but a present
empty String is rejected. It therefore differs from a non-Nullable String with
`SizeMin=0`. Use the latter for ordinary optional text when that distinction
serves no purpose. Size bounds do not supply initial values. Under the
[initialization contract](schema.md#shared-copyfrom-and-copyto-rules), when no
Fixed, Schema Default, or localized default supplies a value, a String starts
unanswered even if it permits empty text, a Nullable value starts as valid null,
and a Union starts unselected. A Default selecting “none” does not establish
that the person confirmed that choice.

### Reviewing a representation change

When changing Nullable text to blank text, review presence checks, validation
rules, label fallbacks, calculations, and grouping keys. Translate `""` to
absence only where the destination uses that meaning, and translate absence
back to `""` for that editor. Do not accidentally group unrelated unnamed
records together. Keep unknown amounts distinct from zero.

Update [source comments](documentation.md#parameter-and-field) and
[AI Import declarations and instructions](llm-document-transformation.md#blank-values-and-output-types)
to match the new blank semantics. Optional imported text needs both an empty
Schema Default and explicit `allowEmpty=true`; making manual entry optional
does not enable blank import results automatically. Inspect the generated
schema and document, and validate the affected entry and import behavior.

Review published protocol and persistent-data compatibility before changing
any representation. Replacing Nullable with a String or custom Union is not
automatically compatible. Follow the
[data-type evolution contract](data-type-evolution.md); changing an editor does
not require changing an existing stored representation.

## External types and App dependencies

Resolve an external type through the native language import/module system. The
parser converts the resolved declaration to the owning App FQN. It never guesses
an unbound plain name by scanning dependencies.

The external package must itself be a Libertas App and must export the exact
type from its entry module. Ordinary npm/Cargo library types cannot become
Libertas schema references.

Use `ForeignType` only when this App publishes its own wire-compatible named
type as a direct representation of one official external type. The compatible
type retains its own FQN; it does not become the official type. The official
type supplies inherited schema/document metadata when consumers materialize
the graph, while explicit compatible attributes override same-name official
attributes.

Do not write versioned `AppDependencies` in source or edit generated locks.
Studio discovers and verifies dependencies, deploys them first, and records the
actual exact versions. The [deployment dependency contract](deployment.md)
defines discovery, Plaza preflight, installation, and failure handling. The
required `Version` member identifies one immutable version. Endpoint `Default`
recommendations do not create a dependency.

## Functions, permissions, and singleton tasks

Public functions are task templates. Their arguments are configuration values.
A function may declare:

- a persistent-data Union;
- required process permissions and same-key default-locale reasons;
- required concrete network hostnames or reserved local/external grants and
  same-key default-locale access reasons;
- semantic validation expressions, each paired with a quoted package
  `StringResources` key whose localized value is the user-facing error;
- package-owned string-resource keys used by the function; and
- `Singleton`, limiting the Hub to at most one task for the App and native
  function name across versions.

Before creating a task, Android and iOS first fail fast on a `Singleton`
conflict, then queue the requested creation while statically checking required
task-process dependencies. They traverse the complete effective input schema.
A client Endpoint carrying `Default` or `Fixed` is absolutely required when
neither it nor an ancestor is `Nullable` and no ancestor List permits zero
items. Every such Endpoint needs a currently compatible server; `Fixed` also
requires the exact named task-process identity. Missing servers open the
required-task installation UI, and the queued creation proceeds only after a
fresh complete check finds every dependency satisfied. Cancelling any required
dependency installation cancels the originating task creation. See the
[task pre-creation dependency check](schema.md#62-task-pre-creation-static-dependency-check).

Permissions grant the task process access to platform data/resources.
The client approval view always shows the raw permission key as its current
summary and the requesting App's same-key localized `StringResources` value as
a separate explanation. Different Apps can request the same permission for
different reasons, so explanations are package-owned rather than global.
TCP and UDP access is denied by default. `AccessHost` declares concrete DNS
hostnames the task process may contact, or the reserved
`{network_access_local}` / `{network_access_external}` broad grants. Every
entry is a package `StringResources` key with a non-empty explanation;
the reserved names are not exempt. A concrete hostname grants both TCP and UDP
to every address the Hub resolves for it. The Hub refreshes DNS while the task
runs and adds new answers without removing older answers until process
restart. Generate hostname-only ordinary entries—never schemes, ports, paths,
wildcards, IP literals, or whitespace around comma separators. See the public
[App network access contract](network-access.md).
The client approval view combines function permissions and network hosts. It
always shows each raw host key as the actual network identity and summary,
alongside—never instead of—its localized reason.
`AccessPrivilege` is different: it limits which endpoint users may invoke one
runtime Request or SubscriptionRequest.

## Persistence responsibility

By default, Data API storage failures require the Hub runtime to stop the task rather than
returning a recoverable App status. See the
[persistence responsibility and restart rationale](protocols.md#persistence-failure-and-the-no-status-api).


A function may opt into checked Data API results using `DataErrorStatus`; see the
[Data API owner](protocols.md#explicit-error-status-opt-in). This exception transfers
consistency handling to that App without adding Hub-automatic restart.

## Task version selection

A task runs the App version selected for it. Installing or upgrading an App
never changes any task's selection. A person with configuration permission on
the task changes it through the task's settings; selecting a version of
another App retargets the task and also requires configuration permission on
that App, as task creation does. The Hub then replaces the task's runtime with
the selected version, keeping its arguments and enabled state. When the
arguments cannot be initialized under the selected version, the change fails
and the task keeps its version. Which version reads the data another wrote
follows the [directional evolution rules](data-type-evolution.md).

## Task removal

On Hubs supporting removal preflight, Android and iOS ask the Hub to identify
tasks affected by removing a task's virtual devices, Endpoint servers, or
on/off device. This includes indirect effects when configuration cleanup
removes another task's outputs. Disabled tasks are included.

The user needs removal permission on the selected task and effective
configuration permission on every affected dependent task. Dependencies alone
produce a localized warning listing the affected tasks, with a choice to
cancel or continue. Missing configuration permission produces a localized
failure identifying the tasks that need permission; removal cannot continue.
The Hub's object-removal handling performs the actual configuration cleanup.

Removing a task removes its persistent data. The confirmation lists every
affected task together with the effect on it. An argument of another task that
bound an Endpoint of the removed task is set to null. The Hub then validates
that task's arguments: when they are still valid, the task is restarted
automatically with the new arguments; otherwise it is disabled with the new
arguments.

The Hub repeats the same analysis immediately before executing removal. An
earlier successful probe cannot bypass a later permission change. When the
repeated check fails, including when the Hub cannot inspect the complete task
configuration, the Hub reports the failure and does not begin removal.

Removal is atomic. The Hub applies it in the order of the dependency map: the
removed task and its data, then each dependent task's nulled arguments and its
restart or disabling. Either the whole removal takes effect or none of it does;
no person or task observes a partial state, and a removal interrupted by a
power loss is completed before the Hub resumes ordinary operation. Older Hubs retain their supported removal operation;
the clients warn that dependency and permission preflight requires a Hub
update.

## Documentation and localization

Source documentation creates the default-locale package document:

- the first documentation line is the display name;
- remaining ordinary lines are the description;
- custom properties use `[PropertyName]` followed by their value;
- `[DefaultText]` is one-line localized default text and belongs only to a
  `String` field or an `Endpoint` server field; unlike the schema `Default`
  value, it may differ by locale;
- `[DefaultTaskName]` is a localizable default task name and belongs only to a
  function document;
- persisted or user-assigned text, including an empty string, is immutable to
  programmatic defaulting; these defaults apply only while creating a value
  for which no persisted value exists;
- list documentation may contain a line of at least four hyphens; content above
  the divider describes the list and content below describes each item; and
- the array divider must appear before custom document properties belonging to
  the list item.

Plain code comments are not localized schema documentation.

Every authored package, function, named type, parameter, field, List container
and item, Enumeration member, Union variant, and inline value must have a
non-empty natural label and description that make sense without source names or
implementation vocabulary. Custom document values and package string resources
follow the same user-language and accessibility rules. The complete normative
contract is the
[schema documentation invariants](documentation.md).

Every public function comment must additionally satisfy the
[developer function-comment invariants](function-comments.md). Its
description is a complete, actor-explicit account of the function's purpose,
user choices and actions, automatic behavior, outputs, and material limits; it
must not rely on generated type text to fill semantic gaps.

String-resource keys are stable identifiers and are never translated. Values
are translated as LMF1: zero-based argument positions, types (including
Libertas types), styles, selectors, plural offsets, braces, and apostrophe
escapes remain structurally unchanged in every locale. LMF1 is not printf, and
the `LibertasEnum` style remains the actual Enumeration type name.

Every single-record name and indexed-database/table name is one canonical
`DataName` byte blob encoding a package resource key plus ordered typed
Libertas Message Arguments. The blob is both the locale-independent database
identity and the input used to resolve and render its localized human name. A
rendered or Base64 String is never the identity. See the
[data and database name contract](protocols.md#data-and-database-names).

## Wire and compatibility discipline

- Protocol and persistent-data schema versions must remain backward compatible.
  Keep each such Union's native name versionless and invariant for the App's
  lifetime; a new App version appends new payload variants to that same Union
  instead of declaring `ProtocolV2` or `StoredDataV2`. See the
  [App data-type design guidelines](data-type-evolution.md).
- Attributes such as `Format`, `PhysicalUnit`, `DateOnly`, `Bitflags`,
  `ValidationRules`, chart metadata, `ReadOnly`, `Hidden`, and `ExcludeUI` affect
  validation/presentation without changing the underlying wire type.
  `ExcludeUI` is specifically a Control-plane projection: the Hub omits a
  marked server Endpoint from its Control UI-device list and generated clients
  omit marked request operations. It never removes nodes from admin creation,
  editing, approval, persistence, routing, or authorization. See the
  [Endpoint declaration contract](protocols.md#declaring-an-endpoint).
- Union declaration order is significant. `variant_index!` uses declaration
  order and append-only protocol/data Union evolution depends on it.
- Append new variants for compatible Union evolution; do not insert, remove,
  reorder, or change existing payload shapes in a published append-only line.
- A protocol may declare conformance separately from its optional variants.
  Configuration checks mandatory client/server compatibility. A client uses
  runtime variant indices to decide whether the selected server supports an
  optional request before sending it.
- The common schema/dependency lock and each existing target's exact App version,
  binary, language, TypesOnly state, and distribution dependencies are immutable
  within one Plaza release. Changing any of them
  creates a new content-derived release; adding or removing targets and changing
  documents or allowed manifest metadata synchronizes the current release. App
  source never chooses the release tag or public version. See the
  [Plaza publication contract](publication.md#app-publication-and-version-identity).

## Completion checklist

- The project is a valid language library/module and typechecks.
- All published types are exact-name root exports.
- Every external plain name resolves through native imports.
- Function roots and `TypesOnly` roots are intentional.
- Every `FormattedText` use satisfies the [static read-only placement and
  request-value contract](presentation.md#static-read-only-placement-and-request-values).
- Mutating endpoint operations explicitly require `Write` access.
- Permission reasons and string resources are complete.
- Defaults, fixed values, bounds, bit masks, and dates satisfy their host types.
- Schema-visible Union order and wire-shape changes are intentional.
- No generated schema/document JSON was edited manually.
