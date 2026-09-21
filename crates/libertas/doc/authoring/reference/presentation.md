# Presentation and localization

Presentation attributes preserve the underlying typed value and wire encoding.
They tell Studio and clients how to validate, edit, localize, summarize, or
chart the value.

## Source documentation

Extraction syntax is only part of the contract. Follow the
[schema documentation invariants](documentation.md) for every
package, function, type, parameter, field, member, variant, and List half. A
public function must additionally follow the
[developer function-comment invariants](function-comments.md) so its
discovery description is complete, written in active voice, and explicit about
who performs every action.

TypeScript uses leading `///` package comments and JSDoc on declarations:

```typescript
/// Weather monitor
/// Helps people review recent conditions measured at one weather station.

/**
 * Weather measurements
 * Lists recent readings from the selected weather station.
 * ----
 * Weather measurement
 * Records one observation made by the selected weather station.
 */
samples: Sample[];
```

Rust uses inner rustdoc for the package and outer rustdoc for declarations:

```rust
//! Weather monitor
//! Helps people review recent conditions measured at one weather station.

/// Weather measurements
/// Lists recent readings from the selected weather station.
/// ----
/// Weather measurement
/// Records one observation made by the selected weather station.
pub samples: Vec<Sample>,
```

The first line is the display name. Remaining ordinary text is the description.
For a List, at least four hyphens separate container documentation above from
item documentation below. Put the divider above custom document properties that
belong to the item. Plain code comments are ignored by document extraction.

