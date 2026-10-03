# Schema documentation invariants

This document is the normative authoring contract for a codebot that creates or
changes localized Libertas schema documentation. It applies to every authored
schema-visible package, function, named type, parameter, field, Enumeration
member, Union variant, inline value, List container, and List item. It also
applies to custom document-property values and package `StringResources`.

These are codebot authoring and review requirements. Generated document JSON
permits empty strings so parsers can represent compiler-generated nodes and
older or incomplete source. That structural allowance is not permission for a
codebot to omit documentation from an authored public declaration.

The only ordinary exception is a compiler-generated TypeScript discriminator
field whose document text the parser intentionally suppresses. Non-schema
implementation details do not need schema documentation. `Hidden`, `ReadOnly`,
protocol, persistent-data, and chart-related nodes are not exceptions: their
meaning can still be used by validation, accessibility, localization, tools,
management interfaces, or a future presentation.

## Source and output invariants

- Source comments are authoritative. Never patch generated schema or document
  JSON to improve wording.
- Give every authored document node at least two ordinary lines: a concise
  natural label followed by a complete plain-language description. Do not rely
  on a parser fallback derived from a native identifier.
- Rust package text uses inner rustdoc (`//!`) at the start of `src/lib.rs`.
  Rust declarations use outer rustdoc (`///`).
- TypeScript package text uses leading triple-slash comments (`///`) on the
  selected entry module. TypeScript declarations use JSDoc (`/** ... */`).
- Ordinary `//` comments are not localized. Use them, a README, or developer
  documentation for engineering details that must not appear in the user
  document.
- Update source documentation in the same change as the behavior, meaning,
  choices, results, constraints, or presentation it describes.
- Inspect the generated default-locale document as an output. Fix the owning
  source comment when a label, description, extension, or List half lands on
  the wrong node.

The final document has one package object, function documents in `List`, named
type documents in `Types`, nested field documents in each node's `List`, and
Enumeration members in `Enum`. A field document's numeric `Type` member is its
schema-node identifier; it is not prose and is never authored or translated.
Documentation traversal stops at a named type reference. Consequently, the
use-site field or parameter and the referenced named type must each make sense
on their own. One never substitutes for the other.

## Universal user-language invariant

Write for the people who use the capability, including nontechnical home,
commercial, research, engineering, and healthcare users. Use specialist domain
terms when that audience genuinely uses them, and explain unfamiliar terms when
the surrounding context does not make them clear.

A label must:

- be a short, natural name or action phrase;
- distinguish the node from its siblings;
- work when read aloud or shown without its parent; and
- not be a source identifier, version token, full sentence, or keyword list.

A description must add meaning beyond the label and declared type. It must say
what the person chooses, enters, observes, receives, or can accomplish. When it
describes behavior, use active voice and identify the actor, action, object,
scope, material condition, and modality. Preserve distinctions such as “can,”
“must,” and “automatically.”

All localized prose must:

- use concrete domain nouns and verbs rather than vague words such as “data,”
  “value,” “item,” “configuration,” or “information” by themselves;
- explain an important empty, unavailable, zero, false, `None`, or `Unknown`
  meaning when a person could otherwise misinterpret it;
- remain understandable without relying on order, screen position, color,
  icon, shape, animation, or a particular gesture;
- avoid instructions tied to one client, such as “click the button” or “tap the
  row,” unless the behavior itself exists only on that client;
- use explicit nouns instead of an ambiguous “it,” “this,” or “they”;
- avoid unexplained acronyms, keyword stuffing, unsupported claims, and lists of
  capabilities the App does not provide; and
- be complete enough to translate without guessing an actor, scope, condition,
  or consequence.

Do not expose internal vocabulary in localized prose. In particular, do not
describe something to a user as a function, field, type, struct, table, enum,
Union, variant, schema, payload, protocol, endpoint, request, response, runtime,
discriminator, index, FQN, Avro value, wire value, or implementation version.
Do not expose native casing, storage encoding, Libertas Schema Paths, integer codes, byte
layouts, or transport mechanics. Preserve required engineering facts in an
ordinary source comment or developer document instead.

