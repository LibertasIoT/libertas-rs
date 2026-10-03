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

## Display Template

Page-specific [button captions](../reference/presentation.md#button-labels) use
`button_labels` on a whole protocol Request, Response, or SubscriptionData
variant. Its optional `go_back`, `next`, `close`, and `actions` slots each name
a package StringResources key or use `null` to restore the platform caption;
slots cascade independently and never inherit into another page. `go_back`
and `next` require Request variants. Only `actions` also accepts
`{"source":"single_action"}`: use the sole final eligible action's localized
variant Name, otherwise the platform caption. Exclude LinkOnly from that
count. Let the task change its offered request through `ConstrainedBy`;
the [section-hub pattern](../reference/presentation.md#task-controlled-section-hub-action)
keeps Continue navigation separate from editor Save and continue submission.
Device `when` is allowed, but data
conditions, switches, field sources, and pseudo-states are not. Captions change
only existing controls: preserve Next submission, PrevRequest behavior,
Actions eligibility/choosers, local Close, native Back, and embedded Go Back
suppression. Use truthful captions for all states of the selected page.

Every package has one mandatory [Display Template](../reference/presentation.md#display-template),
the sole runtime owner of how values are arranged and styled; it is not schema
and never changes the App version. With no `display.json` and no presentation
attributes the package compiles to the valid empty template, which renders the
platform default. Presentation source attributes (`Foldable`,
`InlineChoices`, `SuppressLabel`, `UIIncludeDescription`, `Format`, chart scale
`kind` and guide `position`) remain the way to author defaults: Studio compiles
them into the template at the lowest specificity and drops them from the
schema. Promotion (`promote`), flattening (`flatten`), `label`, and `title`
exist only as rules;
there is no `UIHeader`/`TableHeader` or `UIName` attribute. A dependency's
compiled template, its `display.json` included, applies to the types that
package declares wherever they appear, below every rule of the using package,
so a TypesOnly package may ship a `display.json` for its types.
Device-specific arrangement goes in `display.json` at the project root,
beside `Cargo.toml` or `package.json`; never edit the compiled template. After
editing `display.json`, call the Studio MCP `compile_schema` then
`preview_display` tool so the user sees the change live on a device.

Write each rule as four relations:

- **Where the node is:** `select`, a [selector](../reference/schema.md#display-template-selectors):
  `^<Variant>`, strict `^<Variant.field...>`, `^<function...>`, `$.field`, or
  relaxed `^<Type>`; `[]` after a List selects its members and ` ...` every
  descendant. A bare name matching both a variant or function and a named type
  is rejected: write `package::Type`.
- **What arranges it:** `layout` from the closed [catalog](../reference/presentation.md#display-template-catalog):
  `rows` (default), `form-grid`, `card`, `table`, `tabs`, `master-detail`,
  `accordion`, or `split`. Every layout except `rows` applies only
  where a rule asks; `table` and `master-detail` also need the widening gate
  (`form` `tablet` or `desktop`, or `width: expanded` without `height:
  compact`) and otherwise render as `rows`. `master-detail` takes `detail`, an array of
  selectors (`…[]` for a List's members, otherwise one node) reached through
  struct fields only; every detail node needs a `Link` and opens in the
  trailing pane, and any other link or action replaces both panes. An embedded
  workflow editor offers Close + Next and suppresses every `PrevRequest`
  control; the leading pane owns navigation for the whole arrangement and the
  App must provide all required routes there. Open a flow requiring
  editor-specific previous actions on the whole working surface.
  Full-page editors retain ordinary previous actions, including after narrowing.
  After a Submit in the trailing pane gets a reply that is neither a failure
  nor an Error, the client sends the Request the rule names in `refresh`,
  built from the leading page, unless that reply
  replaced the leading page; `refresh` must be a pure navigation Request with
  no editable input, never a save
  ([panes](../reference/presentation.md#master-detail-panes)), so admit
  navigation requests by identity, not by revision. `split` nests at most three levels and renders as `rows` on
  phones unless a phone rule asks for it. There is no `stepper`: Studio
  rejects it and clients draw `rows`. Sequential screens are a
  [workflow](../reference/protocols.md#workflow-interactions), where the
  service validates, branches, and saves each step; a second in-page
  Previous/Next inside a workflow confuses users.
- **What styles it:** `style` tokens (`density`, `emphasis`, `align`, `role`,
  `color`, `spacing`, `span`, `icon`) and behavior hints (`inline`, `fold`,
  `promote`, `suppress_label`, `include_description`, `format`, `scale`,
  `guide`, `palette`). No CSS, pixels, literal colors, or fonts. `media` is
  deferred until bytes fields have a MIME-type attribute.
- **What removes its label:** [`suppress_label`](../reference/presentation.md#suppresslabel-label-free-value-rows) keeps a scalar, struct, or Union value and its controls while omitting its row label. On a List of structs it removes only the List-contributed member name and position, preserving its heading and member labels. At a suppressed root protocol Union, use the selected variant’s localized name in the heading and top bar; an explicit title/label of that active Request or Response wins. Keep accessible names. Retired `suppress_title` and its source spellings are rejected.
- **What heads the page:** Requests and Responses use platform-owned headings.
  Move Actions to the top bar when its complete localized button fits while
  reserving `min(full title width, 8em)` for title text at the current title font
  size. Suppress the content heading only if the full title also fits without
  clipping or ellipsis; otherwise keep the wrapping heading, with Actions in
  that row when the minimum reservation cannot fit. Preserve descriptions,
  long-press help, keyboard/screen-reader help, and heading accessibility.
  The leading page alone uses the shared top bar in master-detail; the trailing
  page keeps its own heading and Actions. Re-layout preserves the interaction
  and sends nothing. Top-level content keeps normal page or pane padding,
  aligned with any retained content heading, without an extra root indent;
  no App attribute or `flatten` rule is needed.
  Nested structures keep their ordinary indentation rules. See
  [protocol page headings](../reference/presentation.md#protocol-page-heading-and-top-bar).
- **What names it:** on a page (displayed Request or Response variant), struct, Union field, or
  List's members (`…[]`), [`label`](../reference/presentation.md#labels-and-titles)
  replaces only the displayed name (on a List's members, the name and position
  number) and keeps the row's value (its own or a promoted child's); `title` replaces the whole row, label and value, with its
  text, and nothing is promoted into it. Each takes `{ "field": "path" }`, a
  String or FormattedText field below the node through struct fields only,
  never a List or Union, Hidden or ReadOnly allowed, or a string naming a fixed
  `StringResources` key. On a Union field or a node reached only through a
  Union (a branch, a node below one, or a type used only there) only the key
  form is valid, because the branch is known only at run time. A null, unanswered, or empty value draws the node as
  without the member; the display name stays the schema's meaning for AI
  tools. `label` and `title` exclude each other, neither sits beside
  `suppress_label` on one node, and no data condition or switch case sets
  them. A data-titled row is a `label` rule on its struct plus a `promote`
  rule on the child that is its value. Declare every `label` or `title`
  source field Hidden in the schema so it is not drawn twice; the template
  never sets Hidden, and Studio warns at the member's `display.json` line,
  naming a visible source field, without rejecting it. With `use_parent_name` false the row shows the child's own name
  and Studio warns that the `label` shows nowhere. A `title` on the struct
  takes no promoted child: the child keeps its own row and Studio warns; a
  `title` beside `flatten` `{ "name_row": false }` is rejected. With `label` a
  promoted value's link stays on the row; with `title` a struct's link stays
  on the title row ([combinations](../reference/presentation.md#labels-and-titles)).
- **What it promotes:** [`promote`](../reference/presentation.md#tableheader-promoted-fields-and-union-selectors)
  is valid only on a rule selecting a field of a struct (`^<TopicRow>.status`):
  that field's value goes on the owning row, labelled by the owner's displayed
  name (its `label`, when set), or by the field's own name with
  `{ "use_parent_name": false }`; `false` turns it off. Studio rejects it on a
  type, a List member, a Union branch, a function parameter, or a Response.
- **What it flattens:** [`flatten`](../reference/presentation.md#flatten), a
  rule member like `layout`, selects a struct, List, or Union by type
  (`^<Address>`, every use) or by field, as `true`, as `false` to undo a
  lower-ranked flattening such as a dependency's, or, on a struct only,
  `{ "name_row": false }`; Studio rejects it on a List member (`…[]`) or a
  Union branch, which has no row of its own. A struct's fields move to its row's indentation and
  `name_row` false also removes its name-only row (the fold control passes to
  its first remaining row; a List member keeps position and membership
  controls). A List keeps its row, count, and Add, and its members, each with
  its own row and actions, move to its indentation. A Union's selected branch
  fields move to its row's indentation; the selector stays editable. Hidden
  still hides the subtree, and no data condition or switch case sets
  `promote` or `flatten`.
- **What changes it per device:** `when` with `width`, `height`,
  `orientation`, `pointer`, `theme`, and `form`, evaluated against the nearest
  pane; a list value means any, several keys must all hold. Widen layouts by
  `form`, or `width` with `height`, never `width` alone.

Data may also change presentation: `:condition(expression)` on the selector,
or an ordered `switch` of cases with an optional `else`, uses ValidationRules
expressions rooted at the node; `:pending`, `:error`, and `:selected` change
style only. A condition may change style, layout, or fold state, never hide a
field, make it read-only, or change a value: those are schema. Later rules win
among equals in the [cascade](../reference/presentation.md#display-template-cascade), above
source defaults; strict beats relaxed, and within each a `when` rule beats one
without. Unknown tokens or layouts fall back to the
platform default.

Decide arrangement with this procedure:

- A List of small records: `table` with `when` `form` `tablet` or `desktop`,
  and `card` with `form: phone`.
- An Enumeration or Union with at most four record-or-unit choices: `inline`.
- A hub Response with a List whose member type has a `Link`:
  `master-detail` with `form` `tablet` or `desktop`, `rows` on phones,
  whatever the orientation.
- More than about eight fields on a page, counting the fields of nested
  Tables: `sections` (groups of direct children, each optionally titled by a
  `StringResources` key) with `tabs`, a `split` (rows on a phone unless a
  phone rule asks for it), or a fold that starts closed. The lint judges each `when.form`
  separately.
- Numbers compared down a column: `align: end`.
- A status value: `:condition(...)` or `switch` with `color` and `icon`.

**Start from a preset.** `vscode/templates/display/` holds three starting
files that already follow this procedure end to end; each documents its own
`^<Type>`-style placeholder names at the top under `_placeholders` for you to
rename. Start from `form.json` for a request editor: `form-grid` on tablet
and desktop, rows on phone, a folded optional group, and an inline choice.
Start from `hub.json` for a Response with a linked List: `master-detail` on
tablet and desktop, rows on phone, and status badges via `:condition(...)`.
Start from `dashboard.json` for a Response of summaries, a chart field, and a
long line List: `split` with `card` regions on tablet and desktop, `card` on
phone, and `table` for the line List on wide screens. Studio's
[lints](../reference/presentation.md#template-validation) catch the mistakes
this procedure exists to avoid: too many `form-grid` or `table` columns on
phone, more than about eight fields shown together, a required field
folded closed on phone, a clashing `color` role, and a `split` that strands a
required field in a nested region.

## Common field metadata

Proactively complete applicable metadata on every authored App: useful initial
task/server names, localized text defaults, value defaults, units, bounds,
inline help, and grouping. Review these choices before delivery without waiting
for a separate user request. Each annotation must serve the App's actual
behavior; configuration-only examples need no invented protocol or persistence.

Schema attributes: `Default` only for initial data, `Link` to make a row open a
Request: it keeps the label and value, renders a non-editable value as link
text with the whole row as tap target and an editable one with a trailing link
button ([link rendering](../reference/presentation.md#link-rendering)), `ReadOnly` and `Hidden` for
presentation, `Unordered` and `Unique` for List semantics, `Size` for bounds,
and `EnumSource` only on an unsigned integer storing a same-tree List index.
Template: a `promote` rule for [field promotion](../reference/presentation.md#tableheader-promoted-fields-and-union-selectors)
and a `flatten` rule for [flattening a struct, List, or Union](../reference/presentation.md#flatten);
template defaults: `UIIncludeDescription` when a UI node must show both its short display name and
long description inline, and `InlineChoices` on an Enumeration or Union field
or type (not Bitflags) whose few choices should render as
[inline radio controls](../reference/presentation.md#inline-choices). Check the
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
grammar for DateOnly, DateTime, or TimeOnly. It compiles into the template's
`format` hint, which a rule may change per device; follow the
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
or an enclosing ReadOnly Table. A ReadOnly List protects membership and order, not member values.
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
Typed currency arguments default to the ISO code; compatible `number` styles
choose `currency`/`¤` for the symbol, `¤¤` for the code, or `¤¤¤` for a localized
name while preserving the encoded currency and scale. Compatible `date`/`time`
styles choose `short`, `medium`, `long`, or `full`; calendar dates and civil
times never acquire timezone shifts. See the
[currency and temporal contract](../reference/message-format.md#libertas-message-arguments).

## Charts

Charts belong on Response or SubscriptionData output. A primitive chart is a
List of row Tables; the List owns the mark and direct row fields own channels.
Compositions contain independently valid charts. Keep the ordinary structured
view meaningful. Scale `kind`, guide `position`, and palette are template
appearance; channels and every other chart property are schema.

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
