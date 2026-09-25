# Presentation and localization

This page is the compact App-authoring projection of the full
[presentation and localization contract](../reference/presentation.md).
Presentation metadata never changes the underlying typed wire value.

## Source documentation

Give every authored schema node a natural label and complete description.
TypeScript uses entry-file `///` package text and declaration JSDoc; Rust uses
inner package rustdoc and outer declaration rustdoc. A List has plural
container text above `----` and singular item text below it. Put custom
`[PropertyName]` blocks after ordinary text and before TypeScript schema tags.
Follow the [documentation invariants](documentation_invariants.md).

## Common field metadata

Proactively complete applicable metadata on every authored App: useful initial
task/server names, localized text defaults, value defaults, units, bounds,
inline help, and grouping. Review these choices before delivery without waiting
for a separate user request. Each annotation must serve the App's actual
behavior; configuration-only examples need no invented protocol or persistence.

Use `Default` only for initial data, `UIHeader` for [field promotion and flattening types with children](../reference/presentation.md#tableheader-promoted-fields-and-union-selectors),
`UIIncludeDescription` when a UI node must show both its short display name and
long description inline, `InlineChoices` on an Enumeration or Union field or
type (not Bitflags) whose few choices should render as [inline radio controls](../reference/presentation.md#inline-choices),
`ReadOnly` and `Hidden` for presentation, `Unordered`
and `Unique` for List semantics, `Size` for bounds, and `EnumSource` only on an
unsigned integer storing a same-tree List index. Check the
[annotation catalog](schema_annotations.md) before adding source syntax.
A chooser with more than twelve choices — Enumeration, Bitflags, EnumSource, or
a Union variant chooser — automatically shows a text filter field; no
attribute is needed. See [filtering long choosers](../reference/presentation.md#common-field-metadata).

## Date and time

`DateTime` is epoch seconds presented in local time; `TimeOnly` is seconds from
midnight; `DateOnly` is integer Gregorian `yyyyMMdd` without timezone
conversion; `TimeInterval` is duration seconds. Defaults, fixed values, and
runtime values must remain valid for the declared meaning.

## Display formats

`Format` is one portable Unicode LDML display pattern. It never changes raw
storage, parsing, validation, comparison, requests, persistence, or chart
geometry. Use number grammar for ordinary Numbers and the applicable date/time
grammar for DateOnly, DateTime, or TimeOnly; follow the
[full presentation contract](../reference/presentation.md#display-formats)
for the supported subset.

## Physical units

`PhysicalUnit` is a canonical CLDR core unit identifier on a Number. It states
the unit already carried by the raw value and performs no conversion or
rescaling. Never substitute a localized label or display symbol.
All units, including compounds, are localizable in UI and charts. Keep numeric
values and canonical unit IDs unchanged; translate authored labels through
locale documents. See [physical units](../reference/presentation.md#physical-units).

## Bitflags

Bitflags project a documented Enumeration onto an ordinary integer. Each
nonzero member is one unique bit; at most one zero member is allowed. Runtime
values may combine only declared bits, and the integer wire value is unchanged.

## String resources and formatted output

An App has one package-owned `StringResources` catalog. Runtime
`FormattedText` is the public SDK's canonical Bytes encoding of a resource key
plus ordered typed Libertas Message Arguments; never store it in a String or
Base64-wrap it in App code. Use `libertasFormattedText(...)` in TypeScript or
`libertas_formatted_text(...)` in Rust. **Every FormattedText node must be
statically ReadOnly**, including output and dormant types: use its field/type
or an enclosing ReadOnly Table. A ReadOnly List protects membership only.
Requests may carry copied values (including enclosing/root copies), supplied
EditRequest values, or Default/Fixed values; no leaf CopyFrom is required when
those supply it. Required undefined values block submission. Function/configuration
arguments remain forbidden. Follow the
[placement and request-value contract](../reference/presentation.md#static-read-only-placement-and-request-values).
Persisting FormattedText in record payloads is permitted but discouraged; prefer
structured values and render at presentation time. This guidance does not apply
to DataName database/record names; see the
[payload guidance](../reference/presentation.md#persisting-formatted-record-payloads).
Preserve all LMF1 positions, types, styles, selectors, plural structure,
braces, and apostrophe escapes in translation. See
[Libertas Message Format](../reference/message-format.md).

## Charts

Charts belong on Response or SubscriptionData output. A primitive chart is a
List of row Tables; the List owns the mark and direct row fields own channels.
Compositions contain independently valid charts. Keep the ordinary structured
view meaningful.

Every visible text value retains its semantic type: use an Enumeration for a
closed vocabulary, `FormattedText` Bytes for dynamic App wording, and a typed
Libertas object for device/user/Endpoint identity. Plain String is only genuine
literal domain or user-authored text. Hidden context may accompany chart data
without becoming a mark. Follow the complete
[2D chart specification](../reference/charts/2d.md).

### 3D charts

Use only the parallel `Chart3D`, `Chart3DChannel`, `Chart3DScale`,
`Chart3DGuide`, and `Chart3DView` namespace on a 3D path; never mix it with 2D
metadata. Choose a supported primitive/composition rather than inventing a mark
or gesture mode. Camera fields set the initial view and independently enable
orbit, pan, and zoom. Touch and accessibility controls are client-owned, not
response data. Follow the complete
[3D chart specification](../reference/charts/3d.md).

## Translation workflow

Translate localized string values only. Preserve document structure, ordering,
native names, numeric IDs, extension/resource keys, and all LMF1 structure. Use
Studio's locale workflow; never restructure generated document JSON.