Apply this rule both when authoring source comments and when translating their
localized text. Write natural, idiomatic language for the intended audience;
keep it technology-neutral by describing the capability rather than its
implementation. An App's “weather endpoint,” for example, is a “weather
service” to its users. Prefer the specific purpose or the natural equivalent
of “service” in the target language. This is a wording choice, not a mechanical
word replacement: retain specialist terms when they describe the user's domain
and their precise meaning matters. Preserve every material meaning, choice,
control, automatic behavior, result, warning, condition, prerequisite, and limit.
This guidance changes prose only; identifiers, structural keys, and message
placeholders retain their exact spelling and syntax.

### Domain references and discoverability

Document the names people use to identify a supported capability. Where
relevant, connect formal domain references, document or form numbers, standard
abbreviations, and common names in natural descriptive prose. For example, a
tax topic can identify the forms and statutory provisions it covers, while an
equipment topic can identify the industry standard it implements. These are
user-domain references, distinct from internal identifiers and wire codes.
Preserve their exact meaning during translation and explain unfamiliar
abbreviations. Keep labels concise; put this context in descriptions.

State the actual purpose, supported scope, and material conditions. A reference
to a subject does not establish that every case within it is supported. For
links that share one request but bind different topics or records, document
the individual topic or selected choice as well as the common operation, so
the destination is understandable from its own documented meaning and data.
Maintain this information in its owning source documentation when behavior
changes; generated documents and private code comments are not substitutes.

## Node-specific invariants

### Package

The package label is the recognizable product, service, or capability name,
not the package identifier. Its description explains the overall outcome,
intended setting or audience, major capabilities, and material prerequisites or
limits. Avoid internal architecture and unsupported marketing claims.

### Function

The function label names the activity a person can configure or use. Its
description is the complete discovery description and must additionally follow
the [developer function-comment guide](function-comments.md),
including the seven completeness categories, explicit actors, and final
developer attribution.

Plaza creates exactly one discovery embedding per public function from that
function's display name and description. Package, argument, type, protocol,
chart, attribute, and generated appendix text is not added. Excellent type and
field documentation therefore never repairs an incomplete function comment.
Developer attribution belongs only in function descriptions; do not append it
to package, type, field, member, or extension text.

### Named type

Describe the real-world concept represented by one value and why a person sees
or supplies it. Do not say that it is a struct, table, grouping, payload, or a
container of fields. A reusable named type must make sense independently of
every place that references it.

### Parameter and field

Give every parameter and field a contextual natural label. For an input,
explain what the user chooses or enters and the effect or scope of that choice.
For output or read-only information, explain what the App reports, records, or
calculates and when it is meaningful.

