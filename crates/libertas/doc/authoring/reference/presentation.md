# Presentation and localization

Presentation attributes preserve the underlying typed value and wire encoding.
They tell Studio and clients how to validate, edit, localize, summarize, or
chart the value. How a value is arranged and styled on a device belongs to the
package's [Display Template](#display-template); the attributes that only
change that are compiled into it.

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

## Display Template

Every App package has exactly one **Display Template**, version 0.1: the
presentation layer that decides how the values its schema describes are drawn
on a given device. The schema owns what the App accepts, sends, computes,
protects, or means; the template owns only how the same value is arranged and
styled. A template can make a correct App look worse, never make it incorrect.
This section owns the template's file, compiled form, cascade, conditions,
catalog, and behavior hints. The [Libertas Schema Path](schema.md#display-template-selectors)
owns its selector grammar and [publication](publication.md#display-template-identity)
owns its identity and delivery.

- **Mandatory.** Every package has a template, including a TypesOnly package.
  A package with no presentation attributes and no `display.json` compiles to
  the empty template. The empty template is valid, has a digest and a
  documents-manifest entry, and renders exactly the platform default.
- **Sole runtime owner of presentation.** Clients read promotion and
  flattening, titles, folding, inline choices, SuppressLabel,
  UIIncludeDescription, the `Format` display pattern, and chart appearance only
  from the template. Promotion, flattening, labels, and titles are authored
  only as template rules.
  The other source attributes remain the authoring convenience: Studio
  validates them, compiles every occurrence into a template rule at the lowest
  specificity, and removes them from the final schema. Each attribute's section
  below keeps its authoring text and meaning; the template is where that
  meaning reaches a client.
- **Not part of the schema.** The template is absent from the schema JSON, its
  revision digest, and the App version. A template change is a document-only
  revision: it never rebuilds the App or allocates an App version.
- **Opt-in arrangement.** `rows`, one row per child, is the default layout.
  Every other layout applies only where a rule asks for it and, for a gated
  layout, only while its gate holds; otherwise the node renders as `rows`.
- **Presentation only.** No rule, condition, or device state can hide a field,
  make it read-only, change a value, or change a user's submitted payload. The
  [pane arrangement](#master-detail-panes) may load or refresh a leading page
  through a declared pure navigation Request. A condition may
  change style, layout, or fold state only. Hidden, ReadOnly, and every other
  schema rule apply before and regardless of the template.
- **Forward tolerant.** A client that meets a token value, layout, hint, or
  member it does not know uses the platform default for that item and applies
  the rest of the rule, so a newer template never breaks an older client.

### Template boundary

An attribute belongs in the schema when changing it would change what the App
accepts, sends, computes, protects, or means. It belongs in the template when
changing it would only change how the same value is drawn on one device. Apply
three tests in order:

1. **Does the server or the wire depend on it?** Encoding, defaults,
   nullability, ranges, uniqueness, validation, copies, calculations, action
   sets, access, and identity are schema.
2. **Does it state what the value is or means?** Money and currency, units,
   dates, formatted text, labels, help text, and which fields a chart encodes
   are schema; meaning is content, localized with the strings.
3. **Would a different choice be right on another screen size?** Promotion,
   flattening, folding, inline choices versus a chooser, an inline
   description, density,
   arrangement, and chart appearance are template, with a source attribute,
   where one exists, as the default rule.

| Schema only | Template (a source attribute, where one exists, is the default rule) |
| --- | --- |
| Hidden, ReadOnly, Fixed, Default, Nullable, Size, Number ranges, Unique, Unordered | `promote` with `use_parent_name`, and `flatten` with `name_row` |
| ValidationRules, RangeMin, RangeMax, EnumSource, ConstrainedBy, ActionControl | Foldable and `initially_open` |
| CopyFrom, CopyTo, CalculationRequest, Link, LinkOnly, RequestConfirmation | InlineChoices |
| NextRequest, NextResponse, PrevRequest, WorkflowRequest, DefaultRequest, ExcludeUI | SuppressLabel, UIIncludeDescription |
| AccessPrivilege, CacheIdentity | `Format` display pattern and precision, never the value or currency |
| Money, PhysicalUnit, DateOnly, FormattedText, display names, descriptions | Chart scale type, guide position, and palette |
| Chart channel bindings: which field is x, y, color, or tooltip | Template-only concepts: layout, density, emphasis, `label`, `title`, `button_labels`, `action_controls`, device conditions |

Unordered and ExcludeUI look presentational, but the wire and the protocol
depend on them, so they stay in the schema. `Format` looks semantic, but
`Money` already carries the currency and the precision that matter to the
value, so the display pattern moves. [`label` and `title`](#labels-and-titles) look like
schema labels, but the node's display name stays its meaning in the schema;
they change only what a person sees, so they are template. Studio enforces the table: a rule
member that is not in the [catalog](#display-template-catalog), including every
schema-only attribute, is rejected.

### Display Template file

The source of truth is `display.json` at the project root, beside
`Cargo.toml` or `package.json`, under version control. There is one file per
project; device variance is expressed inside it as conditions, never as a
second template for a locale or platform. A missing file is valid and compiles
to a template holding only the rules derived from source attributes. Studio's
editor writes this file in place through the same validated writer as a hand
edit, never a second copy. The Hub stores only the compiled, published copy
and never accepts a template Studio did not compile from source. For the
conversational loop of hand-editing `display.json` and watching it live in
Studio, see [the Studio MCP live-preview guide](../guides/studio.md#live-preview-while-editing-displayjson).

```json
{
  "DisplayVersion": 1,
  "rules": [
    { "select": "^<Hub.topics>[]", "layout": "rows", "style": { "emphasis": "primary" } },
    { "select": "^<Hub>", "when": { "form": ["tablet", "desktop"] },
      "right_pane": ["EditWages", "EditInterest"],
      "detail": ["^<Hub.told>", "^<Hub.topics>[]"], "refresh": "OpenHub" },
    { "select": "^<Review.return_lines>", "when": { "form": "desktop" }, "layout": "table",
      "columns": ["form_line", "amount"] },
    { "select": "^<IncomeDocumentItem>", "layout": "card", "when": { "form": "phone" } },
    { "select": "^<Answer>", "inline": true },
    { "select": "^<Business.expenses>", "fold": { "initially_open": false } },
    { "select": "$.estimate:pending", "style": { "emphasis": "muted" } },
    { "select": "^<TopicRow>:condition([status] is NeedsAttention)", "style": { "color": "error", "icon": "warning" } },
    { "select": "^<Employee>", "when": { "form": ["tablet", "desktop"], "orientation": "landscape" }, "layout": "split", "direction": "row",
      "regions": [
        { "fields": ["photo"], "ratio": 1, "style": { "align": "center" } },
        { "fields": ["name", "date_of_birth", "employee_id"], "ratio": 2, "layout": "form-grid", "columns": 2 }
      ] }
  ]
}
```

The example is schematic: its names stand for an App's own variants, types,
and fields. `DisplayVersion` is the JSON format version, like a document's
`DocumentVersion`; version 0.1 of the template is `DisplayVersion` 1, and the
number never chooses a winner in convergence. `rules` is an ordered array.

A rule is one JSON object with these members:

| Member | Required | Meaning |
| --- | --- | --- |
| `select` | Yes | One [selector](schema.md#display-template-selectors), optionally ending in a pseudo-state or a `:condition(...)` [data condition](#display-template-conditions). |
| `when` | No | A [device condition](#display-template-conditions). Without it the rule applies on every device. |
| `layout` | No | One [catalog layout](#display-template-catalog), with the layout-specific members its row names. |
| `right_pane`, `refresh`, `detail` | No | Leading Response relationships, its pure loader, and optional linked-node highlighting under [master-detail panes](#master-detail-panes). |
| `style` | No | An object of [style tokens](#style-tokens). |
| `label` | No | Replaces the node's displayed name and keeps its value ([labels and titles](#labels-and-titles)): `{ "field": "path" }` takes the text from a String or FormattedText field below the node, and a string names a fixed `StringResources` key. |
| `title` | No | Replaces the node's whole row, label and value, with its text, in the same two forms ([labels and titles](#labels-and-titles)). |
| `button_labels` | No | Localized captions for existing protocol-page controls, by slot: `go_back`, `next`, `close`, `actions` ([button labels](#button-labels)). |
| `action_controls` | No | Labels, placement, and applicable style for App-declared response controls and built-in Actions ([named action controls](#named-action-controls)). |
| `flatten` | No | Draws a struct's, List's, or Union's children at that node's own indentation ([flattening](#flatten)): `true`; `false`, which undoes a flattening that a lower-ranked rule set; or `{ "name_row": false }` on a struct to remove its name-only row as well. |
| `inline`, `fold`, `promote`, `suppress_label`, `include_description`, `format`, `scale`, `guide`, `palette` | No | [Behavior hints](#behavior-hints). |
| `switch` | No | A [conditional switch](#display-template-switch) whose chosen case supplies the presentation members. A rule with `switch` has no other presentation member. |

A rule has exactly one `select` and at least one presentation member, directly
or through its switch. Members it does not name keep their inherited or
default values. Studio rejects any other member.

### Compiled template

Studio compiles one template document per package, in this order:

1. **Defaults:** one rule per occurrence of a source presentation attribute,
   at the lowest specificity. A field attribute becomes a rule selecting that
   field: `^<Type>.field` for a field of a named type, the strict path of each
   occurrence for a field of an inline type, or `^<function>.parameter` for a
   function parameter. A type-declaration attribute becomes a relaxed-type rule,
   `^<Type>`. Type-derived defaults precede field-derived ones, so a field's
   own attribute wins where both set the same hint.
2. **Rules:** the rules of `display.json`, validated, in file order.

```json
{ "DisplayVersion": 1, "defaults": [], "rules": [] }
```

That is the empty template. Each derived rule reproduces its source
attribute's meaning exactly, including direct declaration lookup: a Foldable
on a field yields a field `fold` rule and one on a type yields a type `fold`
rule, and neither is inherited through effective-attribute resolution. The compiled template is derived and never edited by hand. Its
identity is the SHA-256 digest of its canonical JSON, computed with the same
canonicalization as the [schema revision digest](schema.md#113-revision-digest).
`compile_schema` with `includeArtifacts` returns it beside each package's
default document.

### Display Template cascade

A rule applies to a node when its selector matches the node, its `when` holds,
its data condition or pseudo-state holds, and, for a switch, a case or `else`
is chosen. Every presentation member resolves independently: the
highest-ranked applicable rule that sets that member wins, and a layout
resolves together with its layout-specific members. Each `button_labels` slot
resolves independently; setting one slot leaves the others' cascade intact.
Ranks, lowest first:

1. the platform default;
2. a rule of another package's compiled template, on a node that package
   declares; among these rules, that template's own ranks and order decide;
3. a rule derived from a source attribute (`defaults`);
4. a relaxed-type rule: a selector whose first form is `^<Type>`, or any
   selector using `...`;
5. the same with a `when` condition;
6. a strict-path rule: `^<Variant>`, `^<Variant.field...>`, `^<function...>`,
   or `$.field`, with or without `[]`;
7. the same with a `when` condition.

Among rules of equal rank, the later rule in compiled order wins. Data
conditions, pseudo-states, and switches gate a rule; they never change its
rank. The style tokens `density` and `emphasis` inherit down the value tree
unless a descendant resolves its own; no other member inherits, and a layout
never does.

A screen uses the full template of the package that owns its displayed root:
the Endpoint protocol Union's package for a protocol screen, and the function's
package for a configuration editor or stored data. Nodes declared by another
package additionally receive that package's compiled template, its
`display.json` rules included, at rank 2, below every rule of the screen's own
package. A dependency's rule applies where its selector, resolved against that
package's schema, matches a node that package declares. A type therefore keeps
its authored presentation wherever it is shown, the using package can still
change it, and a TypesOnly package may ship a `display.json` for its types.
Every package template a screen needs arrives with its schema.

### Display Template conditions

A `when` object accepts only these keys, each with a closed value set. A value
may be a list, meaning any of the listed values (`"form": ["tablet",
"desktop"]`); several keys in one `when` must all hold.

| Key | Values | Meaning |
| --- | --- | --- |
| `width` | `compact` (under 600 dp), `medium` (600 dp up to 840 dp), `expanded` (840 dp and up) | How much fits across the pane now: the platform's window size class. |
| `height` | `compact`, `medium`, `expanded`, with the same bounds | How much fits down the pane now. |
| `orientation` | `portrait`, `landscape` | The pane's orientation. |
| `pointer` | `coarse`, `fine` | Touch, or a mouse or trackpad. |
| `theme` | `light`, `dark` | The active color theme. |
| `form` | `phone`, `tablet`, `desktop`, `tv` | What the device is, not the window: `phone` when its smallest width in portrait is under 600 dp; `tablet` at 600 dp and up with a coarse pointer; `desktop` with a fine pointer and a resizable window; `tv` for a lean-back display. |

Conditions use size classes and a form factor, never pixels. A 2160 px phone
in landscape is about 800 dp wide (`width: medium`, `height: compact`,
`form: phone`); a 32-inch monitor is `width: expanded`, `height: expanded`,
`form: desktop`. A rule that widens a layout reads `form`, or `width` together
with `height`, never `width` alone. The **widening gate** of `table` and
`master-detail` holds when `form` is `tablet` or `desktop`, or when `width` is
`expanded` and `height` is not `compact`.

Conditions have container semantics: `width`, `height`, and `orientation` are
evaluated against the nearest enclosing pane, not the window, while `form`,
`pointer`, and `theme` describe the device. A list rendered
in the left pane of a `master-detail` layout on a tablet sees that pane's width
class.

A selector may end in one runtime state or one data condition:

| Condition | Holds when | May change |
| --- | --- | --- |
| `:pending` | The node owns an operation still awaiting its reply and shows the existing pending indicator: a submit, a link, or a calculation. | Style only |
| `:error` | The node currently shows validation or submission feedback. | Style only |
| `:selected` | The node is the selected member: the member shown in a `master-detail` detail pane, or the current `tabs` section. | Style only |
| `:condition(expression)` | The expression, evaluated on the displayed value, is true. | Style, layout, or fold state |

A data condition uses the [ValidationRules expression language](validation-rules.md)
rooted at the selected node: `[]` is the node, `[status]` a child, curly
references read the [interaction reference context](protocols.md#interaction-reference-context),
and the same `|`, `is`, and comparison operators apply. The expression must be
Boolean and reads only the selected node and the reference context. Absent
data follows the expression language's null rules, and a condition that does
not yield true does not apply.

**Conditions are locale independent.** Display text is localized, so a data
condition or switch case never inspects text: it may not compare a String or
FormattedText value with a string literal or another text value, nor apply a
string operation to one. It branches on locale-independent data instead: an
Enumeration or Union (`is`), a Boolean, a Number, a date, a null check, or a
FormattedText's resource key from
[`resource(...)`](validation-rules.md#131-string-resource-references), for
example `resource([status]) == "STATUS_NOT_STARTED"`, which compares the key
rather than its localized text. Studio rejects a condition that reads text any
other way.
`^<TopicRow>:condition([status] is NeedsAttention)` with `color` and `icon`
declares a status badge; no schema attribute is needed.

A condition is re-evaluated whenever its inputs change. It may change style,
layout, or fold state only. It can never hide a field, make it read-only,
change a value, or change what is sent; those are schema.

### Display Template switch

A rule may carry a `switch` instead of a `:condition(...)` on its selector: an
ordered `cases` list, each with a `condition` (a Boolean ValidationRules
expression rooted at the selected node, with the same reads as a data
condition) and its presentation members, and an optional `else`. Cases are
evaluated in file order; the first true case applies and evaluation stops, so a
later case never repeats an earlier test. When no case is true, `else`
applies, and without `else` the rule does not apply. `when` stays at the rule
level and gates the whole switch. `:condition(...)` on the selector is the
one-case shorthand.

```json
{ "select": "^<TopicRow>", "switch": {
    "cases": [
      { "condition": "[status] is NeedsAttention", "style": { "color": "error", "icon": "warning" } },
      { "condition": "[status] is Done",           "style": { "color": "primary", "icon": "check" } }
    ],
    "else": { "style": { "emphasis": "muted" } } } }
```

A switch has at least one case and at most one `else`; every case has a
Boolean condition, and no case or `else` lacks a presentation member. A rule
cannot have both a `:condition(...)` selector and a `switch`. Cases and `else`
carry the same members as a rule except `select`, `when`, and `switch`.

### Display Template catalog

The catalog is closed: every renderer implements each layout, token, and hint
below. There is no free-form CSS, pixel value, literal color, or font. Studio
rejects a layout on a node kind its row does not allow.

Sequential screens are a workflow, not a layout. A step-by-step layout would
add a second Previous and Next inside screens that already move through a
workflow's Previous and Next, which confuses users; an App that wants
sequential screens splits the request into a
[workflow](protocols.md#workflow-interactions), where the service validates,
branches, and saves each step. `tabs` is how one request is split into
sections. `stepper` is not a Display Template layout: Studio rejects it, and a
client that meets it draws `rows` under the forward-tolerant rule.

| Layout | Applies to | Behavior |
| --- | --- | --- |
| `rows` | Any node | The default: one row per child. |
| `form-grid` | Table | Fields flow in declaration order into `columns` (1 to 4) per row, each as label and value. A field's `span` token takes more columns. `columns: 1` is the label-and-value form. |
| `card` | Table or List member | A bordered block titled by the node's header text. |
| `table` | List of Tables with scalar fields | One row per member; `columns` names the member fields shown, in order; an optional `sort` names one of them and orders the rows ascending by it for display only, never changing the List. Applies only while the [widening gate](#display-template-conditions) holds; otherwise `rows`. |
| `tabs` | Union, or Table with `sections` | One tab per Union branch or section. |
| `master-detail` | Response | `right_pane` lists the editing Request variants that belong beside this leading Response; declaring it implies this layout. Required `refresh` loads and refreshes the leading page. Optional `detail` selectors name linked nodes eligible for selection highlighting; a selector ending in `[]` names List members. Membership depends on the current editing Request, including an editor opened directly through Go to or AI Search. Applies only while the widening gate holds; otherwise `rows`. See [master-detail panes](#master-detail-panes). |
| `accordion` | Table or List | Folding sections with the [folding](#folding-structs-and-unions) behavior and `initially_open`. |
| `split` | Table | Two or three `regions` in a `direction`, `row` or `column`. See below. |

In ordinary label-and-value rows, Android and iOS measure the rendered label
and value, including icons and controls, using the current font and localized
text. They keep both beside one another when their widths and the platform's
label-value gap fit the available row width; otherwise the value goes below
the label. The available width belongs to the actual pane or `form-grid` cell.
This client-owned arrangement needs no additional template rule.

<a id="master-detail-panes"></a>

**Master-detail panes.** A rule on a leading Response declares `right_pane`,
a non-empty array of editing Request variant native names from the same
protocol, and `refresh`, a pure navigation Request that loads that Response.
`right_pane` implies `layout: "master-detail"`; an explicit layout must agree.
For example:

```json
{
  "select": "^<Income>",
  "when": { "form": ["tablet", "desktop"] },
  "right_pane": ["Wages", "Interest", "InvestmentSales"],
  "refresh": "OpenIncome"
}
```

The **current editing Request** determines membership. Resolve a
[WorkflowRequest](protocols.md#workflow-interactions) to its Request editor,
or adopt the complete Request supplied by EditRequest, before looking up
`right_pane`. The carrier Response, originating link, Go to, AI Search, and
previously displayed section do not determine placement. At most one leading
Response may own that Request under overlapping device conditions. A rule's
relationship cannot depend on a leading-page data condition, pseudo-state,
or conditional switch. Ordinary presentation precedence resolves device rules.
An editor outside an applicable relationship uses the whole working surface.
A displayed Response also uses the whole working surface; a relationship names
editing Requests, not Response variants or navigation Requests that lead to them.

- **Initial leading-page load.** On installing an associated editor above the
  [widening gate](#display-template-conditions), construct `refresh` from the
  current complete editing Request by the shared
  [construction-context rules](schema.md#context-and-syntax), without a bound
  member, and send it as an ordinary correlated Request. Keep the editor's
  server-supplied interaction reference context unchanged. The reply must supply
  the declared leading Response before the editor is embedded; a same-named
  Response already on screen is not evidence that it is the right section or
  record. Resolve placement before rendering the editor: during this load keep
  an existing leading page in its current layout, or show the ordinary loading
  surface when no page is present. Render the editor directly in its resolved
  pane, without first showing it on the whole working surface. The same order
  applies when navigation chooses another editor while the leading pane is
  already visible. This supports entry into any section, including direct entry
  with no previous leading page. It never submits the editing Request automatically.
  On failure, report the ordinary error and keep the editor usable as a full
  page. A stale or duplicate reply cannot replace another editor.
- **Navigation ownership.** Both panes belong to the endpoint's one working
  surface. The leading pane owns complete navigation and the shared toolbar.
  Both panes fill the content height and scroll independently; the editor keeps
  its heading and content actions without another screen toolbar.
  An embedded workflow editor shows **Close** and primary **Next** in one
  compact action row. **Go back disappears**, including every `PrevRequest`
  chooser and shortcut, and the editor has no Back control of its own.
  This is unconditional while embedded; no comparison with Close or another
  action determines suppression. Next retains ordinary validation and explicit
  submission. Close clears the right pane, sends nothing, and keeps the leading
  page; accepted Close follows the ordinary
  [draft lifetime](annotations.md#draft-scope-and-lifetime).
  Outside the pane the editor's eligible Go back and Cancel controls return.
  Platform Back acts on the whole working surface under the
  [landing](protocols.md#control-endpoint-landing-and-native-back) and
  [leaving](protocols.md#exit-while-pending) rules.
- **Complete navigation is the App's responsibility.** The leading page must
  expose the navigation needed to revisit editable sections and leave the
  arrangement. Its links and actions construct from its own context, never
  the right editor's current values or private workflow context. Selecting
  another destination closes an idle editor before opening it; selection does
  not submit or save the edit. A flow requiring a previous action that depends
  on the editing context must remain a full-page flow.
- **One pending operation for both panes.** While an operation awaits its
  reply, request-sending controls in both panes are disabled. Auxiliary
  calculations retain their ordinary ownership. During a leading-page load
  or refresh the working surface accepts no edit, link, import, Submit, or
  Close; it shows loading until the reply. Native Back follows the leaving
  rule. All replies remain owned by their ordinary pending operations.
- **Every successful right-panel commit refreshes.** After explicit Submit
  receives any reply outside the failure class (neither a failure status nor
  an Error), construct the declared `refresh` from the current leading page
  with the ordinary construction rules and no bound member, and send it.
  This includes successful commits that install no next page. If the commit
  reply itself supplies the updated leading Response, it replaces that page
  and clears the editor; no redundant refresh Request is sent. Validation
  failures, rejected commits, Close, calculations, and a link return that keeps
  the editor open do not refresh. Never replay the Request that originally
  produced the leading page: it may have been a save.
- **Following a commit.** Apply the refresh before presenting a new page from
  the commit. A newly resolved editor is placed by its own `right_pane`
  relationship; if it belongs to another leading page, load that page from
  that editor before embedding it. No row match is required. A terminal
  Response or an unassociated editor becomes the whole working surface.
  A refresh failure leaves the previous leading page in place, reports the
  ordinary error, and does not discard the valid commit reply.
- **Refresh replies.** The declared leading Response replaces the leading
  page even without NextRequest or WorkflowRequest; the
  [non-transition acknowledgement](protocols.md#protocol-roles) rule does not
  apply to this page load. An Error leaves the previous leading page in place
  and follows ordinary error handling. Any other admitted refresh reply becomes
  the whole working surface, as on a phone.
- **Optional row highlighting.** `detail`, when present, is a non-empty array
  of selectors naming linked nodes within the leading Response. An entry
  ending in `[]` selects List members; another entry selects one node. Paths
  traverse struct fields only, never another List or Union. Selected nodes
  and their ancestors must be visible, and each node must carry a Link with
  NextResponse. Highlighting uses the schema's explicit
  [enum/protocol association](schema.md#enumeration-protocol-associations),
  matching marked enum values in the current editing Request and linked row,
  including shared Request variants and direct entry. The matching node exposes
  the [`:selected` pseudo-state](#display-template-conditions); the template
  supplies its visible style, for example `"style": { "emphasis": "primary",
  "color": "primary" }`. Restrict `detail` to the intended navigation rows:
  including a summary link that repeats a topic creates an ambiguous match.
  Missing or ambiguous matches clear highlighting and never
  determine pane membership or require a full-page fallback. Clients do not
  compare link-request fields or choose the first row to infer selection.
- **Crossing the widening gate.** Narrowing shows the current editor as a full
  page above its leading page, with ordinary Back, Go back, and Cancel controls.
  This change sends no editing Request. Widening evaluates the current editing
  Request's relationship again and embeds it, restoring Close and suppressing
  Go back. If its leading page has not yet been loaded, perform the initial
  load described above. A pending operation retains ownership through resizing;
  apply its reply once, then use the current device conditions.

The loader must be constructible from both the leading Response and every
listed editing Request, using Fixed, Default, or supplied CopyFrom values.
It has no editable input, RequestConfirmation, or subscription behavior, changes
nothing in the service, and admits the leading Response in NextResponse.
When refresh carries earlier values such as a revision, the service admits
pure navigation by identity rather than rejecting a stale editing revision.
Below the widening gate no pane-only load or refresh is sent.
Retained compiled templates without `right_pane` keep their earlier detail-link
arrangement; new relationships use the explicit editing-Request declaration.

`sections` is an ordered list of `{ "fields": [...], "title": "KEY" }`
objects. `fields` names direct children by native name; `title` optionally
names a package [`StringResources`](#string-resources-and-formatted-output)
key whose localized text titles the section, and without it the section is
titled by its first field's display name. A child appears in at most one
section, and unlisted children go to the last section. Each section shows its
listed fields in the listed order, followed by any unlisted children in
declaration order.

`split` is the one composable layout. Each region has a positive `ratio`, the
direct children it holds as `fields`, optional `style`, and its own nested
layout: `rows` (the default), `card`, `form-grid` with its `columns`, or
another `split` with its own `direction` and `regions`. Splits nest at most
three levels deep. A child appears in at most one region, and unlisted children
go to the last region. A region shows its listed fields in the listed order,
followed by any unlisted children in declaration order. On `form: phone` a split renders as `rows` unless a rule
whose `when` holds on a phone asks for it. A split is how a photo on the left
with name, date of birth, and employee ID as rows on the right is expressed;
every other layout arranges all children one way.

Adjacent split regions have a platform-owned gutter in either direction and
at every nesting level: 16 dp on Android, 16 pt on iOS, and 16 CSS px in Studio.
The ratios divide the available extent after the gutters are deducted; nested
device conditions use the resulting region size. The gutter is automatic,
requires no `display.json` member, and is independent of the row `spacing`
style token. A split rendered as ordinary `rows` has no region gutters.

#### Labels and titles

Two members change the text a person sees on the node their rule selects:

- `label` replaces only the node's displayed name. The row keeps its value:
  the node's own value, or the child a
  [`promote`](#tableheader-promoted-fields-and-union-selectors) rule puts on
  the row.
- `title` replaces the whole row with its text. The row's label and value are
  both removed, and nothing is promoted into it, because it has no value; a
  child that a `promote` rule names keeps its own row, and Studio warns that
  the `promote` has no effect.

Both change only the text, never its typography. The node keeps the
typography of its position: a page's title is drawn as a page title, a group
header inside a page as a group header, and a List member's row as a member's
row. A [`role`](#style-tokens) style token changes that look where an App
wants another one, for example `role: label` for a quieter group header; no
client reads `label` or `title` as a typography instruction.

Both take the same two value forms:

- `{ "field": "path" }` takes the text from the data. `path` is a dot path of
  field native names below the selected node, through struct fields only,
  never through a List or a Union, ending at a String or FormattedText field;
  that field may be Hidden or ReadOnly. Its value, a FormattedText as its
  localized text, is the text.
- `"KEY"` gives fixed text: a package
  [`StringResources`](#string-resources-and-formatted-output) key, as a
  section's `title` is.

A field path never passes through a Union, because the selected branch is
known only at run time. On a Union node, and on a node reached only through a
Union, such as a Union branch, a node below one, or a named type used only
there, only the `"KEY"` form is valid; Studio rejects the field form there
with a message that names the key form.

A `label` or `title` rule may select:

- a page, a displayed Request or Response variant (`^<Hub>`): the text is the page title and
  replaces the page's top row as the member says;
- a struct, by its type (`^<Type>`) or by a field of it: the text replaces the
  struct's row as the member says;
- a Union field, in the `"KEY"` form only: with `label` the selected
  variant stays its value;
- a List's members (`^<Resp.list>[]`): each member row takes its text from its
  own field, which replaces both the member name and its position number.

When the field is null, unanswered, or empty, the node draws as it would
without the member, with its position for a List member, so a row is never
blank. The display name stays the schema's meaning: AI tool descriptions and
every declaration keep it, and `label` and `title` change only what a person
sees. The source field is normally declared Hidden in the schema, so its
value is not drawn twice; a Hidden field remains a valid source. Hidden stays
a schema attribute that the template never sets: Studio neither hides a
visible source field nor rejects it, but reports a warning at the `label` or
`title` line of `display.json` that names the visible field and suggests
Hidden. A visible source field keeps its own row.

`label` and `title` exclude each other on one node, and neither combines with
`suppress_label` on that node. A `label` with a `promote`d child is how a row
is [data-titled](#data-titled-rows). Both resolve through the
[cascade](#display-template-cascade) and `when` like every other member; a
data condition, switch case, or pseudo-state changes style, layout, or fold
state only and never sets either.

On a struct, the two members combine with [`promote`](#field-promotion) on a
child and with [`flatten`](#flatten) on the struct as follows:

| On the struct | With | Result |
| --- | --- | --- |
| `label` | `promote` on a child, `use_parent_name` true | The row's label is the struct's displayed name, that is, the `label` text; the child is the value. |
| `label` | `promote` on a child, `use_parent_name` false | The row's label is the child's own name. Studio warns that the struct's `label` shows nowhere. |
| `title` | `promote` on a child | The child is not promoted and keeps its own row. Studio warns that the `promote` has no effect. |
| `title` | `flatten` with `{ "name_row": false }` | Rejected: one removes the name row and the other replaces it. |

With `label`, the link of a promoted value stays on the row; with `title`, a
link on the struct stays on the title row
([actions](#actions-on-a-promoted-or-suppressed-row)).

```json
{ "select": "^<TopicRow>", "label": { "field": "title" } }
{ "select": "^<TopicRow>.status", "promote": true }
{ "select": "^<Hub>", "title": { "field": "topics.name" } }
```

The first two rules draw each topic row as "W-2 wages   Entered": its `title`
field as the label and its promoted `status` as the value. The third draws the
hub page's top row as just "Income", the name of the section whose topics the
page lists.

#### Button labels

`button_labels` changes the captions of existing controls on one protocol
page. It is a non-empty object with one or more of these slots:

| Slot | Existing control | Behavior retained |
| --- | --- | --- |
| `go_back` | Workflow Go Back | Constructs and sends the selected `PrevRequest`; native Back is unaffected. |
| `next` | Workflow Next/Submit | Validates and submits the current Request; it does not choose a `NextRequest`. |
| `close` | Page or detail-pane Close | Performs the existing local dismissal without submitting. |
| `actions` | Page Actions | No eligible choices hides the control; one selects directly; several open the ordinary chooser. |

Each slot accepts a package `StringResources` key, or `null` to restore the
localized platform caption. Literal display text, field sources, empty strings,
and unknown slots are invalid. Localization uses the package that authored the
winning rule and the ordinary resource fallback. If a client cannot resolve a
non-empty caption, it uses the platform caption, never a blank button or raw
resource key. Omission preserves a lower-ranked override or the platform default;
`null` overrides lower-ranked captions with that default. Slots do not inherit
into children or another page.

The `actions` slot also accepts exactly `{ "source": "single_action" }`.
It uses the sole offered action's localized schema variant `Name`. Count the
final offered set after `NextRequest`, `ConstrainedBy`, access privileges, `ExcludeUI`, and
`LinkOnly` filtering, and an explicit `Actions` array when the page supplies
one ([named action controls](protocols.md#named-action-controls)). With several choices, or a missing or blank name, the
source supplies no caption, and the next source in the
[Actions caption precedence](#named-action-controls) applies: an explicit
Actions field name, then the localized platform Actions caption, never a raw
native name or payload type name. With no choices, retain the existing hidden
control. This source
uses the action's owning document, not the
template author's resources or a display title. No other slot accepts it.
An explicit `null` resets this source just like a resource override.

```json
{ "select": "^<Hub>", "button_labels": { "actions": "GO_TO" } }
{ "select": "^<SectionHub>", "button_labels": {
    "actions": { "source": "single_action" } } }
{ "select": "^<SaveIncome>", "button_labels": {
    "next": "SAVE_AND_CONTINUE", "close": "CLOSE_EDITOR" } }
{ "select": "^<Finish>", "button_labels": { "next": "FINISH_REVIEW" } }
{ "select": "^<SaveIncome>", "when": { "form": "phone" },
  "button_labels": { "next": null } }
```

Rules select whole Request, Response, or SubscriptionData variants of an
Endpoint protocol. `go_back` and `next` require a Request variant. A rule
cannot select a field, reusable payload type, List member, configuration, or
persistence-only variant to rename page controls. Device `when` conditions
are allowed; data conditions, switch cases, and pseudo-states cannot set
`button_labels`. A valid caption remains dormant when its control is absent.

The active displayed variant owns its captions. A workflow editor uses its
Request, not its originating Response. In master-detail, the leading page
owns its toolbar and the trailing page owns its own controls. An embedded
editor's Go Back remains suppressed, even when a caption is configured.
Captions never create controls, change eligible requests or their order,
change input presentation, submit automatically, or alter validation,
confirmation, pending-operation ownership, or draft lifetime. Request names
inside choosers, native navigation, ordinary request-editor Send, and
confirmation/picker Send and Cancel labels are unchanged.

Choose wording that remains truthful for every state in which the page uses
it. For example, "Go to…" can describe a sole destination-chooser request;
"Return to review" is appropriate only when the offered operation actually
returns to review. Renaming Actions does not change its target. "Save changes"
does not promise a particular next page when the service can ask follow-up
questions; Close must not imply submission or saving.

Clients measure the resolved caption with the actual locale and accessibility
text size under the existing responsive control and heading rules, allowing
content action rows to wrap as needed while retaining readable text and touch
targets. The visible caption is also the accessible name. Re-resolve it when
the displayed variant, offered action set, template, locale, or device condition changes. Older
clients ignore this optional member and retain their platform captions.

#### Named action controls

The [ActionControl schema attribute](protocols.md#named-action-controls) declares
the controls and their choices. No authored template is required. A named
control uses its field's ordinary localized display name; this is a document
name, not the retired UIName schema attribute. The built-in Actions control
keeps its platform caption when implicit, and an explicit Actions field can
supply its default name. The selected Requests keep their own localized names
inside a chooser.

Actions is the prominent page-heading control. Its default treatment uses a
filled accent background, contrasting text and a semibold caption. Optional
named controls use secondary emphasis: a transparent background, subtle outline
and normal-weight caption. Controls share normal platform sizing, padding and
corner treatment; emphasis never reduces their touch targets. Authored emphasis
and density tokens retain their ordinary overrides.

Named controls use their field declaration order. By default they join Actions
in the top bar only when the full title and the complete eligible named-control
group fit, after reserving navigation, system controls and Actions. Actions is
always last in a shared action row, at the logical trailing end (right in a
left-to-right layout). Promotion never shortens a full title, displaces Actions,
or creates a repeated content heading. Decide Actions placement first under the
[heading rule](#protocol-page-heading-and-top-bar), then measure named controls
in the remaining space. Move all header-eligible named controls together; do
not promote a smaller subset. With no eligible Actions, apply the same full-title
and complete-group fit test to the named controls alone.

Named controls that stay in content occupy one secondary region immediately
below the page heading and above content, leading aligned with full captions.
They never join the first content title row, even when Actions moves there.
They wrap as space requires; an individually oversized caption wraps within
its button and grows its height. No eligible choices hide that control and
release its space; an empty secondary region takes no space. A template may
keep a named control in this region or move Actions there, after the named
controls. In master-detail, the leading page owns global controls;
trailing Response controls stay in their pane. An embedded Request editor keeps
Close and Next, with no response controls.

On a whole Response or SubscriptionData protocol-page rule, `action_controls`
is an object keyed by an exact declared control name or `Actions`. A rule
cannot invent a control. Each entry can set `label`, `placement`, and the
`emphasis` and `density` style tokens. Members cascade independently under device `when`
conditions; data conditions, switches, and pseudo-states cannot set them.

```json
{
  "select": "^<Hub>",
  "action_controls": {
    "sections": { "label": { "field": "section_caption" }, "placement": "secondary" },
    "Actions": { "label": "PAGE_ACTIONS", "placement": "secondary" }
  }
}
```

`placement` is `secondary` or `header`. For named controls, omission and
`header` both request the adaptive top-bar group placement above; `secondary`
keeps that control below the heading and excludes it from the header-eligible
group. A multiline caption or a button too tall for the native top bar makes
the group remain in the secondary region. For Actions, omission and `header`
retain its independent heading rule; `secondary` moves it into the secondary
region as its last control. A trailing pane does not get another screen
toolbar: its named controls stay below its local heading, while Actions retains
its local heading placement. Styling never changes eligibility or invocation.

`label` accepts a package StringResources key, `{ "field": "path" }`, or
`null` to restore the default caption. The field form uses the existing
[label path](#labels-and-titles) below this displayed payload through struct
fields only, ending at a String or statically ReadOnly FormattedText field.
It never reads the selected Request, a reference context, List or Union
member, or an expression. A Hidden caption source is allowed as an explicit
text source. Invalid paths and types fail Studio validation. An absent, null,
empty, or unresolvable current value falls back to the field's localized name
or platform Actions caption, never a blank or raw key. Caption updates do not
select or submit an action. For Actions, an explicit
`action_controls.Actions.label` takes precedence over `button_labels.actions`,
then an explicit Actions field name, then the platform caption.

#### Task-controlled section-hub action

For a walkthrough with section hubs and a main Review page, declare the
continuation and review Requests in each hub's `NextRequest`, then let the
task's `ConstrainedBy` value offer exactly one of them. Use the
`single_action` caption source so its name and destination change together.
Keep row-opening Requests `LinkOnly`; they remain available to their links
without becoming additional Actions choices. Clients need no walkthrough
flag, navigation history, or conditional caption rule.

For example, the tax task offers **Continue** while walkthrough
steps remain. Each section hub has its own continuation Request, which
records that section's completion and advances to its next applicable
section using the task's existing navigation. When no walkthrough step
remains, the task offers **Review**, opening the main Review
page. If an accepted edit creates unfinished walkthrough work, the task
offers continuation again. Derive this choice from existing completion state
rather than persisting a separate presentation flag. Remove a redundant
next-step or Review row when this button owns that operation.

The section-hub Actions control and an editor's Next control remain separate.
An editor's **Save and continue** submits its answers and follows the task's
save route; a hub's **Continue** follows that section's onward route. The
routes may differ. A hub action cannot submit an open editor's uncommitted
fields. Preserve other required leading-page navigation when replacing a
chooser with one direct action. Keep full captions on phones and use the
responsive sizing and wrapping above.

#### Style tokens

| Token | Values |
| --- | --- |
| `density` | `compact`, `comfortable` |
| `emphasis` | `primary`, `secondary`, `muted` |
| `align` | `start`, `end`, `center` |
| `role` | Typography role: `title`, `body`, `label` |
| `color` | Platform color roles only: `primary`, `secondary`, `tertiary`, `error`, `surface`, `on-surface-variant` |
| `spacing` | A five-step scale, `0` to `4` |
| `span` | `1` to `4`: the columns a field takes inside a `form-grid` |
| `icon` | One of `check`, `warning`, `info`, `lock`, `link`, drawn before the value; with `:condition(...)` it makes a status badge |

Style tokens are visual only. They never disable, hide, or change the
interaction of what they style: a `muted` row keeps its links and its editing,
and hiding stays the schema's Hidden. An App that wants a row out of reach
leaves it out of the data or its allowed choices instead.

Android maps tokens to Material 3 roles and window size classes, iOS to SwiftUI
roles and size classes, and Studio to its control-view variables. A `media`
token for larger inline image placement remains **deferred**. It is not part
of this catalog. The default media field and preview
behavior are defined [below](#mime-aware-bytes-presentation).

#### Behavior hints

Each hint has exactly the meaning its source attribute has, and `promote`,
which has no source attribute, the meaning its section defines; the linked
section keeps that meaning and any authoring syntax.

| Hint | Value | Source attribute and meaning |
| --- | --- | --- |
| `inline` | `true` or `false` | [InlineChoices](#inline-choices) on an Enumeration or Union node. |
| `fold` | `true`, `false`, or `{ "initially_open": Boolean }` | [Foldable](#folding-structs-and-unions) on a Table, Union, or List; `true` means `initially_open` true, `false` makes the node unfoldable. |
| `promote` | `true`, `false`, or `{ "use_parent_name": Boolean }` | [Promotion](#tableheader-promoted-fields-and-union-selectors), with no source attribute. Valid only on a rule selecting a field of a struct, never a function parameter: that field becomes its parent row's value. `true` means `use_parent_name` true; `false` turns promotion off. |
| `suppress_label` | `true` or `false` | [SuppressLabel](#suppresslabel-label-free-value-rows): omit the selected value row’s label; on a List of structs omit the List-contributed member names and positions. |
| `include_description` | `true` or `false` | [UIIncludeDescription](#common-field-metadata). |
| `format` | One pattern string | [`Format`](#display-formats): only the portable pattern grammar that section defines, checked against the node's effective type; no other string is accepted. |
| `scale` | One scale type token | The chart scale type of a scale-bearing chart field, under the [chart specification's](charts/2d.md#6-chartscale) type matrix. |
| `guide` | One guide position token | The position of the selected field's chart guide, under the [guide position matrix](charts/2d.md#64-chartguide-generic-guides). |
| `palette` | `auto`, `categorical`, `sequential`, or `diverging` | The color palette of a chart root; `auto` is the chart specification's default palette. |

Every hint keeps its source attribute's placement rules, including the node
kinds it may select, and a hint never makes a read-only value editable.

### Template validation

Studio's shared validator owns every template rule. It rejects a compiled
template unless:

- `DisplayVersion` is known and every rule has exactly one `select` and at
  least one presentation member;
- every selector resolves against the package schema under the
  [selector grammar](schema.md#display-template-selectors), including its
  name-collision rule;
- a pseudo-state appears only on a rule whose only presentation member is
  `style`;
- every data condition and switch case parses under the ValidationRules
  grammar, is Boolean, and reads only the selected node and the reference
  context, every switch meets the rules [above](#display-template-switch),
  and neither a data condition nor a switch case sets `label`, `title`, `button_labels`, `action_controls`,
  `promote`, or `flatten`;
- no rule member is outside the catalog, including every schema-only
  attribute;
- every layout selects a node kind its row allows; every `master-detail`
  relationship names a non-empty list of distinct Request variants of the same
  protocol, excludes a WorkflowRequest leading Response, has no overlapping
  other owner, and does not depend on data conditions or switches; each optional
  `detail` entry is a `^<…>` path within the selected Response, never a
  `$…` path or a descendant search, and resolves through struct fields only;
  no entry repeats another; no detail node is Hidden or lies under a Hidden
  node; and every detail node carries a `Link` whose request declares
  `NextResponse`; a `refresh` names a Request of the same protocol whose
  `NextResponse` admits the selected Response, whose every field is Hidden or
  ReadOnly and is filled by Fixed, Default, or a `CopyFrom` alternative that
  the selected Response and every listed editing Request supply, and which is neither a
  `SubscriptionRequest` nor marked `RequestConfirmation`, and a rule with
  `refresh` selects a Response that does not carry `SubscriptionData`; `table` columns and `sort` name scalar fields of the member
  type; `sections`, `regions`, and `fields` name direct children; a section's
  `title` and a fixed `label` or `title` name a package `StringResources` key;
  and splits nest at most three levels;
- every `{ "field": "path" }` of a `label` or `title` resolves below the
  selected node through struct fields only, never through a List or a Union,
  and ends at a String or FormattedText field; the selected node is neither a
  Union nor reached only through a Union, where only the `"KEY"` form is
  valid; no node has both `label` and
  `title`, and neither is combined with `suppress_label` on the same node;
  Studio reports these at the member's line of `display.json`;
- every `button_labels` is a non-empty object of the four slots above, with
  package resource-key or null values, selects only whole protocol page
  variants, and sets `go_back` or `next` only on Request variants;
- every `action_controls` key names a control declared on the selected whole
  Response or SubscriptionData page, or built-in Actions; its label source,
  placement, and style satisfy the named-control rule above;
- every `promote` rule selects a field of a struct, never a type, a List
  member, a Union branch, a function parameter, or a Response; every
  `flatten` rule selects a struct, a List, or a Union, by type or by field,
  never a List member or a Union branch, and its value is `true`, `false`, or
  `{ "name_row": false }`; `name_row` appears only on a struct; and no struct
  has both `title` and `flatten` with `{ "name_row": false }`;
- every hint satisfies its source attribute's placement rules and value
  grammar, and every `when` key and value is from the closed sets.

Rules of equal rank that set the same member on the same node are allowed, the
later one winning, and are reported as a warning. Studio also warns, at the
member's line, when the source field of a `label` or `title` is not Hidden,
naming that field; when a struct's `label` shows nowhere because its
promoted child has `use_parent_name` false, and when a `promote` has no effect because its
struct has a `title`. Studio additionally lints
minimum touch targets; more than about eight rendered (non-Hidden) fields
shown together on a screen reachable from a Request, Response, or
SubscriptionData payload or a function's configuration; a required field
folded closed while `form: phone` may apply; contrast by color role; and a
`split` whose regions leave a required field unlisted in a nested region only.

The fields-per-screen lint reports each Table once, under its own name and
source line; an inline Union branch is named `Union.Branch`. A screen sums
the fields of its nested Tables, so several small Tables add up. A field
whose content shows only on demand counts one row: a Union, whose chosen
branch is linted as its own screen, so a Union counts by its largest branch
and never the sum; a Nullable container; and a nested Table reported on its
own. A node is exempt under `sections` with `tabs`, a `split`,
or a fold that starts closed, from `display.json` or a source `Foldable`.
Exemption holds only on the forms its rule's `when.form` names, so a rule
limited to tablet and desktop still leaves one phone warning, which says so.

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
| Inline long description | `@LibertasUIIncludeDescription` | `#[libertas_ui_include_description]` |
| Not editable | `@LibertasReadOnly` | `#[libertas_read_only]` |
| Hidden from ordinary editor | `@LibertasHidden` | `#[libertas_hidden]` |
| Unordered List | `@LibertasUnordered` | `#[libertas_unordered]` |
| Scoped uniqueness | `@LibertasUnique(level)` | `#[libertas_unique(level)]` |
| Size bounds | `@LibertasSize(min=...,max=...)` | `#[libertas_size(min=...,max=...)]` |
| Live Number/DateTime minimum | `@LibertasRangeMin("expression")` | `#[libertas_range_min("expression")]` |
| Live Number/DateTime maximum | `@LibertasRangeMax("expression")` | `#[libertas_range_max("expression")]` |
| Data-backed choice index | `@LibertasEnumSource("$.items")` | `#[libertas_enum_source("$.items")]` |

The inline-description row is compiled into the
[Display Template](#display-template); the others are schema. A summary value
is a template [`promote`](#tableheader-promoted-fields-and-union-selectors)
rule, and flattening a [`flatten`](#flatten) rule; neither has a source
attribute.

`Hidden` suppresses the complete value subtree, including list elements and
controls, nested fields, summaries, and promoted values. Hidden data remains
part of the typed value for validation and protocol encoding. `flatten` does
not expose hidden descendants. `promote` and `flatten` apply to the node their
rule selects, are not inherited through ordinary attribute resolution, and
never override `Hidden`.

`ReadOnly` restricts the user only: clients present the value and offer no
ordinary editing control for it. It never restricts the platform or the
application. Schema initialization (Fixed, Default, localized defaults),
`CopyFrom` from a Response or the reference context, `EditRequest` values,
calculation and link-return `CopyTo` results, draft restoration and the application's own
handling of the submitted value all write a ReadOnly field exactly as they
write an editable one. A value that must not change after initialization is
`Fixed`, not ReadOnly. Hidden fields are ReadOnly in effect, since no editor
shows them, and are written the same way.

`ReadOnly` on an Array/List prevents changing its membership or order: no Add,
Remove, or reorder action may mutate that list. It does not recursively make
member values read-only; members and their fields keep their own editability.
To prohibit member edits, mark the applicable member type or fields as well.
Context menus, swipe actions, pickers, and deferred actions must enforce the
same list-owner rule at both presentation and execution. A promoted row does
not change which list owns a removal or reorder operation.

For promotion, flattening, and nested Union layout, see
[Promotion, flattening, and Union selectors](#tableheader-promoted-fields-and-union-selectors).

`EnumSource` belongs on an unsigned integer. The integer stores the selected
List item's zero-based index. It does not store the item itself.
AI tool-call descriptions automatically explain that index representation and
identify the source List path; they do not enumerate its current items.
Android and iOS present a single-choice picker in source order. Each choice
uses the item's scalar display value or its promoted value; a composite
without a scalar header uses its display name. Numbered labels distinguish
repeated names. Money and explicit scalar formats retain their normal display.

A chooser with more than twelve permitted choices — this EnumSource picker, an
Enumeration picker, a [Bitflags](#bitflags) multiple-choice picker, or the
[Union variant chooser](#tableheader-promoted-fields-and-union-selectors) that
opens when a promoted or unpromoted value is tapped — shows a text filter
field above its list. Typing filters the visible choices by a case-insensitive
substring match against each choice's localized display name and description;
it never changes the permitted set, its order, or the selection, and clearing
the field shows every choice again. The current selection stays visible and
selected even when the query would otherwise filter it out.

The [hierarchical single-field request builder](protocols.md#hierarchical-single-field-requests)
also presents a choice-only Union/Enumeration hierarchy as expandable groups.
It derives grouping from the App's types and removes groups with no eligible
leaves recursively. This is the default compact request presentation for that
shape; ordinary content folding and the Display Template do not change the
underlying selection type, constraints or submission behavior.

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
Studio compiles it into the [Display Template](#display-template) as the
`include_description` hint, which clients read at runtime.

For `Unordered`, AI tool-call descriptions automatically explain that order has
no semantic meaning and that this attribute does not prohibit duplicates.

## MIME-aware Bytes presentation

A `Bytes` field with effective [`MimeTypes`](schema.md#mime-aware-bytes) uses
one media control in Android,
iOS, Studio, and the pure Web client. Its compact row shows the field name, a
bounded thumbnail for a safely decodable raster image or a generic media
icon, the exact MIME and content size when defined, and no local path.
Undefined media shows “No file selected.” It never displays Base64 or hex.
Ordinary Bytes keep their existing editor.

Editable media rows offer Choose file, Remove, and Take photo when the field
accepts an image format that the client's camera can produce. The camera
stores its output as a private temporary file and the editor holds a local
link until outbound encoding. Camera permission denial or cancellation leaves
the previous value intact; Choose file remains available. Read-only rows
offer preview when supported. Tapping or clicking the thumbnail opens a larger
platform preview. A corrupt or unsupported image shows the generic icon and
an error when previewed; clients do not render SVG or arbitrary HTML as an
inline image. Local paths and temporary link identifiers are not displayed or
sent to the Hub. Camera availability is conditional on device, browser, and
host permissions.

A missing temporary file during draft recovery makes the field undefined and
identifies it for reselection. A file that disappears at submission produces a
field error and stops that attempt. The [draft and cleanup rules](annotations.md#mime-aware-bytes-in-workflow-drafts)
and [LMB1 contract](schema.md#mime-aware-bytes) own those behaviors. The
deferred Display Template `media` token may later provide larger inline
placement; the standard media control and explicit preview require no token.

<a id="tableheader-promoted-fields-and-union-selectors"></a>

## Promotion, flattening, and Union selectors

Two Display Template members change how a node's children sit beside its row:
`promote` puts one child field's value on its parent row, and
[`flatten`](#flatten) draws a node's children at that node's own indentation.
Neither changes the encoded data hierarchy. `when` may change either per
device; no data condition, switch case, or pseudo-state sets either.

`promote` is valid only on a rule that selects a field of a struct (a Table),
for example `^<TopicRow>.status`, which selects that field wherever the
`TopicRow` type is used, or a strict path to one field. Studio rejects
`promote` on a rule that selects a type, a List member, a Union branch, a
function parameter, which has no parent row, or a Response. Its value is `true`, `false`, or `{ "use_parent_name": Boolean }`; `true` means
`use_parent_name` true, and `false` turns promotion off for the field, so a
higher-ranked rule can undo a lower one. `promote` never flattens the field's
children: select the node with `flatten` when wanted. There is no source
attribute for promotion or flattening: the retired `TableHeader` attribute
(`@LibertasUIHeader`, `#[libertas_ui_header]`) is rejected by Studio with a
message that names the `promote` and `flatten` rules.

### Field promotion

On a field directly owned by a **Table**, the first visible promoted field in
schema order becomes the Table row's value instead of occupying a separate
child row. Additional promoted fields remain ordinary children. Promotion has
no effect when the field's owner is not a Table, including a Union's selected
branch.

The promoted field supplies the row value. Its `use_parent_name` option chooses
the row label, uniformly for scalar, Table, List, and Union fields:

- `true` (the default): use the owning struct's displayed name, which is its
  [`label`](#labels-and-titles) when a rule gives it one.
- `false`: use the promoted field's own display name. A `label` on the owning
  struct then shows nowhere, and Studio warns.

When the owning struct has a [`title`](#labels-and-titles), the field is not
promoted and keeps its own row, and Studio warns that the `promote` has no
effect.

For a promoted Union, the selected variant's display name supplies the row value;
the option changes the label, not that selected value. Tapping that value opens
the choice selector, unless the Union shows [inline choices](#inline-choices).
Selecting another variant replaces the applicable fields.
The selector remains available even when the selected variant has no fields.
A List member appends its one-based position to the chosen row label
(for example, “Household #1” with the default or “Marital status #1” with
`use_parent_name` false), including for a Union.
A scalar promoted value keeps its normal formatting and editor. A composite
one keeps its children, and a List retains its entries and Add action.

An unpromoted Union retains its field title after selection. For example,
“Marital situation” remains the label and “Married” is the selectable value.
A Union with a scalar payload also identifies the selected branch while keeping
that payload editable. Branch selection remains available independently of
promotion, including after choosing a fieldless variant.

### Name option

```json
{ "select": "^<Schedule>.starts_at", "promote": true }
{ "select": "^<HouseHold>.marital_status", "promote": { "use_parent_name": false } }
```

Both rules promote a field, the first labelled by its owner's name and the
second by its own.

### Example: use the parent name to identify the record

A struct has the display name **Next schedule**. Its **Starts at** field is
promoted with `use_parent_name` true (or `promote: true`), and **duration** is
an ordinary sibling field. For a start time displayed as `09:00` and a duration
of `30 minutes`, the rows are:

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
Union promoted with `use_parent_name` false. The selected variant has the
display name **Married**, and **Number of dependents** is an ordinary sibling
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

<a id="flatten"></a>

### Flattening

The rule member `flatten` applies to a struct, an array (List), or a Union. Its
rule selects the node by type (`^<Address>`, applying wherever the type is
used) or by field (`^<Household>.address`). It is a rule member like `layout`,
not a `style` member, so no data condition, switch case, or pseudo-state can
change it; `when` may. Its value is `true`, `false`, or
`{ "name_row": false }`. `false` undoes a flattening that a lower-ranked rule
set in the [cascade](#display-template-cascade), such as a dependency
package's template for its own type: the node draws with normal indentation
and its name row.

- **Struct.** `true` draws the struct's fields at the struct row's indentation
  and keeps the struct's name row. `{ "name_row": false }` also removes that
  name-only row; the fields stay visible and editable at the flattened level.
  A List member whose struct row is removed keeps its position and membership
  controls, and the fold control of a removed row passes to the first row the
  struct still produces, as [folding](#folding-structs-and-unions) describes.
- **Array (List).** The List row keeps its title, count, and Add action; its
  members sit at the List's indentation instead of one level in. Each member
  keeps its own row, position, and actions.
- **Union.** The selected branch's fields sit at the Union row's indentation;
  the selector stays visible and editable.

Studio rejects `flatten` on any other node kind; on a List member (`…[]`)
and on a Union branch, because neither has a row of its own to flatten, with
a message that names the owning List or Union; `name_row` on a List or a
Union; and `{ "name_row": false }` on a struct that has a
[`title`](#labels-and-titles), because one removes the name row and the other
replaces it. Without a `flatten` rule, children keep normal indentation;
protocol page roots follow the [automatic root alignment](#protocol-page-heading-and-top-bar).
`flatten` never promotes a field, including a field that references the
flattened type: select that field with `promote` when wanted. `Hidden` still
suppresses the whole subtree.

```json
{ "select": "^<Dependents>", "flatten": true }
{ "select": "^<Zone>", "flatten": { "name_row": false } }
```

For example, these independent rules combine promotion and flattening:

```text
Household (Table)
  dependents (display name: Dependents; type: Dependents)

Dependents (Union)
  No
  Yes (Table)
    count (Number)
```

```json
{ "select": "^<Household>.dependents", "promote": true }
{ "select": "^<Dependents>", "flatten": true }
```

With `Yes` selected, the schematic rows are:

```text
Household: Yes             [tap value to change choice]
Count: ...
```

Removing the `flatten` rule indents `Count` under `Household`. Removing the
`promote` rule restores a separate Union row under `Household`. The choices
can be repeated at each nesting level. An unrelated sibling is not flattened
because another field is a promoted or flattened Union. Existing expansion,
visibility, and read-only rules still apply; neither member makes a read-only
selector editable.

### Actions on a promoted or suppressed row

Attributes that add an interaction to a value's row, currently `Link` and
`LLMDocTrans` (AI Import), follow that value when `promote` or `flatten`
changes its presentation. A promoted field's action appears on the owning
Table's visible row. If a Table has no row of its own, because `flatten` with
`{ "name_row": false }` removed it, its action appears on the first visible
row that represents that Table; when the Table is the selected payload of a
promoted Union, this is the Union's owning row. A List member keeps its own
row and its own actions, also inside a flattened List. If several values
represented by one row declare actions, the row offers each applicable action;
a child `Link` takes precedence over an enclosing `Link` for the row tap
target. With a [`label`](#labels-and-titles) on the struct, a promoted value's
link stays on the row; with a `title`, a link on the struct stays on the title
row.

Only the **affordance** moves. Each action retains its declaring value, path,
request context, eligibility, and pending state. Neither `promote` nor
`flatten` copies an attribute to another schema or value node. `Hidden`
suppresses the action with its value subtree; `ReadOnly`, `Fixed`, and other
action-specific restrictions still apply. Data, validation, access, and
lifecycle attributes do not move.

### Presentation only

Promotion and flattening do not change the schema or encoded data hierarchy.
`CopyFrom`, `CopyTo`, `EnumSource`, validation paths, and backend field access
still use the original structural paths, including every nested Union/table
level. They do not supply a default, make required fields optional, or disable
fields based on another answer.

Protocol actions remain owned by the enclosing protocol variant, even when its
header is another Union. See [protocol transaction stack](endpoint-presentation.md#protocol-transaction-stack).
Separate same-indent Union rows retain their own edit/removal ownership;
sharing an indentation level does not make them the same data object.

<a id="uiname-data-titled-rows"></a>

## Data-titled rows

A row is **data-titled** when its label comes from the data it shows, such as
a calculated line labelled by its description with its amount as the value.
Two independent rules express it:

- a Display Template [`label`](#labels-and-titles) rule on the row's struct, usually
  `"select": "^<Type>"` or, for a List's members, `"^<Response.list>[]"`, names
  the field whose value labels the row;
- a [`promote`](#field-promotion) rule on a child of the struct promotes that
  child's value into the row.

```json
{ "select": "^<LineItem>", "label": { "field": "description" } }
{ "select": "^<LineItem>.amount", "promote": true }
```

With `description` Hidden, each `LineItem` renders as one row, "W-2 wages
$60,000.00", and its other visible children render beneath it. The names are
schematic.

Each rule also works alone: a label without a promoted child labels a row
whose children render beneath it, and a promoted child without a label keeps
the struct's display name. With the default `use_parent_name`, the row's
label is the struct's displayed name, which is the `label` text; with
`use_parent_name` false it is the child's own name, the `label` shows nowhere,
and Studio warns. Folding, flattening,
links, and a List member's membership controls stay on the row as they do on
any other row. A row that should show only text, with no value, such as a
heading taken from the data, uses [`title`](#labels-and-titles) instead; a
`title` takes no promoted child, so the child keeps its own row.

Neither rule has a source attribute. The retired `UIName` attribute
(`@LibertasUIName`, `#[libertas_ui_name]`) is rejected by Studio with a
message that names this replacement: its title is a `label` rule and its value
a `promote` rule on the named child.

### SuppressLabel: label-free value rows

`@LibertasSuppressLabel` in TypeScript and `#[libertas_suppress_label]` in
Rust suppress the visual label contributed by the selected node. The parser
result carries `{"Name":"SuppressLabel","Value":"true"}`; Studio compiles
it into the [Display Template](#display-template) as the Boolean
`suppress_label` hint and removes it from the final schema. A template rule
can set `false` to restore the ordinary label under the
[cascade](#display-template-cascade). The rule does not propagate to children.

| Selected node | With `suppress_label: true` |
| --- | --- |
| Scalar value, including an Enumeration | Omit its row label; keep its value and editor. |
| Struct | Omit its own row label; keep its promoted value, visible children, and actions. A struct without a promoted value keeps its children and any row controls. |
| Union | Omit the Union's own label; keep the localized selected variant value, selector, visible payload, and actions. An unselected editable Union keeps its selection affordance. |
| List whose effective member type is a struct (Table) | Omit the member name and position contributed by the List to each member row; keep each member's own template label or title, promoted value, visible children, and controls. Keep the List's heading, count, and membership actions. |

Other List member types and non-value declarations are invalid hosts. Studio
rejects them; source syntax does not extend the placement rules. List
suppression retains the previous List-of-struct behavior, rather than
suppressing the List's own heading.

The hint suppresses a visual label, never a value or subtree. It does not
change folding, editing, links, menus, membership permissions, help, or the
semantic name used for accessibility and editor dialogs. A control without a
visible label still has its localized accessible name. On a promoted row,
only its displayed label is omitted; the promoted value and its declaring
link remain available under the [action rule](#actions-on-a-promoted-or-suppressed-row).
`Hidden` continues to suppress the entire presentation subtree and wins
before every template rule.

An explicit `label` or `title` and `suppress_label` are mutually exclusive on
one selected node, including `false`; List suppression can still coexist
with a `label` or `title` on its distinct member nodes. The old source names
`@LibertasSuppressTitle` and `#[libertas_suppress_title]`, parser attribute
`SuppressTitle`, and template member `suppress_title` are retired authoring
forms. Modern compilation rejects them and names their corresponding
replacement; they are not aliases.

A reader of an already validated, retained compiled Display Template accepts
its historical `suppress_title` Boolean solely for the former List-of-struct
member-name and position behavior. At document ingress, translate that member
to `suppress_label` only when the rule has no `suppress_label`; an explicit
modern member, including `false`, wins. Normalize a runtime copy without
rewriting the retained document bytes or their digest. This isolated artifact
reader does not accept the old spelling in new source or `display.json`, and
renderers consume only the modern hint.

#### Protocol page heading and top bar

The title naming the active protocol page is its screen heading on every
client, including editable Requests, workflow editors and Responses. Its
placement follows the available top-bar space below. When the first content
row retains the heading, use the platform's heading typography: larger,
semibold text in the normal foreground color, wrapping for long localized
titles, and extra space before the fields below. Omit the ordinary field-row
divider and expose the title as an accessibility heading. Keep its existing
controls and the typography of nested fields. This treatment is platform-owned
and requires no App attribute.

The content heading, when present, and the page's top-level content share the
same leading edge within normal page or pane padding. Neither the root protocol
Union nor its selected Request or Response payload adds an indentation level. This automatic root
alignment applies to full-page Requests, workflow editors, and Responses, and
independently inside each master-detail pane. The master-detail arrangement
uses the full content width within the working surface's normal padding,
without a root indent around the pair of panes. Root alignment is independent
of `label`, `title`, and `suppress_label`, and requires no `flatten` rule;
`flatten: false` retains this normal root alignment. Nested structs, Lists,
and Unions continue to follow their ordinary indentation and template rules.

When the displayed root protocol Union has `suppress_label: true`, the page
heading and its native top bar use the localized selected variant name,
without the protocol label. Resolve an explicit `title` or `label` rule on
the displayed Request or Response variant first, using the same rendering
and empty-value fallback as [labels and titles](#labels-and-titles). Otherwise
use that variant's localized display name. With no selected variant, retain
the ordinary localized protocol title rather than a blank heading.

The title belongs to the active displayed value. Apply this rule to editable
Request pages, including workflow editors, as well as Response pages; do not
take an editor's title from its originating Response. An explicit variant
rule also titles that active page when root suppression is absent.
For example, a protocol labelled “Tax return” with selected variant “About
you” uses “About you” as its page title under:

```json
{ "select": "^<TaxProtocol>", "suppress_label": true }
```

The selector in an App must use its actual protocol type name. No duplicated
resource key or field is needed solely to remove “Tax return”.

**Adaptive heading and Actions placement.** Apply this rule on every client
to a protocol page with its own screen top bar, at any screen width. It is
independent of the widening gate for two-panel layouts. **Actions** is the
control for the displayed variant's eligible `NextRequest` choices, under the
[request-selection rules](endpoint-presentation.md#protocol-transaction-stack);
it is separate from an editor's primary Next/Submit and previous-action
controls. Render its full localized caption, resolved by the
[Actions caption precedence](#named-action-controls) (Actions by default, or
the active page's [button label](#button-labels)), and retain the platform's normal
button target and spacing. The fit rules below measure that resolved caption.
They place Actions while it keeps this default heading placement; an
`action_controls` placement of `secondary` moves it into the
[named-control region](#named-action-controls) instead.

Measure the title and button with their actual localized text, current font,
weight, and accessibility text size. **Fits** means the complete text renders
without clipping or ellipsis in the normal top-bar layout; never reduce the
font size or button target to make it fit. Available title width is the actual
title-text slot after reserving system insets, navigation and other controls,
icons, gaps, and padding, including any space required by native title
alignment. A title icon does not count as title-text space.

Let `T` be the measured full title-text width. Its minimum reservation while
Actions occupies the top bar is **`min(T, 8em)`**, where `1em` is the current
rendered top-bar title font size. Thus `8em` is 160 layout units for a title
font of size 20 and scales with accessibility text size; a short title needs
only its full measured width. This reservation is a minimum, not a fixed or
maximum width: the title uses additional available space.

Evaluate the following cases in order, with Actions included in the proposed
top-bar layout:

| Available space | Top bar | First content row |
| --- | --- | --- |
| Complete title and complete Actions button fit | Full title and Actions | Suppress the redundant page-heading row. |
| Actions fits and the title-text slot is at least `min(T, 8em)`, but the complete title does not fit | Title with end ellipsis as needed, and Actions | Retain the full, wrapping title; remove Actions from this row. |
| Actions cannot fit while preserving that title reservation | Ordinary compact title, with ellipsis as needed | Retain the full, wrapping title and Actions at its trailing end. |

An ellipsized title never qualifies for suppressing the content heading. With
no eligible action, omit Actions and apply the full-title fit test alone.
Place the heading's Actions control in one of these two locations, never
both. If Actions cannot fit beside the content title, wrap it beneath the
title within the heading area, before the separate named-control region.
Keep its full caption and prominent treatment in either location. After this
decision, optional [named controls](#named-action-controls) may join the top
bar only when the full title and their complete group also fit. Named controls
never cause Actions or a previously full title to move. Suppressing the
heading removes only its redundant title presentation:
retain any Union selector, promoted value, other row control, and requested
inline description in the content, with their ordinary behavior. Content
retains the root alignment and normal page padding above even when its
heading moves to the top bar.

**Description and accessibility.** A top-bar title supports the same long
press as a regular title row and opens the same localized description popup.
Keep a keyboard-accessible and screen-reader-accessible **Show description**
action when a description is available; help must not depend on hover or
long press alone. Moving the heading preserves its full accessible name and
heading semantics. If the content heading remains, expose that as the page
heading without adding a duplicate heading for the compact top-bar title.
Description lookup remains bound to the same active page when its title is
shortened or relocated.

In a [master-detail arrangement](#master-detail-panes), only the leading page
uses the working surface's top bar. The trailing page retains its own content
heading and eligible Actions; selecting it does not take over the shared
title or toolbar. A page shown on the whole working surface after narrowing
uses the same placement rule for its own title and Actions. Re-evaluate the
complete candidate set when the available width or height, title, control
availability or captions, template, locale, font, text size or device conditions
change, including rotation and window resizing. Restore controls to the top
bar when they fit again. Measure actual localized controls and native slots,
never a device-category breakpoint or a fixed estimate for system controls.
Preserve logical order in right-to-left layouts and the full accessible names.
Re-layout preserves values, focus, interaction and reference-context ownership,
and pending-operation restrictions, and sends no request.

### Presentation only

Encoding, paths, `CopyFrom`, `CopyTo`, `EnumSource`, validation, comparison,
digests, ordering, uniqueness and generated documents are unchanged. The
marker supplies no default and makes no field optional.

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
requests, persistence, or chart geometry. Studio validates the pattern, compiles
it into the [Display Template](#display-template) as the `format` hint, and
removes it from the final schema; clients read the display pattern only from
the template, where a rule may choose another portable pattern per device.

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
unrelated currencies separate. Money follows the
[exclusions of numeric semantic attributes](schema.md#numeric-semantic-exclusions).

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
coexist and formats only the numeric portion. PhysicalUnit follows the
[exclusions of numeric semantic attributes](schema.md#numeric-semantic-exclusions).

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

Bitflags follows the
[exclusions of numeric semantic attributes](schema.md#numeric-semantic-exclusions).

## String resources and formatted output

One package-owned catalog supplies notification templates, permission reasons,
data-name templates, `FormattedText`, and formatted chart labels for all
functions and types in the App. Task package context identifies the catalog;
encoded tuples carry no function/type owner or resource FQN.

### Expanding long displayed text

Clients that shorten long displayed text behind **Show more** measure the full
text using its actual font, styling and available width. If it fits within the
combined height of the shortened preview, disclosure row and their spacing,
display the full text and omit the disclosure. Do not reduce the text size to
make it fit.

Otherwise, **Show more** reveals the complete text and **Show less** restores
the preview. Reevaluate the fit when the value, locale, width or text size
changes; a disclosure is unnecessary whenever the complete text fits. This
behavior concerns displayed text, independently of structural folding in the
Display Template. Clients that already display the complete wrapped text need
no disclosure.

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
 * @LibertasResponse
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
    #[libertas_response]
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

Chart appearance belongs to the [Display Template](#display-template). Studio
validates the complete chart metadata on the parser result, then compiles the
scale `type` of each `ChartScale` and `Chart3DScale` into a `scale` hint and
the `position` of each `ChartGuide` into a `guide` hint, and removes those
properties from the final schema; an attribute left with no property is
removed. Channels, scale IDs, domains, and every other chart property remain
schema. A template rule may also choose a chart `palette`. Clients read the
effective scale type, guide position, and palette only from the template, and
a template value is validated under the same chart matrices as the source.

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

## Inline choices

`InlineChoices` shows an Enumeration's or Union's choices in its row as radio
controls, the way [Boolean answer controls](#boolean-answer-controls) render,
instead of a value that opens the choice selector. Author it as
`@LibertasInlineChoices` in TypeScript or `#[libertas_inline_choices]` in
Rust. It takes no argument and emits `{"Name":"InlineChoices","Value":"true"}`
into the parser result; Studio compiles it into the
[Display Template](#display-template) as the `inline` hint and removes it from
the final schema.

Place it on an Enumeration or Union field, or on an Enumeration or Union type
declaration. Each declaration is read directly, not through effective-attribute resolution:
a marker on either the field or its type enables inline choices, and a field
cannot switch off its type's marker. Studio rejects the marker on any other
effective type and on a Bitflags catalog, including a field typed by one.

- The radio controls offer the choices that remain after
  [ConstrainedBy](protocols.md#backend-constrained-choices) filtering, with
  their localized display names. Selecting one is the same edit as choosing it
  in the selector; an unanswered field shows no selected control.
- For a Union, the selected branch's payload renders beneath the row as it does
  after choosing in the selector. Selecting another choice replaces the
  applicable fields.
- The singleton allowed set and answered-Union rules of
  [backend-constrained choices](protocols.md#backend-constrained-choices) are
  unchanged.
- Visibility, read-only, promotion, and folding rules still apply;
  the marker does not make a read-only value editable.

There is no size threshold: a row grows with its choices, so mark only fields
whose choices fit, or let a template rule turn inline choices on or off per
device. The marker changes presentation only, never data, encoding, or paths.
A client without inline support keeps the choice selector.

## Link rendering

A linked row may be highlighted through its [enum/protocol association](schema.md#enumeration-protocol-associations); the [pane contract](#master-detail-panes) owns that use.

A [link](protocols.md#links) adds an affordance to its row. It never removes
the row's label or value and never replaces the value with a button. Whether
the value is editable decides how the link appears:

- **Value not editable:** every Response or SubscriptionData row, and a
  ReadOnly row in a request editor. The label stays plain, the value renders as
  link text in link styling, and the whole row is the tap target. On a struct,
  Union branch, or List member, the header text is the link text, and an
  expanded child row taps the nearest enclosing link unless the child row
  declares its own link, which wins. On a FormattedText field the whole block
  is the tap target; links inside FormattedText markup are not supported.
- **Value editable:** the label, value, and editing are unchanged. Tapping the
  value still edits it, and direct controls such as Boolean answers,
  [inline choices](#inline-choices), and a Bitflags editor still work. The
  link is a trailing link button beside the value, named after the target
  Request's localized name. This is the only case that renders a button.

A primitive with no label of its own, such as an item of a List of scalars,
shows its value as the link text. While a target with no editable field is
pending, the row shows the existing pending indicator and the link is disabled
until the reply lands; repeated taps never send twice. A link adds no chevron
or other glyph and never edits its field. Hidden rows have no link, since
Studio rejects that placement. Promotion, flattening, folding, and
read-only rules are unchanged by a link.

## Folding structs and unions

Structs (`Table`), unions, and lists remain expanded unless `Foldable` is declared
on their type or field. Author it as `@LibertasFoldable` in TypeScript or
`#[libertas_foldable]` in Rust, with an optional Boolean `initially_open`
that defaults to true: `@LibertasFoldable(initially_open=false)` or
`#[libertas_foldable(initially_open = false)]`. Studio permits this marker
only on an effective Table, Union, or List. It changes presentation, not encoded data.

In the parser result the attribute's Boolean string value carries the option:
`{"Name":"Foldable","Value":"true"}` starts expanded and `"false"` starts
collapsed. Presence enables folding; `false` does **not** disable it. Studio
compiles it into the [Display Template](#display-template) as the `fold` hint
and removes it from the final schema; clients read folding only from the
template, where a rule may also change it per device.

An unmarked struct, union, or list shows no disclosure control and cannot be
collapsed. A marked value shows one disclosure chevron at the leading edge of
its title line, pointing toward the text when collapsed and down when expanded
(mirrored for right-to-left text), and exposes its expanded state to
assistive technology. The title line toggles the fold, except on a row that is
also a [link](protocols.md#links), where only the chevron toggles and the rest
of the row follows the link. A marked value starts expanded, or collapsed
when `initially_open` is false, and permits manual collapse and expansion; a
collapsed value shows only its row, its title and promoted value
included. The initial state applies to rows that exist when a page is
displayed. A value the user creates while the page is open, such as a new
List member or a materialized nullable, always opens, whatever its marker
says, so its fields are visible to fill in. Each nested value owns its folding permission;
a marker does not grant permission to its descendants. Lists follow the same explicit-permission rule. Changing a union variant reveals its new fields. Union selectors
remain editable independently of folding.

For a promoted union, the shared owning row may fold when either
the owning Table or the promoted union has Foldable on its type or field.
Promotion and [`flatten`](#flatten) remain independent of folding.

The collapsed task editor value is a short presentation summary, never a
recursive JSON-like rendering of its children. A Table with a promoted value
field shows that field's normal formatted value. A Union shows its selected
variant name, including when the variant has a scalar payload. A List shows its
visible item count (for example, “3 items” or “0 items” when empty); a List
whose members are all hidden shows no count. An ordinary Table with no promoted
value shows no value text beside its title and fold control. Null remains
distinct from an empty List or a Table without a summary.

When [`flatten`](#flatten) with `{ "name_row": false }` removes a struct's
own row, the struct still assigns a dropdown owner, in this order: the row of its promoted child, which is
the struct's surviving value row, else the row of its first visible child.
When the struct is Foldable, that row carries its disclosure chevron and
collapses the struct's remaining rows beneath it; its own value stays editable
as usual. An unmarked struct gives that row no chevron.
Hidden fields produce no row and are skipped; a struct that produces no rows
folds nothing. A List member
whose struct row is suppressed keeps its position and membership controls on
the row that owns its dropdown.

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
