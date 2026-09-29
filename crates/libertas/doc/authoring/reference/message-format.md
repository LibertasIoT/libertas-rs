# Libertas Message Format

Libertas Message Format (LMF) is the platform standard for localized,
structured messages. LMF version 1 (LMF1) is based on ICU MessageFormat 1
(MF1) and adds Libertas semantic types. An LMF1 renderer must therefore be a
Libertas-aware implementation; substituting a platform ICU formatter is not
sufficient.

LMF is used for notifications, Endpoint output marked with `FormattedText`,
human-readable data/database names, and formatted chart labels:

```text
message tuple = resource key + ordered, typed Libertas Message Arguments
invariant blob = canonical_encode(message tuple)
rendered text  = LMF1(localized template, resolved arguments)
```

The canonical tuple is locale-independent and is suitable for transport,
storage, chart identity, or an object key. Rendered text is a locale-specific
view for people and AI systems. Preserve the tuple; never replace its identity
with pre-rendered text.

## Uses and storage

| Use | Stored or transported value | Additional behavior |
| --- | --- | --- |
| Notification | Resource key and typed arguments inside a delivery envelope | Adds recipients, importance, and source. |
| `FormattedText` `Bytes` | Canonical encoding of one resource key and ordered typed arguments, including an empty argument list | Runtime output presentation; the emitting App package catalog supplies localization. |
| Data name | Invariant blob containing the canonical resource key and typed arguments | The blob is the single-record key or indexed-database/table name. |
| Chart value | The same formatted Bytes tuple | Rendered only as presentation; the raw blob remains category and interaction identity. |

`FormattedText=true` requires a `Bytes` host and does not change its wire
serialization. Use `@LibertasFormattedText` in TypeScript or
`#[libertas_formatted_text]` in Rust, without arguments. The Bytes payload is
always the canonical tuple, even when it contains only a resource key and an
empty argument list. Payloads never contain templates.

A formatted String host would have to Base64-encode the tuple and then decode
it again before every render. That back-and-forth adds no information, storage
benefit, localization capability, or identity guarantee, while making an
encoded tuple look like ordinary literal text. Libertas therefore keeps the
runtime value typed as Bytes end to end. That sharp type distinction makes it
less likely that developers will accidentally mix formatted values with
literal Strings. A generic JSON boundary may represent those bytes as Base64
as part of its normal Bytes codec, but that transport spelling is not an
App-authored String value and must not be copied into one.