Match the wording to the [type's use](app-contract.md#user-facing-and-non-user-facing-types).
For ordinary optional text stored as `""`, say “Leave it empty if absent,” and
explain any consequence of leaving it empty, such as using another name or
leaving an activity ungrouped. Do not say “Leave it undefined” when the answer
is an empty String. For a genuinely nullable answer, explain what absence means
and how it differs from a present empty answer or zero. AI-import instructions
must use the same meaning under the
[blank-value conversion contract](llm-document-transformation.md#blank-values-and-output-types).

An output description explains the reported result and its availability rather
than telling the person to enter it. A non-user-facing schema field still needs
domain documentation: describe what the App records and the meaning of missing
or empty content. Keep mapping code, storage encodings, and other engineering
details in ordinary `//` comments.

Additional field rules:

- A Boolean label states the positive choice or state. Its description makes
  the `true` and `false` consequences unambiguous when they matter; “Enabled” by
  itself is not enough.
- For a nullable value, distinguish “not supplied” or “unavailable” from zero,
  false, and an empty collection where that affects meaning.
- For a number, date, time, duration, or physical quantity, describe the domain
  meaning and scope. Do not explain its integer width, epoch, raw date encoding,
  or storage unit as implementation mechanics. Declare machine-readable unit
  and format metadata with the appropriate schema annotation.
  The platform supplies [AI tool-call representation hints](presentation.md#ai-tool-call-descriptions)
  from that metadata; do not duplicate those hints in source comments.
- For a Libertas system object, name its actual role, such as “Rain gauge,”
  “Patient,” “Delivery vehicle,” or “Sprinkler valve.” Never call it a device
  ID, user ID, endpoint ID, or plain integer. Keep the Libertas system-object
  type intact so clients can resolve its localized identity.
- For free text, describe what content is appropriate and any material use or
  visibility. Never present a secret as ordinary documented input.
- A use-site field retains its own documentation even when its named type is
  thoroughly documented.

### Enumeration and bitflags

The Enumeration type describes the decision, state, or classification as a
whole. Every member has a natural label and a description of its observable
meaning or consequence. This includes `None`, `Unknown`, and zero-valued
members. Never expose the native constant name or numeric code.

A Bitflags catalog follows the same rules, but each nonzero member describes
one independently selectable option. Do not phrase its type as one mutually
exclusive choice.

The parsers' public positional `@LibertasEnum(A, B, C)` and
`#[libertas_enum(A, B, C)]` compatibility forms fabricate member nodes without
a source-document location for each member. A codebot therefore must not
generate those forms: they cannot satisfy this all-member documentation
invariant. Use a native numeric enum whose declaration and every member can be
documented. Existing positional catalogs remain parseable, but parser support
does not waive the codebot authoring rule.

### Union and variant

The Union type describes the family of possible actions, events, states, or
results. Every variant has a standalone natural label plus a description of the
observable action or state, its trigger or scope, and its important consequence.
Do not label variants “Request,” “Response,” “V1,” or by their discriminator
tokens. Document every inline field inside a variant by the ordinary field
rules.

TypeScript may obtain final variant text from a discriminator member, the Union
branch, or the variant's named payload type. Choose one authoritative source
for the intended result. If more than one source contains prose, keep the text
semantically identical; never depend on precedence to hide contradictory
wording. Rust takes the variant's own rustdoc.

### List container and item

Every List has two complete document sections separated by a line containing at
least four hyphens:

- Above `----`, use a plural label and explain the collection's purpose,
  meaningful ordering, limits, or selection behavior.
- Below `----`, use a singular label and explain what one entry represents and
  how a person recognizes or uses it.

Author both halves even when the item is a separately documented named type.
Custom properties above the divider belong to the List container; properties
below it belong to the List item.

### Hidden, interaction-context, and chart data

Document hidden and read-only values truthfully without revealing a secret.
Explain their domain purpose, such as associating a follow-up action with the
same inspection, rather than describing them as a cookie, token, cursor, or
transaction mechanism.

Chart documentation describes the domain relationship and the meaning of each
visible quantity or category. It must remain useful to a screen reader and must
not depend on axis position, color, shape, rendering library, or client-computed
geometry. Visible human text uses a localized Enumeration, `FormattedText`, or
the appropriate Libertas system-object type when applicable. A plain `String`
is reserved for genuine literal domain or user-authored text, not a fixed
English vocabulary.

## Exact source-document syntax

The first ordinary line becomes `DisplayName` for a package, function, or named
type, and `Name` for a field, item, member, or variant. Remaining ordinary lines
become `Description`. For reliable cross-language output, always provide both a
label line and at least one description line.

Long descriptions use HTML-style flowing prose. A single logical line break in
`Description` is source wrapping and is canonicalized to exactly one space. A
run of two or more logical line breaks is canonicalized to one LF so it remains
one intentional paragraph break. CRLF counts as one logical line break, as do a
lone CR or LF; horizontal whitespace around a break does not change the rule.
Codebots may wrap a long source sentence across adjacent rustdoc/JSDoc lines,
but must leave a blank documentation line only when one paragraph break is
intended. This normalization applies to `Description`, not custom-property or
`StringResources` values.

TypeScript function parameters may use `@param name` documentation. The parser
merges that text over documentation attached directly to the parameter, so use
one authoritative location or keep both semantically identical. Rust parameter
rustdoc and schema attributes are written on the typed parameter inside the
exported function signature.

### TypeScript example

```typescript
/// Greenhouse climate care
/// Helps growers monitor rooms and maintain suitable plant conditions.
/// @LibertasDefaultLocale("en-US")

/**
 * Fan behavior
 * Describes how ventilation responds to the selected temperature.
 */
export enum FanBehavior {
    /**
     * Automatic ventilation
     * The greenhouse starts and stops the fan according to room temperature.
     */
    Automatic,

    /**
     * Fan remains off
     * The greenhouse does not start the fan automatically.
     */
    Off,
}

/**
 * Growing-room settings
 * Describes the conditions the grower chooses for one room.
 */
export interface RoomSettings {
    /**
     * Room thermostat
     * The grower chooses the thermostat that measures this growing room.
     */
    thermostat: LibertasDevice;

    /**
     * Notes
     * The grower records a short note shown with this room.
     * [DefaultText]
     * Check seedlings each morning
     */
    notes: string;

    /**
     * Alert recipients
     * Lists the people who receive temperature warnings for this room.
     * ----
     * Alert recipient
     * Identifies one person who receives this room's warnings.
     */
    recipients: LibertasUser[];
}

/**
 * Care for a growing room
 * The greenhouse monitors one room and warns the selected people when its
 * temperature leaves the grower's preferred range. The grower can change that
 * range and the fan behavior at any time. Developed by Example Developer
 * (example_developer).
 * [DefaultTaskName]
 * Seedling room care
 */
export function careForRoom(
    /**
     * Growing-room choices
     * The grower chooses the room, warning recipients, and starting notes.
     */
    settings: RoomSettings,
): void {
    void settings;
}
```

### Rust example

```rust
//! Greenhouse climate care
//! Helps growers monitor rooms and maintain suitable plant conditions.
//! #[libertas_default_locale("en-US")]

/// Growing-room settings
/// Describes the conditions the grower chooses for one room.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct RoomSettings {
    /// Room thermostat
    /// The grower chooses the thermostat that measures this growing room.
    pub thermostat: LibertasDevice,

    /// Notes
    /// The grower records a short note shown with this room.
    /// [DefaultText]
    /// Check seedlings each morning
    pub notes: String,

    /// Alert recipients
    /// Lists the people who receive temperature warnings for this room.
    /// ----
    /// Alert recipient
    /// Identifies one person who receives this room's warnings.
    pub recipients: Vec<LibertasUser>,
}

/// Care for a growing room
/// The greenhouse monitors one room and warns the selected people when its
/// temperature leaves the grower's preferred range. The grower can change that
/// range and the fan behavior at any time. Developed by Example Developer
/// (example_developer).
/// [DefaultTaskName]
/// Seedling room care
#[libertas_export]
pub fn care_for_room(
    /// Growing-room choices
    /// The grower chooses the room, warning recipients, and starting notes.
    settings: RoomSettings,
) {
    let _ = settings;
}
```

Examples use a placeholder developer profile. A codebot must substitute the
current Plaza account and display name under the function-comment contract.

## Custom document properties

A custom property, also called a document extension, uses this exact form:

```text
[PropertyName]
Property value starts on the next line.
It continues until another property marker or the end of this document section.
```

Rules:

- The case-sensitive `[PropertyName]` marker is the only content on its line.
- Put extensions after the ordinary label and description.
- Put all ordinary text and extension blocks before the first
  `@Libertas...` schema tag in TypeScript JSDoc. The TypeScript document parser
  stops collecting document text at that first tag. This ordering is also safe
  in Rust rustdoc.
- Multiple extensions are allowed. Never repeat a property name in one section.
- Every value is a string. Leading and trailing blank space is removed while
  internal newlines are preserved.
- The property is flattened beside the owner's ordinary document members; it
  is not nested inside an `Extensions` object.
- Property names and structural keys are stable and are never translated.
  Property values are localized.
- On a List, the `----` divider controls property ownership as well as ordinary
  text: properties above it belong to the container and properties below it
  belong to the item.
- Unknown valid properties are preserved and translated, but Libertas gives
  them no generic display or behavior. Use one only when a documented consumer
  contract defines its semantics. Never invent an extension from its name.

The names `DisplayName`, `Description`, `List`, `StringResources`, `Strings`,
`Enum`, and `DocumentVersion` are globally reserved. A name that collides with
a structural key at its owning level, including `Locale`, `Types`, `Name`, or
`Type`, is also reserved. `Strings` is prohibited; document format `0.0.2` has
one package-owned `StringResources` map and no function- or type-owned string
table.

### Standard properties

`[TrueText]` and `[FalseText]` are optional localized Boolean field labels.
Each supplied value is one non-empty line. They label `true` and `false`
respectively; omitted labels use localized Yes and No. Either property selects
an explicit two-choice control. Labels do not supply defaults, change Boolean
values, or cause submission. Use ordinary document localization and fallback.

```rust
/// Accept this invitation?
/// Choose whether to accept or decline.
/// [TrueText]
/// Accept
/// [FalseText]
/// Decline
pub accept_invitation: bool,
```

Without a schema Default, a required Boolean starts unanswered. Both choices
are unselected; the editor holds an undefined value and refuses submission until
one is chosen. It never encodes undefined as false or null. A nullable Boolean's
explicit null remains distinct from the true/false choices and follows ordinary
nullable-field controls. CopyFrom and EditRequest preserve supplied values.

A Boolean uses an on/off switch only when it has an explicit Boolean Default
and neither custom label is supplied. Otherwise it uses the two-choice control,
including when a default preselects an answer. This decision follows metadata,
not whether the user has already answered. Fixed and ReadOnly retain their
existing restrictions on editing; no new schema attribute or opcode is added.


`[DefaultText]` is valid only on a field whose effective schema type is
`String`, or on a built-in Endpoint field that carries `EndpointServer`. Its
value is exactly one localized line of real, safe initial content suitable for
that locale and context. It is not a placeholder instruction, sentinel,
resource key, source identifier, or schema `Default`. It does not emit or
replace the `Default` schema attribute. Omit it when no truthful general value
exists; never fabricate identity, clinical, financial, security, or other
user-specific content.

`[DefaultTaskName]` is valid only on a function document. Its value is a short,
localized noun phrase suitable as the initial name of one configured task, such
as “Back garden watering.” It is distinct from the stable native function name,
the function display name, package name, description, and developer
attribution. Omit it when no broadly useful initial name can be stated
truthfully.

Codebots must save people from naming routine generated objects themselves:
add a useful localized `[DefaultText]` to every authored Endpoint server field
and a useful localized `[DefaultTaskName]` to every authored App function. Use
the supported property for each owner—a function does not use `[DefaultText]`.
Choose a concise domain name that is immediately useful but remains suitable as
an editable initial value; never copy a native identifier or invent personal
details merely to satisfy this requirement.

Neither standard property substitutes for the owner's ordinary label and
description.

Persisted or user-assigned text has absolute priority and is immutable to
programmatic defaulting. Clients must not replace it during locale, document,
or schema refresh, migration, upgrade, or any later default application. This
includes a deliberately empty saved value or task name. The remaining priority
chain applies only when no persisted value exists and the client is
constructing a new value. For a new String or Endpoint-server presentation
value left to ordinary initialization by the
[shared copy rules](schema.md#shared-copyfrom-and-copyto-rules),
an applicable schema `Fixed`/`Default` value has first initialization
priority, localized `DefaultText` is next, and the ordinary empty or localized
field-label fallback is last. For a new blank task name, localized
`DefaultTaskName` has first initialization priority, then the localized
function `DisplayName`, then the native function name.

## StringResources prose

`StringResources` is one package-owned table shared by every function and type.
Stable keys are identifiers and are never translated. They are normally not
substitutes for localized prose. The task-access approval UI is the explicit
exception: it always shows each raw function `Permissions` key or `AccessHost`
key as the request summary, then separately shows that App package's localized
same-key value as the explanation. Two Apps may request the same permission for
different reasons, so each App owns and supplies its own explanation. Every
default-locale value and every translated value must be natural user-facing
text and, when formatted, a complete LMF1 message rather than a fragment meant
to be concatenated.

- A permission or network-access reason identifies the App action or service
  involved, what access is needed, and the user benefit. “Network access
  required” is not sufficient.
- A concrete `AccessHost` reason explains in ordinary terms why the named
  service is contacted. Reasons for `{network_access_local}` and
  `{network_access_external}` explain why broad local or external connectivity
  is necessary; the reserved key does not excuse vague prose.
- An error message says what happened and, when possible, what the person can
  do next. Do not expose stack traces, codes, or diagnostics as the explanation.
- A notification or status message names its actor, object, scope, and material
  condition. Do not assemble grammar from separately translated fragments.
- Every single-record name and indexed-database or table name uses a `DataName`
  canonical byte blob. That blob encodes one stable resource key plus ordered,
  typed Libertas Message Arguments. It is simultaneously the locale-independent
  database identity and the complete input needed to select a localized LMF1
  template and render its argument data for people.
- Never use rendered text, a literal database-name string, a Base64 string, or
  a separately maintained machine key as the identity of data that a person may
  see. The canonical bytes are the one identity. Use exactly the same resource
  key, argument order, argument types, and argument values for every operation
  on the same logical record or indexed table.
- Preserve LMF1 argument positions, types, styles, selectors, plural offsets,
  braces, and apostrophe escapes while translating. A Libertas system object or
  Enumeration remains a typed Libertas Message Argument rather than a rendered
  integer or literal string.

The localized template itself remains in the package `StringResources` table;
it is not copied into the byte blob. A locale or wording change therefore does
not rename data. Changing the resource key or any encoded argument identity does
create a different data name and requires an intentional migration. Single
records and indexed tables are separate namespaces even when their `DataName`
bytes match, and an indexed record's signed index is separate from the table
name rather than an LMF1 argument.

## Translation invariant

Translate all user text: package, function, and type display names and
descriptions; parameter, field, List-container, List-item, Enumeration-member,
Union-variant, and inline-field names and descriptions; custom-property values;
and `StringResources` values.

Do not translate native names, structural keys, numeric document IDs, extension
names, resource keys, or LMF1 structure. Preserve deliberately empty generated
text. `[DefaultText]` remains one line in every locale. See the
[schema document translation guide](../guides/translation.md)
for the complete translation workflow.

## Codebot workflow

1. Inventory every schema-visible declaration and each document node it emits,
   including both List halves and every Enumeration or Union member.
2. Separate user meaning from engineering notes. Put user meaning in extracted
   rustdoc/JSDoc and preserve engineering facts in ordinary comments or
   developer documentation.
3. Write a standalone natural label and complete description for every authored
   node. Apply the node-specific rules above.
4. Apply the separate function-comment discovery and attribution checklist to
   every public function.
5. Add only documented custom properties, on the correct owner and List side.
   In particular, give every authored Endpoint server a useful `[DefaultText]`
   and every authored App function a useful `[DefaultTaskName]` so a newly
   created endpoint or task already has a sensible editable name.
6. Write complete, specific default-locale `StringResources` values and verify
   every referenced permission or host reason exists.
7. Generate and inspect the default document. Confirm ownership, flattening,
   labels, descriptions, extensions, and intentionally empty generated nodes.
8. Run the language checker and Studio schema validation. Treat failures as
   source defects; never patch generated JSON.

## Review checklist

- Every authored package, function, type, parameter, field, List half,
  Enumeration member, Union variant, and inline value has a non-empty natural
  label and description.
- Each label is concise, sibling-distinguishable, meaningful aloud, and not a
  source name, version, sentence, or keyword list.
- Each description adds purpose or meaning and names actor, action, object,
  scope, condition, and modality wherever behavior is involved.
- Input text explains what the user chooses and its effect; result text explains
  what is shown, recorded, calculated, or delivered.
- Empty, unavailable, zero, false, `None`, and `Unknown` meanings are clear where
  material.
- Every Enumeration member and Union variant stands alone. Every List has a
  plural container and singular item on the correct sides of `----`.
- No localized prose leaks source names, protocol roles, transport/storage
  mechanics, or visual-only cues. Domain terminology is audience-appropriate.
- Hidden and interaction-context values have truthful, non-secret domain
  descriptions.
- Every custom property is defined by a consuming contract, placed on the right
  owner, and follows the same prose rules. `DefaultText` and `DefaultTaskName`
  are legal, localized, and do not replace ordinary documentation.
- Every authored Endpoint server has a useful localized `DefaultText`, and every
  authored App function has a useful localized `DefaultTaskName`; neither is a
  native identifier, placeholder, or fabricated personal value.
- `StringResources` messages and reasons are truthful, specific, complete, and
  preserve LMF1 semantics.
- Every function separately passes the function-comment completeness,
  discovery, and attribution checklist.
- The generated default document was inspected and no generated schema or
  document JSON was edited by hand.

## Related contracts

- [Developer function-comment guide](function-comments.md)
- [Schema source annotation reference](annotations.md)
- [Presentation and localization](presentation.md)
- [App authoring contract](app-contract.md)
- [Schema reference](schema.md)
- [Schema document translation guide](../guides/translation.md)
