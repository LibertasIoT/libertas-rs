# Libertas Rust Developer's Manual

This manual provides a quick reference for writing Rust code for the Libertas Platform. Libertas is fundamentally about **self-describing software**. By writing standard Rust, Libertas automatically extracts your code's documentation, structure, and attributes to understand its capabilities. A robust user interface is just one of the many great features automatically generated from this self-describing architecture, alongside API layers and Agent integrations.

## Contents

- [Project contract and library requirements](#project-contract-and-library-requirements)
- [Documentation and localization](#documentation-and-localization)
- [Libertas type system](#libertas-type-system)
  - [Optional types](#optional-types)
  - [Labeled decisions and internal absence](#labeled-decisions-and-internal-absence)
- [Schema attributes and field presentation](#schema-attributes-and-field-presentation)
- [Functions, persistent data, and resources](#functions-persistent-data-and-resources)
- [Endpoint protocols and presentation](#endpoint-protocols-and-presentation)
- [Publishing and interoperability](#publishing-and-interoperability)
- [Build and deployment metadata](#build-and-deployment-metadata)
- [Parser attribute reference](#parser-attribute-reference)

## Project contract and library requirements

Libertas relies exclusively on information extracted directly from your Rust source code. You do not need to configure any external JSON schemas or metadata files.

For schema extraction, Rust attribute syntax, native resolution, traversal, and
emission, the Libertas Rust parser is authoritative. Libertas Studio's shared
schema validator is authoritative for semantic rules decidable from the emitted
parser result and its materialized dependencies. This manual is a maintained
usage summary and must remain aligned with those owners.

For runtime SDK behavior that is not part of schema extraction, follow the
public rustdoc comments in
[`crates/libertas/src/lib.rs`](https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/src/lib.rs)
and the runtime implementation. In particular, the `LibertasEndpoint` rustdoc
is normative for endpoint subscription and signaling behavior.

The 64-bit SDK (0.3.0) represents device, endpoint, LAN-device, user, task,
and peer identities with `u64` aliases. Allocated IDs are positive and no greater
than `i64::MAX`; retain peer handles unchanged. Transaction and timer IDs remain
32-bit. Rebuild Apps with this SDK for the matching Hub runtime; existing task
records can be retained while tasks are disabled for the update.

Core application requirements:

1. **Global types**: Use the `libertas` crate to access native platform types.
2. **Standard Rust**: Your library must compile successfully (`cargo check --lib` should pass) before the UI schemas can be generated.
3. **Library application**: A Libertas application must be a Cargo library target rooted at `src/lib.rs`. A binary target rooted at `src/main.rs` is not a Libertas application and is not accepted for schema generation or deployment.
4. **Entry point**: Schema scanning always starts from the application library root, `src/lib.rs`.
5. **Cross-file support**: You can freely organize your library using modules and `use` statements. Libertas automatically tracks references across files and dependency crates.
6. **Host-side generation**: Libertas Studio generates the schema on the local VS Code extension host. Prebuilt generators are included for Linux, macOS, and Windows on both x64 and ARM64; the Hub does not generate the development schema.

### Matter SDK dependency

Add the Matter SDK only when the App uses Matter, preserving the project's
existing branch, tag, or revision policy:

```toml
[dependencies]
libertas-matter = { git = "https://github.com/LibertasIoT/libertas-matter.git", default-features = false, features = ["alloc"] }
```

Do not silently change an existing dependency pin while making an unrelated
App change. Exact runtime APIs and feature availability come from the selected
SDK revision.

Schema discovery and Rust shape rules:

- Every public function is emitted in `Functions` unless its rustdoc contains
  the exact no-argument `/// #[libertas_ignored]` parser directive.
  `#[libertas_export]` is the SDK's function wrapper for consuming parser-only
  argument attributes during Rust compilation; it is not what makes a public
  function visible to the schema parser.
- In an ordinary App, those functions are the only roots. The parser publishes
  the minimum transitive set of non-builtin types reached from function
  arguments and schema-reference attributes. Derives and schema attributes on
  an otherwise unreachable type do not publish it.
- In a `TypesOnly` App, every public package-owned named type exported from the
  root `lib.rs` is automatically a root unless it carries the no-argument
  `/// #[libertas_ignored]` directive. If an included graph reaches an ignored
  type, parsing fails instead of emitting a dangling reference.
- A schema struct or enum must be public and must derive
  `LibertasAvroDecode`. The normal declaration derives both `LibertasExport`
  and `LibertasAvroDecode`: the former makes parser-only helper attributes
  legal Rust syntax, while the latter is explicitly required by the parser.
  Derives may use imported names or the canonical `libertas::` and
  `libertas_macros::` paths, with an optional leading `::` on qualified paths.
  For example, `#[derive(libertas_macros::LibertasExport,
  libertas_macros::LibertasAvroDecode)]` needs no derive-name imports.
  An unrelated crate's identically named derive does not satisfy this requirement.
- Structs must have named fields. A tuple struct is rejected. In a Union enum,
  a tuple variant may contain exactly one payload; a named variant becomes an
  inline `Table`, and a unit variant becomes `Nil`.
- Published type aliases must be public. `Option<T>` adds `Nullable=true`,
  `Box<T>` preserves the inner schema shape, `Vec<u8>` is `Bytes`, and other
  `Vec<T>` values are `List` with one item schema.
- Every named type published by the App must be publicly defined or re-exported
  from the root `lib.rs` under its exact declaration name.

Resolution starts in the local project. Each type reference follows the
imports and module declarations in the scope where it is written; a dependency
is explored only when a native path names that crate. Root exports do not
implicitly import names into child modules. An import alias may be used inside
a module, but the App root must still export the referenced declaration under
its original name. An unrelated declaration with the same name cannot satisfy
that requirement.

For example, `src/lib.rs` can publish types from separate modules:

```rust
mod model;
mod protocol;
pub use model::Answer;
pub use protocol::Request;

pub fn configure(value: Request) {}
```

`src/model.rs` defines the answer:

```rust
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Answer {
    pub accepted: bool,
}
```

`src/protocol.rs` explicitly imports the root export:

```rust
use crate::Answer;

#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Request {
    pub answer: Answer,
}
```

The same scope rules apply to nested modules, `self::` and `super::` paths,
explicit module `#[path]` declarations, and re-export chains. Missing bindings,
ambiguous imports, and cyclic re-exports that resolve to no declaration are
errors. A public declaration inside a module alone is not a root export.

```rust
/// Connection retry choices
/// Describes how a person names a connection and limits repeated attempts.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    /// Connection name
    /// The person enters the name used to recognize this connection.
    pub name: String,

    /// Retry limit
    /// The person may limit repeated connection attempts or leave the limit
    /// unset to use the service's normal retry behavior.
    pub retries: Option<u16>,
}
```

## Documentation and localization

Libertas uses specific comment styles to extract localized documentation for your components, ensuring regular code comments don't accidentally leak into the user interface. See [Libertas type system](#libertas-type-system) for more details.

Every authored schema-visible package, function, named type, parameter, field,
Enumeration member, Union variant, and both halves of a List must follow the
[schema documentation invariants](../documentation.md):
provide a non-empty natural label and description, explain meaning in ordinary
domain language, and keep implementation vocabulary out of localized text.
For every public function, additionally follow the
[developer function-comment invariants](../function-comments.md):
state the purpose first, cover every user-relevant capability, use active voice,
and name who performs each action.

1. **Module Documentation (`//!`)**:
   - The `//!` comments define the **Package-level** documentation.
   - They should only be placed at the very top of the application library root, `src/lib.rs`.

2. **Item Documentation (`///`)**:
   - Standard Rustdoc comments (`///`) are used for documenting structs, enums, traits, and properties.
   - **Short Document (First Line)**: The very first line of the comment is extracted as the **Short Document**, which is used as the Short Name or Display Name in the UI.
   - **Long Document (Remaining Lines)**: Any remaining lines after the first line are extracted as the **Long Document**, which serves as the detailed Description field in the UI or LLM tool call description. Long descriptions use HTML-style flow: one logical CR/LF break becomes exactly one space, while two or more logical breaks become one LF paragraph break. CRLF counts as one break, so leave a blank rustdoc line only when a paragraph break is intended.

3. **Array Documentation Divider (`----`)**:
   - In documentation for a `Vec<T>`, a line containing at least four hyphens separates documentation for the list container from documentation for each list element.
   - Text and custom document properties above the divider belong to the list; those below it belong to the element.
   - See [Array Documentation Dividers](#array-documentation-dividers) for the complete example and placement rules.

4. **Custom Document Properties (`[PropertyName]`)**:
   - After the short and long documents, place a consumer-defined `[PropertyName]` marker on its own line. Text beginning on the next line is the property value and continues until the next marker or the end of that comment section. Unknown properties have no platform behavior; use one only when its consumer contract defines the name, owner, and meaning.
   - Multiple properties are supported. They are emitted as string keys directly beside `DisplayName` and `Description` in document JSON.
   - `DisplayName`, `Description`, `List`, `StringResources`, `Strings`, `Enum`, and `DocumentVersion` are reserved. `Strings` is prohibited and is not a document field. A structural key at that location, such as `Name`, `Type`, `Locale`, or `Types`, is also unavailable.
   - With a `Vec<T>` `----` divider, properties above the divider belong to the list and properties below it belong to each element.
   - Display names and descriptions are always present in generated documents. The JSON structure permits an empty value only so generated, legacy, or incomplete nodes can be represented; a codebot-authored schema node must always provide both a non-empty natural label and description. Every document uses format version `0.0.2` and one top-level package `StringResources` map. Per-function/per-type `Strings` fields do not exist.
   - `[DefaultText]` is one-line localized default text on a `String` field or an `Endpoint` server field. Unlike the schema `Default` value, it may differ by locale.
   - `[DefaultTaskName]` is a localizable default task name and is valid only in a function's documentation. It does not replace the function's stable native `Name` or localized `DisplayName`.
   - Put ordinary label/description text before extension blocks and put both
     before parser-directive rustdoc lines.
   - The complete user-language, standard-extension, reserved-name, List-side,
     and translation contract is in the
     [schema documentation invariants](../documentation.md).

5. **Code Comments (`//`)**:
   - Standard double-slash `//` comments are **completely ignored**. Use them freely for internal code documentation.

```rust
//! Garden watering
//! Helps gardeners schedule and adjust watering for the areas they maintain.

/// Configure watering
/// The watering controller follows the gardener's chosen schedule. The
/// gardener can adjust each area's watering at any time. Developed by Example
/// Developer (example_developer).
/// [DefaultTaskName]
/// Back garden watering
#[libertas_export]
pub fn configure(
    /// Watering choices
    /// The gardener chooses the schedule used for this watering task.
    configuration: Configuration,
) {}

/// Watering choices
/// Describes the schedule a gardener chooses for one watering task.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    /// Schedule name
    /// The name people use to recognize this watering schedule.
    /// [DefaultText]
    /// Weekday schedule
    pub schedule_name: String,
}

/// Watering controller
/// Describes the controller a gardener wants to use for watering.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct MyDeviceConfig {
    // This is an internal code comment and will NOT appear in the UI
    /// Controller name
    /// The gardener enters the name printed on or assigned to the controller.
    pub id: String,
}
```

Replace the example Plaza identity before publication. Put parameter rustdoc
directly on each typed parameter of a `#[libertas_export]` function.

The exhaustive current source-attribute catalog is the
[schema annotation inventory](../annotations.md).
Do not infer an authoring attribute from generated schema metadata or a
parser-internal alias.

### Localized documentation

Libertas supports full localization of package documentation. The IDE updates
the structure of localized documentation files when Rust source changes, so
translations remain aligned without manual structural editing.

## Libertas type system

Your Rust coding style directly defines the schema of your software. Because Libertas software is self-describing, the types you choose dictate how the system understands your data, which in turn determines the resulting UI representation and API boundaries. You can use standard Rust features combined with Libertas native types to organize your data intuitively.

### Native global types

When defining properties and function parameters, utilize the provided native Libertas types. These types allow the platform to understand the specific semantic meaning of your data (which automatically translates into the correct UI controls like device pickers, date/time selectors, or specialized inputs):

- **`LibertasEndpoint`**: Represents a protocol endpoint for interactions with humans, AI, and other software.
- **`LibertasDateTime`**: A timestamp. In the UI, this automatically displays in the user's local timezone.
- **`LibertasTimeOnly`**: Provides a 24-hour time selector in the UI.
- **`Vec<u8>`**: Maps to the built-in `Bytes` schema type for binary data.
- **`LibertasUser`**: Presents a system user or user group picker. Use explicit [User and group defaults](../schema.md#user-and-group-defaults) for initial suggestions; the picker does not infer the current user.
- **`LibertasDevice`**: Presents a physical or logical device picker.
- **`LibertasVirtualDevice`**: Presents a virtual device picker.
- **`LibertasLanDevice`**: Presents a WiFi or Ethernet network device picker.

For a MIME-aware `Vec<u8>` field, use
`#[libertas_mime_types("image/png,image/jpeg")]`. The parser emits `MimeTypes`;
Studio validates its filter and effective Bytes host. The
[MIME-aware Bytes contract](../schema.md#mime-aware-bytes) owns LMB1 encoding,
empty values, and client presentation. Apps can use
`libertas::encode_mime_bytes("image/png", content)` to create a defined value
and `libertas::decode_mime_bytes(bytes)` to inspect one; an empty `Vec<u8>` is
the valid media-undefined value. Keep unannotated `Vec<u8>` fields for raw
binary protocols.

### Matter status values

`libertas-matter` exposes client `MatterStatus` as a raw `u32`; zero means
success. Virtual-device response APIs use `IMStatusCode`, or
`MatterResponseStatus` for a standard cluster status:

```rust
request.respond_status(IMStatusCode::InvalidAction);
```

The [Matter status reference](../matter/status.md) owns bit layout, supersession,
per-attribute results, and error overrides.

Built-in names may be unqualified after importing them or qualified exactly as
`libertas::LibertasDevice`, `libertas::LibertasEndpoint`, and so on. A different
module prefix ending in a reserved built-in name is rejected.

### Arrays and structs

#### Struct composition

You can build reusable models by nesting or composing public structs. A field
whose type is another named struct becomes a reference to that named schema
type. The referenced type retains its own fields, attributes, and documents;
field-level attributes and comments belong to the reference node. Rust structs
do not have schema inheritance, and composing one struct inside another does
not copy or flatten the referenced definition into the owner.

#### Array documentation dividers

When documenting a `Vec<T>`, you often need to provide instructions for the list as a whole, as well as instructions for a single item. Libertas supports **Array Divider Handling** using four hyphens (`----`):

```rust
/// Watering plan
/// Describes the watering areas a gardener wants the controller to maintain.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Config {
    /// Watering zones
    /// Add all the sprinkler zones you want to control.
    /// [Hint]
    /// Keep the list in watering order.
    /// ----
    ///
    /// Sprinkler zone
    /// Configure the details for a single watering area.
    /// [Hint]
    /// Select the valve for this area.
    #[libertas_size(min = 1)]
    pub zones: Vec<String>,
}
```
* **UI Presentation**: The UI uses the text above the divider to label the array container, and uses the text below the divider to label each individual item within the list.

### Enums: Enumerations and Union types

Rust enums are highly flexible in Libertas and can be interpreted in two distinct ways depending on how you define their variants:

#### Enumerations (no payloads)

Standard Rust enums without payloads will present to the user as a simple dropdown or selection list.

#### Union types (with payloads)

If the enum variants contain data (payloads), it acts as a Union Type. You can use either separate named structs or inline variants in your enums. The UI handles inline structs seamlessly, providing the exact same input form as if you had used a named struct.

### Optional types

Separate user input, displayed output, and internal records when choosing types.
`pub` makes a declaration available to schema discovery; it does not mean that
people edit every occurrence. Follow the shared
[type-selection guidance](../app-contract.md#user-facing-and-non-user-facing-types).

For ordinary optional text, use `String` with an explicit empty Default and
size bounds that permit zero. Required text uses a positive minimum and can
remain unanswered until the person enters it:

```rust
/// Student name
/// Describes the parts of a student's name that a school records.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Student {
    /// First name
    /// Enter the student's first name as recorded by the school.
    #[libertas_size(min = 1, max = 120)]
    pub first_name: String,

    /// Middle name
    /// Enter the student's middle name. Leave it empty if absent.
    #[libertas_default("")]
    #[libertas_size(min = 0, max = 120)]
    pub middle_name: String,
}
```

The middle name starts as `""` and may stay empty. The first name has no
Default and starts unanswered. `SizeMin=0` alone would not initialize a String
to `""`; Rust's `String::default()` is not a Schema Default either.

Use `Option<T>` when absence is a distinct, valid domain answer. For example:

```rust
/// Reported payment
/// Describes the amount known for one payment.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct ReportedPayment {
    /// Payment amount
    /// Enter the known payment amount, including zero when no money was paid.
    /// Omit the amount when it is unknown.
    #[libertas_money("USD", 2)]
    pub amount: Option<i64>,
}
```

Here `None` means unknown and `Some(0)` means known zero. Retain Nullable text
when absence must differ from a present answer, including an optional
identifier with `#[libertas_size(min = 9, max = 10)]`: absence is valid, but
any supplied identifier must meet that length constraint.

`Option<String>` with a minimum of 1 is valid. The size rule applies to the
present String; it rejects `Some("")`, not `None`. Prefer the simpler blank
String for ordinary optional text when this distinction is unnecessary.

### Labeled decisions and internal absence

A binary decision can justify a named Union. Its localized labels explain what
the person chooses, and a selected payload branch exposes the relevant input.
Use an Enumeration when neither choice needs a payload. For example:

```rust
/// Email contact preference
/// Choose whether email contact is wanted and where messages should be sent.
#[derive(LibertasExport, LibertasAvroDecode)]
#[libertas_inline_choices]
pub enum EmailContact {
    /// No email contact
    /// Choose this when email messages are not wanted.
    NoEmail,

    /// Receive email
    /// Choose this to receive messages at the supplied email address.
    Receive {
        /// Email address
        /// Enter the address where messages should be sent.
        #[libertas_size(min = 1, max = 254)]
        address: String,
    },
}
```

This Union offers two labeled radio choices through
[InlineChoices](../presentation.md#inline-choices); without that attribute,
the normal choice selector is used. It has no Default, so the person must
choose before submitting. Selecting `Receive` requires its address. An
`Option<String>` instead starts as valid absence and supplies a generic
presence/absence control. Defaulting a Union to its empty branch also removes
the requirement to make a new choice; a Default is not proof of confirmation.
Native names such as `Nothing` and `Something` are permitted, but their comments
must provide meaningful standalone choice labels.

For an internal email address that only needs presence or absence,
`Option<String>` avoids an unnecessary named choice type. If an input String
uses blank for absence while an existing record uses `Option<String>`, map at
the boundary, for example:

```rust
fn record_text(input: String) -> Option<String> {
    if input.is_empty() { None } else { Some(input) }
}

fn editor_text(stored: Option<String>) -> String {
    stored.unwrap_or_default()
}
```

Use this mapping only when exact empty text means absence in that domain;
trimming whitespace would be a separate policy. Keep a reusable type when its
meaning and constraints fit both uses. If the goal is only to collapse extra
details, use `#[libertas_foldable]` on a struct: [folding](../presentation.md#folding-structs-and-unions)
keeps its values, while selecting another Union branch changes the answer.
Follow the shared [representation-change review](../app-contract.md#reviewing-a-representation-change)
for validation, fallbacks, grouping, import, and published compatibility.

### Enum and variant documentation

When a Union enum variant wraps a named type, documentation on the variant
takes precedence over documentation on that named type. Attributes from the
variant and referenced type are combined.

| Priority | Location | Description |
| :--- | :--- | :--- |
| **1 (highest)** | Enum variant node | Rustdoc directly preceding the variant. |
| **2** | Referenced variant type | Rustdoc preceding the wrapped struct or enum. |

```rust
/// Watering-area details
/// Reports which watering area was found by the controller.
#[derive(LibertasExport, LibertasAvroDecode)]
#[libertas_response]
pub struct ZoneInfoMessage {
    /// Watering area number
    /// Identifies the watering area whose details are being reported.
    pub zone_id: i32,
}

/// Watering-area actions
/// Describes the actions and results available while checking watering areas.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum ZoneDataProtocol {
    /// Check watering-area details
    /// Asks the controller for the current details of a watering area.
    #[libertas_request]
    GetZoneInfo,

    /// Current watering-area details
    /// Reports the watering area found by the controller and allows a hold-off
    /// update to follow.
    #[libertas_next_request("UpdateHoldOff")]
    ZoneInfo(ZoneInfoMessage),

    /// Update watering hold-off
    /// Asks the controller to change when watering may resume.
    #[libertas_request]
    UpdateHoldOff,
}
```

In the example, the localized text on `ZoneInfo` takes priority over the
localized text on `ZoneInfoMessage`, while both sources remain meaningful to a
person when viewed independently.

Rust enums already identify their active variant. Do not define a discriminator
field or use custom tag or untagged layouts; native variant names are the Union
case names.

## Schema attributes and field presentation

Libertas utilizes custom Rust attributes (starting with `#[libertas_...]`) to control how the UI and interactions behave.
The parser validates each attribute's argument count and whether arguments are
positional or named. Do not interchange those forms. In particular,
`libertas_endpoint_schema`, `libertas_copy_from`, `libertas_device_type`,
`libertas_virtual_device_type`, and `libertas_foreign_type` take positional
values rather than key-value arguments. No-argument markers reject parentheses
containing values. `libertas_string_resources` and `libertas_permissions` each
take exactly one positional Rust identifier.

`libertas_string_resources` is package-level source metadata written in the
leading inner documentation as
`//! #[libertas_string_resources(CONST_NAME)]`. It is invalid on functions,
types, variants, and fields.

### General field attributes

- `#[libertas_default(value)]` is the canonical input-default form. The named
  `#[libertas_default(default = value)]` variation remains parser compatibility
  syntax and codebots must not generate it. Enumeration member names or numeric
  defaults follow the [shared enumeration contract](../schema.md#53-enumerations).
  User and User-list defaults use the
  [identity-selector contract](../schema.md#user-and-group-defaults), for example
  `#[libertas_default("current-user")]`.
- `#[libertas_fixed(value)]` emits one fixed encoded value. On an Endpoint
  client, the value instead is a `developer-package::function` task-process
  identity: the Endpoint selector remains editable, compatible servers are
  restricted to that process, and its package becomes a mandatory dependency.
  Native enum and Union declarations continue to supply their generated member
  `Fixed` values.
  On `LibertasUser`, Fixed is a [Hub-enforced security constraint](../schema.md#fixed-current-user-security-constraint); only `current-user` is valid, at a request root or within nested request values outside Lists, optionally Hidden.
- `#[libertas_read_only]` emits `ReadOnly=true`.
- `#[libertas_hidden]` emits `Hidden=true`.

For document-assisted Request entry, place
`#[libertas_llm_doc_trans(instruction = "...", validation = "@.facts",
doc_type = "image/*", accept_multiple_inputs = true)]` on a named struct.
Only `instruction` is required. Mark direct result fields with
`#[libertas_llm_doc_field]` or
`#[libertas_llm_doc_field(allow_empty = false)]`. String defaults to
`allow_empty = false`, even when manual entry permits empty text. For imported
optional text, explicitly allow blanks and supply an empty Schema Default:

```rust
/// Supplier slip
/// Records the supplier and reference printed on one supplier slip.
#[derive(LibertasExport, LibertasAvroDecode)]
#[libertas_llm_doc_trans(
    instruction = "Read the supplier slip. Return an empty string for an absent reference. Report unreadable text as an error.",
    doc_type = "image/*"
)]
pub struct SupplierSlip {
    /// Supplier name
    /// Enter the supplier name printed on the slip.
    #[libertas_size(min = 1, max = 120)]
    #[libertas_llm_doc_field]
    pub supplier_name: String,

    /// Slip reference
    /// Enter the reference printed on the slip. Leave it empty if absent.
    #[libertas_default("")]
    #[libertas_size(min = 0, max = 120)]
    #[libertas_llm_doc_field(allow_empty = true)]
    pub reference: String,
}
```

An editable Request occurrence offers AI Import for this struct. A blank
reference converts to `""`, and a blank supplier name is rejected. The field
comments and instruction agree on missing text; JSON `null` is reserved for
Nullable destinations. A non-Nullable numeric
field defaults to `allow_empty = true` and therefore needs a meaningful
`#[libertas_default(value)]` for a truly blank form box. Keep the field's
domain type; do not add `Option` solely for document recognition. The
[document transformation contract](../llm-document-transformation.md#app-schema-authoring)
owns fact paths, supported result types, input media, and human review.

`#[libertas_exclude_ui]` is deliberately narrower than `libertas_hidden`. It
emits `ExcludeUI=true` only on a `LibertasEndpoint` field carrying
`#[libertas_endpoint_server]` and a resolved protocol schema, or on a reachable
protocol `Request`/`SubscriptionRequest` variant. The Hub Control service omits
a marked server Endpoint from its UI-device list and LLM tool collection;
generated Control GUI and Hub LLM tool-definition generation omit a marked
request operation, while admin creation/editing uses the complete schema. The
marker does not remove the node from the schema, change a protocol role, affect
serialization, routing, subscriptions, or authorization. Every server Endpoint
sharing one resolved
`ProtocolSchema` root must use the same `ExcludeUI` value; see the
[Endpoint declaration contract](../protocols.md#declaring-an-endpoint).

`#[libertas_llm_tool_call(false)]` on a protocol enum excludes its Request
variants from Hub LLM tool definitions. A variant can opt back in with
`#[libertas_llm_tool_call(true)]`; omission defaults to true. The same Boolean
attribute on one Request variant affects only that variant. See the
[shared attribute contract](../annotations.md#endpoint-and-protocol-annotations).

<a id="table-header-promotion"></a>

### Promotion, flattening, labels, and titles

Promotion, flattening, row labels, and row titles have no Rust attribute:
write them as [`promote`](../presentation.md#tableheader-promoted-fields-and-union-selectors),
[`flatten`](../presentation.md#flatten),
[`label`, and `title`](../presentation.md#labels-and-titles) rules in the project's
`display.json`, beside `Cargo.toml`. A `promote` rule, valid only on a struct
field, puts that field's value on the struct's row. A `flatten` rule on a
struct, `Vec`, or enum, by type or by field, draws its children at its own
indentation, and on a struct `{ "name_row": false }` also removes the struct's
name-only row. A `label` rule with a promoted field gives a
[data-titled row](../presentation.md#data-titled-rows). The retired
`#[libertas_ui_header]` and `#[libertas_ui_name]` are rejected.

`#[libertas_suppress_label]` suppresses a scalar, struct, or Union row’s visual label while retaining its value and controls. On a `Vec` of structs or its type alias it suppresses only List-contributed member names and positions, retaining the List heading. Studio compiles it into the [Display Template](../presentation.md#display-template) and enforces [placement and page-title rules](../presentation.md#suppresslabel-label-free-value-rows). The former `#[libertas_suppress_title]` spelling is rejected.

### Inline descriptions

`#[libertas_ui_include_description]` on a schema data field or type is
compiled into the [Display Template](../presentation.md#display-template) as its `include_description` hint. Clients keep the localized short display name and
also render the localized long description inline in that UI node. The marker
is client-only presentation metadata and does not change the typed value or
wire encoding.

### Matter device descriptor attributes

`LibertasDevice` and `LibertasVirtualDevice` select different kinds of runtime
object, while their schema attributes carry the Matter descriptor used to
constrain or construct that object. The parser supports these exact mappings:

| Rust attribute | Required effective host | Emitted schema attribute |
| --- | --- | --- |
| `#[libertas_device_type("encodedDescriptor")]` | `LibertasDevice` | `DeviceType=encodedDescriptor` |
| `#[libertas_virtual_device_type("encodedDescriptor")]` | `LibertasVirtualDevice` | `VirtualDevice=encodedDescriptor` |

The meaningful argument is one positional string. The parser removes the Rust
string quotes and stores its contents verbatim; it does not decode or validate
the descriptor. Studio's Device Type Editor and Virtual Device Type Editor are
the canonical generators. They produce the versioned Base64 attribute value:

```text
Attribute.Value = Base64(uvarint32(formatVersion) || descriptorBody)
```

```rust
/// Matter lighting choices
/// Describes the physical and virtual lights used for one lighting task.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct MatterConfiguration {
    /// Room light
    /// The person chooses the physical light controlled by this task.
    #[libertas_device_type("BQEBAYACAQYD")]
    pub light: LibertasDevice,

    /// Optional virtual light
    /// The person may choose a virtual light to mirror the room light; leaving
    /// it empty means that no virtual light is used.
    #[libertas_virtual_device_type("BQEBgAIBAQYBBAAAAQAAAwABAgA=")]
    pub virtual_light: Option<LibertasVirtualDevice>,
}
```

`Option<LibertasDevice>` and `Option<LibertasVirtualDevice>` remain valid hosts
because `Option` only adds `Nullable=true`. The attributes are rejected on
other primitives and on a `Vec<...>` list container; a list never inherits a
descriptor attribute from its item type. The attribute payload is descriptor
metadata, not the runtime object ID. See
[Matter device type attribute encodings](../matter/descriptors.md) for the exact
physical-device and virtual-device byte layouts.

### Numeric modifiers

- `#[libertas_number(min = value, max = value, step = value)]`: Constrains the input to a specific numeric range and step size. The underlying numeric type (integer or float) is automatically inferred from the Rust field's type, so it cannot be specified in the macro.
  Explicit bounds and step replace the corresponding native defaults, including through `Option<T>` and `Box<T>`; omitted settings retain their native defaults.
- `#[libertas_time_interval]`: In the Task Editor, this triggers a specialized `HH:MM` time interval input for users.
- `#[libertas_date_only]`: No-argument marker for an integer field whose value is a Gregorian calendar date in `yyyyMMdd` form. It emits `DateOnly=true`, keeps the inferred integer wire representation, and generates a native date chooser in task configuration and runtime interaction. It cannot be combined with `#[libertas_time_interval]` or `#[libertas_enum_source]`; defaults and fixed values must be valid eight-digit dates.
- `#[libertas_bitflags]` / `#[libertas_bitflags(TypeName)]`: The no-argument form projects a struct or `bitflags!` declaration into a localized Enumeration catalog. The one-type form projects that catalog onto an integer field without changing its integer wire value.
- `#[libertas_money("USD", 2)]`: Declares `i64` money in exact integer minor units and
  emits `Money=USD/2`. Currency and scale are fixed schema metadata; clients
  display currency and edit major units in place. See the [Money contract](../presentation.md#money)
  for supported scales, patterns, validation, and chart behavior.
- `#[libertas_format("pattern")]`: Display-only `Format`, compiled into the [Display Template](../presentation.md#display-template), on a
  numeric, DateOnly, `LibertasDateTime`, or `LibertasTimeOnly` field. Ordinary
  numbers use
  [Unicode UTS #35 number patterns](https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns);
  temporal hosts use
  [Unicode UTS #35 date/time patterns](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns).
  Locale symbols and names are chosen by the viewer. Storage, Avro, bounds,
  sorting, and protocol values remain raw.

```rust
/// Service schedule
/// Describes the calendar day chosen for the next service visit.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Schedule {
    /// Service day
    /// The person chooses the calendar day when service should occur.
    #[libertas_date_only]
    #[libertas_format("yyyy-MM-dd")]
    #[libertas_default(20260815)]
    pub service_day: u32,
}
```

### Bitflag catalogs and integer fields

Put no-argument `#[libertas_bitflags]` on a struct declaration. It may be a
plain struct with associated integer constants or a declaration inside the
popular [`bitflags!`](https://docs.rs/bitflags/latest/bitflags/macro.bitflags.html)
macro. The declaration may remain private because the parser fabricates a
schema-only `Enumeration`; it is never the runtime or wire type.

```rust
use bitflags::bitflags;
use libertas_macros::{libertas_bitflags, LibertasAvroDecode, LibertasExport};

bitflags! {
    /// File access choices
    /// Describes the independent actions a person may allow for a file.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[libertas_bitflags]
    struct Access: u8 {
        /// No file access
        /// Prevents reading, changing, or running the file.
        const NONE = 0;

        /// Read the file
        /// Allows a person to view the file's contents.
        const READ = 1 << 0;

        /// Change the file
        /// Allows a person to modify the file's contents.
        const WRITE = 1 << 1;

        /// Run the file
        /// Allows a person to run the file as a program.
        const EXECUTE = 1 << 2;
    }
}

/// File access settings
/// Describes the file actions a person chooses to allow.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Settings {
    /// Allowed file actions
    /// The person selects every action that should be allowed for the file.
    #[libertas_bitflags(Access)]
    pub access: u8,
}
```

The equivalent declaration without `bitflags!` is also valid:

```rust
/// File access choices
/// Describes the independent actions a person may allow for a file.
#[libertas_bitflags]
struct Access(u8);

impl Access {
    /// No file access
    /// Prevents reading or changing the file.
    const NONE: Self = Self(0);

    /// Read the file
    /// Allows a person to view the file's contents.
    const READ: Self = Self(1 << 0);

    /// Change the file
    /// Allows a person to modify the file's contents.
    const WRITE: Self = Self(1 << 1);
}
```

The field form also accepts a qualified native path to a catalog owned by a
different Libertas App:

```rust
/// Shared file access settings
/// Describes the file actions a person chooses from the shared access catalog.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Settings {
    /// Allowed file actions
    /// The person selects every shared action that should be allowed for the
    /// file.
    #[libertas_bitflags(shared_flags::Access)]
    pub access: u8,
}
```

Here `shared_flags` is the Rust crate path. The parser resolves the complete
path and emits `Bitflags=shared-flags::Access`, using the dependency App's
canonical package identity rather than copying the source spelling. The
external catalog may remain private, but it must be declared under the exact
name `Access` at that App's root `lib.rs`. During deployment planning Studio
fetches/explores the owning package from the selected Hub if it is not a local
project. Its authoritative Hub version is included in finalized
`AppDependencies` before this App is submitted.

The catalog becomes an `Enumeration` with `Bitflags=true`; `NONE`, `READ`,
`WRITE`, and `EXECUTE` become numeric children with `Fixed=0`, `1`, `2`, and
`4`. Each nonzero member must be a unique single bit. One zero value is allowed
for a localized empty label such as “None” or “N/A”. The field must be an
integer wide enough for every declared bit. Defaults, fixed values, task values,
and runtime values may combine declared bits but may not contain unknown bits.

Studio displays a value using localized flag names joined by commas and edits
it with a multiple-choice chooser. The Hub is deliberately unaffected: it sees,
stores, and transports only the ordinary integer (`u8` above). `Bitflags`
cannot share a field with DateOnly, TimeInterval, EnumSource, Format, or
PhysicalUnit.

### Size constraints

- `#[libertas_size(min = value, max = value)]`: Restricts size for `String` fields, list/array fields such as `Vec<T>`, and byte array fields such as `Vec<u8>`.

### Enumerations declared by attribute

The parser can turn a public type alias into an `Enumeration` with
`#[libertas_enum(...)]`. For compilable Rust, put this parser-only declaration
on an exact rustdoc line because the SDK does not export `libertas_enum` as a
standalone attribute macro for type aliases. This positional form cannot attach
source documentation to each fabricated member, so it cannot satisfy the
codebot documentation contract. It is parser-supported compatibility syntax,
not a form that codebots may generate.

The following deliberately incomplete grammar schematic is **non-copyable**;
it shows only the compatibility spelling:

```rust
/// Account role
/// Classifies the responsibilities assigned to one account.
/// #[libertas_enum(Admin, User, Guest)]
pub type Role = String;
```

The parser emits one numeric member in source order, with fixed values `0`,
`1`, and `2` in this example. It accepts positional quoted strings or
identifier-like values. On a `String` argument of a public function, the same
attribute is emitted as `EnumValues`. On a field of a named schema type, it is
temporary metadata for resolving a symbolic `Default` and is removed from the
completed schema. Because those fabricated members have no per-member source
documentation syntax, a codebot must prefer a documented native Rust enum for
every authored reusable choice catalog and must not generate the positional
form until such syntax exists:

```rust
/// Account role
/// Classifies the responsibilities assigned to one account.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum Role {
    /// Administrator
    /// Allows the account to manage people and shared settings.
    Admin,

    /// Standard member
    /// Allows the account to use shared services without managing accounts.
    User,

    /// Guest
    /// Allows the account to use only the services explicitly shared with it.
    Guest,
}
```

### Array attributes

- `#[libertas_unordered]`: Marks an array as an unordered set, affecting how arrays are merged or displayed.
- `#[libertas_unique(level)]`: Emits the `Unique` ancestor level for a scalar
  field. The level is positional, must be at least `1`, and defaults to `99`
  when omitted. Studio rejects `Unique` on a `Table` or `List` field.

### Union content naming

Rust enums with payloads become Libertas unions. Their variants define the
union cases directly. The parser supports `UnionContentName` through the exact
parser-directive rustdoc line shown below. The outer `#[libertas_export]`
compiler macro is function-only, so do not apply that spelling directly to an
enum.

```rust
/// Written messages
/// Describes the kinds of written updates a person can receive.
/// #[libertas_export(content = "payload")]
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum Message {
    /// Written update
    /// Carries wording intended to be shown exactly as it was supplied.
    Text(String),
}
```

Custom tag, untagged, and discriminator-layout attributes are not accepted.

### Data-backed enum indexes

Use `#[libertas_enum_source("path")]` when an unsigned integer stores the zero-based index of an item in another list in the same schema-data tree. The attribute emits `EnumSource`; its one positional string must be a [Libertas Schema Path](../schema.md#context-and-syntax) in the EnumSource context. Rust infers the unsigned width from the field type. Studio rejects signed integers, floats, non-number hosts, unresolved paths, and paths whose target is not a list.

```rust
/// Report choices
/// Describes the available reports and the report currently selected.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Settings {
    /// Available reports
    /// Lists the reports a person can choose in their displayed order.
    /// ----
    /// Available report
    /// Names one report the person can choose.
    pub choices: Vec<String>,

    /// Selected report
    /// Identifies the report the person chose from the available reports.
    #[libertas_enum_source("$.choices")]
    pub selected_choice: u16,
}
```

Here, `selected_choice = 0` refers to the first item in `choices`.

## Functions, persistent data, and resources

In a `TypesOnly` package, exact-name public re-exports of schema types are
roots. Re-exported constants, statics, and helper functions are not schema types
and do not become type roots. Unresolved re-exports and invalid schema types
still report errors; non-type exports need no exclusion annotation.


### Public functions

Every public function is emitted in the schema unless it has the exact
`/// #[libertas_ignored]` parser directive. The same directive may be placed on
a public named type to omit that type as a root. It does not permit an included
root to reference the ignored type. Use the function-only
`#[libertas_export]` SDK wrapper when the function arguments carry parser-only
attributes that must be consumed before Rust compilation.

A `TypesOnly` package may contain public helper functions only when each helper
is ignored. Any public function that reaches the schema makes the package
invalid as `TypesOnly`.

Use the no-argument `#[libertas_singleton]` attribute on a public function when
a Hub may contain at most one task for that App function:

```rust
/// Coordinate property irrigation
/// The irrigation coordinator prevents overlapping watering across one
/// property's configured equipment. A gardener starts or stops this continuous
/// service. Only one coordinator can operate on the same system. Developed by
/// Example Developer (example_developer).
#[libertas_export]
#[libertas_singleton]
pub fn coordinate_irrigation() { }
```

It emits the exact function attribute `Singleton=true`. The marker is invalid
on the package, types, fields, variants, and parameters; arguments and duplicate
markers are rejected. Singleton identity is the App plus the native function
name, independent of the selected App version. Studio prevents an obvious
conflict before submission, while the Hub transactionally enforces create,
retarget, and version-change requests. The current task may update itself. If
the two selected versions disagree, either version's marker makes the pair
conflict, which prevents an older schema from bypassing a singleton task.

### Persistent data schemas

Ordinary Data API failures stop the task in the Hub runtime without permanently
disabling it or automatically restarting it. A function may explicitly opt into
checked status-returning calls with `DataErrorStatus`. See the
[Data API failure contract](../protocols.md#persistence-failure-and-the-no-status-api)
for the flag, return values, consistency responsibility, and third-party `Enable` action.

For the opt-in, import `libertas_macros::libertas_data_error_status` and mark
the public task function with `#[libertas_data_error_status]`. Checked calls
use the `libertas_data_try_` prefix and return `Result<T, DataError>`; the existing
`libertas_data_` calls retain default failure behavior, even on opted-in tasks.

Use `#[libertas_data_schema(Type)]` on a public function to associate that
function with its persistent-data Union. The positional `Type` argument is
required for a meaningful declaration; placing the attribute on the enum does
not mark it as persistent data. `Type` may be a simple name or a qualified Rust
path, with or without quotes. Qualification is used only to resolve the Rust
source. A locally declared target is emitted with its simple schema name; a
target declared in another App is emitted as its canonical
`developer-package::TypeName`, even when imported or re-exported locally. Its
schema and documents remain owned by that original App.
`#[libertas_endpoint_schema(Type)]` follows the same type-resolution rule.

Use only the public `libertas_data_schema` form above.

### Permissions

Use `#[libertas_permissions(CONST_NAME)]` on a function to declare the
permissions it requires. The referenced item must be a `const` string array;
the parser rejects a missing constant, another type, or a non-string element.
Every permission must also be a key in the package string resources, and
the default-locale value must be a non-empty explanation.

```rust
//! Local weather service
//! Helps people use nearby weather conditions when planning outdoor work.
//! #[libertas_string_resources(WEATHER_STRINGS)]

const WEATHER_PERMISSIONS: &[&str] = &[
    "libertas.permission.ACCESS_FINE_LOCATION",
];

const WEATHER_STRINGS: &[(&str, &str)] = &[
    (
        "libertas.permission.ACCESS_FINE_LOCATION",
        "Use this property's location to request the correct local weather forecast.",
    ),
];

/// Follow local weather
/// The weather service provides local forecasts for people planning outdoor
/// work. The weather service uses the property's location. A person starts or
/// stops the continuous service. Developed by Example Developer
/// (example_developer).
#[libertas_export]
#[libertas_permissions(WEATHER_PERMISSIONS)]
pub fn start_weather_service() { }
```

This emits `Permissions=libertas.permission.ACCESS_FINE_LOCATION` and stores
the explanation under the same key in the package document's `StringResources`
document. Missing keys and blank explanations are parser errors.

The same attribute may be placed on a built-in Endpoint field marked
`#[libertas_endpoint_server]`. It then declares downstream access requirements
and does not require same-key reason resources. The Hub compares those
requirements with the selected client App task's function permissions during
task approval or configuration.

```rust
/// Weather information connection
/// Describes where another service can receive current local weather updates.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct WeatherServer {
    /// Weather updates
    /// Lets an authorized service receive the local weather updates it needs.
    #[libertas_endpoint_schema(WeatherProtocol)]
    #[libertas_endpoint_server]
    #[libertas_permissions(WEATHER_PERMISSIONS)]
    pub endpoint: LibertasEndpoint,
}
```

### Network host access

Use `#[libertas_access_host("host1,host2")]` on a function to declare the DNS
hostnames its task process needs to contact. Every exact hostname must be a key
in the package string resources, and its non-empty default-locale value must
explain why the access is required.

```rust
//! Weather-aware watering
//! Helps gardeners adjust irrigation using recent local weather observations.
//! #[libertas_string_resources(NETWORK_STRINGS)]

const NETWORK_STRINGS: &[(&str, &str)] = &[
    (
        "api.weather.example",
        "Download weather observations used to adjust the watering schedule.",
    ),
];

/// Weather-aware irrigation
/// The watering controller automatically adjusts a configured schedule using
/// recent local weather observations. A gardener starts or stops the service
/// when needed. The service requires Internet access to the declared weather
/// provider. Developed by Example Developer (example_developer).
#[libertas_access_host("api.weather.example")]
pub fn irrigation_controller() { }
```

The value is a comma-delimited hostname list with no whitespace. Hostnames are
unique without regard to DNS letter case. Do not include a URL scheme, port,
path, wildcard, or IP address; those are not hostname identities in this
contract. DNS may resolve the declared name to different addresses over time,
while the resource key and localized access explanation remain stable.

Without this attribute the task has neither local nor external TCP/UDP access.
A concrete host grants both TCP and UDP to all of its resolved IPv4/IPv6
addresses. Use `{network_access_local}` or `{network_access_external}` only for
the corresponding broad class; both are reserved resource keys and still need
localized reasons. The Hub refreshes DNS at runtime and adds new answers
without removing older answers until the task process restarts. Changing the
declaration forces that restart. See [App network access](../network-access.md).

### String resources

Declare one localized catalog for the package. Notifications, permission and
host-access reasons, data names, `FormattedText`, and formatted chart labels from every
function use the same stable package-local keys. The task identifies the App
package, so encoded tuples need no function/type owner or resource FQN.

```rust
//! Sprinkler control
//! Helps gardeners start watering and understand the resulting status.
//! #[libertas_string_resources(APP_STRINGS)]

pub const APP_STRINGS: &[(&str, &str)] = &[
    ("MSG_SUCCESS", "The sprinkler started watering the selected zone."),
    ("MSG_ERROR", "The sprinkler could not start watering: {0}"),
];

/// Start zone watering
/// The sprinkler starts the configured zone for a gardener who needs immediate
/// watering. The sprinkler reports completion or failure through localized
/// status wording. Developed by Example Developer (example_developer).
#[libertas_export]
pub fn trigger_sprinkler() { }
```

For a `TypesOnly` package, the same package declaration takes exactly one
identifier naming a non-empty literal `const` or `static` tuple slice with
unique, non-empty keys and literal string values:

```rust
//! Shared service wording
//! Provides reusable localized status wording for compatible services.
//! #[libertas_types_only]
//! #[libertas_string_resources(PROTOCOL_STRINGS)]

pub const PROTOCOL_STRINGS: &[(&str, &str)] = &[
    ("READY", "The shared service is ready."),
];

/// Localized service status
/// Carries service status wording that can be shown in the person's language.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct OutputText {
    /// Status message
    /// Reports the service's current state in localized, human-readable words.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub status: Vec<u8>,
}
```

The keys are emitted in the schema package attributes; localized values are
stored once in `PackageDocumentJson.StringResources`. `OutputText` has no
resource field or document table.

## Endpoint protocols and presentation

### Endpoint schema

Apply `#[libertas_endpoint_schema(MySchemaUnion)]` to a `LibertasEndpoint`
field to define its application payload. The argument is positional, not
`schema=...`. A simple name may be unquoted; a qualified Rust path may be
quoted. The resolved target must be a Union. Add
`#[libertas_endpoint_server]` when the field provides the endpoint.

An Endpoint client may identify a task process that can own its peer Endpoint
server. Mutual protocol-schema compatibility always determines the initial
candidate set. `#[libertas_default("developer-package::function")]` does not
filter existing candidates; when none exists, it recommends provisioning that
process. For an absolutely required Endpoint occurrence, that recommendation
becomes a blocking task-installation dependency.
`#[libertas_fixed("developer-package::function")]` additionally
restricts compatible servers to that process and forces its package into
finalized `AppDependencies`; an absolutely required occurrence with no match
blocks creation until the required task process is installed. The two
attributes are mutually exclusive. Both Endpoint-ID
selectors remain editable and may offer multiple server instances. Libertas
FQNs use `::`, not a single colon. The string is not the encoded integer
Endpoint ID and is not resolved as a schema type.

An occurrence is absolutely required when neither the Endpoint nor an ancestor
is `Nullable` and no ancestor `Vec` permits zero items. Clients traverse the
complete effective function-input tree, queue the original creation while
installing every missing required task, and rerun the check after installation.
Cancelling any required dependency installation cancels the originating task
creation. See the normative
[task pre-creation dependency check](../schema.md#62-task-pre-creation-static-dependency-check).

### Protocol conformance during configuration

`#[libertas_protocol_conformance("VariantA & (VariantB | !VariantC)")]`
declares variants required from the opposite endpoint when the annotated Union
is used as a protocol. It is valid on a Union declaration and on a variant of a
Union when that variant's effective type resolves to another Union. A Union
declaration does not itself reveal whether it will later be used as an endpoint
protocol or whether that endpoint will be a client or server, so the parser
does not enforce endpoint usage or role and preserves the expression unchanged.

When placed on a Union-typed variant, the variant's expression becomes the
conformance evaluation for the nested Union in that declaration context. This
allows separate parent variants that use the same nested Union type to declare
different requirements. The variant expression takes precedence over a
`ProtocolConformance` expression on the referenced Union declaration for that
specific use.

Expressions use native variant names with `!` (NOT), `&` (AND), `^` (XOR),
`|` (OR), and parentheses. Precedence from highest to lowest is `!`, `&`, `^`,
then `|`.

Client/server matching during configuration is mutual. The client context's
expression is evaluated against the server's advertised variants, and the
server context's expression is evaluated against the client's advertised
variants. Both expressions must pass. A scalar variant operand evaluates to
whether the opposite Union advertises that name. A Union-typed variant operand
instead becomes its nested Union's conformance result, with the variant
attribute providing that nested context's expression. Recursion therefore
follows variants evaluated by the containing rule without making unrelated
optional branches mandatory. If either Union context omits the attribute,
every direct variant declared by that context is mandatory. A malformed
expression is incompatible and fails closed.

In compact form, the match requires both conditions:

```text
client requirements(server variants) && server requirements(client variants)
```

This configuration-time match covers mandatory protocol requirements. It does
not prove that the selected server supports every optional payload type that a
client might use later.

### Runtime checks for optional variants

At runtime, the client retains the server's protocol declaration, while the
server does not retain the client's declaration. Before sending a request that
uses an optional payload, the client must inspect the retained server
declaration. For a nested Union payload, it must verify every variant along the
declaration path.

Derive `VariantIndex` and use `variant_index!` from the public
[`libertas_macros` crate](https://github.com/LibertasIoT/libertas-rs/tree/main/crates/libertas_macros)
to obtain each zero-based declaration index as a compile-time constant. Use
those constants with the runtime server-declaration check instead of embedding
numeric ordinals:

```rust
use libertas_macros::{variant_index, VariantIndex};

/// Service actions
/// Describes the routine and optional actions available from this service.
#[derive(LibertasExport, LibertasAvroDecode, VariantIndex)]
#[libertas_protocol_conformance("Ping | Optional")]
pub enum ServiceProtocol {
    /// Check service availability
    /// Asks whether the service is currently available.
    #[libertas_request]
    Ping,

    /// Additional service action
    /// Offers an additional diagnostic or calibration action when the service
    /// supports it.
    #[libertas_protocol_conformance("ReadDiagnostics")]
    Optional(OptionalRequest),
}

/// Additional service actions
/// Describes optional diagnostic and calibration actions.
#[derive(LibertasAvroDecode, VariantIndex)]
pub enum OptionalRequest {
    /// Read service diagnostics
    /// Asks for the service's current diagnostic observations.
    #[libertas_request]
    ReadDiagnostics,

    /// Calibrate the service
    /// Asks the service to calibrate using the supplied calibration level.
    #[libertas_request]
    Calibrate(u8),
}

const OPTIONAL_INDEX: usize = variant_index!(ServiceProtocol::Optional);
const CALIBRATE_INDEX: usize = variant_index!(OptionalRequest::Calibrate);
```

Published protocol and data Unions evolve append-only: a compatible version may
append variants, but must not remove, reorder, or insert before an existing
variant. Libertas development tools enforce that rule, keeping declaration
indices stable across compatible versions. Keep each protocol or persistent-data
Union's native name versionless and invariant for the App's lifetime; append
new payload variants to the same Union instead of replacing it with a `V2`
Union. See the
[App data-type design guidelines](../data-type-evolution.md).

### Protocol roles and context

`#[libertas_endpoint_base_objects("path1,path2")]` filters the endpoint's
base-object context using one positional comma-delimited path string.

Every protocol variant has one or more roles:

- `#[libertas_request]` or `#[libertas_response]`
- `#[libertas_subscription_request]` or `#[libertas_subscription_data]`

Add the no-argument `#[libertas_error]` marker beside
`#[libertas_response]` when the complete response represents an error result
rather than a normal result. It emits `Error=true`; it does not imply the
`Response` role and does not alter the payload, discriminator, wire encoding,
routing, or transition graph. The payload remains an ordinary typed response.
A consuming client may use the marker to select its standard error procedure.
The error is out-of-band of the current protocol state transition: Android and
iOS consume the correlated response without installing or visualizing it,
preserve the current interaction and current response data, and leave retry
available. They show a persistent error dialog with the first renderable `FormattedText` found in
deterministic depth-first response order (including hidden fields), or a
localized generalized error when none can be rendered. Error classification
takes precedence over normal default-response, chart, and data presentation.

Outside workflows and automatic WorkflowRequest entry, a normal correlated
`#[libertas_response]` may omit its effective
`#[libertas_next_request(...)]`. Omission is a non-transition acknowledgement,
commonly used by shared `Success` or `Failure` variants; it never enables every
Request in the Union. Clients complete the in-flight attempt without displaying
the acknowledgement as protocol data, changing the current choices, or
displacing a live DataReport. Its reference context follows the
[interaction reference-context rule](../protocols.md#interaction-reference-context).
For a variant that is both Response and SubscriptionData, the explicit runtime operation
controls: response delivery follows this rule, while
SubscriptionData/DataReport delivery still refreshes subscribed state.
`#[libertas_error]` remains the higher-priority out-of-band procedure above.

The [protocol contract](../protocols.md#default-and-explicit-subscriptions)
owns automatic versus explicit subscription selection, callback prerequisites,
and per-user access. Argument-free does not mean automatic.

On `#[libertas_request]` or `#[libertas_subscription_request]`, add
`#[libertas_access_privilege("Read")]`,
`#[libertas_access_privilege("Write")]`, or
`#[libertas_access_privilege("Read,Write")]` to restrict which end users may
invoke that runtime operation. The parser emits the language-neutral integer
bitmap `AccessPrivilege=1`, `2`, or `3`. These are alternative access levels:
any matching effective access to the owning endpoint authorizes the operation.
Omission defaults to `Read,Write` (`3`), which lets a read-only user invoke the
operation. Libertas cannot infer an application-specific operation's semantic
effect, so the developer must explicitly select `Write` for a mutating
operation. Studio filters unavailable operations; the Hub independently decodes
the selected protocol input and enforces the bitmap before routing it.

The term *privilege* distinguishes this runtime end-user check from task
`Permissions`. Permissions grant the whole task process access to data and
resources during configuration; `AccessPrivilege` limits one endpoint protocol
operation for the current user. Currently `Read=1` and `Write=2` are defined,
with other bitmap values reserved.

```rust
/// Service controls
/// Describes the status and operating choices available for the service.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum ControlProtocol {
    /// Check current status
    /// Asks the service to report its current operating status.
    #[libertas_request]
    #[libertas_access_privilege("Read")]
    GetStatus,

    /// Change service availability
    /// Lets an authorized person choose whether the service is running or
    /// stopped.
    #[libertas_request]
    #[libertas_access_privilege("Write")]
    SetEnabled(bool),
}
```

Use `#[libertas_next_request("...")]` and
`#[libertas_next_response("...")]` to name allowed next variants. One
comma-delimited string may name multiple targets. A target must exist and carry
the role required by the delivery opcode; EditRequest replies carry Request. `NextRequest` is optional on a Response; omission is the
non-transition acknowledgement above, not an implicit list of every Request.

`#[libertas_calculation_request("CalculatePrice,CalculateShipping")]` is a
field-only helper accepted by `LibertasExport`. Its one comma-delimited string
lists the protocol Request variants that use the field. Studio validates the
input/participation relationship; see the shared
[field calculation contract](../protocols.md#field-calculation-requests).
It adds metadata without changing the Rust field or generating calculation code.

`#[libertas_copy_to("$.orderTotal")]` assigns a successful calculation response
field into the originating request editor. It supports zero-based array indices.
See [calculation results](../protocols.md#applying-calculation-results) for atomic
assignment, nullability, output ownership, reset, and deferred execution rules.
On a reduced SubscriptionData variant, `#[libertas_copy_to("^<FullState>,^<OtherState>")]`
selects exactly one destination in each admitted stable value; see the
[reduced subscription pattern](../protocols.md#reduced-subscriptiondata-design-pattern).

`#[libertas_copy_from("$.path1,$.path2")]` is valid only on protocol Request or
SubscriptionRequest data. The [interaction reference-context rule](../protocols.md#interaction-reference-context)
supplies the ordinary construction source; ordered selection, missing values,
and initialization follow the [shared copy rules](../schema.md#shared-copyfrom-and-copyto-rules).
The expression is positional; `paths=...` is not accepted. Runtime
CopyFrom uses the same strict variant-based paths as directed subscription
CopyTo, for example `^<FullState.value>,^<OtherState.value>`. It searches in
order for the first match, while CopyTo rejects multiple matches as an App
design flaw. `$` root-relative shorthand remains supported for CopyFrom;
`[0]` selects a literal zero-based array element. See the
[shared path contract](../schema.md#shared-copyfrom-and-copyto-rules). The broader
BaseObjects navigation grammar does not apply.


`#[libertas_workflow_request("EditProfile")]` declares the
[WorkflowRequest semi-role](../protocols.md#workflowrequest-semi-role).
`#[libertas_prev_request("ReopenProfile,ReopenAddress")]` belongs on a Request and
declares ordinary Back actions constructed from the current editor. Their
controls are suppressed in embedded master-detail trailing panes under the
[pane contract](../presentation.md#master-detail-panes).
NextResponse also permits Request variants returned as
correlated EditRequest messages; they need no Response role. See the
[workflow contract](../protocols.md#workflow-interactions) for completion,
initialization, explicit submission, and operation ownership.

`#[libertas_link("OpenTopic")]` on any field, or on a struct or enum
declaration, names the one Request variant that tapping that node opens;
see [links](../protocols.md#links) and [link rendering](../presentation.md#link-rendering).

`#[libertas_link_only]`, with no argument, on a Request variant that a link
targets offers it only through links, never in Actions; see
[link-only requests](../protocols.md#link-only).

`#[libertas_request_confirmation]`, with no argument, on a Request variant
makes a client ask the person to confirm before it sends the request; see
[critical requests](../protocols.md#request-confirmation).

### Semantic validation rules

Use `#[libertas_validation_rules("...")]` on a public function, any field or
variant supported by `LibertasExport`, or any named schema data type. A function
rule sees one synthetic Table containing all arguments. Field-local, inherited-
field, and referenced named-type rules are cumulative and rooted at the
concrete annotated value; `[]` denotes that value.

```rust
//! #[libertas_string_resources(VALIDATION_MESSAGES)]

static VALIDATION_MESSAGES: &[(&str, &str)] = &[
    ("NONNEGATIVE", "Enter zero or more."),
    ("PERCENTAGE_MAXIMUM", "Enter a percentage no greater than 100."),
    ("INVALID_RANGE", "The lower limit cannot exceed the upper limit."),
];

/// Nonnegative quantity
/// Represents a quantity that may be zero or greater when it is supplied.
#[libertas_validation_rules(r#"[] == null || [] >= 0 : "NONNEGATIVE";"#)]
pub type NonNegative = i32;

/// Percentage range
/// Describes a percentage that a person may choose from zero through one
/// hundred.
#[derive(LibertasExport)]
pub struct Range {
    /// Percentage
    /// The person chooses a percentage no greater than one hundred.
    #[libertas_validation_rules(r#"[] == null || [] <= 100 : "PERCENTAGE_MAXIMUM";"#)]
    pub percentage: NonNegative,
}

/// Choose an operating range
/// Lets a person choose inclusive lower and upper limits for an operating
/// range. Both limits must be supplied before final submission, and the lower
/// limit cannot exceed the upper limit. The range service saves the valid
/// choices. Developed by Example Developer (example_developer).
#[libertas_export]
#[libertas_validation_rules(
    r#"[minimum] == null || [maximum] == null || [minimum] <= [maximum] : "INVALID_RANGE";"#
)]
pub fn configure(
    /// Lower limit
    /// The person chooses the smallest value included in the operating range.
    minimum: i32,

    /// Upper limit
    /// The person chooses the largest value included in the operating range.
    maximum: i32,
) {}
```

Square paths read the current evaluation root. A curly reference such as
`{$.limit,$.legacy_limit}` searches the
[current interaction reference context](../protocols.md#interaction-reference-context)
in source-then-path order under the [shared lookup rule](../schema.md#availability-null-and-interaction-ownership).
It returns the first non-null value, or `null` when none is available.
Null comparisons are valid for non-nullable fields because clients evaluate
incomplete editor state; final submission still requires structural
validity and all rules to be true. Prefix a rule string with
`FinalOnly=false;` to opt that complete rule set into incremental validation;
omission means `true`, and every rule remains in final validation. This directive
stays inside the `ValidationRules` value and does not emit a separate schema
attribute. Every expression must be followed by `:`, a quoted package
`StringResources` key, and `;`. Studio rejects undeclared keys and empty
default-locale messages. Clients stop at the first failing applicable rule in
declaration order and display its localized message. A missing or blank
installed message produces a generic validation failure. See the normative
[rule language](../validation-rules.md).

Single `|` selects the first non-null operand from left to right and preserves
its type:

```rust
#[libertas_validation_rules(
    r#"([preferred_limit] | [legacy_limit] | 100) >= 0 : "VALID_LIMIT";"#
)]
```

Evaluation stops at the winner. `false`, zero, empty text/collections, and
objects are non-null and do not fall through. `|` binds after additive
operators and before `is` and comparisons. The selected value may have any
supported intermediate type, but the complete rule must be Boolean; use an
explicit comparison for non-Boolean values.

Use `#[libertas_range_min("expression")]` and
`#[libertas_range_max("expression")]` on a Number or `LibertasDateTime` field
or named type
when its inclusive range comes from current data or the reference context:

```rust
#[libertas_range_min("{$.limits.minimum}")]
#[libertas_range_max("{$.limits.maximum}")]
pub type Percentage = i32;
```

Each attribute contains one Data Validation bound expression compatible with the host type without a
message key, rule-pair semicolon, or `FinalOnly`. A Boolean predicate such as
`[] <= 100` is invalid; use `100` to set that maximum on an ordinary Number.
Date controls require their respective semantic date types. The
[dynamic range contract](../validation-rules.md#31-dynamic-numeric-ranges)
defines static/runtime type rejection, compatible date values, first-response
absence, and enforcement in both the GUI and final validation.

Evaluation retains semantic types at every step. DateOnly plus/minus an integer
shifts Gregorian calendar days, DateTime plus/minus an integer shifts epoch
seconds, and subtracting two values of the same date type returns elapsed days
or seconds. DateOnly arithmetic never operates on its `yyyyMMdd` wire digits.

### Formatted runtime text

Use the no-argument `#[libertas_formatted_text]` marker on an effective `Vec<u8>`
(`Bytes` in the schema), together with static `ReadOnly` under the
[placement and request-value contract](../presentation.md#static-read-only-placement-and-request-values).
The marker emits `"FormattedText": "true"` without changing
serialization.
Formatted bytes are the canonical resource key plus ordered, typed
`LibertasMessageArgument` values; build them with `libertas_formatted_text`.
Localized templates use zero-based LMF1, and runtime payloads never contain
resource templates.

Do not use a `String` host containing Base64. Rendering would only decode that
String back to the same byte tuple, adding a pointless encode/decode cycle and
providing no benefit. Keeping formatted values as Bytes creates a sharp type
distinction from literal text, making developer mistakes less likely. App code
keeps the value as `Vec<u8>`; if a generic JSON transport represents Bytes as
Base64, that is only the transport codec and never the field's semantic type.

The public SDK function `libertas_formatted_text(resource_name, arguments)`
returns the canonical `Vec<u8>`. App code ordinarily does not decode it;
clients resolve and render the LMF1 template at the presentation boundary.

```rust
//! Equipment status
//! Helps people read localized equipment conditions and readiness messages.
//! #[libertas_string_resources(STATUS_STRINGS)]

pub const STATUS_STRINGS: &[(&str, &str)] = &[
    ("READY", "The equipment is ready."),
    ("DEVICE_STATUS", "{0,LibertasDevice} reports a temperature of {1,number,0.0}."),
];

/// Equipment status wording
/// Carries concise and detailed equipment status messages in the person's
/// language.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct StatusBody {
    /// Readiness summary
    /// Reports whether the equipment is ready in concise localized wording.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub summary: Vec<u8>, // canonical READY tuple with no arguments

    /// Detailed equipment status
    /// Identifies the equipment and describes its current state in localized
    /// wording.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub detail: Vec<u8>, // canonical DEVICE_STATUS tuple
}

/// Equipment status actions
/// Describes how a person requests and receives current equipment status.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum StatusProtocol {
    /// Check equipment status
    /// Asks for the equipment's current readiness and detailed state.
    #[libertas_request]
    Read,

    /// Current equipment status
    /// Reports the equipment's current readiness and detailed state.
    #[libertas_response]
    Status(StatusBody),
}

// Private encoder helper; this is implementation code, not an App function.
fn ready_summary() -> Vec<u8> {
    libertas_formatted_text("READY", &[])
}

// Private encoder helper; typed arguments preserve object and measurement
// semantics for localized rendering.
fn equipment_temperature(device: u32, temperature_celsius: f64) -> Vec<u8> {
    let arguments = [
        LibertasMessageArgument::Object(device),
        LibertasMessageArgument::UnitDouble {
            unit_type: "temperature-celsius",
            value: temperature_celsius,
        },
    ];
    libertas_formatted_text("DEVICE_STATUS", &arguments)
}
```

The App package owns the catalog. `StatusBody` carries a reusable, statically
read-only Bytes shape; a `TypesOnly` package may publish it without choosing a
direction. A concrete App that later emits it uses that App's package catalog.
Studio checks each use against the linked placement contract.

Locally, Studio validates the exact marker and effective Bytes host,
plus the syntax and default-document membership of the optional package
catalog. Only the package may declare `StringResources`; a function, protocol,
type, variant, or field never owns a separate catalog. Studio does not require
every formatted field or output path to prove a resource owner or key
membership. See
[Libertas Message Format](../message-format.md) for the LMF1 grammar,
Libertas Message Argument contract, encoded-type priority, and chart behavior.

### Chart presentation

Chart metadata is part of the ordinary endpoint protocol schema. It adds no
payload bytes or chart-specific message. Apply `#[libertas_chart(mark)]` to a
`Response` or `SubscriptionData` enum payload whose effective shape is
`Vec<Row>`. Direct scalar row fields use `#[libertas_chart_channel(...)]` and
may use `#[libertas_chart_scale(...)]` when they own one scale-bearing channel.
`#[libertas_chart_guide(...)]` configures generic guides independently of scale
mapping. A scale's `kind` and a guide's `position` are chart appearance,
compiled into the [Display Template](../presentation.md#display-template). The four parser-only attributes are accepted by `LibertasExport`;
standalone type aliases may import their pass-through macros from
`libertas_macros`.

```rust
/// Soil-moisture reading
/// Records the soil moisture observed at one time.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Sample {
    /// Observation time
    /// Identifies when the soil-moisture reading was observed.
    #[libertas_chart_channel(x, tooltip)]
    #[libertas_chart_scale(id = time, kind = utc)]
    pub at: LibertasDateTime,

    /// Observed soil moisture
    /// Reports the soil-moisture reading measured at the observation time.
    #[libertas_chart_channel(y, tooltip)]
    #[libertas_chart_scale(id = value, kind = linear, zero = false)]
    pub value: f64,
}

/// Soil-moisture updates
/// Describes how a person requests and receives recent soil-moisture readings.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum Protocol {
    /// Check soil-moisture history
    /// Asks for recent soil-moisture readings.
    #[libertas_request]
    Request,

    /// Soil-moisture history
    /// Lists recent soil-moisture readings in observation order.
    /// ----
    /// Soil-moisture reading
    /// Records the soil moisture observed at one time.
    #[libertas_subscription_data]
    #[libertas_chart(line)]
    History(Vec<Sample>),
}
```

For a List carried directly by a tuple variant, put the container text above
the variant's `----` divider and the item text below it. The parser assigns the
two sections to the List and its child respectively; do not duplicate the
container text on the unnamed tuple field.

`Chart` accepts primitive `point`, `line`, `area`, `bar`, `rect`, `rule`,
`arc`, `text`, and `polygon`, plus composition `layer`, `hconcat`, and
`vconcat`. Point, line, rule, and text have Cartesian and polar coordinate
forms; polygon accepts Cartesian or polar vertices and uses implicitly closed
`evenOdd` compound fills.
Theta and radius are quantitative numeric positions; their explicit scale kind
may be `linear`, `log`, `symlog`, `sqrt`, or `identity` with already normalized
`0...1` values, not a discrete or temporal kind.
For a primitive mark, the annotated value may be either `Vec<Row>` or a carrier
struct with exactly one non-hidden direct `Vec<Row>` field. Every other direct
carrier field must use `#[libertas_hidden]`; those values remain in the decoded
response for typed `NextRequest`/`CopyFrom` transactions but never become chart
rows, channels, tooltips, accessibility content, or resource usage.
Compositions are structs whose non-hidden direct fields independently carry
`Chart`; direct non-chart fields are allowed only with `#[libertas_hidden]`.
A response/subscription enum variant may similarly wrap exactly one
chart-bearing field plus hidden transaction fields. The client retains the
complete variant and projects only the chart-bearing field.
A `layer` accepts unit children or nested layers, which flatten in schema order;
it cannot directly contain `hconcat` or `vconcat`. Concat grids may nest.
Channels are deduplicated and emitted in canonical order. Scale source key
`kind` is emitted as language-neutral key `type`; accepted scale kinds are
`linear`, `log`, `symlog`, `sqrt`, `time`, `utc`, `ordinal`, `band`, `point`,
and `identity`.

`#[libertas_chart_scale]` accepts `id,kind,min,max,mid,zero,reverse`; it has no
guide property. Canonical generated DateOnly `min` and `max` must be exact valid
Gregorian `yyyyMMdd` values; an equivalent source numeric spelling is
normalized before that check, and clients map the result by calendar-day
ordinal. DateTime bounds must be integral epoch seconds in
`0..253402214399`, and TimeOnly bounds must lie in `0..86400` inclusive.
Numeric scale properties use one shortest
round-tripping binary64 spelling. The effective target/scalar/scale-kind matrix
is closed; offsets and facets are band-only, and a numeric ordinal category
requires an integral, non-duration schema number.
`#[libertas_chart_guide]` requires `target` and `source`,
followed by optional `position,title,domain,ticks,labels,grid`. Sources are
`scale`, `value`, `span`, or `none`; positions are
`auto/top/right/bottom/left`; component values are `auto` or `none`. A
scale/none guide is placed on the field owning its matching primary target. A
value/span guide is placed on a guide-label field with no scale-bearing channel:

The following field-only parser-syntax schematic is **non-copyable** because it
omits its enclosing documented type:

```rust
/// Reporting period
/// Identifies the calendar period represented by the accompanying readings.
#[libertas_date_only]
#[libertas_chart_channel(tooltip)]
#[libertas_chart_guide(
    target = x,
    source = span,
    position = bottom,
    title = none,
    grid = none
)]
pub bucket_date: u32,
```

`value` targets `x`, `y`, `theta`, or `radius`; `span` requires continuous
Cartesian `x,x2` or `y,y2`. Non-identity primary scales infer a scale-sourced
guide, while identity and offset scales infer none. One explicit declaration
for a target replaces the implicit guide. The exact target/side/component
matrix, layer and facet deduplication, localized titles and typed labels, tick
algorithm, span grouping, and limits are defined by
[Client-side charting](../charts/2d.md#64-chartguide-generic-guides).
Guide overlay is deterministic runtime policy rather than author metadata:
measure outside first, then overlay every eligible left/right guide together
when the post-guide plot would be narrower than 200 logical points. The exact
tap, accessibility, and chart failure presentation is defined by that
contract; clients do not choose their own breakpoints.
Preview uses a 240-point chart viewport with a 180-point charting region.
Fullscreen uses 75% of current usable height, capped in portrait by current
usable width. Complete vertical legend overflow remains in the one outer chart
scroll, whose one-shot initial position exposes the first charting region.
These are runtime policies and add no `ChartGuide` property.
Discrete positional `band` and `point` domains remain complete. A client uses
the contract's 20-point minimum leaf-category pitch and bounded X/Y category
viewports instead of growing the chart or dropping categories. Heatmaps can pan
both overflowing dimensions; a one-dimensional category chart consumes only
that dimension. This is runtime presentation and adds no annotation.
Dense monotone Cartesian lines and vertical areas are automatically resampled
per logical plot point and disclose that state with the noninteractive `≈`
indicator. Source rows remain exact for domains, tooltips, requests, and the
ordinary data view. Libertas imposes no chart-specific hard limit on row,
primitive, or source-path counts; structural/metadata limits and bounded
visible work remain enforced.

App task code may perform any data transformation, including time projection,
before returning the snapshot. Use native DateTime/DateOnly positions when the
automatic temporal scale is wanted, or emit transformed numeric coordinates
and attach typed `source=value`/`source=span` guide labels. The client localizes
those labels and never reverse-engineers the task's transform.

After parser extraction and dependency materialization, Studio's shared chart
validator rejects duplicate/misplaced metadata, invalid chart shapes,
incompatible scalar/channel and scale combinations, missing mark geometry,
secondary endpoints without primaries, and conflicting shared scale IDs.
Nearest same-name attribute lookup is a complete replacement, not a merge.
`Enumeration`, `LibertasDevice`, `LibertasVirtualDevice`, `LibertasUser`,
`LibertasEndpoint`, and `LibertasLanDevice` are categorical values whose raw
member/reference remains chart identity while the client resolves a localized,
permission-aware display name. `Action` is reserved and unsupported.
Formatted `Vec<u8>` output may serve categorical or textual roles, but never a
quantitative role, continuous domain, or identity scale.
A visible plain `String` asserts genuine literal domain/user data such as a
proper name or free-form annotation. Never use one for a fixed App vocabulary,
dynamic App-authored wording, or a copied Libertas object name: use an enum,
formatted output, or the corresponding typed Libertas object reference. This
applies to every label, legend, facet, tooltip, value/span guide, and
accessibility path; key-only and guide-less identity appearance Strings are
non-textual exceptions.
See [Client-side charting](../charts/2d.md) for the complete channel,
scale, geometry, composition, and fallback contract.

#### 3D chart source attributes

Three-dimensional charts use a separate set of parser-only Rust attributes so
their metadata cannot enter the existing 2D chart path accidentally:
`#[libertas_chart3d(...)]`, `#[libertas_chart3d_channel(...)]`,
`#[libertas_chart3d_scale(...)]`, `#[libertas_chart3d_guide(...)]`, and
`#[libertas_chart3d_view(...)]`. They emit the language-neutral `Chart3D`,
`Chart3DChannel`, `Chart3DScale`, `Chart3DGuide`, and `Chart3DView` namespaces.
As with 2D charts, these attributes add only schema metadata; the response keeps
its ordinary typed wire representation.

```rust
/// Surface measurement
/// Records a height at one pair of measurement coordinates.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct SurfaceSample {
    /// First measurement coordinate
    /// Identifies where the sample lies along the surface's first measured
    /// dimension.
    #[libertas_chart3d_channel(x)]
    pub x: f64,

    /// Second measurement coordinate
    /// Identifies where the sample lies along the surface's second measured
    /// dimension.
    #[libertas_chart3d_channel(y)]
    pub y: f64,

    /// Surface height
    /// Reports the height observed at this pair of measurement coordinates.
    #[libertas_chart3d_channel(z, tooltip)]
    #[libertas_chart3d_scale(id = height, kind = linear)]
    #[libertas_chart3d_guide(target = z, source = scale)]
    pub height: f64,
}

/// Surface measurements
/// Describes how a person requests and receives a three-dimensional height map.
#[derive(LibertasExport, LibertasAvroDecode)]
pub enum Protocol {
    /// Read the surface
    /// Asks for the current surface-height measurements.
    #[libertas_request]
    ReadSurface,

    /// Surface height map
    /// Lists the current surface-height measurements in sample order.
    #[libertas_response]
    #[libertas_chart3d(surface)]
    #[libertas_chart3d_view(
        projection = perspective,
        azimuth = 45,
        elevation = 30,
        zoom = 1,
        fov = 45,
        orbit = true
    )]
    Surface(
        /// Surface height map
        /// Lists the current surface-height measurements in sample order.
        /// ----
        /// Surface measurement
        /// Records the height observed at one pair of measurement coordinates.
        Vec<SurfaceSample>,
    ),
}
```

Primitive marks are `point`, `line`, `bar`, `surface`, `wireframe`, and
`stem`; compositions are `layer`, `hconcat`, and `vconcat`. Channels are `x`,
`x2`, `y`, `y2`, `z`, `z2`, `u`, `v`, `color`, `fill`, `stroke`, `opacity`,
`fillOpacity`, `strokeOpacity`, `size`, `shape`, `strokeWidth`, `strokeDash`,
`tooltip`, `detail`, `order`, `key`, `row`, and `column`.
`#[libertas_chart3d_scale]` mirrors the 2D scale properties and uses Rust source
key `kind`, which the parser translates to schema key `type` and Studio
compiles into the [Display Template](../presentation.md#display-template); it also accepts
`logBase > 1` for a logarithmic scale, non-negative `growMin/growMax` fractions
for automatic continuous x/y/z domain growth, and inclusive `[0,1]` `padding`
for a `bar` base axis. Guides add `tickCount` and
`tickStep`, and may target `detail`.

`bar` accepts discrete or sparse continuous x/y bases with a continuous z
value. `x2`, `y2`, and `z2` may appear independently; omitted x/y extents and
the z baseline are inferred, while all six coordinates form an explicit
cuboid. The legacy any-orientation form remains valid when exactly two primary
axes are band scales and the sole continuous primary is the value axis. Surface
and wireframe `u,v` topology values may be arbitrary finite values and may
contain holes.

`#[libertas_chart3d_view]` accepts, in canonical order, `projection`, `azimuth`,
`elevation`, `zoom`, `fov`, `orbit`, `aspectX`, `aspectY`, `aspectZ`, `targetX`,
`targetY`, `targetZ`, `pan`, and `zoomEnabled`. `projection` is `perspective` or
`orthographic`; `orbit`, `pan`, and `zoomEnabled` are booleans; azimuth,
elevation, and target coordinates are finite; zoom and each aspect are greater
than zero; and
`0 < fov < 180`. Orthographic projection accepts `fov` but ignores it. The Rust
parser only translates source syntax; Studio performs portable vocabulary,
placement, defaulting, and conflict validation and emits canonical property
order. No fixed Chart3D schema cap limits rows, vertices, composition units,
guides, or tooltips; normal transport and runtime resource admission still
applies. See [Client-side 3D charting](../charts/3d.md) for the
normative contract.

`zoomEnabled=false` disables user zoom independently of orbit and pan without
changing the authored initial `zoom`.

These booleans independently gate the client's fixed camera actions; they do
not select or remap touch gestures. A preview is gesture-neutral, and its
**Explore 3D chart** action opens full-screen Chart Explorer, where one-finger
drag rotates, two-finger translation moves the view, and pinch/spread zooms.
There is no `mode`, gesture mapping, sensitivity, or control-label attribute or
runtime field. Tap-only controls and accessibility actions name Rotate, Move,
Zoom, and Reset explicitly. See the
[universal 3D interaction contract](../charts/3d.md#12-interaction-accessibility-and-fallback).

Point and stem-tip shapes use
`circle,square,triangle,diamond,sphere,cube,pyramid,cylinder` in that automatic
order; bars use `box,cylinder,pyramid,sphere`. Size zero suppresses or
degenerates the corresponding geometry without deleting its source row.

### Runtime subscription and signaling

Use the [endpoint signaling orientation](../endpoint-lifecycle.md)
and the public `libertas-rs` endpoint rustdoc before implementing callbacks.
The SDK owns the complete contract. Its Rust liveness entry point is
`libertas_endpoint_peer_alive(endpoint, peer: Option<LibertasPeer>)`.

## Publishing and interoperability

### Types-only libraries

Place the exact no-argument package directive below in the leading `//!`
documentation of `src/lib.rs` to publish reusable types, schemas, and localized
documents without producing deployable executable code:

```rust
//! Shared service messages
//! Provides reusable actions and results for compatible services.
//! #[libertas_types_only]
```

A `TypesOnly` package automatically publishes every public package-owned named
type exported from `src/lib.rs`; `LibertasExport` is not a root-selection
marker. A public type or helper function is omitted only when its documentation
contains the exact `/// #[libertas_ignored]` directive. Any other public
function makes the package invalid. The package remains a Cargo library and may
not use `src/main.rs`.

### Published type ownership

Every named type published by an App must be publicly exported or re-exported
under its exact declaration name from that App's root `src/lib.rs`. A type that
is merely public in a private module does not meet this requirement. For a type
owned by another Libertas App, the parser discovers the owning package and emits
its canonical `developer-package::TypeName` reference; a local import or
re-export does not transfer ownership of its schema or documents. An
unqualified external name must be brought into the current Rust scope by a
valid `use`/glob binding. If it cannot be resolved there, parsing fails; the
parser never guesses by searching dependencies for the same terminal name.

### Compatible external representations

Use `#[libertas_foreign_type(developer-package::TypeName)]` on a published,
compatible struct, enum, or type alias to identify the one official type from
another App that it represents. The argument is one direct, positional,
fully-qualified schema name. The declaration is not inherited by nested or
referencing types.

```rust
//! Shared status compatibility
//! Provides a locally named view of status wording from a trusted package.
//! #[libertas_types_only]

/// Portable status
/// Carries status wording that compatible services can show to a person.
#[derive(LibertasExport, LibertasAvroDecode)]
#[libertas_foreign_type(origin-shared_types::SharedStatus)]
pub struct PortableStatus {
    /// Status wording
    /// Reports the current condition in wording supplied by the originating
    /// service.
    pub message: String,
}
```

At runtime materialization, the official type supplies its canonical schema
and documents. Explicit attributes on the compatible local declaration are
then applied as local overrides. The local declaration must remain structurally
compatible; `libertas_foreign_type` is an interoperability mapping, not a type
conversion.

## Build and deployment metadata

Development schema generation reads Rust source. Native deployment has two OS
package sets, and neither belongs to the Libertas type schema:

- **Build packages** provide compilers, generators, headers, and native metadata
  needed before Cargo can link the App. The developer declares these in
  `Cargo.toml` because Cargo metadata cannot reliably infer distribution package
  names from a Rust dependency graph.
- **Runtime packages** own the shared libraries actually linked by the completed
  native App. The Hub derives these from the binary after a successful build and
  records them in that Hub's target manifest.

Logical Libertas dependencies are separate. The
[deployment dependency contract](../deployment.md) owns their discovery,
preflight, exact pinning, and dependency-first deployment. Cargo metadata must
not predict or maintain generated Hub App versions.

### Local development and public release sources

Use local Cargo path dependencies during internal development and pinned public
Git dependencies when those packages are released. Keep one SDK source identity
throughout the native dependency graph; a local SDK and a Git checkout are
separate Cargo crates even when their package names and versions match.

For local paths, Studio uploads the transitive source graph once per canonical
crate directory, including local App crates. It rewrites paths only in the
upload copy, so direct and transitive users of the same SDK resolve to one
crate. Workspace-inherited settings are materialized for that upload. The
upload is content-addressed: the Hub keeps each file's content by SHA-256, and
Studio transfers only contents the Hub does not already hold, so a shared SDK
is sent once and redeploying unchanged files transfers no file bytes. The
schema and localized documents travel the same way, so no deployment is
limited by the size of its translations. Source
size is limited only by the Hub's free disk space, which the Hub checks before
accepting the upload; an upload may contain at most 4096 files. Public
Git dependencies retain their authored repository and revision selectors;
Studio does not replace them with private workspace paths. When moving to a
public release, align SDK Git pins throughout the graph.

Ordinary libraries have no separate deployment queue entry or generated App
wrapper. Build the executable App and let Cargo build its dependencies naturally.
A TypesOnly App installs schema and documents without a native executable build.
A Libertas-style Cargo package name alone does not make a library an App.

These native build inputs do not replace logical App dependencies. The queue
still installs their schemas and documents first and records exact App version
bindings. Deployment does not publish local SDK source to a public repository.

### Declaring native build packages

Declare packages required on every supported Hub under
`package.metadata.libertas.build.packages`. Add target rules when a package is
specific to a Libertas distribution, base distribution, base version, or
architecture:

```toml
[package.metadata.libertas.build]
packages = ["pkg-config", "libssl-dev"]

[[package.metadata.libertas.build.targets]]
distribution = "resolute"
architecture = "arm64"
packages = ["protobuf-compiler"]

[[package.metadata.libertas.build.targets]]
base-distribution = "ubuntu"
base-version = "26.04"
architecture = "x86_64"
packages = ["libclang-dev"]
```

The optional selectors have these exact meanings:

| Selector | Hub value | Current examples |
| --- | --- | --- |
| `distribution` | Libertas distribution codename | `resolute` |
| `base-distribution` | Underlying package ecosystem | `ubuntu` |
| `base-version` | Underlying distribution version | `26.04` |
| `architecture` | Libertas target architecture | `x86_64`, `arm64`, `arm32` |

Every selector in a target rule must match the compiling Hub. All matching rule
package arrays are unioned with the common package array, then validated,
deduplicated, and sorted. A target rule must contain at least one selector and a
non-empty `packages` array. Package names use the Hub's restricted Debian-name
grammar; protected operating-system packages are rejected. A `TypesOnly`
package is not compiled, so its build-package metadata is not evaluated.

The Hub determines target matches and installed prerequisites. When declared
packages are missing, a user with current Hub super-user permission can review
the complete requested and transitive installation plan. The plan installs no
recommended packages and removes no packages. The build is not queued while
approval is pending; rejecting or closing the plan ends deployment.

**Install and Continue** applies the approved short-lived plan, rechecks current
permission and installed versions, and retries the same source revision. A lost
connection can retry that approved operation without authorizing a different
plan. The Hub checks prerequisites again before queueing the build.

Build-package installation requires Hub super-user permission. The Hub never
guesses a package from compiler error text and never installs an undeclared
package automatically. APT resolves declared names to available versions,
virtual-package providers, and transitive dependencies; it does not determine
which Ubuntu package a Rust project needs. If native code needs an undeclared
header or tool, Cargo fails normally and the developer must correct
`Cargo.toml`.

### Runtime distro dependencies

After linking, the Hub resolves the Debian packages that own the native shared
library's loaded libraries. Those packages form the runtime target manifest:

```json
{
  "target": { "distribution": "resolute", "architecture": "arm64" },
  "DistroDependencies": {
    "manager": "apt",
    "packages": ["libc6", "libgcc-s1"]
  }
}
```

The `DistroDependencies` runtime list is validated, deduplicated, sorted, and installed before the App
is persisted. Build-only packages are deliberately not copied into this runtime
list. Every Hub stores only its own separately versioned manifest, including
the deterministic Studio source revision and artifact/schema/document hashes.
The dedicated Libertas Plaza center document's **Publish current project**
wizard resolves local Rust path and schema dependencies, publishes local
dependencies before the selected project, leaves dependencies without local
projects unchanged, and publishes the selected Hub manifests as immutable
distro/architecture targets. See
[Plaza publication contract](../publication.md#hub-target-manifest)
for the complete JSON contract and append-only publication rules.

## Parser attribute reference

The [source annotation reference](../annotations.md) owns the
complete canonical TypeScript/Rust inventory, placement, argument forms, and
links to each detailed contract. Use its Rust column together with the
language-specific examples above. Emitted JSON properties do not imply macros.

### Sending workflow edits

Use `libertas_endpoint_edit_request(endpoint, &request, trans_id, peer)` to
reply with a complete Request value for editing. Pass the transaction and opaque
peer from the incoming operation unchanged. The client installs the payload
directly and waits for the user to submit. Cookie responses instead use the
ordinary `libertas_endpoint_response` helper and `#[libertas_workflow_request]`.
See the [workflow contract](../protocols.md#workflow-interactions).

`#[libertas_foldable]` permits manual folding on a struct, union, or list type or field; `#[libertas_foldable(initially_open = false)]` starts it collapsed. Both are compiled into the [Display Template](../presentation.md#display-template). See [folding structs and unions](../presentation.md#folding-structs-and-unions).
`#[libertas_inline_choices]` on an enum declaration or an enum-typed field shows its choices inline as radio controls and is compiled into the [Display Template](../presentation.md#display-template); see [inline choices](../presentation.md#inline-choices).

## Backend-constrained choices

`#[libertas_constrained_by("$.allowed_choices")]` constrains an enum or union request field using a hidden,
backend-provided integer list in the current request. See the
[complete contract](../protocols.md#backend-constrained-choices) for numeric identity,
initialization, immutability, validation, and payload editing.


Default endpoint entry uses `#[libertas_default_request]` on a Request variant whose payload needs no user input (Nil, or only hidden/read-only Fixed or Default fields, such as a hidden `#[libertas_fixed("current-user")]` caller). Response-level ConstrainedBy filters NextRequest through a hidden integer array. See [default entry and dynamic actions](../protocols.md#default-request-entry-and-dynamic-actions).

For additional response controls, put `#[libertas_action_control("sections")]`
and `#[libertas_hidden]` on a direct `Vec<i32>` field of a Response or
SubscriptionData payload with `NextRequest`. The field's values are original
protocol Union Request discriminants; `"Actions"` names the built-in button.
See [named action controls](../protocols.md#named-action-controls).

### Union member identity and payload constraints

Rust payload enums emit an explicit numeric `Discriminator` on every Union
member from the native encoded variant index, consecutively from zero and in
that same Children order. A single unnamed payload remains
one schema member named after the variant: `VariantA(u32)` needs no fabricated
child field. `#[libertas_fixed(7)]` constrains that payload independently of its
wire discriminator. No new discriminator macro is required. Rust never emits `UnionTagField`;
that marker belongs exclusively to ScriptLanguage native tag fields. See
[the Union schema contract](../schema.md#54-unions).

For large workflow request payloads, `#[libertas_cache_identity]` marks direct root-record
identity fields for opt-in draft recovery. Small requests omit it. See the
[CacheIdentity contract](../annotations.md#cacheidentity) for immutable multi-field
identity, optional application protocol revision, and restoration rules.

## Subscription array append

On a report's root array field:

```rust
#[libertas_append_to("Entries.entries")]
#[libertas_append_index("$.start_index")]
added_entries: Vec<Entry>,
```

`AppendTo` names one working variant and root array. Optional `AppendIndex`
references the report's non-null integer field holding the expected current
length. Elements append in order, retaining duplicates and assignable nulls;
invalid indices or resulting values reject the entire report. The App supplies
complete initial snapshots and owns ordering and retention. See the
[array append contract](../protocols.md#subscriptiondata-array-append) for
placement, combining CopyTo with append, exclusion of index controls from copied
data, writer conflicts, independent recipients, and live charts.