Codebots always author at least one natural label line and one complete
description line for every authored node. Put custom `[PropertyName]` blocks
after that ordinary text and before the first TypeScript `@Libertas...` tag.
They also provide a useful localized `[DefaultText]` for every authored Endpoint
server and `[DefaultTaskName]` for every authored App function, giving each new
endpoint or task a sensible editable name without asking the person to invent
one. A function uses `[DefaultTaskName]`, not `[DefaultText]`.
The full standard and custom extension contract, including `[DefaultText]`,
`[DefaultTaskName]`, reserved names, List ownership, localization, and
user-friendly values, is in the
[schema documentation invariants](documentation.md#custom-document-properties).

## AI tool-call descriptions

The Hub appends value-representation guidance to AI tool parameter and response
descriptions from the effective schema attributes, including inherited
attributes, and built-in date/time types. Authored descriptions remain intact.
The attribute sections below identify the generated guidance; App authors and
CodeBots do not need to repeat it in source comments. Keep comments focused on
domain meaning, scope, consequences, and any App-specific limits. This does not
remove the requirement for a natural label and complete description on every
authored node.

These hints describe the tool JSON representation. They do not change App
storage or protocol encoding, supply missing values or dynamic choices, or
execute copying or calculations.

## Common field metadata

`SizeMin`/`SizeMax` apply to Lists, Strings, and Bytes: respectively element
count, UTF-16 code-unit length, and byte count. Both TypeScript
`@LibertasSize(min=..., max=...)` and Rust
`#[libertas_size(min=..., max=...)]` emit these same attributes. The bounds are
inclusive; size zero is valid unless excluded by the minimum. Nullable null
remains separate from an empty value.

User and group pickers do not implicitly select the current user or a singleton
candidate. Declare an explicit default using the
[User and group default contract](schema.md#user-and-group-defaults).

| Meaning | TypeScript | Rust |
| --- | --- | --- |
| Initial value | `@LibertasDefault(value)` | `#[libertas_default(value)]` |
| Summary/header field | `@LibertasUIHeader(use_parent_name=true)` | `#[libertas_ui_header(use_parent_name = true)]` |
| Inline long description | `@LibertasUIIncludeDescription` | `#[libertas_ui_include_description]` |
| Not editable | `@LibertasReadOnly` | `#[libertas_read_only]` |
| Hidden from ordinary editor | `@LibertasHidden` | `#[libertas_hidden]` |
| Unordered List | `@LibertasUnordered` | `#[libertas_unordered]` |
| Scoped uniqueness | `@LibertasUnique(level)` | `#[libertas_unique(level)]` |
| Size bounds | `@LibertasSize(min=...,max=...)` | `#[libertas_size(min=...,max=...)]` |
| Live Number/DateTime minimum | `@LibertasRangeMin("expression")` | `#[libertas_range_min("expression")]` |
| Live Number/DateTime maximum | `@LibertasRangeMax("expression")` | `#[libertas_range_max("expression")]` |
| Data-backed choice index | `@LibertasEnumSource("$.items")` | `#[libertas_enum_source("$.items")]` |

`Hidden` suppresses the complete value subtree, including list elements and
controls, nested fields, summaries, and promoted values. Hidden data remains
part of the typed value for validation and protocol encoding. Layout flattening
does not expose hidden descendants. `TableHeader` is the placement-sensitive
exception: look it up directly on the field declaration for promotion and
directly on any type declaration with children for flattening. Do not inherit or merge
these markers through ordinary attribute resolution; neither overrides `Hidden`.

`ReadOnly` on an Array/List prevents changing its membership or order: no Add,
Remove, or reorder action may mutate that list. It does not recursively make
member values read-only; members and their fields keep their own editability.
To prohibit member edits, mark the applicable member type or fields as well.
Context menus, swipe actions, pickers, and deferred actions must enforce the
same list-owner rule at both presentation and execution. A promoted TableHeader
row does not change which list owns a removal or reorder operation.

For header promotion and nested Union layout, see
[TableHeader: promoted fields and Union selectors](#tableheader-promoted-fields-and-union-selectors).

`EnumSource` belongs on an unsigned integer. The integer stores the selected
List item's zero-based index. It does not store the item itself.
AI tool-call descriptions automatically explain that index representation and
identify the source List path; they do not enumerate its current items.
Android and iOS present a single-choice picker in source order. Each choice
uses the item's scalar display value or its promoted `TableHeader`; a composite
without a scalar header uses its display name. Numbered labels distinguish
repeated names. Money and explicit scalar formats retain their normal display.

The source uses a [Libertas Schema Path](schema.md#context-and-syntax) in the
current data tree; it does not read the interaction reference context. An explicit Default or
Fixed retains ordinary initialization; otherwise the selection is unanswered.
Nullable absence remains valid. A present selection must identify a current
List item and satisfy the field's numeric bounds and step, including dynamic
bounds. An empty or unavailable source offers no selectable options.

Path traversal follows the [shared resolution rules](schema.md#context-and-syntax),
including reusable named types, List/Union levels, and each enclosing use.
A selector in a reusable type is validated in configuration, protocol, and stored
data contexts where it occurs; its declaration does not introduce another root.

Cancel preserves the previous value. A source-row replacement, reorder, removal,
or label change while the picker is open rejects that pending selection;
reopen the picker to choose from the current list. A saved index remains a
position, not a persistent record identity. Apps that need record identity
must resolve the selection against the appropriate accepted-data revision.

`RangeMin` and `RangeMax` constrain numeric controls and date/time choosers
with the same inclusive limits used for final validation. Their expressions
must produce bounds compatible with the control’s semantic type; see
[dynamic numeric ranges](validation-rules.md#31-dynamic-numeric-ranges) for
result typing, unavailable bounds, and intersection with static constraints.

`UIIncludeDescription` is a no-argument, client-only presentation marker. The
UI node keeps its localized short `DisplayName` and also renders its localized
long `Description` inline. Without the marker, clients retain their ordinary
compact short-name presentation, including a description tooltip where one is
available. The marker does not affect stored values, requests, or wire encoding.

For `Unordered`, AI tool-call descriptions automatically explain that order has
no semantic meaning and that this attribute does not prohibit duplicates.

## TableHeader: promoted fields and Union selectors

`@LibertasUIHeader` in TypeScript and `#[libertas_ui_header]` in Rust have an
optional Boolean argument, `use_parent_name`, which defaults to `true`.
Placement controls two independent presentation choices.
Use direct lookup on each declaration, not effective-attribute resolution:
the field declaration controls promotion; any type declaration with children
(Union, Struct/Table, Array/List) controls flattening. Resolving a type reference locates the declaration but does not
transfer a field marker to it.
Neither changes the encoded data hierarchy.

### Field promotion

On a field directly owned by a **Table**, the first visible marked field in schema
order becomes the Table row's value instead of occupying a separate child row.
Additional marked fields remain ordinary children. A field marker has no effect
when its owner is not a Table, including a Union's selected branch.

The promoted field supplies the row value. Its `use_parent_name` option chooses
the row label, uniformly for scalar, Table, List, and Union fields:

- `true` (the default): use the owning Table/struct's display name.
- `false`: use the promoted field's display name.

For a promoted Union, the selected variant's display name supplies the row value;
the option changes the label, not that selected value. Tapping that value opens
the choice selector. Selecting another variant replaces the applicable fields.
The selector remains available even when the selected variant has no fields.
A List member appends its one-based position to the chosen row label
(for example, “Household #1” with the default or “Marital status #1” with
`use_parent_name=false`), including for a Union.
A scalar header keeps its normal formatting and editor. A composite header
keeps its children, and a List header retains its entries and Add action.

An unpromoted Union retains its field title after selection. For example,
“Marital situation” remains the label and “Married” is the selectable value.
A Union with a scalar payload also identifies the selected branch while keeping
that payload editable. Branch selection remains available independently of
`TableHeader`, including after choosing a fieldless variant.

### Name option, encoding, and compatibility

The source declarations are:

```typescript
/** @LibertasUIHeader */                       // use_parent_name defaults to true
/** @LibertasUIHeader(use_parent_name=true) */
/** @LibertasUIHeader(use_parent_name=false) */
```

```rust
#[libertas_ui_header]                          // use_parent_name defaults to true
#[libertas_ui_header(use_parent_name = true)]
#[libertas_ui_header(use_parent_name = false)]
```

In schema JSON, the attribute's Boolean string value carries this option:
`{"Name":"TableHeader","Value":"true"}` selects the parent name and
`{"Name":"TableHeader","Value":"false"}` selects the field name when placed on a field,
or hides a struct declaration's name row as described below. Attribute
presence enables promotion or flattening; `false` does **not** disable TableHeader.
Omitting the source argument emits `Value="true"`. Existing schemas already
carry `TableHeader=true`, so updated clients read them as `use_parent_name=true`
without rewriting those schemas. Older clients need not understand the new
explicit-false form. Removing the attribute disables its layout effect.

### Example: use the parent name to identify the record

A struct has the display name **Next schedule**. Its **Starts at** field is a
scalar TableHeader with `use_parent_name=true` (or the argument omitted), and
**duration** is an ordinary sibling field. For a start time displayed as
`09:00` and a duration of `30 minutes`, the rows are:

```text
Next schedule: 09:00
  duration: 30 minutes
```

The parent name tells the reader what the time and duration describe: the next
schedule. Using the field name instead would produce `Starts at: 09:00`, which
describes the time but loses the record's identity when several schedules or
other timed activities appear on the same screen. The promoted start time acts
as the schedule's summary value, so the parent name is useful here.

### Example: use the field name to identify the property

A struct has the display name **HouseHold**. Its **Marital status** field is a
Union marked TableHeader with `use_parent_name=false`. The selected variant has
the display name **Married**, and **Number of dependents** is an ordinary sibling
field. The rows are:

```text
Marital status: Married
  Number of dependents: 2
```

The field name tells the reader exactly which property “Married” answers.
The default would produce `HouseHold: Married`; “HouseHold” identifies the
record but does not name that property. Keeping **Marital status** makes the
selected choice understandable without requiring the reader to infer the
question from its answer.

In both examples, the option changes only the promoted row's label. The field
still supplies its value, sibling fields retain their normal placement, and
the Union selector still changes the selected variant.

### Types with children

On **any type declaration with children** (Union, Struct/Table, Array/List),
`TableHeader` puts the children at the same indentation as their owning row.
For a Union these are the selected children, and its selector remains visible
and editable. Without that type marker, children retain normal indentation.
This marker does not promote a referencing field into its owning Table: mark
that field separately when wanted. Both Boolean values retain the flattening effect. On a **struct/Table
declaration**, `use_parent_name=false` also omits its name-only row; its children
remain visible and editable at the flattened level. A promoted field still has
a value row, using its own field name when the struct name is suppressed. A
List member retains its position and membership controls without the struct
name. A suppressed struct row cannot collapse its children. Hidden still
suppresses the whole subtree.

On Union and Array/List declarations the option does not hide their selector or
collection row. The field's direct option controls ordinary promotion naming;
a referenced type's option is not inherited as a field attribute.
Field promotion alone does not flatten the selected children.

For example, these independent placements express both choices:

```text
Household (Table)
  dependents (display name: Dependents; field: TableHeader; type: Dependents)

Dependents (Union type: TableHeader)
  No
  Yes (Table)
    count (Number)
```

With `Yes` selected, the schematic rows are:

```text
Household: Yes             [tap value to change choice]
Count: ...
```

Removing the marker from the Union type indents `Count` under `Household`.
Removing the field marker restores a separate Union row under `Household`.
The choices can be repeated at each nesting level. An unrelated sibling is not
flattened because another field is a promoted or flattened Union.
Existing expansion, visibility, and read-only rules still apply; `TableHeader`
does not make a read-only selector editable.

### Presentation only

Header promotion does not change the schema or encoded data hierarchy.
`CopyFrom`, `CopyTo`, `EnumSource`, validation paths, and backend field access
still use the original structural paths, including every nested Union/table
level. It does not supply a default, make required fields optional, or disable
fields based on another answer.

Protocol actions remain owned by the enclosing protocol variant, even when its
header is another Union. See [protocol transaction stack](endpoint-presentation.md#protocol-transaction-stack).
Separate same-indent Union rows retain their own edit/removal ownership;
sharing an indentation level does not make them the same data object.

## Date and time

| Type | Raw meaning | Presentation |
| --- | --- | --- |
| `LibertasDateTime` | Epoch seconds | Viewer-local date/time; optional timezone in `Format`. |
| `LibertasTimeOnly` | `0 <= seconds < 86400` | Time chooser, no date. |
| integer plus `DateOnly` | Valid decimal Gregorian `yyyyMMdd` | Calendar-date chooser, no timezone. |
| number plus `TimeInterval` | Duration seconds | Duration editor/label. |

Never convert `DateOnly` through an epoch or timezone. Defaults, fixed values,
requests, and runtime data must all be valid calendar dates.

AI tool-call descriptions automatically specify the tool JSON representation:
`DateOnly` uses a `YYYY-MM-DD` calendar-date string without timezone conversion;
`DateTime` uses an ISO 8601 date-time string with an explicit UTC offset or `Z`;
`TimeOnly` uses an `HH:MM:SS` string without a date; `TimeInterval` uses numeric
seconds, with a conversion example. These representations leave the raw App
values in the table above unchanged.

## Display formats

`Format` accepts one double-quoted, programming-language-neutral Unicode LDML
pattern. It never changes parsing, storage, comparison, bounds, defaults,
requests, persistence, or chart geometry.

AI tool-call descriptions automatically warn that `Format` affects display
only: arguments use the underlying tool value without display formatting or
display scaling.

```typescript
/**
 * Invoice total
 * Shows the billed total with two decimal places and parentheses when it is negative.
 * @LibertasNumber(type=f64)
 * @LibertasFormat("#,##0.00;(#,##0.00)")
 */
amount: number;

/**
 * Service date
 * Identifies the calendar day when the service occurs.
 * @LibertasNumber(type=u32)
 * @LibertasDateOnly
 * @LibertasFormat("yyyy-MM-dd")
 */
serviceDay: number;
```

```rust
/// Invoice total
/// Shows the billed total with two decimal places and parentheses when it is negative.
#[libertas_format("#,##0.00;(#,##0.00)")]
pub amount: f64,

/// Service date
/// Identifies the calendar day when the service occurs.
#[libertas_date_only]
#[libertas_format("yyyy-MM-dd")]
pub service_day: u32,
```

The portable number grammar supports `0`, `#`, `.`, `,`, positive/negative
subpatterns separated by `;`, `%`, `‰`, `E`, affixes, and apostrophe quoting.
Currency symbol `¤` and currency code `¤¤` are supported with explicit
`Money` metadata. Significant digit `@`, padding `*`, wider currency tokens,
and rounding-increment digits are not supported.

Date fields are `G y M d E`; time fields are `H h m s S a`; `z` is available
only for DateTime. DateOnly accepts only date fields, TimeOnly only time fields,
and one pattern cannot combine `H` with `h`.

References:

- [UTS #35 number patterns](https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns)
- [UTS #35 date/time patterns](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)

## Money

Use `Money` on a signed `i64` Number to store an exact amount in minor units.
The currency code and scale are explicit schema metadata: `USD/2` means that
integer `12345` represents USD 123.45. Rust storage and Avro remain `i64`,
not a decimal string or floating-point amount. Clients may retain an integer
string internally to avoid a JavaScript `Number` conversion.

AI tool-call descriptions automatically include the declared currency, scale,
and integer minor-unit conversion, and require the exact integer without a
currency symbol or decimal amount.

```rust
/// Wage amount in USD cents.
#[libertas_money("USD", 2)]
pub wages: i64,
```

```typescript
/** @LibertasNumber(type=i64) @LibertasMoney("USD", 2) */
wages: number;
```

Both annotations emit `{"Name":"Money","Value":"USD/2"}`. The code is three
uppercase ASCII letters identifying the currency; the scale is an integer
from 0 through 6. Choose the currency's applicable minor-unit scale, such as
USD/2, JPY/0, or KWD/3. Locale and currency-data updates never alter that saved
meaning. This uses [CLDR currency formats and metadata](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies);
CLDR does not prescribe the application's numeric storage type.

Without `Format`, the viewer's locale supplies currency placement, symbol,
digits, and separators, with the declared scale. An optional currency pattern,
for example `¤#,##0.00;(¤#,##0.00)` or `¤¤ #,##0.00`, controls presentation only.
Currency-pattern display rounding uses half up; it never rounds the stored
amount or implements tax-law rounding. Quoted `¤` remains literal text.
Money patterns cannot use percent, per-mille, or scientific notation.

Android and iOS edit money in place. The editor shows the currency code beside
an ungrouped amount using locale digits and its decimal separator. Committing
`123.45` for USD/2 stores `12345`. Grouping separators, currency symbols inside
the amount, excess fractional digits, and values outside signed i64 are
rejected without replacing the entered text. `ReadOnly` amounts remain visible
and cannot be edited. Nullability follows the ordinary schema contract.

`Default`, `Fixed`, `NumberMin`, `NumberMax`, and `NumberStep` are integer minor
units; the step must be positive. Dynamic bounds and ValidationRules also
operate on minor units and preserve exact numeric values. Currency identity
does not introduce conversion: applications own exchange rates and must keep
unrelated currencies separate. Money cannot coexist with DateOnly,
TimeInterval, EnumSource, Bitflags, or PhysicalUnit.

Charts retain each source integer for labels, tooltips, category identity, and
accessibility. Coordinates and synthesized scale ticks use the chart's normal
floating-point geometry; their presentation applies the declared currency and
scale. Shared scales and paired interval endpoints require matching Money
metadata. A display pattern can intentionally hide lower digits without
changing the source amount.

## Physical units

`PhysicalUnit` accepts a canonical CLDR core unit identifier on a Number:

```typescript
/**
 * Daily water depth
 * Sets the depth of water applied during one day.
 * @LibertasNumber(type=f64)
 * @LibertasPhysicalUnit("millimeter-per-day")
 */
rate: number;
```

```rust
/// Daily water depth
/// Sets the depth of water applied during one day.
#[libertas_physical_unit("millimeter-per-day")]
pub rate: f64,
```

Simple, powered, product, and generated compound identifiers are valid under
the [UTS #35 unit-identifier contract](https://unicode.org/reports/tr35/tr35-general.html#Unit_Identifiers).
All physical units, including compound units, are localizable. Keep the canonical
identifier in the schema; the viewing locale supplies unit presentation in
ordinary UI and charts. Translate field names, descriptions, chart titles, and
other authored text through the App's locale documents, not by changing unit
identifiers or appending translated unit strings to numeric values.
Number-wheel editors show the localized unit in place of the generic “Level”
label, including when the field has no initial value. Prefer the full unit name
when it fits the available label width; otherwise use its localized short form.
Schema validity is independent of a particular client ICU catalog. A client may
omit unit decoration when it cannot format an otherwise valid identifier; the
numeric value and metadata remain intact.

The identifier states the provided unit and never performs conversion.
`percent` raw `50` means fifty percent; it does not rescale `0.5`. `Format` may
coexist and formats only the numeric portion. PhysicalUnit cannot coexist with
DateOnly, TimeInterval, or EnumSource.

AI tool-call descriptions automatically include the exact `PhysicalUnit`
identifier and explain that no automatic conversion occurs. For `percent`,
they also explain that `50` means fifty percent.

## Bitflags

Bitflags project a localized Enumeration onto an ordinary integer. Each
nonzero catalog member is a unique single bit and one zero label is allowed.
The field may combine declared bits but cannot contain undeclared bits. Clients
show localized selected labels and use a multiple-choice editor. The integer
wire value is unchanged.

AI tool-call descriptions automatically explain the integer mask, bitwise OR
combinations, and prohibition of undeclared bits, and include the declared
flag values and available localized member descriptions. Author the meaning
and consequences of each flag in its member comments.

Bitflags cannot coexist with DateOnly, TimeInterval, EnumSource, Format, or
PhysicalUnit.

## String resources and formatted output

One package-owned catalog supplies notification templates, permission reasons,
data-name templates, `FormattedText`, and formatted chart labels for all
functions and types in the App. Task package context identifies the catalog;
encoded tuples carry no function/type owner or resource FQN.

### AI tool-call output

The Hub renders each `FormattedText` tool result as a localized JSON string
using the emitting package's resources and locale. Templates and the resource
catalog remain internal to rendering; they are not included in the AI tool
description or result. If a value cannot be resolved or rendered, its result
is `{"error":"Unresolved FormattedText"}` rather than guessed text.

This output view does not change the canonical LMF1 tuple. Read-only request
fields retain their original Base64 Bytes representation in tool arguments;
rendered text cannot replace that payload for copying or resubmission.
Ordinary Bytes remain unchanged. Declare `FormattedText` and document the
message's domain purpose; do not repeat templates or rendering instructions in
field comments for the AI.

### Static read-only placement and request values

Every `FormattedText` node MUST be statically `ReadOnly`. Declare it on the
field or its effective declared type, or place the node inside a statically
read-only Table. `ReadOnly` on a List protects membership and order only; it
does not make its elements or their fields read-only. Mark the applicable
element type or field when that is the only enclosing protection.

This requirement applies to every use, including Response and SubscriptionData
output and dormant or `TypesOnly` declarations. An output role, hidden field,
or runtime editing restriction does not replace the static declaration. Shared
Studio validation checks resolved types, inheritance, containers, and
materialized external dependencies. Language parsers extract the attributes.

Request and SubscriptionRequest fields may carry these display-only values.
Their values come from `CopyFrom` or `CopyTo`, including a copy of an enclosing
record or request root; supplied `EditRequest` values; or `Default`/`Fixed`
initialization. A leaf does not need its own `CopyFrom` when one of those paths
supplies it. Apply the [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules)
to copied values. The client never asks the user to edit a formatted tuple.
An unanswered local field may remain undefined; a required undefined field
blocks submission. Null is valid only where the destination permits it.

`FormattedText` remains forbidden in function/configuration arguments, even
when statically read-only. Reusable types must satisfy this placement rule and
that runtime-use restriction on every path.

### Persisting formatted record payloads

`FunctionDataSchema` record payloads may contain statically read-only
`FormattedText`. Persistence is supported, but generally discouraged for these
payload fields: `FormattedText` supplies runtime rendering, so prefer storing
structured domain values and constructing the formatted value at presentation
time. This is design guidance, not a schema rejection rule.

This recommendation does not apply to [DataName](message-format.md#data-and-database-names).
Using the canonical resource-key and typed-argument tuple for database and
record names is a supported use: it supplies both storage identity and the
semantic information for localized human display.

### Declaring and encoding formatted values

```typescript
/// Room condition reports
/// Gives occupants clear summaries of current room conditions.
/// @LibertasStringResources(STATUS_STRINGS)

const STATUS_STRINGS = {
    READY: "Room condition reporting is ready.",
    ROOM_STATUS: "{0,LibertasDevice} reports a room temperature of {1,number,0.0}."
} as const;

/**
 * Room condition
 * Describes the latest reported condition in one room.
 */
export interface RoomState {
    /**
     * Room summary
     * Gives occupants a short overview of the room's latest condition.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    summary: Uint8Array;

    /**
     * Current condition
     * Tells occupants what is happening in the room and includes the relevant measurement.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    status: Uint8Array;
}
```

```rust
//! Room condition reports
//! Gives occupants clear summaries of current room conditions.
//! #[libertas_string_resources(STATUS_STRINGS)]

pub const STATUS_STRINGS: &[(&str, &str)] = &[
    ("READY", "Room condition reporting is ready."),
    ("ROOM_STATUS", "{0,LibertasDevice} reports a room temperature of {1,number,0.0}."),
];

/// Room condition
/// Describes the latest reported condition in one room.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub struct RoomState {
    /// Room summary
    /// Gives occupants a short overview of the room's latest condition.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub summary: Vec<u8>,

    /// Current condition
    /// Tells occupants what is happening in the room and includes the relevant measurement.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub status: Vec<u8>,
}
```

For `ROOM_STATUS`, encode argument 0 as the `LibertasDevice` object and
argument 1 as a unit-qualified temperature number. Do not pass a translated
state as a literal or resource-text grammar fragment. A closed Enumeration
instead uses `{1,LibertasEnum,ActualEnumType}` and an integer argument encoded
with that Enumeration's semantic type.

FormattedText always uses the public SDK's canonical Bytes encoding of a
resource key plus ordered typed Libertas Message Arguments, including an empty
argument list. Do not Base64-wrap it in App code. A String host would only
encode the tuple and force the renderer to decode it again, providing no
benefit; Bytes also keeps a sharp type distinction from literal String data and
prevents authoring mistakes. A generic JSON byte codec may use Base64 as a
transport spelling without changing the field's Bytes type. Runtime payloads
never carry format templates.

TypeScript calls `libertasFormattedText(resourceName, arguments)` and Rust calls
`libertas_formatted_text(resource_name, arguments)`. These public encoders
return the byte value directly. App code ordinarily does not decode it; the
presentation client resolves and renders LMF1.

Use zero-based LMF1 positions so translations may reorder arguments:

| Value | Form |
| --- | --- |
| Untyped value | `{0}` |
| Libertas device | `{0,LibertasDevice}` |
| Libertas enum | `{0,LibertasEnum,StatusEnum}` |
| Styled number | `{0,number,0.00}` |
| Plural | `{0,plural,one {# item} other {# items}}` |

Every translation preserves argument positions, types, styles, selector keys,
plural offsets, braces, and apostrophe escapes. The encoded Libertas Message
Argument type takes precedence over a conflicting template type.

## Charts

This is a compact authoring summary. The normative vocabulary, validation
matrix, renderer behavior, and audited text-field dispositions are in
[Client-side charting](charts/2d.md). Language parsers own native
syntax and extraction; Studio's shared chart validator owns portable semantic
validation.

Chart metadata belongs on a Response or SubscriptionData payload. A primitive
chart is a List of row Tables; the List declares the mark, and direct scalar row
fields declare channels.

```typescript
/**
 * Temperature reading
 * Records the temperature observed at one time.
 */
export interface Sample {
    /**
     * Observation time
     * Identifies when the temperature was observed.
     * @LibertasChartChannel(x, tooltip)
     * @LibertasChartScale(id=time, kind=utc)
     */
    at: LibertasDateTime;

    /**
     * Observed temperature
     * Reports the temperature measured at the observation time.
     * @LibertasNumber(type=f64)
     * @LibertasPhysicalUnit("celsius")
     * @LibertasChartChannel(y, tooltip)
     * @LibertasChartScale(id=temp, kind=linear)
     */
    temperature: number;
}

/**
 * Temperature history
 * Lists temperature readings in observation order.
 * ----
 * Temperature reading
 * Records the temperature observed at one time.
 * @LibertasSubscriptionData
 * @LibertasChart(line)
 */
export type History = Sample[];
```

```rust
/// Temperature reading
/// Records the temperature observed at one time.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub struct Sample {
    /// Observation time
    /// Identifies when the temperature was observed.
    #[libertas_chart_channel(x, tooltip)]
    #[libertas_chart_scale(id = time, kind = utc)]
    pub at: LibertasDateTime,

    /// Observed temperature
    /// Reports the temperature measured at the observation time.
    #[libertas_physical_unit("celsius")]
    #[libertas_chart_channel(y, tooltip)]
    #[libertas_chart_scale(id = temp, kind = linear)]
    pub temperature: f64,
}

/// Temperature updates
/// Describes the temperature observations that can arrive over time.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub enum ChartProtocol {
    /// Temperature history
    /// Lists temperature readings in observation order.
    /// ----
    /// Temperature reading
    /// Records the temperature observed at one time.
    #[libertas_subscription_data]
    #[libertas_chart(line)]
    History(Vec<Sample>),
}
```

Marks are `point`, `line`, `area`, `bar`, `rect`, `rule`, `arc`, `text`, and
`polygon`. Compositions are `layer`, `hconcat`, and `vconcat`; each direct
composition child is independently a chart.

The closed channel vocabulary is:

- positions: `x`, `x2`, `y`, `y2`, `xOffset`, `yOffset`, `theta`, `theta2`,
  `radius`, `radius2`;
- appearance: `color`, `fill`, `stroke`, `opacity`, `fillOpacity`,
  `strokeOpacity`, `size`, `shape`, `angle`, `strokeWidth`, `strokeDash`;
- text: `text`, `tooltip`;
- structure and identity: `detail`, `order`, `key`; and
- facets: `row`, `column`.

A field has at most one scale-bearing channel. Secondary endpoints require
their primary and cannot own a scale. `tooltip` and `detail` may appear on
multiple fields.

Scale types are `linear`, `log`, `symlog`, `sqrt`, `time`, `utc`, `ordinal`,
`band`, `point`, and `identity`. Use continuous numeric scales for physical
quantities. DateTime position infers time; DateOnly uses timezone-free calendar
day geometry. Categorical Enumeration and Libertas object references retain
their raw identity while clients resolve localized labels. Plain Bytes are not chart scalars.

Every value that can become chart text keeps its semantic presentation source.
Use an Enumeration for a closed App vocabulary, FormattedText Bytes for dynamic
App-authored wording, and `LibertasDevice`, `LibertasVirtualDevice`,
`LibertasUser`, `LibertasEndpoint`, or `LibertasLanDevice` when the value is a
system object. A visible plain String is only genuine literal domain/user data,
such as a proper name or free-form annotation. This applies to axes, facets,
legends, text marks, tooltips, guides, and accessibility in every chart family.
Key-only and guide-less identity appearance Strings are the non-textual
exceptions. Never flatten an Enumeration or copy an object display name into a
chart-only String.

FormattedText remains a byte blob end to end. Its tuple carries a package-local
resource key and typed arguments, not a type owner or resource FQN; the task
identifies the package's one `StringResources` catalog. Generic JSON may encode
the Bytes value as Base64 at that transport boundary, but the chart field is
never a String. Encoding it as a String only to decode it for rendering has no
benefit and would erase the sharp schema distinction from literal text.

Source scale properties are `id`, `kind`, `min`, `max`, `mid`, `zero`, and
`reverse`; the extracted and final language-neutral spelling of source `kind`
is `type`.
Fields sharing a nonempty scale ID must have compatible scalar type, channel,
Format, and PhysicalUnit. Different but convertible units are still different
scales unless the App emits one common unit.

Use `@LibertasChartGuide(target=..., source=...)` or
`#[libertas_chart_guide(target = ..., source = ...)]` only for an explicit
guide. Targets are `x`, `y`, `xOffset`, `yOffset`, `theta`, `radius`, `color`,
`fill`, `stroke`, `opacity`, `fillOpacity`, `strokeOpacity`, `size`, `shape`,
`angle`, `strokeWidth`, `strokeDash`, `row`, and `column`. Sources are `scale`,
`value`, `span`, or `none`; `value` is limited to `x`, `y`, `theta`, or
`radius`, while `span` is limited to continuous Cartesian `x/x2` or `y/y2`.
The required properties are `target` and `source`. Optional properties are
`position`, `title`, `domain`, `ticks`, `labels`, and `grid`; positions are
`auto`, `top`, `right`, `bottom`, or `left`, and component switches are `auto`
or `none`. Prefer inferred scale guides unless the App needs an explicit
override.

### 3D charts

Use the parallel five-attribute namespace for 3D output:
`@LibertasChart3D`, `@LibertasChart3DChannel`, `@LibertasChart3DScale`,
`@LibertasChart3DGuide`, and `@LibertasChart3DView` in TypeScript, or
`#[libertas_chart3d(...)]`, `#[libertas_chart3d_channel(...)]`,
`#[libertas_chart3d_scale(...)]`, `#[libertas_chart3d_guide(...)]`, and
`#[libertas_chart3d_view(...)]` in Rust. Do not substitute the 2D attributes or
mix the two namespaces on one chart path.

Primitive marks are `point`, `line`, `bar`, `surface`, `wireframe`, and `stem`;
compositions are `layer`, `hconcat`, and `vconcat`. The channels are `x`, `x2`,
`y`, `y2`, `z`, `z2`, `u`, `v`, `color`, `fill`, `stroke`, `opacity`,
`fillOpacity`, `strokeOpacity`, `size`, `shape`, `strokeWidth`, `strokeDash`,
`tooltip`, `detail`, `order`, `key`, `row`, and `column`. Scale properties
mirror 2D scale properties; write `kind` in source and let extraction emit
schema `type`. A 3D scale also accepts `logBase > 1` for `kind=log`,
non-negative `growMin/growMax` fractions for automatic continuous x/y/z domain
growth, and inclusive `[0,1]` `padding` on a `bar` base axis.

Map a requested chart family to geometry instead of inventing a mark token:
scatter and point clouds use `point`; bubble charts add `size`; 3D lines,
waterfalls, and spectrum traces use `line`; columns use `bar`; height maps,
non-uniform grids, and sampled mathematical functions use `surface`; wire grids
use `wireframe`; contours layer `line` over `surface`; and impulses use `stem`.
Use `detail` for homogeneous multiple series and `layer` for heterogeneous
series.

Bars accept discrete or sparse continuous x/y bases with a continuous z value.
`x2`, `y2`, and `z2` may appear independently; omitted x/y extents and the z
baseline are inferred, while all six coordinates form an explicit cuboid. The
legacy any-orientation form remains valid when exactly two primary axes are
band scales and the sole continuous primary is the value axis. Surface and
wireframe `u,v` coordinates may be arbitrary finite topology values and may
contain holes.

View properties, in canonical order, are
`projection,azimuth,elevation,zoom,fov,orbit,aspectX,aspectY,aspectZ,targetX,targetY,targetZ,pan,zoomEnabled`.
Use `perspective|orthographic` for projection and booleans for orbit, pan, and
`zoomEnabled`. Set `zoomEnabled=false` to disable user zoom independently
without changing the authored initial `zoom`.
Azimuth, elevation, and target coordinates are finite; zoom and aspects are
greater than zero; and `0 < fov < 180`. An orthographic view may carry `fov`,
which is ignored while orthographic. Keep camera choices in one `Chart3DView`
on a physical chart scene. Guide properties are required `target,source` plus
optional `title,domain,ticks,labels,grid,tickCount,tickStep`; `detail` is a
valid target. Do not invent fixed schema count caps; ordinary payload and
runtime resource admission still applies. Follow
[Client-side 3D charting](charts/3d.md) for the exact geometry,
topology, composition, validation, interaction, fallback, and resource rules.

Do not invent a camera or gesture `mode`, encode touch mappings in response
data, or add sensitivity or control-label fields. `orbit`, `pan`, and
`zoomEnabled` only gate independent client capabilities. The preview is
gesture-neutral; **Explore 3D chart** opens full-screen Chart Explorer, whose
fixed client vocabulary maps one-finger drag to Rotate, two-finger translation
to Move, and pinch/spread to Zoom. Tap-only and accessibility actions expose
Rotate, Move, Zoom, and Reset separately, without a Rotate/Move selector.

Point and stem-tip shape values use
`circle,square,triangle,diamond,sphere,cube,pyramid,cylinder` in that automatic
order; bar shapes use `box,cylinder,pyramid,sphere`. Size zero intentionally
suppresses or degenerates geometry while retaining the source row.

Always keep the ordinary structured data view meaningful. Charts are an
additional presentation, not the only representation of returned data.

## Translation workflow

Studio owns structural synchronization. A translation changes localized
strings only. Preserve:

- `DocumentVersion`, structure, arrays, ordering, numbers, and native names;
- string-resource keys;
- schema identifiers; and
- every LMF1 argument position, type, style, selector, plural offset, brace,
  and apostrophe escape.

Use Studio's locale chooser and translation export/import workflow. Do not
manually invent a locale entry or restructure a generated `.doc.json` file.

## Boolean answer controls

Required Booleans without a default are explicit, initially unselected choices.
An explicit Boolean Default permits a switch only when no custom labels exist.
Use localized [TrueText and FalseText document properties](documentation.md#standard-properties)
for labels such as Accept and Decline. Display labels never change wire values.

## Folding structs and unions

Structs (`Table`), unions, and lists remain expanded unless `Foldable=true` is declared
on their type or field. Author it as `@LibertasFoldable` in TypeScript or
`#[libertas_foldable]` in Rust, without arguments. Studio permits this marker
only on an effective Table, Union, or List. It changes presentation, not encoded data.

An unmarked struct, union, or list may show a down arrow, but tapping it or opening its
menu cannot collapse the fields. A marked value starts expanded and permits
manual collapse and expansion. Each nested value owns its folding permission;
a marker does not grant permission to its descendants. Lists follow the same explicit-permission rule. Changing a union variant reveals its new fields. Union selectors
remain editable independently of folding.

For a TableHeader-promoted union, the shared owning row may fold when either
the owning Table or the promoted union has Foldable on its type or field.
TableHeader promotion and union-child flattening remain independent of folding.

## Unanswered fields and null

Required fields without an encodable value use subdued italic placeholders:
“Select…” for choices, “Enter…” for typed input, “Choose date…” or “Choose time…”
for temporal input, and “Set up…” for a container needing initialization.
Readonly fields without a value show “Not set.” Boolean choices retain their
unselected Yes/No controls. These labels are localized, with English fallbacks.

A valid nullable value is shown as “None” inside an outlined capsule. A literal
string containing “None” keeps ordinary text styling. Empty strings, false,
zero, and selected Nil union branches retain their normal value presentation.
Placeholders never change stored data or trigger errors merely by appearing;
failed submission uses the usual field validation feedback.