Formatted fields follow the [static read-only placement and request-value
contract](presentation.md#static-read-only-placement-and-request-values).
That owner defines declaration requirements and permitted runtime uses;
the byte encoding here does not grant an exception.

`StringResources` is owned exactly once by the App package. The schema package
attribute contains its stable, comma-delimited keys, and the package document's
top-level `StringResources` object contains the localized LMF1 templates.
Functions and types do not carry resource catalogs. Notifications, data names,
permission reasons, `FormattedText`, and formatted chart labels emitted by any
function in the package all resolve through this one catalog. The encoded tuple
therefore needs no function or type owner and never carries a resource FQN.

Studio requires the schema's declared key set and the package document's key
set to match exactly. Duplicate keys or package catalog declarations, missing
document values, and undeclared document keys are validation errors.

The public SDK encoders `libertasFormattedText(resource, arguments)` in
TypeScript and `libertas_formatted_text(resource, arguments)` in Rust produce
the canonical `Uint8Array`/`Vec<u8>` representation for a Bytes field. Pass
those bytes directly; do not Base64-wrap them in App code. App code ordinarily
does not decode formatted text: a client resolves and renders it at the final
presentation boundary.

## LMF1 argument syntax

Every argument identifier is a zero-based position. Named arguments and
one-based printf numbering are not LMF1. The three basic forms are:

```text
{0}
{0,type}
{0,type,style}
```

For a resource that displays one literal-text argument, use `"{0}"`, not
`"%1$s"` or `"%s"`. Printf placeholders are ordinary text in LMF1 and appear
unchanged on screen. The same syntax applies to every locale's
`StringResources`, including translated resources. A `LiteralText` argument
is inserted as text; braces or percent signs inside it are not parsed again.

The complete grammar summary is:

```text
Untyped:
    {argument}

Simple:
    {argument,number[,style]}
    {argument,date[,style]}
    {argument,time[,style]}
    {argument,spellout[,ruleset]}
    {argument,ordinal[,ruleset]}
    {argument,duration[,ruleset]}

Complex:
    {argument,plural,variants}
    {argument,selectordinal,variants}
    {argument,select,variants}
    {argument,choice,choices}
```

LMF1 retains MF1 apostrophe quoting, plural `offset:`, exact plural selectors
such as `=0`, required `other` branches, and `#` substitution inside plural
variants. For example:

```typescript
const StatusStrings = {
    ROOM_TEMPERATURE: "{0,LibertasDevice} reports a room temperature of {1,number,0.0}.",
    ALERTS: "{0,plural,=0 {No alerts} one {# alert} other {# alerts}}",
    OWNER: "{0,select,male {His} female {Her} other {Their}} device"
} as const;
```

Translators may reorder positional arguments and translate literal text inside
variants. They must preserve argument positions, types, styles, selector keys,
plural offsets, braces, and apostrophe escaping.

## Libertas extensions

An LMF1 type may be a Libertas type:

```text
{0,LibertasDevice}
{1,LibertasEnum,StatusEnum}
```

`LibertasEnum` carries an integer value. Its `style` is required and is the
actual Enumeration type name used to resolve the localized variant. Other
Libertas object types, including `LibertasDevice` and `LibertasUser`, resolve
through the client's ordinary permission-aware object resolver.

The encoded Libertas Message Argument type has priority over a conflicting
template type. This overlap with MF1 is intentional: message data remains
self-describing even when a translation is stale or wrong. For example, an
encoded device ID must still resolve as a device when a template mistakenly
requests `number`; an encoded physical-unit value keeps its unit while the
LMF1 number style controls numeric presentation.

The template type supplies semantics only when compatible with the encoded
argument. `LibertasEnum` may refine an integer-backed enum argument with the
Enumeration name in `style`; it must not reinterpret an argument encoded as a
device, literal text, resource text, or physical unit.

## Number and date/time styles

The MF1 `style` on `number`, `date`, and `time` is authoritative presentation
metadata. There is no parallel Libertas syntax for specifying a message's
number or date format. In particular, schema `Format` attributes describe
ordinary scalar fields and do not override an LMF1 argument style.

Use standard MF1 styles or a portable decimal pattern:

```text
{0,number}          locale-default number
{0,number,integer}  integer presentation
{0,number,percent}  percent presentation
{0,number,0.00}     exactly two fractional digits
{0,date,long}       long localized date
{0,time,short}      short localized time
```

Unit-qualified encoded arguments use the same number style while retaining
their encoded physical unit and client-side conversion policy. Do not append a
hard-coded unit in the template.

## Libertas Message Arguments

Each formatted argument carries a one-byte argument-type discriminator for the
currently defined variants, followed by its value. Use the most specific
semantic type available. Encoding a `LibertasDevice` ID takes the same payload
space as encoding a plain integer of the same width; developers must encode it
as the device ID rather than erase its meaning.

Current argument categories include:

- literal text, which is inserted without resource lookup;
- resource text, which resolves another key in the same resource set;
- Libertas objects, resolved to an authorized display name;
- booleans, signed/unsigned integers, floats, and doubles;
- unit-qualified signed/unsigned integers, floats, and doubles; and
- plural counts.

The Rust SDK exposes the variants directly through the
`LibertasMessageArgument` enum. The core TypeScript SDK exposes a numeric
discriminated `LibertasMessageArgument` union; the optional
`@libertas-ai/consts` package supplies its named
`LibertasMessageArgumentType` constants.

For example:

```rust
let arguments = [
    LibertasMessageArgument::Object(room),
    LibertasMessageArgument::ResourceText("HVAC_HEATING"),
    LibertasMessageArgument::UnitFloat {
        unit_type: "temperature-celsius",
        value: 22.5,
    },
];
```

The same array can render a notification, formatted runtime Bytes field, or
data name. The resource key and typed arguments—not the rendered sentence—are
the value's stable identity.

## Notifications

Send a stable resource key and typed values. Recipients, importance, and source
are delivery metadata rather than LMF1 arguments:

```rust
libertas_notification_send(
    &[recipient],
    NotificationImportance::Info,
    Some(room),
    "ROOM_STATUS",
    &arguments,
);
```

Use literal notifications only for genuinely non-localizable diagnostics. Do
not put sensitive identifiers into literal text when a typed object argument
provides permission-aware presentation.

## Runtime fields with `FormattedText=true`

Structured fields remain authoritative for automation, validation, controls,
and chart geometry. Formatted fields follow the
[static read-only placement and request-value contract](presentation.md#static-read-only-placement-and-request-values):

```rust
//! Room condition messages
//! Gives occupants localized summaries of current room conditions.
//! #[libertas_string_resources(PROTOCOL_STRINGS)]

pub const PROTOCOL_STRINGS: &[(&str, &str)] = &[
    ("READY", "Room condition reporting is ready."),
    ("ROOM_STATUS", "{0,LibertasDevice} reports a room temperature of {1,number,0.0}."),
];

/// Room condition
/// Describes the latest reported condition in one room.
#[derive(LibertasExport, LibertasAvroEncode)]
pub struct RoomState {
    /// Availability summary
    /// Tells occupants whether the room condition report is ready.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub summary: Vec<u8>,

    /// Current room condition
    /// Tells occupants what is happening in the room and includes its relevant measurements.
    #[libertas_read_only]
    #[libertas_formatted_text]
    pub status: Vec<u8>,
}

fn encode_room_state(room: u32, temperature_celsius: f64) -> RoomState {
    let status_arguments = [
        LibertasMessageArgument::Object(room),
        LibertasMessageArgument::UnitDouble {
            unit_type: "temperature-celsius",
            value: temperature_celsius,
        },
    ];
    RoomState {
        summary: libertas_formatted_text("READY", &[]),
        status: libertas_formatted_text("ROOM_STATUS", &status_arguments),
    }
}
```

The client decodes both Bytes tuples before looking up `READY` or
`ROOM_STATUS`. It resolves arguments at display time and never parses a
rendered sentence back into data.

## Charting

Charting uses the same LMF1 renderer for `FormattedText` text marks, axes,
facets, legends, tooltips, and accessibility labels. The renderer uses the
catalog owned by the App package and the client's normal
object, Enumeration, unit, locale, and number/date services.

LMF1 affects only the label. The formatted Bytes blob remains the raw chart
domain, grouping, ordering, shared-scale, and
interaction identity. Two tuples that render to the same text remain distinct
categories. A locale or template change may alter measurement and therefore
guide layout, but must never alter chart geometry derived from data, domain
membership, or category identity.

An unresolved or malformed optional formatted chart value is omitted. If the
value is essential to distinguish visible categories or supply an accessible
name, the complete chart follows the
[chart failure policy](charts/2d.md#10-validation-and-fallback). Raw keys, blobs, and
typed diagnostics may appear in bounded developer diagnostics, never as chart
presentation text.

## Data and database names

Single records and indexed databases use the same canonical tuple:

```text
DataName = resource key + ordered, typed Libertas Message Arguments
data key = canonical_encode(DataName)
```

The resulting byte blob is the sole logical name: it uniquely identifies the
single record or indexed database/table within its task's applicable data
namespace and also carries the semantic information needed to select a
localized template and render its typed data for people. Distinct tuples remain
distinct identities even when their rendered labels are the same. The
blob contains the template identity, not the template text. Task package
context selects the package `StringResources` catalog, the resource key selects
the LMF1 template, and the typed arguments provide its values. Do not add a
second literal, physical-table, rendered, or Base64 name.

The task App package owns the data-name resource even when its
`FunctionDataSchema` is an externally owned type. That external type describes
stored value shape; it does not contribute a catalog owner or resource FQN.

Changing localized LMF1 text or number style does not rename data. Changing a
resource key, argument type, argument order, or argument value creates a
different name and requires an intentional migration. Single-record and
indexed-database names are separate namespaces; an indexed record's signed
64-bit index is additional identity and is not an LMF1 argument.

## Failure handling and review checklist

Rendering must never mutate canonical data. If a template, resource, object,
Enumeration, unit, argument, or style cannot be resolved, retain the encoded
value and use the established safe fallback outside chart presentation.

Before publishing an App:

- declare every notification, data-name, permission-reason, and formatted-output
  key in the package `StringResources` catalog;
- use stable resource keys and realistic default-locale LMF1 templates;
- use zero-based positions and verify every locale against the same typed
  argument list;
- preserve all LMF1 structure during translation;
- use Libertas object/enum and physical-unit arguments instead of erasing them
  to plain integers or preformatted text;
- keep templates trusted—never treat user input as an LMF1 template; and
- test reordered arguments, apostrophe escaping, number/date styles,
  plural/select/selectordinal/choice branches, missing resources, unresolved
  Libertas values, and malformed formatted-text bytes.
