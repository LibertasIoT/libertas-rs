# Libertas data model reference

This document describes the language-neutral data model emitted by the Libertas
TypeScript and Rust parsers and consumed by Libertas Studio and the Hub. It is a
reference for the generated artifacts and their invariants, not a source-language
annotation manual.

This reference defines generated artifact shape, identity, compatibility, and
validation guarantees. [Source annotations](annotations.md) and
the language manuals define authoring syntax; linked topic references own
expanded behavior. Generate schemas through Studio rather than editing them.

Language extraction resolves native declarations. Studio validates the resulting
schema against its default document and exact materialized dependencies. Source
locations accompany diagnostics, but transient parser metadata is absent from
final published artifacts.

## Libertas Schema Path

A **Libertas Schema Path** addresses a node in a schema-described value tree.
Schema compilation resolves the declared structure and checks the path in its
usage context; runtime evaluation locates the corresponding value. Fields use
case-sensitive Native Names, not localized Display Names. Named type references
and inheritance are resolved without introducing extra value-tree levels.
A Libertas Schema Path is distinct from a package/type fully qualified name, a
protocol capability prefix, or a filesystem path. The attribute or expression
that contains it determines its root, allowed
syntax, cardinality, and treatment of unavailable values.

A **reference context** is the lookup environment supplied by an operation.
Its **reference objects** are schema-described values in the defined source
order. A **reference path** selects a value within one such object; the term
does not imply that the value is a record. The context table below specifies
each operation's sources. The [interaction reference-context rule](protocols.md#interaction-reference-context)
owns their membership, order, and lifetime. An operation-specific copying
source does not implicitly replace the interaction's reference context.

### Relationship to JSONPath

Libertas Schema Path uses familiar JSONPath ideas, including `$` for a root and
named child selection. It is a schema-aware addressing facility, not an
implementation of the full JSONPath query language. Standard JSONPath defines
`@` within filter evaluation and also provides selectors such as wildcards,
slices, filters, and descendant queries. Libertas uses its own context rules,
including host-relative `@`, parent `^`, and typed-ancestor selection where
specified below. Those Libertas forms do not imply support for general JSONPath
queries; the descendant combinator of [Display Template selectors](#display-template-selectors)
exists only in that context. See [JSONPath, RFC 9535](https://www.rfc-editor.org/rfc/rfc9535.html).

### Shared CopyFrom and CopyTo rules

CopyFrom and CopyTo describe a directional, typed mapping between one source
node and one destination node. CopyFrom identifies the source for the annotated
receiving node; CopyTo identifies the destination for the annotated source node.
Either node may be a primitive or a composite value. Copying a Table or an array,
including a reduced projection, remains one mapping to the selected node.

For each annotated receiving node, CopyFrom visits reference objects in context
order. Within each object, it evaluates the comma-delimited alternative paths
in declaration order. It skips absent/undefined values and stops at the first
available value, including valid null. Source order takes priority over path
order: exhaust the paths in the first object before trying any path in the next.
In a subscription context, therefore, all alternatives inspect the most recent
successful correlated Response first, when present, then the current
SubscriptionData snapshot. The [interaction context rule](protocols.md#interaction-reference-context)
owns which values occupy those positions.

Selection is independent for each annotated receiving node. A selected composite
value is projected from that one source; do not merge reference objects or fill
its absent members by searching another object. With no reference objects or
no available path, CopyFrom performs no copy. Ordinary request construction then
uses the normal initialization rules below. A required calculation mapping with
no available input instead blocks that calculation. The operation supplies the
context; it does not redefine path matching or field projection.

An explicit path may address a node at a different nesting depth or with a
different Native Name. Type or Protocol Variant qualifiers identify the permitted
schema context; a compatible type alone does not identify the intended value.
Within a copied Table, corresponding members retain exact, case-sensitive Native
Name matching. Selecting a differently located Table does not implicitly rename
its members; such a difference requires an explicit mapping of the relevant
node where permitted, or a payload with corresponding member names.

Both CopyFrom and CopyTo use the same reduced-field projection rules. Match
record fields by exact, case-sensitive Native Name and recurse through records
and array elements. Source-only fields absent from the destination type MUST
be ignored without a copy error; they are not assignments and do not
participate in destination compatibility, overlap, or protection checks.
Matching fields retain the applicable type, nullability, semantic scalar, and
Union identity requirements. This does not make a required destination field
optional or permit an incompatible matching field. A valid null assigns null
to a nullable destination; it does not mean unchanged. A received CopyTo payload
must remain valid under its own schema. CopyFrom may project an unfinished
editor under the destination's completeness rules.

CopyFrom and CopyTo override initialization only at the nodes they actually
assign. On a newly constructed destination, every unaffected node retains its
ordinary Fixed, Default, localized default, nullable, and type initialization.
This includes destination-only fields within a copied Table or array element.
Ordinary initialization may yield a value, valid null, or undefined; it need not
make the constructed request complete. Type initialization, which applies to a
node with no Fixed, Default, or localized default, is the same on every client.
A Nullable node is null, a complete answer. A Table is its fields, each
initialized by these rules; a List is empty; a Union has no selected branch.
Every other required node is undefined, which is unanswered: a String, a
number, a Boolean, an Enumeration, Bytes, a DateOnly, a TimeOnly, and a
DateTime alike. A client never supplies an empty string, zero, a range bound,
or the current time in its place, so a present value always means an answered
one. Merely supplying a reference context
does not change an unaffected node's initialization. On an existing destination,
unaffected nodes keep their current values; copying another field does not
reset them.

Initialization and validation are separate. Studio rejects `Fixed` together
with `CopyFrom` on one field: a copied value that differed from the Fixed
value would leave a protected field that blocks submission. Within a copied
Table or array, a copied member takes precedence over
a Fixed initializer, but still has to satisfy the Fixed constraint and every
other applicable constraint. Copying does not waive destination protections
such as the calculation CopyTo prohibition on overwriting Fixed fields.
An unfinished value in a mapped calculation source is also distinct from a
destination-only field: ordinary defaults must not replace that missing input.
The [calculation completeness rules](protocols.md#reduced-record-and-array-copies)
determine whether the constructed request can be sent.

The operation defines how the projected values are applied. CopyFrom constructs
its destination; CopyTo merges into an existing destination. Source selection,
destination uniqueness, array correspondence, and operation ownership retain
their respective contracts.

A CopyTo path list expresses alternative destination shapes. For each CopyTo
declaration, exactly one destination must match in the eligible bound context.
Multiple matching paths,
including duplicate paths to the same node, are an App design error and are
rejected before any assignment; list order does not resolve that ambiguity.
A report may reach several independent subscribers, each with its own unique
destination. Paths do not search or broadcast across array elements: an array
must be selected as one value, or an item must have an explicit index or binding
permitted by the interaction. Reduced-array copying follows the [typed projection rules](protocols.md#reduced-record-and-array-copies).

Both directions require compatible receiving types and preserved operation
ownership. Schema compilation checks resolvable declarations; runtime checks
actual values and bindings, including any ambiguity not decidable statically.
The context below determines allowed syntax, source availability, and failure
handling.

### Context and syntax

| Usage | Accepted form | Source or destination context |
| --- | --- | --- |
| EnumSource; EndpointBaseObjects/BaseObjects | `$`, `@`, or one or more `^`, followed by named child segments | Root, annotated host, or actual parents in that schema-data tree. EnumSource takes one path; BaseObjects takes a comma-delimited list. |
| LLMDocTrans validation | One `$`, `@`, or parent-relative `^` Schema Path to a Table | The current edited Request: `$` is its root, `@` is the bound document struct, and `^` follows actual value-tree parents. Omission supplies no facts (`{}`); an unavailable declared target blocks the action. The Table's children become [document validation facts](llm-document-transformation.md#app-schema-authoring). |
| Ordinary CopyFrom | Strict `^<Variant>` or `^<Variant.field...>` with a field/index suffix; `$` root-relative shorthand also remains supported; comma-delimited alternatives | The [current interaction reference context](protocols.md#interaction-reference-context), when present. |
| Ordinary CopyFrom: link's bound occurrence on a Response or SubscriptionData | Relaxed `^<Type>` or strict `^<Variant.field...>` naming a self-or-ancestor occurrence on the tapped node's path, at any depth and across nested Lists, with a field/index suffix, on a [link](protocols.md#links) target | The occurrence the tap bound in the displayed value; with no tap, unbound, so the next alternative applies. See [bound occurrences](#typed-ancestors-and-bound-occurrences). |
| CopyFrom when constructing a PrevRequest action | The same variant-rooted or root-relative copy syntax | The construction context contains the current edited request as its reference object. |
| Calculation CopyFrom/CopyTo | Root-relative copy syntax, parent-relative `^`/`^^`, relaxed `^<Type>`, or strict `^<Variant.field.item>` with a field/index suffix | Inputs use the current-editor-first lookup defined by the [interaction reference-context owner](protocols.md#interaction-reference-context); the editor and a reference object may designate the same value. CopyTo addresses the corresponding bound current-request destination. See [calculation requests](protocols.md#field-calculation-requests). A [link](protocols.md#links) declared in a request editor constructs its target and applies its [return](protocols.md#link-return) the same way, with the host field's occurrence bound self-inclusively and parent-relative `^` stepping from the host field. |
| AppendTo | Exactly one `Variant.field` using native names | A root array of an admitting working SubscriptionData; see [array append](protocols.md#subscriptiondata-array-append). |
| AppendIndex | Exactly one root-field `$.field` | A non-null integer control field in the incoming append report; never copied as ordinary data. |
| ConstrainedBy | Request-field form: exactly one root-relative copy path, or one or more `^` followed by named child segments. Response form: exactly one root-relative copy path | Request-field form: `$` is the current request's selected payload; `^` steps from the constrained field to its actual value-tree parents, so each List row can carry its own allowed set. Response form: that response's payload. It is neither an interaction reference-context read nor an alternatives list. |
| CopyTo-directed reduced SubscriptionData | One or more comma-delimited strict `^<StableVariant>` or `^<StableVariant.field...>` destinations | Each admitted stable working value, subject to the shared destination-uniqueness rule above. |
| ValidationRules, RangeMin, RangeMax: current-data reference | `[name/child]`, `[items/0/name]`, or `[]`; an in-scope cursor can be the first segment | The attribute owner's evaluation root or the concrete node bound to that cursor. |
| ValidationRules, RangeMin, RangeMax: external reference | `{$.name}` or `{$.first,$.fallback}`; dot-separated fields only | The same interaction reference context in its defined source order, including editable-request and inherited rules. |
| Display Template selector | The forms and extensions of [Display Template selectors](#display-template-selectors); no literal index and no comma alternatives | Studio resolves it statically against the package schema; a client matches every displayed node the selector names. A `:condition(...)` reads the selected node and the interaction reference context. |

For host-relative schema paths, a child can use `.name`, `[name]`, `["name"]`,
or `['name']`. `$` selects the current value-tree root; `@` selects the host;
each `^` moves to its actual parent. An enclosing named type is not another
runtime parent. List items and selected Union variants remain value-tree levels
for these schema-data paths. For example, `^.choices` selects a sibling List.

The root-relative copy grammar is:

```text
copy-path = "$" { "." identifier | "[" index "]" }
identifier = (letter | "_" | "$") { letter | digit | "_" | "$" }
index = "0" | nonzero-digit { digit }
```

Here letters and digits are ASCII. An index must be a non-negative safe integer;
negative, computed, wildcard, and slice indices are not accepted. Quoted bracket
field names from the host-relative grammar are not part of this copy grammar.
An indexed path such as `$.items[0].quantity` selects one literal position.
A comma in CopyFrom separates complete alternative paths, not array selectors.
Curly validation references use the dot-only form: their entries do not accept
`[index]`, even though ordinary CopyFrom does.

Strict variant-rooted copy paths use `^<Variant>` to select the active variant's
payload and `^<Variant.field...>` to select a schema descendant. Every schema
level, including a selected nested Union branch, must be explicit. Ordinary
field and literal index segments may follow the selector, for example
`^<Variant.items>[0].value`. A path rooted in a different active variant does
not match. `$` is shorthand for the reference object's root where the context
table permits it.

### Typed ancestors and bound occurrences

Calculation paths also accept `^` for the invoking field's parent, `^^` for
its grandparent, and further parent steps, followed by ordinary field/index
segments. Calculation CopyFrom and CopyTo also accept two selector forms:

- `^<EditOrder.items.item>.subtotal` follows a strict, top-down Libertas Schema Path.
  The first name is the active protocol variant's native name; each following
  name is an immediate schema child, in order. Include the array element's
  schema native name (for example `item`) and any selected nested Union branch.
  These names identify schema occurrences, not numeric array positions. No
  intermediate node may be skipped. `^<EditOrder.items>` selects the array.
- `^<OrderItem>.subtotal` is relaxed: resolve the named schema type within the
  active variant's reachable schema, then select its nearest enclosing value
  occurrence. Intermediate schema levels need not be written. Named types use
  normal schema/package qualification (`package::Type`), resolved identity,
  and never display labels or structural resemblance.

Both forms cover the active variant itself: `^<EditOrder>` selects its entire
payload, including a primitive payload. A bare name matching the active variant
selects that root before named-type lookup (a
[Display Template selector](#display-template-selectors) instead checks the
collision statically and accepts it only when the type occurs solely as that
variant's payload); qualification can disambiguate a
named type. Strict paths must start at the variant, not an arbitrary package
root type. Within a selected source tree, neither form can select a sibling
value, another active variant, or climb into the enclosing protocol Union. Missing bindings are unavailable, not
null; unknown or ambiguous type names and invalid Libertas Schema Paths fail validation.

Resolution is grounded in a top-down traversal from the active variant. Bind
only occurrences on the invoking node's ancestry, starting at its parent and
including the root. The root itself remains eligible when it is the invoking
node, which makes primitive variant roots addressable. A
[link](protocols.md#links) binds self-or-ancestor: its invoking node, the tapped
node or the host field, is itself eligible, so `^<TopicRow>` on a link declared
by `TopicRow` selects that row. A strict path pins the
schema occurrence; relaxed matching selects the nearest enclosing occurrence of
its named type. The selector may omit a suffix to select the bound value itself;
otherwise suffixes use ordinary fields and literal zero-based array indices.
`$` still selects the complete current request. Ordinary CopyFrom
uses each reference object in context order as a candidate variant root; calculation
bindings do not add an invoking context to it or
permit CalculationRequest on protocol variants. These current-request bindings
do not exclude calculation inputs from the interaction reference context; see
[calculation requests](protocols.md#field-calculation-requests). The two logical
contexts do not require separate stored copies.

Ordinary CopyFrom accepts a bound occurrence only on the target of a
[link](protocols.md#links) on a Response or SubscriptionData, whose invoking
node is the tapped node. The selector names the nearest self-or-ancestor
occurrence of its type on the tapped node's path, at any depth and across
nested Lists; a tap pins the whole path, so a type held in several places or in
nested Lists is not ambiguous. The relaxed `^<TopicRow>.section` names that
type; a name matching a protocol variant selects that variant instead. In the
strict `^<Hub.topics.topic>.section`, the first name is the displayed variant
and each following name is an immediate schema child down to the bound
occurrence, including List element names. Real schemas use the relaxed form
today: both parsers emit an empty native name for a List element (the same
holds for calculation selectors), so the strict form becomes usable once they
emit element names. The displayed value remains the reference object; the
binding only selects which occurrence the selector denotes. With no tap, from
Actions, a menu, automatic selection, or any other entry, the selector is
unbound, which is unavailable rather than null, and CopyFrom continues with the
next alternative. Parent-relative `^` remains calculation-only, and a relaxed
`^<Type>` in ordinary CopyFrom denotes only this bound occurrence. A link in a
request editor binds like a calculation, from the host field, under the
[context table](#context-and-syntax).

Bindings belong to the operation. A later reply cannot retarget another item
because it occupies the same index; different items remain distinct invocation
contexts. The calculation contract owns scheduling, invalidation, atomic
assignment, and deferred re-execution.

### Current-data references in validation expressions

Square-bracket references use slash-separated field names and non-negative
integer array positions. `[]` selects the owner root. `[item]` selects an
in-scope traversal cursor's concrete node; `[item/value]` continues from it.
Otherwise the first segment begins at the evaluation root. For example,
`[items/0/name]` accesses one element's field.

If a path exists, it resolves to its value; otherwise, it resolves to null.
Resolve segments over actual values in source order:

1. A named segment selects a child of each current non-array node.
2. At an array, a named segment fans out to each element, applying the same
   unconsumed segment to each one.
3. An integer segment selects that position from each current array.
4. Results retain parent-path order followed by array-element order.

Multiple results form an ordered collection. Collection-aware operations consume
collections; scalar-only operators reject them. A traversal source must select terminal arrays; their
elements are concatenated in path order for cursor binding. A cursor preserves
its concrete absolute path for diagnostics. Union access follows the active
payload without a special variant selector: an absent field in the active
variant resolves to null. This is expression
resolution, not an instruction to copy or mutate data.

The expression forms use these productions (with the same ASCII identifier
meaning as above):

```text
current_data_schema_path = "[" [ segment { "/" segment } ] "]"
segment = identifier | digit { digit }
reference_object_schema_path = "{" dot_path { "," dot_path } "}"
dot_path = "$" { "." identifier }
```

Surrounding whitespace in slash-separated segments and comma-separated
references is ignored. Neither form evaluates an index expression. Curly
references select ordered alternatives; square references follow current data.

### Availability, null, and interaction ownership

Reference-context lookup is a shared schema reference capability. Any schema
attribute whose semantics support reading a referenced value may use this
context when its contract permits it. Each attribute defines its accepted path
or expression syntax, expected value type, and handling of an unavailable
reference; this capability does not make every attribute accept a path.
The [interaction reference-context rule](protocols.md#interaction-reference-context)
owns the server-supplied context and its replacement. A reference lookup never
performs an implicit server fetch or substitutes task parameters, sibling
request values, task-upgrade data, pooled endpoint data, or another interaction's
reference context.

Curly validation references visit reference objects in the same context order.
Within each object, they inspect comma-delimited paths in declaration order and
return the first non-null value. Missing, undefined, and null alternatives are
skipped, continuing into later objects when necessary; no match yields null.
They are pure reads. Unlike CopyFrom, a valid null does not stop this lookup.
With no reference context, including first-time composition, every such read yields
null; authors must handle that absence. CopyFrom follows the distinct
[copy-selection and initialization rules](#shared-copyfrom-and-copyto-rules).
Studio checks their syntax but treats their result type as schema-unknown and
nullable because server-supplied reference-object schemas may differ.

Editable-request rules use the interaction's current reference context
through curly references. Function rules see the argument Table;
a field/type rule is rooted at its owner. A required field can be temporarily
unanswered locally, but a valid path expression does not make it optional or
permit an incomplete transmitted request. EditRequest construction follows the
[interaction reference-context rule](protocols.md#interaction-reference-context).

A syntactically valid path does not grant permission, create a missing value,
prove a runtime index exists, or transfer ownership of a pending reply. The
containing attribute defines assignment, validation, and lifecycle behavior.
ConstrainedBy, EnumSource, BaseObjects, and calculation rules retain those
operation-specific requirements in their owning sections.

## Display Template selectors

A [Display Template](presentation.md#display-template) rule selects the nodes
it styles with one selector. Selectors reuse the Libertas Schema Path forms, and
Studio validates each one against the package schema exactly as it validates
CopyFrom paths. A template is not evaluated inside one active interaction, so a
selector names schema occurrences: at runtime it matches every displayed node
that is such an occurrence, and it never binds a single value.

| Form | Selects | Rank in the [cascade](presentation.md#display-template-cascade) |
| --- | --- | --- |
| `^<Variant>` | The whole value of a variant of an Endpoint protocol Union or a function data Union: a protocol screen or a stored record. | Strict |
| `^<Variant.field...>` | One schema occurrence, explicit at every level including selected Union branches, with the [strict-path rules](#typed-ancestors-and-bound-occurrences). | Strict |
| `^<function>`, `^<function.parameter...>` | An App function's configuration editor, the synthetic Table of its arguments, or one occurrence inside it. | Strict |
| `$.field...` | Root-relative in whichever variant or configuration is displayed. | Strict |
| `^<Type>`, `^<Type>.field...` | Every occurrence of a named type, or that field in every occurrence. Named types use `package::Type` qualification; a Rust `crate::Type` maps to it. | Relaxed |

Template selectors add these extensions, valid only in a template:

| Extension | Meaning |
| --- | --- |
| `[]` after a List | Every member of that List, as in `^<Hub.topics>[]`, optionally followed by field segments. Parsers emit an empty native name for a List element, so the strict element form cannot be written; `[]` stands for it. |
| ` ...` descendant combinator | Every node beneath the selected one, as in `^<Review> ...`. A selector using it ranks as relaxed. |
| `:pending`, `:error`, `:selected` | A runtime state that no data expresses, as the [conditions](presentation.md#display-template-conditions) define. Style only; never on layout. |
| `:condition(expression)` | A data condition: a Boolean ValidationRules expression rooted at the selected node, with `[]` the node, `[status]` a child, and curly references reading the interaction reference context. |

```text
selector  = base [ " ..." ] [ state ]
base      = "^<" name { "." name } ">" { suffix } | "$" suffix { suffix }
suffix    = "." identifier | "[]"
state     = ":pending" | ":error" | ":selected" | ":condition(" expression ")"
```

`name` is a native name, or a qualified `package::Type` in the relaxed form. A
state applies to each selected node. Literal indices, comma-delimited
alternatives, and parent-relative `^` steps are not selector syntax.

Name collisions follow the rule for [typed ancestors](#typed-ancestors-and-bound-occurrences):
a bare name in `^<...>` that matches a variant selects the variant, and a named
type with the same name is written qualified, `package::Type`. Because a
template rule is not evaluated inside one active variant, Studio checks
collisions statically across every protocol and data Union of the App and its
functions. A bare `^<Name>` that matches both a protocol variant and a named
type is accepted when that type occurs in the protocol only as that variant's
payload, since the variant and the type then denote the same node; whenever the
type occurs anywhere else, the name is rejected and the diagnostic names the
qualified form. A bare name that matches both a function and a named type, or
more than one variant or function, is likewise rejected.

## 1. Artifact model

A Libertas App version has two related, independently useful JSON artifacts:

1. The **schema package** is the machine contract. It describes callable
   functions, a graph of named and inline types, constraints, protocol roles,
   cross-package identities, and revision metadata.
2. The **package document** is the localized presentation layer. It describes
   package, function, type, field, enumeration, and string-resource text without
   changing wire identity or field order.

Each package also has one locale-independent
[Display Template](presentation.md#display-template), which decides how the
schema's values are arranged and styled on a device. It is not part of the
schema: it is absent from the schema JSON and its revision digest, and it is
published and versioned as a [document](publication.md#display-template-identity).

The Hub assigns an application version when deployed code or schema changes. It
captures one UTC timestamp and encodes the version as
`yyyyMMdd.HHmmss.milliseconds.counter`: `major` is the UTC date, `minor` is the
UTC time, `build` is the `0..999` millisecond component within that second, and
`revision` is a counter that distinguishes versions created during the same UTC
minute. A document-only update preserves the application version and
independently versions the changed document metadata.

The normal generation pipeline is:

```text
TypeScript or Rust source
    -> language parser and external-reference discovery
    -> expanded compiler graph for validation
    -> ownership pruning and FQN reference rewriting
    -> language-neutral local schema with AppDependencies=[] + default-locale document
    -> Studio dependency-first deployment and authoritative Hub version read-back
    -> complete flattened AppDependencies list
    -> deterministic schema sorting and revision digest
    -> document normalization/localization and Display Template compilation
    -> deployment as schema + documents (including the template) + code
```

The separation is deliberate. `NativeName` remains a stable programmatic name in
the schema, while localized `DisplayName`, field `Name`, `Description`, and
package `StringResources` values live in documents. Translating presentation text therefore does not
break type references or other schema links.

## 2. Schema package

The shared in-memory contract is:

```typescript
interface QWHAScriptPackageJson {
  AppDependencies: QWHAAppDependencyJson[];
  Types: QWHATypeInfoJson[];
  Functions: QWHAScriptSignatureJson[];
  Attributes: QWHAScriptAttributeJson[];
  Revision?: QWHASchemaRevisionJson;
}

interface QWHAAppDependencyJson {
  package: string;        // canonical developer-package identity
  Version: string;        // required exact yyyyMMdd.HHmmss.mmm.counter pin
}

interface QWHAScriptSignatureJson {
  NativeName: string;
  Parameters: QWHATypeInfoJson[];
  Attributes: QWHAScriptAttributeJson[];
}

interface QWHATypeInfoJson {
  ID: number;
  NativeName: string;
  Type: string;
  BuiltinType: boolean;
  Children: QWHATypeInfoJson[];
  Attributes: QWHAScriptAttributeJson[];
  Heritages?: string[];
  NativeType?: string;
  ForeignSchema?: string;   // parser/normalization carrier
  Namespace?: string;
  isNullableFlag?: boolean; // parser/normalization carrier
}

interface QWHAScriptAttributeJson {
  Name: string;
  Value: string;
}

interface QWHASchemaRevisionJson {
  SourceTimestampUtcMs: number;
  ContentSha256: string;
}
```

`AppDependencies`, `Types`, `Functions`, and `Attributes` are always arrays in a
normalized schema. The language parsers emit `AppDependencies: []` and initially
omit `Revision`; Studio finalizes both deployment-owned fields before submitting
the schema.

### 2.1 Top-level collections

| Field | Meaning |
| --- | --- |
| `AppDependencies` | Studio-generated complete flattened logical App dependency lock. Each entry carries a canonical package identity and the exact Hub App version whose schema was used for this deployment. `Version` is required and identifies that exact version. |
| `Types` | Named types whose declarations are owned by this App. A type declared by another package is never copied here, even when it is imported, used, or re-exported locally. |
| `Functions` | Exported callable functions that are not explicitly ignored. Functions are identified by `NativeName`; parameters are positional and retain source order. |
| `Attributes` | Package-wide string key/value metadata. |
| `Revision` | Studio-owned source timestamp and canonical content digest. It is not part of the schema content being hashed. |

The generated function model contains parameters but no return-schema field.
Request/response flows are modeled through endpoint protocol unions and persistent
function data through a function data-schema union.

### 2.2 Deployment dependency list

`AppDependencies` records the complete flattened set of exact App-version pins.
The required `Version` key means exact identity, never a lower bound.
The [deployment dependency contract](deployment.md) owns source discovery,
preflight, installation order, read-back verification, and failure behavior.

A dependency entry contains only `package` and `Version`. Missing, null,
malformed, noncanonical, or unknown version fields are rejected, as are duplicate
package entries even when they name the same version. These requirements apply
to every new installation and save. The Hub can still read already-retained
schemas using `minimumVersion` as the same exact pin, or predating the
`AppDependencies` collection. This compatibility path never writes legacy data.
To save an updated schema, regenerate and redeploy it in dependency-first order.
Do not rename fields in retained artifacts: the original bytes and revision
digest are immutable. The new schema receives a new revision and deployed App
version.

Android and iOS retain legacy-Hub operation through their separate legacy
protocol and schema adapters. That path does not relax modern dependency
validation. Studio does not support legacy Hubs.

### 2.3 Type-node fields

| Field | Meaning and invariant |
| --- | --- |
| `ID` | A parser-assigned structural ID used to associate schema nodes with localized field documents. It is local to a named-type tree or function parameter tree, not a package-global type identity. |
| `NativeName` | The source-level simple name of a type, field, parameter, union variant, or enum member. An unnamed list element may use an empty string. |
| `Type` | For a built-in node, the Libertas runtime type. For a reference node, either the simple name of a locally owned type in `Types` or the canonical FQN of a type owned by another package. |
| `BuiltinType` | `true` when the node directly describes a Libertas runtime type; `false` when `Type` is a named-type reference resolved locally or through the FQN package resource graph. |
| `Children` | Ordered structural children. Child order is wire-significant for tables, lists, and unions. Named references normally have no inline children. |
| `Attributes` | String-valued constraints, roles, UI hints, identity metadata, and parser-generated markers. Marker attributes use `Value: "true"`. |
| `Heritages` | Optional ordered list of base type references. A local base uses its simple name; an externally owned base uses its canonical FQN. Consumers merge inherited fields without replacing same-named fields declared by the derived type. |
| `NativeType` | Native-language type metadata. The Rust parser records concrete types such as `u16`; the TypeScript parser normally emits an empty string. Shared validation initializes a missing value to `""`. Wire behavior is defined by `Type` and wire-affecting attributes, not by this field. |
| `Namespace` | Legacy normalization carrier for older expanded schemas. New external references carry the complete package identity directly in `Type`; local nodes normally use `""`. |

`ForeignSchema`, `isNullableFlag`, and Rust's `EnumValues` may exist transiently
inside a parser:

- Shared normalization converts `ForeignSchema` to `Namespace` and removes it.
- `isNullableFlag` is converted to `Nullable=true` and removed.
- `EnumValues` is consumed while constructing/normalizing an inline enumeration
  and removed by the Rust parser.

The Rust parser resolves and extracts its complete native source graph. Studio
materializes required FQN dependencies before running shared semantic
validation, so individual parsers do not need a parallel local-only
implementation of rules decidable from the emitted graph.

### 2.4 Runtime types

The parsers reduce source-language types to the following Libertas runtime model:

| Category | `Type` values | `Children` |
| --- | --- | --- |
| Primitive | `Nil`, `Boolean`, `Number`, `String`, `Bytes` | Empty. Numeric width and bounds are attributes rather than distinct `Type` values. |
| Composite | `Table`, `List`, `Enumeration`, `Union` | Table fields, one list element schema, enum members, or union variants in wire order. |
| Semantic scalar | `DateTime`, `TimeOnly`, `User`, `Device`, `LanDevice`, `VirtualDevice`, `Endpoint` | Empty. The semantic type selects platform validation and UI behavior while retaining its runtime scalar representation. |
| Reserved placeholder | `Action` | Unsupported. Its enum slot is retained for numbering stability; it has no authoring alias, picker, value codec, or invocation API. |

Numbers use `Type=Number` plus `NumberType`. TypeScript schema annotations accept
`u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f32`, and
`f64`; native `number` does not preserve this information. The Rust parser
currently maps concrete `u8` through `u64`, `i8` through `i64`, `f32`, and `f64`.
Rust derives the width from the native type. Integer nodes default to
`NumberStep=1` in parser paths that construct integer constraints. A
cross-language contract must use a width supported by both representations.

### 2.5 Structural order and IDs

- Table fields, function parameters, list elements, and union variants preserve
  parser/source order. This order is part of the wire contract.
- IDs are assigned in structural traversal order and restart for each named type
  and each function's parameter forest.
- TypeScript enum-member IDs use an enum-local counter beginning at zero and do
  not consume the surrounding tree's sequence. The Rust parser currently applies
  its ordinary depth-first allocator to enum members. Cross-language tooling maps
  IDs through verified structure rather than assuming the two native schemas use
  identical enum IDs.
- An ID is meaningful together with its owning function/type document. It is not
  a substitute for a canonical type identity.
- Top-level sorting never sorts `Children`; it therefore cannot alter wire order.
- A document carries the corresponding node ID in `FieldDocumentJson.Type`.

## 3. Named-type and package identity

Libertas separates native source identity from canonical wire identity.

### 3.1 Local named types

A local reference has `BuiltinType=false` and `Type=<simple type name>`. Lookup
matches that name in the current package's `Types` collection.

Every self-canonical App type must be publicly exported from the App root
`index.ts` or `lib.rs` under exactly its declaration name. Renamed exports and
duplicate flattened simple names are rejected. Qualified source-language paths
resolve declarations. If resolution finds a declaration in another package, the
emitted runtime reference is the canonical package FQN regardless of how the type
was imported or re-exported.

### 3.2 Foreign wire types

The canonical cross-package form is:

```text
developer-package::TypeName
```

Both `developer` and `package` use lowercase snake case, separated by exactly one
hyphen. `TypeName` is a source-language identifier-like simple name.

For every externally owned reference, this complete FQN is stored directly in
`Type`, `Heritages`, `Bitflags`, `ForeignTypeSchema`, `ProtocolSchema`, or
`FunctionDataSchema`, depending on the reference site. Every such FQN contributes
its package to the direct dependency set used for pre-deployment Hub exploration
and the finalized `AppDependencies` closure. An FQN always means external ownership, so its package component
must differ from the package that owns the referencing schema; local references
use the simple type name. These reference FQNs are always inferred by a parser
from language-native source references. An annotation may spell a qualified
native reference inside a string, but that spelling must resolve through the
native-language symbol graph and is never copied as the final schema identity.
Every unqualified native reference, whether used in a type slot, heritage, or
annotation, must resolve in the declaration's actual Rust or TypeScript scope.
An imported external declaration is canonicalized to its owning App FQN; an
unbound name is a parser error even when an unrelated source file or dependency
declares the same terminal identifier. Parsers never use a global same-name
fallback.
The owning type definition and its type document remain only in
the original package. Importing, using, or publicly re-exporting a type does not
transfer ownership to the consuming App.

Every resolved type outside the current package is foreign by definition. Its
reference uses the native owning App FQN after verifying that the owner is a
valid Libertas App and publicly exports that exact type from its root. A
fabricated Bitflags catalog is the visibility exception: it may be a private
exact-name declaration at the owning App root because it never becomes a
runtime source-language type.

`ForeignTypeSchema` appears only on a published named compatible type. Its value
is the direct official type FQN from another App. The compatible type remains published under its
own App FQN; the attribute is not inherited or copied onto its fields or
references. The official target may not itself be compatible.

`ForeignTypeSchema` is the only source annotation whose FQN value is interpreted
literally as schema identity. Language extraction resolves native ownership;
Studio validates the emitted declaration's placement, external official target,
and structural compatibility against the materialized dependency graph.

### 3.3 Native compiler metadata

Final artifacts contain each local named type once and external references as
owning-package FQNs. They do not embed copied external type definitions or
transient resolution/source-location metadata. `NativeType` does not
independently change wire bytes. `ExternalNativeType` is removed; native imports
resolve declarations while serialized references keep their actual owner.

## 4. Attribute model

All attributes use the same `{ Name, Value }` representation. Scope and host type
determine their meaning. The tables below list attributes emitted or synthesized
by the current parsers and shared validator.

### 4.1 Package attributes

| Attribute | Meaning |
| --- | --- |
| `DefaultLocale` | Locale of the generated default document. Defaults to `en`. It must be 1–128 ASCII alphanumeric characters in segments separated by one `-` or `_`; leading, trailing, and repeated separators are invalid. |
| `StringResources` | One non-empty, unique, comma-delimited package catalog resolved from a source constant. The default package document stores matching LMF1 templates in its top-level `StringResources` object. Every function and type in the package uses this catalog implicitly. |
| `TypesOnly` | `true` for a schema/document package with no executable functions. Every public package-owned named type exported from the language entry root is automatically included unless ignored; `Functions` must be empty. |
| `no_std` | Rust parser marker recording a crate-level `#![no_std]`. It is source/runtime metadata rather than a data-node constraint. |

### 4.2 Function attributes

| Attribute | Meaning |
| --- | --- |
| `FunctionDataSchema` | Local simple name or external App FQN of a union used as the function's persistent data schema. Validation synthesizes `LibertasData=true` on that union. |
| `Permissions` | On a function, comma-delimited permissions resolved from a source constant. Source order is preserved and every permission must have a same-key, non-empty reason in the package document's `StringResources`. |
| `AccessHost` | One or more comma-delimited network names required by the function's task process. Normal entries are DNS hostnames; `{network_access_local}` and `{network_access_external}` are reserved broad grants. Every entry is a package `StringResources` key with a non-empty localized access explanation. See [App network access](network-access.md). |
| `Singleton` | Exact marker value `true`. At most one task for this App function may exist on a Hub. |
| `ValidationRules` | Pure boolean expressions evaluated against one synthetic Table containing all function arguments. Each expression is followed by `:`, a quoted package `StringResources` error-message key, and `;`. |

### 4.3 Value, constraint, and presentation attributes

| Attribute | Meaning |
| --- | --- |
| `Nullable` | The value may be absent/null. Optional source types normalize to this marker. |
| `Fixed` | The node has a fixed encoded value. Enumeration members use it for their numeric values; union wire identities use `Discriminator`. On an Endpoint client it instead carries a task-process FQN, `developer-package::function`, which constrains otherwise schema-compatible servers and forces that package into `AppDependencies`; the Endpoint-ID choice remains editable. |
| `Default` | Initial value. Enumeration defaults follow the [shared enumeration contract](#53-enumerations). On a Union field it names, by native name, the variant that initialization selects; that variant's payload initializes as an ordinary selection, and Studio rejects a name that is not one of the union's variants. User and `List<User>` values follow [User and group defaults](#user-and-group-defaults). On an Endpoint client it instead names the task process recommended for provisioning when no schema-compatible server exists; it does not filter existing candidates and the Endpoint-ID choice remains editable. |
| `NumberType` | Numeric width/signedness (`u*`, `i*`, or `f*`). |
| `NumberMin`, `NumberMax`, `NumberStep` | Numeric bounds and step on Number or DateTime. DateTime is implicitly an unsigned 64-bit integer of Unix epoch seconds; an explicit `NumberType` must be `u64`. Step is measured from `NumberMin`, or zero when no minimum exists. |
| `RangeMin`, `RangeMax` | One Data Validation bound expression compatible with the host type on a Number or DateTime; applies inclusively to the GUI and final validation. Result typing, compatible dates, and unavailable bounds follow [dynamic numeric ranges](validation-rules.md#31-dynamic-numeric-ranges). |
| `LengthMin`, `LengthMax` | Retained String-length metadata from the removed TypeScript String annotation. Current TypeScript and Rust source uses SizeMin/SizeMax. |
| `MinLengthAbs`, `MaxLengthAbs` | Parser-generated absolute native string constraints, currently used by the Rust `char` mapping. |
| `SizeMin`, `SizeMax` | Inclusive size bounds for List element count, String UTF-16 code-unit length, or ordinary Bytes byte count. [MIME-aware Bytes](#mime-aware-bytes) use content length only when media is defined. Nullable null is governed separately by Nullable. Table is not a valid host. |
| `Unique` | Requires a scalar value to be unique within the requested number of ancestor-list scopes. The no-argument default level is `99`. |
| `Unordered` | Declares list ordering semantically irrelevant to management/UI behavior. |
| `ReadOnly` | Value is displayed but not user-editable. Initialization, CopyFrom, EditRequest, calculation CopyTo and draft restoration still write it; only `Fixed` protects a value. See the [ReadOnly rule](presentation.md#common-field-metadata). |
| `Hidden` | Value is hidden from ordinary editors. On a chart carrier table, composition, or chart-bearing Union variant wrapper, it also marks direct non-chart fields as preserved transaction context that renderers exclude from chart projection. |
| `SuppressLabel` | Parser result only; compiled into the Display Template. Presentation marker suppressing the visual row label of a scalar, Table, or Union; on a List of Tables it suppresses only List-contributed member names and positions ([label-free value rows](presentation.md#suppresslabel-label-free-value-rows)). |
| `Link` | One non-empty Request variant native name of the same Endpoint protocol Union, never a list, on any field or on a Table, Union, or Enumeration type declaration; the field's target wins over its type's. Tapping the displayed node opens that Request ([links](protocols.md#links), [rendering](presentation.md#link-rendering)). |
| `InlineChoices` | Parser result only; compiled into the Display Template. Exact marker value `true` on an Enumeration or Union field or type declaration, never a Bitflags catalog. Clients show the permitted choices inline as radio controls ([inline choices](presentation.md#inline-choices)). |
| `Foldable` | Parser result only; compiled into the Display Template. Boolean `initially_open` on an effective Table, Union, or List; presence permits manual [folding](presentation.md#folding-structs-and-unions). |
| `UIIncludeDescription` | Parser result only; compiled into the Display Template. Exact marker value `true`. Clients render the node's localized long `Description` inline together with its short `DisplayName`; it changes presentation only, not the typed value or wire encoding. |
| `TimeInterval` | Displays a numeric value as a duration. It does not create a distinct wire type. |
| `DateOnly` | Exact marker value `true` on an integer `Number`. The stored and wire value is a valid Gregorian calendar date in decimal `yyyyMMdd` form; clients present a date chooser and localized calendar-date labels without applying a timezone. |
| `Bitflags` | `true` on a fabricated `Enumeration` catalog, or that catalog's local type name/external App FQN on an integer `Number`. Catalog members have explicit `Fixed` values. Clients display selected localized names comma-delimited and edit them with a multiple-choice chooser; the wire value remains an ordinary integer. |
| `Format` | Parser result only; compiled into the Display Template. One Unicode LDML display pattern on `Number`, `DateOnly`, `DateTime`, or `TimeOnly`. The effective host type selects number-pattern versus date/time-pattern grammar. It never changes storage, wire encoding, validation, or comparison. |
| `Money` | Currency/scale metadata such as `USD/2` on signed `i64` Number. Raw values, bounds, and defaults are integer minor units. See [Money](presentation.md#money) for the exact representation, supported CLDR currency tokens, inline editing, and chart rules. |
| `PhysicalUnit` | One canonical CLDR core unit identifier on a `Number`, including generated compounds such as `millimeter-per-day`. It preserves the raw unit without changing serialization; a client localizes the unit when its ICU implementation supports it. |
| `FormattedText` | Marker value `true` on an effective `Bytes` host; serialization is unchanged. Follow the [formatted-output contract](presentation.md#static-read-only-placement-and-request-values) for mandatory static ReadOnly, permitted request initialization, and runtime-use restrictions, and [LMF1](message-format.md) for the canonical tuple. |
| `MimeTypes` | Comma-delimited exact media types or ranges on an effective `Bytes` host. It restricts the value domain and uses the [LMB1 MIME-aware Bytes encoding](#mime-aware-bytes). It cannot coexist with `FormattedText` or occur within a `CacheIdentity` field. |
| `EnumSource` | One Libertas Schema Path to a `List`; an unsigned integer host stores the selected item's zero-based index. |
| `Chart` | One primitive mark (`point`, `line`, `area`, `bar`, `rect`, `rule`, `arc`, `text`, `polygon`) or composition (`layer`, `hconcat`, `vconcat`) token on a response/subscription-data payload. A primitive may use a carrier Table with exactly one non-hidden List of rows; compositions and single-chart Union wrappers may contain additional direct non-chart fields only when those fields carry `Hidden=true`. |
| `ChartChannel` | Canonically ordered comma-delimited visual roles on a direct scalar field of a chart row. |
| `ChartScale` | Canonically ordered scale properties (`id,type,min,max,mid,zero,reverse`) for one scale-bearing chart channel. The final schema omits `type`, which is compiled into the Display Template's `scale` hint. |
| `ChartGuide` | Canonically ordered generic guide properties (`target,source,position,title,domain,ticks,labels,grid`) on a target binding or guide-label field. The final schema omits `position`, which is compiled into the Display Template's `guide` hint. |
| `Chart3D` | One 3D primitive (`point`, `line`, `bar`, `surface`, `wireframe`, `stem`) or composition (`layer`, `hconcat`, `vconcat`) token on a response/subscription-data payload. |
| `Chart3DChannel` | Canonically ordered comma-delimited 3D visual roles on a direct scalar field of a 3D chart row. |
| `Chart3DScale` | Canonically ordered scale properties (`id,type,logBase,min,max,mid,zero,reverse,growMin,growMax,padding`) for one scale-bearing 3D chart channel. `logBase` is greater than 1 and valid only for `log`; non-negative `growMin/growMax` expand automatic continuous position domains; `padding` is an inclusive `0..1` bar-base setting. The final schema omits `type`, which is compiled into the Display Template's `scale` hint. |
| `Chart3DGuide` | Canonically ordered 3D guide properties (`target,source,title,domain,ticks,labels,grid,tickCount,tickStep`) on a target binding or guide-label field; `detail` is also a guide target. |
| `Chart3DView` | Canonically ordered camera/projection properties (`projection,azimuth,elevation,zoom,fov,orbit,aspectX,aspectY,aspectZ,targetX,targetY,targetZ,pan,zoomEnabled`) on a 3D chart root. The three booleans independently gate fixed client actions; no camera/gesture mode exists. |
| `CalculationRequest` | Field-only comma-delimited list of distinct calculation Request variant names in the same protocol. Its input `CopyFrom` mappings and participating fields must agree in both directions, except that [invariant inputs](protocols.md#invariant-calculation-inputs) need not participate. See [field calculation requests](protocols.md#field-calculation-requests) for invocation context, validity, and implementation scope. |
| `CopyTo` | Declares destinations under the [shared copy rules](#shared-copyfrom-and-copyto-rules). The [context table](#context-and-syntax) selects the grammar and destination; [calculation results](protocols.md#applying-calculation-results) and [reduced subscriptions](protocols.md#reduced-subscriptiondata-design-pattern) define application and lifetime. |
| `CopyFrom` | Declares source paths on protocol Request/SubscriptionRequest data under the [shared copy rules](#shared-copyfrom-and-copyto-rules). The [context table](#context-and-syntax) selects the reference context and grammar. |
| `ValidationRules` | Pure boolean expressions on any field or schema data type. Each expression is paired with a quoted package `StringResources` error-message key using `expression : "KEY";`. Field-local, inherited-field, and referenced named-type rules are cumulative. |
| `DeviceType` | Device-type restriction for a `Device` value. |
| `VirtualDevice` | Virtual-device-type restriction for a `VirtualDevice` value. |

Rows marked "parser result only" are presentation attributes. Studio validates
them on the parser result, compiles every occurrence into the package's
[Display Template](presentation.md#display-template) at the lowest
specificity, and then removes them from the final schema, with the transient
source metadata and before sorting and revision hashing. The final schema never
carries them, so changing one alone never changes schema identity.

`TableHeader` and `UIName` are retired. Studio rejects either in a parser
result with a message that names its replacement: promotion is the
template's [`promote`](presentation.md#tableheader-promoted-fields-and-union-selectors)
rule and flattening its [`flatten`](presentation.md#flatten) rule, and a [data-titled row](presentation.md#data-titled-rows) is a template
[`label`](presentation.md#labels-and-titles) rule with a promoted child as its value.

The exact versioned Base64 attribute layouts used by `DeviceType` and
`VirtualDevice` are specified in
[Matter device type attribute encodings](matter/descriptors.md).

`ValidationRules` is string-valued and is valid on a function, any schema data
field, and any named or inline schema data type. Function rules are rooted at a
synthetic Table of all arguments. Field/type rules are rooted at the annotated
value, and `[]` denotes that value itself. Square paths such as `[limits/max]`
read the current evaluation root. Curly paths such as
`{$.limits.max,$.legacy.max}` are ordered, pure reads from the optional current
[interaction reference context](protocols.md#interaction-reference-context),
including in editable Request/SubscriptionRequest trees.
These reads do not depend on CopyFrom being present. Source order, path order,
and first-non-null selection follow the
[shared lookup rule](#availability-null-and-interaction-ownership).

`RangeMin` and `RangeMax` use the same field-local `[]` and optional
external `{...}` references. Their result-type requirements, date
compatibility, null handling, GUI/final enforcement, and interaction with
static constraints are defined once in
[dynamic numeric ranges](validation-rules.md#31-dynamic-numeric-ranges).

Within any validation expression, single `|` is the value-preserving null
fallback operator. It evaluates left to right, returns the first non-null
operand unchanged, and does not evaluate later operands. `false`, zero, empty
strings, empty arrays/collections, and objects are non-null winners. `|` binds
after `+`/`-` but before `is` and comparisons. It may produce any supported
intermediate type, but the complete rule must remain Boolean; non-Boolean
values require explicit comparison because there is no general truthiness.

The string may begin with `FinalOnly=true;` or `FinalOnly=false;`; omission
defaults to `true`. This is a leading directive within that `ValidationRules`
value, not a separate `FinalOnly` schema attribute. Only explicit `false` opts
that rule set into incremental editor validation; every rule remains included
in final validation.
Each cumulative field, inherited, or named-type rule set retains its own phase.

Every rule expression must then be followed by `:`, a quoted package
`StringResources` key, and its terminating semicolon. Studio rejects an
undeclared key or an empty default-locale message. When a rule fails, clients
display the localized value for the first failing rule's key in declaration
order. They stop at that failure without generating a structural explanation.
If its installed message is missing or blank, they display a generic validation
failure.

Studio owns grammar and every schema-decidable semantic check. It validates
square paths against the owner root and treats curly-path results as unknown
and nullable because an interaction-compatible peer may have a different
schema. Clients evaluate the rules against incomplete editor state as well as
complete data: `== null` and `!= null` are legal even for a non-nullable field.
Mandatory-field validation and `ValidationRules` are complementary. A
null-tolerant rule may pass for an incomplete value, but it never changes the
field's `Nullable` contract or suppresses the final mandatory-field failure.
Clients use the same rules and semantics for mandatory incremental editor
validation and required final validation. Final submission independently
requires every static schema constraint and every validation rule to pass.
After every proposed Task Editor node change, Android and iOS evaluate all
incrementally applicable rule sets across the complete editable data. A failed
change is rolled back to the exact prior data before the client displays the
violation as an error toast. Rule sets led by `FinalOnly=true;`, including
those using the omitted-directive default, do not participate in this
incremental transaction.
The complete language, precedence, null behavior, arrays, unions, and source
spellings are normative in the
[Libertas Data Validation Rules Specification](validation-rules.md).

`PhysicalUnit` is emitted by exactly one positional quoted
`@LibertasPhysicalUnit("identifier")` or
`#[libertas_physical_unit("identifier")]` declaration. Its CLDR identifier
rules, provided-unit/no-conversion rule, localization behavior, chart surfaces,
and compatibility requirements are normative in
[Client-side charting §5.5.1](charts/2d.md#551-physical-units).
It is valid only on `Number`, follows the
[exclusions of numeric semantic attributes](#numeric-semantic-exclusions),
and never changes raw constraints, storage, requests, grouping,
or scale geometry. Invalid CLDR identifiers and invalid placement are schema
errors. The schema accepts generated CLDR compound identifiers such as
`millimeter-per-day` independently of any particular client's ICU catalog. A
client localizes a supported identifier and may omit unit decoration when its
platform cannot format it; the metadata and numeric value remain intact.

`Format` is emitted by exactly one positional double-quoted
`@LibertasFormat("pattern")` or `#[libertas_format("pattern")]` declaration.
Its language-neutral syntax comes from the Unicode Locale Data Markup Language:
[UTS #35 number format patterns](https://unicode.org/reports/tr35/tr35-numbers.html#Number_Format_Patterns)
for ordinary `Number`, and
[UTS #35 date/time format patterns](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)
for `DateOnly`, `DateTime`, and `TimeOnly`. The viewing user's preferred locale
supplies localized digits, decimal/grouping signs, month and weekday names, day
periods, and time-zone names; the pattern supplies their order, width, and
literal punctuation.

The portable number subset supports required `0` and optional `#` digits,
decimal `.`, primary/secondary grouping `,`, one positive and optional negative
subpattern separated by `;`, `%`, `‰`, scientific `E`, affixes, and LDML
apostrophe quoting. Currency `¤` and `¤¤` require explicit `Money` metadata.
Significant digit `@`, padding `*`, wider currency tokens, and rounding-increment
digits `1`–`9` are rejected. A pattern is at most 128 Unicode characters, 21
integer digits, and 20 fractional digits.

The portable date/time subset supports `G`, `y`, `M`, `d`, and `E` for dates;
`H`, `h`, `m`, `s`, `S`, and `a` for times; and `z` for a `DateTime` time-zone
name. Repetition follows LDML width rules, and apostrophes quote literal text.
`DateOnly` accepts only date fields and never acquires a timezone. `TimeOnly`
accepts only time fields and remains seconds from local midnight. `DateTime`
accepts both groups, formats epoch seconds in the viewer's local timezone, and
alone permits `z`. A pattern cannot combine `H` and `h`. Other LDML fields are
reserved until every Libertas presenter can implement them identically.

The following documented field fragments belong inside a documented schema
type:

```typescript
/**
 * Invoice amount
 * Shows the billed amount with two decimal places and parentheses when negative.
 * @LibertasFormat("#,##0.00;(#,##0.00)")
 */
amount: number;

/**
 * Service day
 * Identifies the calendar day when the service occurs.
 * @LibertasNumber(type=u32)
 * @LibertasDateOnly
 * @LibertasFormat("yyyy-MM-dd")
 */
serviceDay: number;

/**
 * Last updated
 * Identifies when the latest service record became available.
 * @LibertasFormat("d MMMM yyyy, HH:mm:ss z")
 */
updatedAt: LibertasDateTime;
```

`Format` is valid only on the four effective hosts above and may be inherited
through a named type. On an ordinary `Number` it follows the
[exclusions of numeric semantic attributes](#numeric-semantic-exclusions);
on a `DateOnly` Number it uses date-pattern
grammar. It may coexist with `PhysicalUnit`, in which case it formats only the
numeric portion and the unit remains separate localized semantic metadata.
Money uses the [exact currency editor](presentation.md#money). Other editors retain native number/date/time controls and may show a formatted
preview. Renderers apply `Format` only at presentation boundaries—labels,
summaries, table cells, tooltips, chart ticks, and accessibility text. Parsing
user input, Avro payloads, defaults, bounds, sorting, grouping, chart geometry,
cache keys, and protocol compatibility always use the raw scalar.

`Singleton` is function metadata because a task instantiates a function, not a
package. The Hub identifies the function by App ID plus `NativeName`, independent
of the selected App version. Creating, retargeting, or changing the version of a
task is rejected when another task has the same function identity and either
selected function schema carries `Singleton=true`. An update may replace its own
task record. Checking both schemas prevents an older non-singleton version from
bypassing an already installed singleton declaration.

`@LibertasExport` remains a TypeScript source-level parser directive, but it
does not emit a schema attribute. The former `Library=true` marker has been
removed from the language-neutral schema; schemas that still contain it are
rejected.

`DateOnly` is an integer encoded as `yyyyMMdd`. Client GUIs present a date
chooser. For example, August 15, 2026 is stored as `20260815`; the chooser may
display `2026-08-15` and serializes the selection back to `20260815`.
The marker accepts no arguments,
requires an effective integer `NumberType`, and follows the
[exclusions of numeric semantic attributes](#numeric-semantic-exclusions).
When `Format` is present, its date pattern replaces only the localized label;
the chooser and encoded value remain governed by this `DateOnly` contract.

`Bitflags` projects a localized flag catalog onto an ordinary integer. It has
two complementary forms:

- a named built-in `Enumeration` carries `Bitflags=true`; every child is a
  `Number` with one explicit non-negative decimal `Fixed` value;
- an effective integer `Number` carries `Bitflags=<local catalog type name>` or
  `Bitflags=<developer-package::external catalog type name>`.

Every nonzero catalog value must have exactly one bit set, and no two members
may have the same value. One zero member is allowed so documents can localize
the empty selection as `None`, `N/A`, or similar. The integer host must be wide
enough for the union of all declared bits. A field's `Default`, `Fixed`, task
configuration value, outgoing runtime value, and received runtime value may
contain any combination of declared nonzero bits and no undeclared bit.

The catalog is fabricated presentation/validation metadata and need not be a
public source-language type. A private external catalog must be declared under
its exact name at its owning App root. Parsers resolve the complete native
qualified reference and serialize the owner FQN; Studio fetches that package
from the selected Hub during deployment planning, materializes its catalog and
localized document for validation/UI, and records its authoritative version in
the final `AppDependencies`. Documents localize Enumeration members in the
normal way. Read-only surfaces join the selected localized names with `, `;
editors use a multiple-choice chooser, one checkbox per nonzero member, and
encode the bitwise union of the choices. The zero member is never a choice:
leaving every box unchecked is the empty selection `0`, and the zero label is
shown only as the read-only text of an empty selection.

```json
{
  "NativeName": "Access",
  "Type": "Enumeration",
  "BuiltinType": true,
  "Attributes": [{ "Name": "Bitflags", "Value": "true" }],
  "Children": [
    { "NativeName": "None", "Type": "Number", "Attributes": [{ "Name": "Fixed", "Value": "0" }] },
    { "NativeName": "Read", "Type": "Number", "Attributes": [{ "Name": "Fixed", "Value": "1" }] },
    { "NativeName": "Write", "Type": "Number", "Attributes": [{ "Name": "Fixed", "Value": "2" }] }
  ]
}
```

`Bitflags` is intentionally invisible to the Hub. It does not change Avro,
storage, protocol compatibility, or any Hub parser/runtime behavior; the field
is encoded exactly as its `NumberType`. Studio/shared schema validation owns
the portable catalog rules, while Studio clients own UI presentation and
runtime-mask validation. `Bitflags` follows the
[exclusions of numeric semantic attributes](#numeric-semantic-exclusions).

<a id="numeric-semantic-exclusions"></a>

**Exclusions of numeric semantic attributes.** Each of these attributes gives
a Number one meaning, so two of them cannot stand on the same number unless
the table permits the pair. The relation is symmetric. Studio rejects an
excluded pair on the effective attributes of the field and its type.

| Attribute | Cannot coexist with |
| --- | --- |
| `DateOnly` | `TimeInterval`, `EnumSource`, `PhysicalUnit`, `Money`, `Bitflags` |
| `TimeInterval` | `DateOnly`, `PhysicalUnit`, `Money`, `Bitflags`, `Format` |
| `EnumSource` | `DateOnly`, `PhysicalUnit`, `Money`, `Bitflags`, `Format` |
| `PhysicalUnit` | `DateOnly`, `TimeInterval`, `EnumSource`, `Money`, `Bitflags` |
| `Money` | `DateOnly`, `TimeInterval`, `EnumSource`, `PhysicalUnit`, `Bitflags` |
| `Bitflags` | `DateOnly`, `TimeInterval`, `EnumSource`, `PhysicalUnit`, `Money`, `Format` |
| `Format` on a Number | `TimeInterval`, `EnumSource`, `Bitflags` |

`Format` may stand with `DateOnly`, `Money`, or `PhysicalUnit`: it then
formats the date, the amount, or the numeric portion.

`FormattedText` follows the [static read-only placement and request-value
contract](presentation.md#static-read-only-placement-and-request-values),
including for reusable and `TypesOnly` declarations. The concrete App that
emits values supplies its one package catalog.

Resource declarations are package-local. Exactly one package-level
`StringResources` attribute may declare the stable keys used by all functions
and types in that App. Its default document contains the matching top-level
`StringResources` map. A formatted tuple carries only a package-local key and
typed arguments: task/package context already identifies the catalog, so adding
a type owner or resource FQN would complicate and bloat the artifact without
benefit. The validator checks declared catalog syntax, requires the schema key
set and package-document key set to match exactly, and rejects duplicate
package declarations. A source catalog cannot contain the same key twice, so
Studio reports that conflict before deployment. Validation does not prove the
runtime key encoded in every Bytes value.

`AccessHost` uses the same package catalog for informed network-access
decisions. Its source forms are
`@LibertasAccessHost("api.example.com,telemetry.example.net")` and
`#[libertas_access_host("api.example.com,telemetry.example.net")]`, and it is
valid only on a function. The emitted value contains DNS identities rather than
resolved addresses: resolution can change over time, while the localized
same-key resource remains a stable explanation shown to the user or
administrator. The list has no scheme, port, URL path, wildcard, or IP literal;
transport and port policy are separate concerns. Hostname comparison for
duplicates is case-insensitive, but each source spelling must exactly match its
`StringResources` key.

Task-access approval presents function `Permissions` and `AccessHost` entries
together. Clients always render each raw key as the summary and the same App
package's localized value separately as its explanation. For `AccessHost`, the
raw key is also the actual network identity. The explanation supplements the
permission or hostname; it never replaces or hides it. Different Apps may give
different same-permission explanations because each App owns its own catalog.

App TCP and UDP networking is denied by default. A concrete hostname grants
both protocols to its Hub-resolved IPv4/IPv6 addresses. The reserved names
`{network_access_local}` and `{network_access_external}` grant their respective
address classes and remain mandatory localized string-resource keys. They are
reserved for this purpose; Studio rejects either key when it is not used by a
function `AccessHost`. While a task process runs, the Hub refreshes DNS and
monotonically adds new answers; old answers remain until process restart.
Changing the declaration during a reload forces that restart. These are
kernel-enforced process invariants, not SDK behavior. See
[App network access](network-access.md) for the complete address-class, DNS,
and lifecycle contract.

Runtime payloads never carry format templates. A formatted `Bytes` value is the
canonical encoded tuple of a resource key and ordered, typed
`LibertasMessageArgument` values, including an empty argument list.
Localized templates use zero-based Libertas Message Format 1 (LMF1), not
printf. See [Libertas Message Format](message-format.md).

FormattedText does not allow a String host containing Base64. That design would
encode the tuple only to decode it again for every render, adding no information
or storage benefit and obscuring the difference between formatted data and
literal text. Keeping the runtime host as Bytes provides a sharp type
distinction that makes developer mistakes less likely. A generic JSON transport
may spell its Bytes value as Base64, but that is only the transport's byte
codec; the schema and runtime value remain Bytes end to end.

#### User and group defaults

`User` stores a user or group ID. Neither a `User` nor a `List<User>` implicitly
selects the current user, including when only one picker candidate exists.
Without `Default`, ordinary initialization applies, including undefined.
Copying retains the [shared initialization precedence](#shared-copyfrom-and-copyto-rules).

The `Default` string accepts these case-sensitive selectors:

| Selector | Initial identity |
| --- | --- |
| `admin` | The Hub's built-in administrator user. |
| `admin-group` | The Hub's built-in administrator group. |
| `user-group` | The Hub's built-in regular-user group. |
| `guest-group` | The Hub's built-in guest group. |
| `current-user` | The authenticated session's user. |
| Decimal ID from `0` through `9223372036854775807` | That explicit user or group ID; legacy ID `0` remains valid. |
| `current-user-groups` | The current user's direct and inherited groups; `List<User>` only. |

A scalar `User` accepts exactly one selector except `current-user-groups`.
A `List<User>` accepts a comma-delimited sequence; an empty or whitespace-only
string means an empty List. Trim surrounding whitespace for every selector.
Unknown tokens, empty entries within a nonempty sequence, and unsupported
scalar/list combinations are schema errors.

Resolve built-in identities from Hub metadata and current-user identities from
the authenticated session, never from display names. Expand current-user groups
in ascending numeric ID order, removing duplicates; then process the complete
selector sequence left to right, keeping each ID's first occurrence. Missing
required Hub, session, or membership context leaves the whole default unresolved
(undefined), never a partial selection. A resolved empty membership yields an
empty group expansion. Final values still satisfy ordinary field constraints.

For example, `@LibertasDefault("current-user")` initializes one User, while
`#[libertas_default("current-user,current-user-groups")]` on a `List<User>`
initializes the current user followed by their groups. These are initialization
selectors, not runtime string values or new schema attributes.

##### Fixed current-user security constraint

`Fixed` on `User` (`LibertasUser` in source) is always a security constraint
that the Hub MUST validate. The only currently valid value is the exact string
`current-user`. Numeric IDs, built-in identity selectors, group selectors, and
selector lists are invalid for Fixed; they remain available under Default
where documented. Future Fixed values require an explicit security contract
and corresponding Hub enforcement; an unknown value MUST be rejected.

`Fixed("current-user")` is not merely an initialization hint. It requires a scalar `User` at a Request or SubscriptionRequest root, or
within its nested structs and selected union branches. List items, list-level selector combinations,
configuration parameters, response-only uses, and declarations with no valid
request use are schema errors. Reuse in a forbidden position remains an error
even when the same declaration also has a valid request use. The client resolves the authenticated session's user ID and prevents
editing. It may be combined with `Hidden`; hiding the field changes neither
its encoded value nor its validation and authorization requirements.

**Recommended declaration order:** declare one `Fixed("current-user")` field
as the first payload field of each Request or SubscriptionRequest variant,
and reuse that verified identity in App code. This keeps the identity near
the start of the payload after any protocol variant selectors. First position
is a recommendation, not a schema requirement; nested protocol variants and
identity fields within nested request values remain supported, excluding Lists.
If additional fields carry this constraint, the Hub MUST validate every
applicable occurrence; validating the first never exempts later fields.
Declaration order enables efficient identity checks but does not guarantee
that a Hub implementation decodes only one field.

Before invoking App code, the Hub MUST compare the submitted value with the
user identity established by authentication for that invocation. It MUST reject
a mismatch, null, an unresolved identity, or malformed input. It MUST NOT repair
a submitted mismatch by replacing the value. Client autofill, ReadOnly, Hidden,
labels, and a claimed user ID in the payload are never evidence of identity.

The same requirement applies to ordinary requests, subscription requests,
workflow submissions, calculation requests, and AI tool calls. An App-to-App or forwarded invocation
without a trusted authenticated user binding MUST be rejected when it requires
this constraint; the App owner or a user ID copied from another invocation is
not a substitute. Copies and supplied EditRequest values retain their bytes
and must pass the constraint when submitted. Stored results do not constitute
proof of the current caller's identity.

The rule uses effective attributes, including aliases, inheritance, and
materialized dependencies. Every submission must carry the resolved current
user ID. A client that cannot resolve it must leave the request unfinished;
Hub admission rejects it rather than assuming a user or weakening the check.

For mandatory notification recipients, combine `Default` with static `ReadOnly`:
on a scalar User it protects that ID; on a List it protects membership and order,
so mark the User item type ReadOnly as well to protect each recipient ID. Follow
the [ReadOnly placement rules](presentation.md#common-field-metadata). An unresolved
explicit default blocks submission even when ReadOnly; never silently substitute recipients.

### MIME-aware Bytes

Android, iOS, Studio, and the pure Web Control client implement this contract.
The pure Web client requires origin-private file storage to retain selected
media links across page reloads; without it, file selection uses an in-memory
value for the current editor and camera capture is unavailable.

`MimeTypes` is a field or named-type attribute on an effective `Bytes` host. A
field declaration replaces its type's entire filter. Its value is one or more
unique lowercase ASCII entries separated by commas without spaces. An entry
is an exact `type/subtype`, a `type/*` range, or the sole entry `*/*`. Each
name component has 1–127 characters, begins with a lowercase letter or digit,
and then uses lowercase letters, digits, or `! # $ & ^ _ . + -`. Parameters,
file extensions, MIME aliases, and quality values are excluded. A value always
names one exact MIME type. Studio validates the grammar, effective Bytes host,
duplicates, literal defaults/fixed values, and conflicts. `MimeTypes` cannot
coexist with `FormattedText` and cannot occur in a `CacheIdentity` field or
its descendants, because the draft identity tuple persists its value.

An unannotated `Bytes` field remains a fully supported opaque binary type,
including for machine-to-machine protocols. `MimeTypes` is optional. A defined
MIME-aware value has this byte sequence inside the ordinary Avro `bytes`
primitive:

```text
ASCII "LMB1" | u8 mime_length | mime_length ASCII MIME bytes | content bytes to end
```

`mime_length` is 3–255. The MIME string is exact lowercase `type/subtype`,
matches the effective filter, and has no parameters. The enclosing Avro length
delimits the content; there is no content-length field, filename, local path,
or file-link token in the wire value. A decoder rejects malformed, truncated,
unknown-version, or disallowed nonempty bytes. JSON transports may Base64-wrap
the entire sequence. An App must validate actual content when the declared
MIME label matters to its processing or security.

Zero-length `Bytes` means **media undefined**. It is a present and valid wire
value, including on a non-`Nullable` field. A server may send it; clients show
no selected media. It is distinct from nullable null and from a valid `LMB1`
value with zero content bytes. For [ValidationRules](validation-rules.md),
zero-length MIME-aware Bytes evaluates as null; ordinary Bytes retain their
existing valuation. A mandatory media field needs an explicit rule such as
`[] != null : "IMAGE_REQUIRED";`. `SizeMin` and `SizeMax` run only for defined
media and measure content bytes, excluding the LMB1 header. Thus `SizeMin=1`
rejects a defined empty file but does not require a file to be selected.
Transport limits count the entire sequence.

Clients may hold the field as undefined, as a client-local file link, or as
nonempty in-memory bytes. Server-sent full values use the in-memory form;
server-sent empty bytes become undefined. A selected file or camera image is
staged privately and held by a local link. Every outbound serialization opens
and validates linked files, then materializes the complete LMB1 value before
sending. If a link disappears during submission, the client stops that attempt,
marks the field undefined, reports the field error, and retains other edits.
It never silently submits empty bytes for a disappeared file. The
[local draft rule](annotations.md#mime-aware-bytes-in-workflow-drafts) governs
recovery and cleanup; [media presentation](presentation.md#mime-aware-bytes-presentation)
governs the default controls and preview.

`MimeTypes` changes the semantic Bytes type without changing the Avro
primitive. Raw and MIME-aware Bytes are not implicitly compatible. A producer
MIME filter is compatible with a recipient only if every exact type it admits
is accepted by the recipient; dynamic values are checked against the
destination at runtime. Copies preserve the complete envelope; clients do not
guess or transcode media from a filename or destination filter. Adding
`MimeTypes` to an existing raw field requires a new immutable App version.
Legacy mobile-Hub paths retain their raw Bytes behavior.

### 4.4 Chart presentation attributes

`Chart` is valid only on an effective `Response` or `SubscriptionData` payload.
The [2D chart data contract](charts/2d.md#3-data-contract-and-placement) owns
primitive, carrier Table, composition, and Union-wrapper shapes, including
preservation of `Hidden` context. Chart metadata is schema presentation
semantics: it adds no payload bytes, message type, or Hub state.

`ChartChannel` uses the canonical role order defined in
[Client-side charting](charts/2d.md#5-chartchannel). Roles are
deduplicated and ordered by Studio's shared validator after language-specific
extraction. A field may have only one scale-bearing role; `tooltip` and `detail`
are the only roles repeatable across fields. Secondary endpoints (`x2`, `y2`,
`theta2`, `radius2`) require their primary and cannot own a scale.

FormattedText chart fields use the same custom LMF1 renderer as notifications
and data names. LMF1 changes only localized labels and their measurement; raw
resource keys or tuple bytes remain the categorical domain, grouping, ordering,
shared-scale, and interaction identity. An LMF1 argument's number/date style is
not replaced by a field `Format` or a chart scale formatter.

`point`, `line`, `rule`, and `text` accept either their Cartesian or polar
coordinate form; `polygon` accepts Cartesian or polar vertices and fills its
implicitly closed compound subpaths with the fixed `evenOdd` rule. Theta and
radius remain quantitative numeric positions: their explicit scale type may be
`linear`, `log`, `symlog`, `sqrt`, or `identity` with already normalized
`0...1` values, never `ordinal`, `band`, `point`, `time`, or `utc`. The complete
equal-aspect mapping, polygon
grouping, guide, interaction, and limit rules are defined in
[Client-side charting](charts/2d.md#53-cartesian-and-polar-geometry).
A `layer` accepts unit children or nested layers (flattened in schema order),
not a direct `hconcat` or `vconcat`; concat grids may nest freely.

`Enumeration`, `Device`, `VirtualDevice`, `User`, `Endpoint`, and `LanDevice`
are typed categorical chart values. The raw member/reference value defines
category identity, grouping, domain order, and focus keys; the client localizes
an Enumeration member or resolves a Libertas object through its normal
permission-aware display resolver. Apps do not
add duplicate String label fields solely for charts.

A plain `String` on any role that can emit human-readable chart text is a
conformance assertion that the value is genuine literal domain/user data, such
as an externally supplied proper name or free-form annotation. A closed
App-defined vocabulary uses an Enumeration; App-authored dynamic wording uses
formatted Bytes output; and an identity that is a Libertas system object
retains its `Device`, `VirtualDevice`, `User`, `Endpoint`, or `LanDevice` type.
This rule covers positions, facets, ordinal legends, text marks, tooltips,
value/span guides, and accessibility. A key-only String or guide-less
identity-scale appearance String is not rendered as text and is exempt. Studio
can validate the type/role mechanics but cannot infer String provenance from
schema JSON, so App review and response fixtures must enforce that assertion.
The reconciled bubble, activity-heatmap, direct-label, network, Sankey, and
traffic-source field dispositions are listed in
[Client-side charting §8.1](charts/2d.md#81-localization-and-schema-document-dependency).

A `Bytes` value carrying `FormattedText=true` is also categorical chart data.
It may be used only for categorical and textual roles with an automatic,
`ordinal`, `band`, or `point` scale. Quantitative roles, continuous domains,
and identity scales are invalid. Its raw encoded tuple remains
the category, group, order, and interaction identity; localized rendering is a
presentation view and never changes those semantics. Plain `Bytes` is not a
chart scalar.

An integer `Number` carrying `DateOnly=true` is a temporal calendar-date chart
scalar. Position scales infer `time`; explicit `time` and `utc` both preserve
the same timezone-free civil date, while `band` and `point` opt into sparse
date categories. Quantitative roles and `linear`, `log`, `symlog`, `sqrt`,
`ordinal`, or `identity` scales are invalid. Shared scales distinguish DateOnly
from ordinary Number values. Continuous DateOnly geometry maps valid
`yyyyMMdd` identities through their timezone-free Gregorian day ordinal, not
through decimal integer distance. Generated `min` and `max` must be exact valid
Gregorian `yyyyMMdd` values after source-number canonicalization.

`ChartScale` properties are emitted in `id,type,min,max,mid,zero,reverse`
order with no whitespace. Source annotation key `kind` becomes wire key `type`.
Scale IDs, closed type tokens, booleans, finite domains, channel/type
compatibility, identity-scale restrictions, and same-ID compatibility are
validated by Studio's shared chart validator after parser extraction and
dependency materialization. Scalar properties use one shared shortest
round-tripping binary64 spelling, so lexically different equal values cannot
create false shared-scale conflicts. Canonical generated DateOnly bounds must
be valid eight-digit `yyyyMMdd` dates; equivalent source numeric spellings are
normalized before this check. DateTime bounds must be integral epoch seconds
in `0..253402214399` inclusive. TimeOnly bounds are seconds in `0..86400`
inclusive, with `86400` reserved as
the end-of-day scale endpoint. The closed target/scalar/scale-type matrix,
including band-only offsets/facets and ordinal integral appearance values, is
normative in Client-side charting; `TimeInterval` remains continuous and never
acts as an ordinal or identity value. Attribute lookup follows ordinary nearest
same-name replacement, including official compatible-type materialization.

`ChartGuide` has required `target` and `source` properties followed canonically
by optional `position,title,domain,ticks,labels,grid`. Sources are `scale`,
`value`, `span`, and `none`; positions are `auto`, `top`, `right`, `bottom`,
and `left`; every component is `auto` or `none`. Scale/none declarations sit on
the matching primary scale binding. Value/span declarations sit on a direct
scalar guide-label field with no scale-bearing channel; value targets
`x/y/theta/radius`, while span targets a continuous Cartesian `x` or `y` with
its paired endpoint. Non-identity primary scales infer a scale guide; identity
and offset scales infer none. One explicit target declaration replaces that
default. Guide kinds, component and side matrices, shared-layer ownership,
typed localization, ticks, span grouping, resource limits, and fallback are
normative in [Client-side charting](charts/2d.md#64-chartguide-generic-guides).
The deterministic 200-point post-guide plot-width threshold and tap-activated
overlay are runtime layout policy defined there, not schema properties;
renderers do not invent a different breakpoint or relocation policy.
The same runtime contract fixes the preview viewport at 240 points with a
180-point charting region. Fullscreen uses three quarters of the current usable
height, capped in portrait by the current usable width, keeps complete legends
in the one outer vertical chart viewport, and initially scrolls only enough to
expose the first charting region. None of these policies adds schema grammar.
Discrete positional `band` and `point` scales likewise add no grammar. Clients
preserve every category in a bounded plot, use an exact 20-point minimum
leaf-category pitch (including nested offset lanes), and expose overflow through
an X and/or Y category viewport. A two-dimensional heatmap may pan both axes;
ordinary category charts consume only their overflowing dimension. Category
viewporting never aggregates or samples rows.
Dense monotone Cartesian lines and vertical areas use the contract's automatic
logical-point resampling with a visible `≈` disclosure. That is renderer-owned
level of detail, not grammar or an App transform. There is no chart-specific
hard limit on source rows, primitives, or path vertices; structural/metadata
limits, checked scans, cancellation, viewport culling, and visible LOD provide
the safety boundary.
The complete token tables and geometry rules are normative in
[Client-side charting](charts/2d.md).

The App task owns every data transformation, including arbitrary time
projection, binning, normalization, and layout. It may emit a native temporal
position, or transformed numeric geometry plus a typed `ChartGuide`
`source=value/span` label field. The client renders and localizes that
chart-ready snapshot; it does not infer or execute an App transform.

The opt-in 3D namespace is parallel to, and does not extend, the preceding 2D
namespace. `Chart3D`, `Chart3DChannel`, `Chart3DScale`, `Chart3DGuide`, and
`Chart3DView` add schema metadata only and remain mutually exclusive with their
2D counterparts on one chart path. Primitive `Chart3D` tokens are `point`,
`line`, `bar`, `surface`, `wireframe`, and `stem`; compositions are `layer`,
`hconcat`, and `vconcat`. The closed channel order is `x,x2,y,y2,z,z2,u,v`,
then `color,fill,stroke,opacity,fillOpacity,strokeOpacity,size,shape,strokeWidth,strokeDash`,
then `tooltip,detail,order,key,row,column`.

User-facing chart-family names never become additional schema tokens. Select
the primitive by geometry: scatter and point clouds use `point`; bubbles add
`size`; 3D lines and waterfall/spectrum traces use `line`; columns use `bar`;
height maps, non-uniform grids, and App-sampled mathematical surfaces use
`surface`; wire grids use `wireframe`; contour overlays compose `surface` and
`line` in a `layer`; and impulses use `stem`. Homogeneous multiple series use
`detail`, while heterogeneous series use `layer`.

`Chart3DScale` mirrors `ChartScale`; source `kind` is emitted as schema `type`.
It additionally accepts `logBase > 1` for `type=log`, non-negative
`growMin/growMax` fractions for automatic continuous x/y/z domain growth, and
inclusive `[0,1]` `padding` on a `bar` base axis.
`Chart3DGuide` uses required `target,source`, optional
`title,domain,ticks,labels,grid,tickCount,tickStep`, and permits `detail` as a
target. `Chart3DView` properties are, in canonical order,
`projection,azimuth,elevation,zoom,fov,orbit,aspectX,aspectY,aspectZ,targetX,targetY,targetZ,pan,zoomEnabled`.
Projection is `perspective` or `orthographic`; `orbit`, `pan`, and
`zoomEnabled` are booleans;
azimuth, elevation, and target coordinates are finite schema values without
rewriting authored data. The client canonicalizes effective azimuth and clamps
effective authored/default and transient elevation to the closed
`[-90,90]`-degree interval; target coordinates retain no artificial range.
Zoom and each aspect are greater than zero, and `0 < fov < 180`. Orthographic
projection accepts `fov` and ignores it while orthographic.
`zoomEnabled=false` disables user zoom independently without changing the
authored initial `zoom`.
The three interaction booleans enable or disable the universal client-owned
orbit, pan, and zoom actions independently. They do not select gestures, remap
a disabled action, or create an interaction mode. There is no `mode`, gesture
mapping, sensitivity, control-label, or other client-control property in schema
or runtime data. A preview remains gesture-neutral; the existing full-screen
Chart Explorer alone owns the fixed touch, tap-control, keyboard, and
accessibility vocabulary.
In full-screen portrait, every physical scene uses a single-height horizontal
control strip before the scene and lowers appearance/detail guides below it. In
full-screen landscape or square, it atomically uses a measured trailing
control-and-guide utility rail when the remaining final post-guide plot still
satisfies the normative `M = min(200,C)` width rule; otherwise it retains the
portrait stack. This placement is client runtime policy and adds no schema
property.

Point and stem-tip shape identity and automatic assignment use
`circle,square,triangle,diamond,sphere,cube,pyramid,cylinder` in that order;
bars use `box,cylinder,pyramid,sphere`. A numeric size may be zero to suppress
or degenerate the corresponding geometry without removing its source row.

`bar` accepts discrete or sparse continuous x/y bases with a continuous z
value. `x2`, `y2`, and `z2` are independently optional: absent x/y extents and
the z baseline are inferred, while all three pairs form an explicit cuboid.
The legacy any-orientation form remains valid when exactly two primary axes are
band scales and the sole continuous primary is the value axis. Surface and
wireframe `u,v` values may be arbitrary finite topology coordinates; they need
not be integral or dense, and missing coordinates form holes. There are no
fixed Chart3D schema caps on admitted
rows, vertices, composition units, guides, or tooltips; ordinary transport and
runtime resource admission still applies. The complete geometry, topology,
view, interaction, fallback, and resource rules are normative in
[Client-side 3D charting](charts/3d.md); its
[interaction section](charts/3d.md#12-interaction-accessibility-and-fallback)
defines the universal client control vocabulary.

### 4.5 Union, endpoint, and data attributes

| Attribute | Meaning |
| --- | --- |
| `UnionTagName` | TypeScript parser metadata naming a native union discriminator field. |
| `UnionContentName` | Rust parser metadata naming the content field of an adjacently represented enum. |
| `Discriminator` | Zero-based consecutive integer wire identity on a Union member: Children[i] has Discriminator=i, independent of payload constraints. |
| `UnionTagField` | **ScriptLanguage only** (for example TypeScript and Python): marks the native tag field inside a union variant. Native compiled languages such as Rust must not emit it. |
| `ProtocolSchema` | Local simple name or external App FQN of the union carried by an `Endpoint`. |
| `LibertasProtocol` | Validator-generated marker on a union referenced as an endpoint protocol. |
| `EndpointServer` | Endpoint is the server side of the protocol. |
| Endpoint-client `Default` / `Fixed` | Mutually exclusive peer-selection/provisioning metadata using the established `developer-package::function` FQN grammar. Protocol-schema compatibility determines eligible servers. `Default` leaves that set unchanged; `Fixed` further restricts it to the named process and forces its package into finalized `AppDependencies`. Both Endpoint-ID selectors remain editable and may expose multiple choices. If none are eligible, either FQN recommends the process to provision. The separator is `::`, never `:`. The string is neither an Endpoint ID nor a schema-type reference and is not materialized as a type. |
| `ExcludeUI` | Exact marker value `true` on an Endpoint server carrying a resolved `ProtocolSchema`, or on a protocol `Request`/`SubscriptionRequest` variant. The Hub Control service omits a marked server Endpoint from its UI-device list before grouping and from LLM tool collection; generated Control GUI and Hub LLM tool-definition generation omit marked request operations. Admin creation, editing, approval, persistence, compatibility, and dependency/assignment calculations use the complete schema. Runtime bindings identify a server by its resolved `ProtocolSchema`, so all Endpoint servers sharing one resolved protocol root must use the same `ExcludeUI` value. The endpoint remains registered; protocol roles, serialization, routing, subscriptions, and authorization are unchanged. |
| `RequestConfirmation` | Exact marker value `true` on a protocol variant carrying `Request`. A client asks the person to confirm before it sends the request, and the AI assistant asks the user before it calls the generated tool; the wire form and validation are unchanged. Rejected on any other node, with `DefaultRequest`, and on a request that a `CalculationRequest` names. See [critical requests](protocols.md#request-confirmation). |
| `LLMToolCall` | Boolean tool-definition switch, default `true`. On a protocol Union, `false` excludes descendant Request variants from Hub LLM tool generation; a variant's explicit `true` overrides the Union. The nearest declaration or occurrence on the path wins. `false` on one Request variant excludes only that variant. It does not remove an Endpoint or Request from Control, alter wire data, or bypass `ExcludeUI` or access checks. |
| `Permissions` | On a built-in Endpoint carrying `EndpointServer`, comma-delimited downstream permission requirements. No endpoint string-resource reasons are required. The Hub checks the selected client task's function permissions once during approval/configuration, not per endpoint message. |
| `Request`, `Response` | Protocol roles for a union variant. |
| `Error` | Exact marker value `true` on a variant reachable from an Endpoint protocol Union and carrying the effective `Response` role. It classifies the complete response as an error result that is out-of-band of the current protocol state transition. It does not imply `Response`, prescribe payload fields, or change serialization, routing, transitions, or wire compatibility. |
| `AccessPrivilege` | Integer bitmap on a `Request` or `SubscriptionRequest`: `Read=1`, `Write=2`. The set bits are alternative end-user endpoint access levels allowed to invoke that runtime operation. Omission means `Read\|Write` (`3`). |
| `SubscriptionRequest`, `SubscriptionData` | Subscription protocol roles for a union variant. |
| `WorkflowRequest` | Response semi-role; see the [WorkflowRequest definition](protocols.md#workflowrequest-semi-role). |
| `PrevRequest` | Request-only ordered list of distinct Request targets; workflow Back constructs the selected action from the current editor. Controls are suppressed in embedded trailing panes under the [pane contract](presentation.md#master-detail-panes). |
| `NextRequest`, `NextResponse` | NextRequest names state-machine actions. On requests NextResponse names allowed server replies; on SubscriptionData it declares directional compatible updates. See the workflow contract for editor transitions and completion. |
| `ProtocolConformance` | Management-tooling expression over Union variant native names. Parsers accept it on a Union declaration or a Union-typed variant; when used in an endpoint protocol, it declares requirements for the opposite endpoint at that Union level. |
| `BaseObjects` | Comma-delimited Libertas Schema Paths defining the objects to which an endpoint instance is bound. |
| `LibertasData` | Marks a persistent data union, either from source or synthesized from `FunctionDataSchema`. |

An Error-marked correlated Response completes the current request attempt
without participating in the protocol state transition. The client must preserve the current
interaction and its current response data while presenting the error, and the
same interaction must remain available for retry. The error response must not
be installed as current response data, used as transaction context, navigated
to, or visualized as ordinary data or a chart.

Android and iOS inspect the complete decoded error response in deterministic
depth-first response order: fields follow schema order and List elements follow
encoded order, including hidden fields. A persistent error dialog shows the first non-empty
`FormattedText` value that can be rendered. If the response contains no
renderable `FormattedText`, the dialog shows their localized generalized error
message. Error classification takes precedence over default-response, chart,
and ordinary-data presentation. After consuming the correlated response, the
client clears only the in-flight wait state; it does not close the current
composer or replace the currently displayed response.

Outside a workflow, a normal correlated `Response` with no `WorkflowRequest`
may omit its effective `NextRequest`. This is a
deliberate non-transition, commonly used by shared `Success` or `Failure`
acknowledgements. The client consumes the response and completes the in-flight
attempt without installing it as the displayed protocol data, changing the
request choices, or adding a protocol-data navigation destination. The
[interaction reference-context rule](protocols.md#interaction-reference-context)
still applies to that accepted Response. Omission does not authorize all Request
variants. A present `NextRequest` remains non-empty and installs exactly its
declared choices.

These correlated-response rules apply under the
[shared protocol role rule](protocols.md#protocol-role-reuse).

### 4.6 Identity attributes

| Attribute | Meaning |
| --- | --- |
| `ForeignTypeSchema` | Direct official type FQN declared on a published compatible type. Runtime consumers merge official attributes into a transient copy and preserve explicit compatible overrides. |
| `FullyQualifiedName` | Parser/native package lookup metadata for externally declared types. |

## 5. Composite-type semantics

### 5.1 Tables and inheritance

A `Table` is an ordered record. Each child is one field. A derived table can list
base types in `Heritages`; inherited children are resolved recursively and added
unless the derived table already declares the same `NativeName`.

### 5.2 Lists

A `List` contains one child schema describing every element. The element may be a
built-in node, an inline composite, or a named reference. List-level and
element-level documentation are distinct even though both originate around the
same source declaration.

### 5.3 Enumerations

An `Enumeration` contains ordered member nodes. A member is represented as a
number node and can carry `Fixed` with its source numeric value. Documents carry
an `Enum` array for display names and descriptions. Language parsers extract
the members, resolve native numeric constants, and preserve the supplied
`Default`. Studio's shared schema validator checks a `Default` supplied as a
member name or fixed numeric value and normalizes it to the selected member's
numeric value. This semantic validation and normalization is centralized;
individual language parsers must not duplicate it.

### 5.4 Unions

A `Union` contains variants in wire order. An internally discriminated union uses:

- `UnionTagName` on the union;
- a numeric discriminator field as the first field of each variant;
- `Discriminator=<numeric tag>` on each variant.

`UnionTagField=true` is **exclusively ScriptLanguage metadata**, for languages
such as TypeScript and Python that represent the tag as a native field. It is
not general Union-member metadata and must never be emitted for Rust. The
TypeScript parser marks its native discriminator field this way. Rust payload enums use their variants as union cases and may name an
adjacent content field with `UnionContentName`; they do not emit tag, untagged,
or externally-tagged metadata.

The discriminator field is native compiler structure and is omitted when
compatible union structures are compared or official documents are projected.

Every newly emitted Union member carries an explicit `Discriminator`. For a
Union with N members, values must be exactly `0, 1, …, N−1`, and `Children`
must already be sorted so that `Children[i].Discriminator == i`. Parsers emit
that order. Studio and runtime schema loaders reject negative values, gaps,
duplicates, non-integers, and mismatched ordering. Decoding directly indexes
`Children` after checking the discriminator bounds; no search or remapping
within a Union is required. These identities are never positions in a filtered list. Native Rust payload
enums emit their encoded variant index; TypeScript emits the native numeric tag.
`Fixed` exclusively constrains a value (including the established Endpoint
process constraint). It never identifies a modern Union member. A Rust
`VariantA(u32)` remains one member named `VariantA`, with both
`Discriminator=0` and `Fixed=7` permitted: the first case carries the fixed payload 7.
A discriminator alone never initializes, validates, hides, or locks the payload.

The Hub and all clients resolve each Union member from its own explicit `Discriminator`, or
its zero-based declaration position when absent, without rewriting stored JSON
or digests. **No runtime interprets `Fixed` as a Union discriminator**, even
when `Discriminator` is absent. `Fixed` always retains its payload meaning.
An explicit discriminator must equal its declaration index; invalid values or
ordering are errors, never reasons to fall back or reinterpret Fixed. Cache the identity on the member occurrence owned by the effective
Union, never on a shared payload type. `Discriminator=true` is not a supported
native tag-field marker; ScriptLanguage parsers emit `UnionTagField=true`.


Published protocol and persistent-data schemas are backward-compatible,
append-only contracts. Their Union type names are versionless and remain
invariant for the App's lifetime. New App versions append new payload variants
to the end of the same Union and preserve every existing variant's position,
discriminator, name, payload shape, meaning, and compatibility contract. They
must not replace the container with a version-named Union. See the
[App data-type design guidelines](data-type-evolution.md)
for the authoring and review rules.

## 6. Endpoint protocols and persistent data

### 6.1 Endpoint protocol validation

Every `ProtocolSchema` target must resolve in the expanded compiler graph and be a `Union`. Validation adds
`LibertasProtocol=true`. Each reachable protocol union variant must then satisfy
one of these rules:

1. The variant or its resolved named type has at least one role:
   `Request`, `Response`, `SubscriptionRequest`, or `SubscriptionData`; or
2. it is itself a union; or
3. it contains exactly one field whose resolved type is a union, and validation
   recurses into that union.

A role-less circular union chain is invalid.

Automatic and explicit subscription selection, access privileges, and lifetime
follow the [protocol contract](protocols.md). An effective `Nil`
payload alone does not make every subscription automatic.

On requests, each `NextResponse` entry names a protocol variant marked Response,
SubscriptionData, or Request for EditRequest delivery. On SubscriptionData, it
names other non-null Table SubscriptionData variants under the
[compatible-data contract](protocols.md#compatible-subscriptiondata). Each `NextRequest` entry must
name one marked `Request` or `SubscriptionRequest`. The transition attributes
are optional; Studio validates their non-empty target lists when present and
does not synthesize targets when they are absent.

`AccessPrivilege` is valid only on a protocol `Request` or
`SubscriptionRequest`; the same variant may carry both roles. Roles declare
permitted uses and never select the runtime operation. A client invocation must
carry an explicit Request or SubscriptionRequest opcode, which the Hub validates
against the selected variant's complete role set. Its schema value is a
canonical decimal integer bitmap containing only currently defined bits:
`Read=1`, `Write=2`, or `Read|Write=3`. The bitmap lists alternatives, so a
operation is authorized when the caller's effective access to the owning endpoint
has any listed bit. The omitted default is `3`: because the platform cannot infer
whether an arbitrary operation only reads data or changes state, a read-only user
may invoke an unannotated operation. A developer who knows that an operation
changes state must explicitly set `Write`; a caller with only `Read` is then
blocked by the client UI and rejected authoritatively by the Hub. More bitmap
values are reserved for future access classes.

The word *privilege* is deliberate. `Permissions` describe data/resource access
granted to an entire task process and are checked when tasks and endpoint peers
are configured. `AccessPrivilege` instead constrains an end user's individual
runtime protocol operation against that user's access to the owning endpoint.
It does not grant task-process permissions and cannot replace `Permissions`.

`ProtocolConformance` is accepted on any `Union` declaration and on a child of
a `Union` whose effective type resolves to another `Union`. A Union declaration
does not identify whether it will later be used as an endpoint protocol, so
parsers do not attempt to enforce protocol usage or a client/server role. On a
Union-typed variant, the variant's attribute becomes the conformance expression
for that nested Union in that declaration context and takes precedence over an
attribute on the referenced Union declaration. The expression uses the direct
variant `NativeName` operands of the Union being evaluated with `!`
(NOT), `&` (AND), `^` (XOR), `|` (OR), and parentheses. Operator precedence,
from highest to lowest, is `!`, `&`, `^`, then `|`.

Endpoint client/server compatibility is a mutual configuration-time check. The
client context's conformance rule is evaluated against the server's advertised
variants, and the server context's rule is evaluated against the client's
advertised variants; both results must be true. A scalar variant operand
evaluates to whether the opposite Union advertises that name. A Union-typed
variant operand instead evaluates to the nested Union's conformance result,
using the attribute on that variant as the nested context's expression. Thus
recursion follows the variants actually evaluated by the containing rule rather
than making unrelated optional branches mandatory. If either Union context has
no `ProtocolConformance` attribute, every direct variant declared by that
context is mandatory (equivalent to joining its variant names with `&`). A
malformed explicit expression is incompatible; management tooling must fail
closed rather than falling back to the default.

Name presence is not sufficient by itself. Corresponding root variants must
have pass-through-compatible payload layouts: positional Table fields and
overlapping nested Union slots retain the same names, order, effective types,
nullability, numeric representation, and enumeration values. Root Union
variants may be reordered because the client task host remaps the first Avro
discriminator by variant name. Nested discriminators are not remapped; an
optional nested branch is safe only when appended beyond the smaller Union, so
the smaller receiver rejects that out-of-range index. The task-host translator
does not deserialize the payload. Each receiving App's decoder remains
responsible for payload validation.

Every task editor uses the same filtered candidate sets for manual selection
and initialization. After a `ScriptValue` is created or populated,
the editor visits every uninitialized leaf in the editable value tree of type
`Device`, client `Endpoint`, or `Enumeration`. It fills a leaf only when
exactly one valid candidate remains. Device-type filters,
endpoint protocol compatibility, and the full `Unique` ancestor-list scope rule
are applied before counting. The `Unique` value is the number of ancestor
`List` scopes to check, with `99` as the default. Zero or multiple candidates
leave the value uninitialized for explicit user choice. `Unique` never affects
leaves outside its deterministic ancestor-list scope. Each creation or
population event causes one whole-tree traversal so every uninitialized leaf is
evaluated against the values currently present in its own scope.
User selection follows [User and group defaults](#user-and-group-defaults)
and is not inferred from the candidate count.

Manual `Device` selection applies that same filtered candidate set before it
opens a chooser. If no candidate remains, Android and iOS preserve existing
source membership, roll back an exact provisional `List` member created by the
same Add action, and report the localized no-available-device error. They do not
open an empty chooser or retain an unusable provisional row.

For a client Endpoint, mutual protocol-schema compatibility is always the first
filter. `Default` never changes the compatible candidate set. `Fixed` applies a
second filter requiring the server Endpoint's owning task process to equal its
`developer-package::function` value. Neither attribute makes the selected
Endpoint ID read-only: one task process can provide multiple compatible server
instances. When the final set is empty, both attributes identify the process
recommended for provisioning. `Fixed` additionally makes that App package a
deployment dependency, so Studio includes the package and its authoritative
version in finalized `AppDependencies`; `Default` does not create a deployed
App dependency. This is distinct from the task-process installation dependency
enforced by clients before task creation.

### 6.2 Task pre-creation static dependency check

This gate concerns configured service tasks. The separate
[schema availability requirement](deployment.md#schema-availability-and-task-requirements)
ensures that dependency schemas and documents are available for every editor
choice; a listed App dependency alone does not require a service task.

Before transmitting a task-creation request, every Android or iOS control
client runs the following ordered gate against the selected Hub. This is a
client-side static check in addition to, never a replacement for, the Hub's
authoritative transactional validation.

1. Check the App-and-native-function `Singleton` identity first. On conflict,
   report the failure and stop immediately. Do not queue the request and do not
   start dependency installation.
2. Snapshot the requested task configuration into an immutable client-local
   pending-creation record, then traverse the complete effective function-input
   schema tree. Follow materialized named types, inherited and compatible
   schema nodes, Tables, Lists, and every Union alternative. Traversal must be
   cycle-safe, but a globally visited type set must not suppress another
   occurrence reached by a different field path because ancestor optionality
   is path-dependent.
3. Inspect every client `Endpoint` carrying `Default` or `Fixed`. An occurrence
   is absolutely required when neither that Endpoint nor any node on its path
   to the function root has effective `Nullable`, and no ancestor source array
   or schema `List` permits zero items. A `List` permits zero when effective
   `SizeMin` is absent or zero; a strictly positive `SizeMin` does not. No other
   ancestor kind, including selection of a Union alternative, makes the
   occurrence optional for this static check.
4. For every absolutely required occurrence, calculate the live eligible
   server set from the Hub's current task/Endpoint snapshot. Apply complete
   mutual protocol-schema compatibility first. `Default` leaves every
   compatible existing server eligible. `Fixed` additionally requires the
   server's owning task process to equal its exact
   `developer-package::function` value. The requirement is satisfied only when
   this final set contains at least one server.
5. If any requirement is unsatisfied, present the required-task installation
   UI for the process named by that Endpoint's `Default` or `Fixed` value.
   Requirements may be grouped by identical process identity for presentation,
   but every Endpoint path remains independently checked and all requirements
   must be satisfied simultaneously. Installing the package alone is not
   sufficient: the resulting task must expose a server that passes the same
   compatibility and, for `Fixed`, identity filters. If the named App package
   is absent, clients use the current Libertas Plaza flow to install that exact
   package before opening its required-task editor. A legacy Hub without Plaza
   support can create the dependency task only from an already installed App.

The original creation record remains queued while required installations run;
no task-create/add-update request is sent for it. Creating a dependency task
runs this same gate and may therefore enqueue its own dependency chain. The
client tracks the originating root request and rejects a cycle of unsatisfied
required task-process dependencies instead of recursively opening installation
UI forever.

After each successful installation, the client refreshes the authoritative Hub
task/Endpoint snapshot and reruns the entire gate beginning with `Singleton`.
It must not resume from cached candidate IDs or assume installation alone met a
requirement. Only one fresh reevaluation that simultaneously satisfies every
requirement may dequeue and transmit the original request. Cancelling any
dependency installation in that request's chain immediately cancels and
removes the originating creation record; a later installation or topology
change must not resurrect it. Other independent queued task creations are
unaffected.

### 6.3 Function data schemas

`FunctionDataSchema` is a function attribute whose value resolves in the expanded
compiler graph. The target must be a union. A local target serializes as a simple
name; an external target serializes as its canonical FQN. Successful validation adds
`LibertasData=true` to that union. This data schema is separate from the
function's ordered call parameters.

## 7. Path-based object and choice binding

The [Libertas Schema Path specification](#libertas-schema-path) defines syntax,
context, and resolution. The rules below define the attributes that use it.

`EnumSource` must contain exactly one path, its host's effective type must be
`Number` with unsigned `NumberType` (`u8`, `u16`, `u32`, `u64`, or `u128`), and
the target must resolve to `List` in the same schema-data tree.

`BaseObjects` is a comma-delimited list of paths and is valid only on a built-in
`Endpoint`. At task-value validation time every path must resolve to set values,
and two endpoint instances may not resolve to exactly the same base-object set.
Base objects are Libertas objects rather than devices specifically; for example,
they may be devices, users, or other object types.

For an endpoint carrying `BaseObjects`, the Hub derives and caches its displayed
name and icon from the resolved base objects, in their declared order:

- The name is the comma-and-space-delimited concatenation of the available,
  non-empty base-object names. The Hub stops before the result would exceed the
  platform's maximum logical-device name length.
- The icon is the first available, non-empty base-object icon. If no base object
  has an icon, the endpoint has no derived icon.

When a base object's name or icon changes, dependent endpoint names and icons
refresh, including transitive references through other endpoints. A cycle does
not cause unbounded refresh.

Android and iOS task editors retain a server endpoint with `BaseObjects` but
omit its separate editable row. The same ordered naming and icon rules apply;
the endpoint remains eligible as a compatible server choice for client endpoints.

## 8. Types-only and interoperable packages

`TypesOnly=true` means:

- every public package-owned named type exported from `index.ts` or `src/lib.rs`
  is automatically a schema root unless its source declaration carries the
  language's no-argument `Ignored` directive;
- no export/schema decorator is required to make those public types roots;
- a named type may publish `FormattedText` fields under the
  [formatted-output contract](presentation.md#static-read-only-placement-and-request-values)
  without choosing a runtime direction; its package may publish one shared
  `StringResources` catalog;
- `Functions` must be empty;
- Studio skips executable compilation and deploys an empty code payload;
- the Hub must reject marker/code mismatches and must not create an executable
  artifact for the version.

Without `TypesOnly`, public non-ignored functions are the only roots and the
package publishes the minimum transitive closure of non-builtin types reachable
from their arguments and reachable schema-reference attributes. A decorator on
an otherwise unreachable type does not publish it. `Ignored` is valid on a
public function or public named type; reaching an ignored type from an included
root is an error rather than permission to emit a dangling reference.

A compatible type declares exactly one `ForeignTypeSchema` on that published
named type. Compatibility declarations are per type, so one App may contain
ordinary types and compatible types that refer to different official Apps.
Localized type-document sharing follows each declaration automatically.

At runtime the consumer resolves the official schema, validates structural wire
compatibility, and creates a transient merged view. Official attributes provide
the base at every corresponding node; attributes present on the corresponding
compatible schema node override them by name. The persisted compatible schema is never
rewritten. The final merged graph receives the normal contextual attribute
validation. Document projection uses the same structural ID mapping while
preserving language-native nodes with no official counterpart, such as a
TypeScript-only discriminator field. Package-level display text remains owned by
the compatible package.

## 9. Static and runtime validation

Schema generation performs static validation before deployment. The section
that owns an attribute or mechanism states its checks; the shared layer
enforces those and, among others, the following:

- `TypesOnly` multiplicity, marker value, and empty functions;
- exact function-only `Singleton=true` placement and multiplicity;
- endpoint protocol existence, union shape, roles, transitions, cacheability,
  and role-less cycle rejection;
- optional `NextRequest` semantics: a present value has a non-empty valid target
  list, while omission on a Response remains a valid non-transition and never
  means every Request variant;
- exact `Error=true` placement on a variant reachable from an endpoint protocol
  Union and carrying the effective `Response` role; the marker is
  classification metadata and does not define a response payload. The
  [protocol error procedure](protocols.md#protocol-roles) presents a persistent
  dismissible dialog while preserving the current interaction, response data,
  and retry;
- exact `ExcludeUI=true` placement on an Endpoint server carrying a resolved
  `ProtocolSchema`, or a `Request`/`SubscriptionRequest` variant reachable
  from an endpoint protocol Union,
  plus equal `ExcludeUI` values on all Endpoint servers sharing one resolved
  `ProtocolSchema` root;
- `AccessPrivilege` multiplicity, `Request`/`SubscriptionRequest` placement, and canonical
  nonzero bitmap containing only the currently defined `Read`/`Write` bits;
- function data-schema existence and union shape;
- enumeration default validity and normalization;
- non-empty `BaseObjects` on built-in endpoints only;
- `Unique` only on non-composite hosts with integer level at least one;
- exact `DateOnly=true` placement on an integer Number, valid `yyyyMMdd`
  `Default`/`Fixed` values, and exclusion of `TimeInterval` and `EnumSource`;
- exact `Bitflags=true` Enumeration catalogs, explicit unique single-bit
  member values with the zero exception, integer host/reference/width checks,
  declared-bit `Default`/`Fixed` masks, and conflicting presentation metadata;
- `Format` multiplicity, effective Number/DateOnly/DateTime/TimeOnly placement,
  the host-specific portable Unicode LDML grammar, and ordinary-Number
  exclusion of `TimeInterval` and `EnumSource`;
- `EnumSource` host type, path syntax/resolution, and list target;
- `FormattedText`'s exact marker and Bytes host, mandatory static ReadOnly,
  and runtime-use restrictions under the
  [formatted-output contract](presentation.md#static-read-only-placement-and-request-values);
- exact package-only placement plus syntax, non-empty unique keys, source-value,
  and default-document membership checks for `StringResources`, without
  requiring every formatted declaration to have a catalog or proving runtime
  resource-key membership;
- chart protocol placement, unit/composition shape, channel multiplicity and
  scalar compatibility, mark geometry, scale grammar/configuration, and shared
  scale consistency;
- foreign identity consistency and top-level type deduplication.

Language parsers handle native syntax, resolution, source traversal, and
extraction, including exact root exports and source representation. They do not
provide parallel implementations of schema-attribute rules decidable from the
emitted result and dependencies. Studio owns those shared semantic checks,
reports their available source file and one-based line, and strips all transient
source metadata before artifact sorting, hashing, comparison, or publication.

External references are resolved against the declared exact dependencies for
validation and use. Resolution does not modify or republish persisted schemas
or documents.

Task-value validation is a separate phase. It applies the generated model to
actual values and checks nullability, scoped uniqueness, endpoint base objects,
enum membership, numeric type/range/step constraints, string and byte lengths,
list size, and table structure. Schema validity does not imply that every runtime value is valid.

## 10. Localized package documents

The presentation contract is:

```typescript
interface PackageDocumentJson {
  DocumentVersion: "0.0.2";
  Locale: string;
  DisplayName: string;
  Description: string;
  StringResources: Record<string, string>; // package-owned LMF1 templates
  List: TypeDocumentJson[];  // functions; historical field name
  Types: TypeDocumentJson[];
}

interface TypeDocumentJson {
  Name: string;              // stable function/type NativeName
  DisplayName: string;       // localized natural name
  Description: string;
  List: FieldDocumentJson[];
}

interface FieldDocumentJson {
  Type: number;              // corresponding schema-node ID
  Name: string;              // localized natural field/member name
  Description: string;
  Enum?: FieldDocumentJson[];
}
```

Document format `0.0.2` uses a single package-owned `StringResources` map and
removes the former per-function/per-type `Strings` members. It also supports
flattened custom string properties. A custom
property is encoded directly in the object it documents, beside `DisplayName`
and `Description`; there is no nested JSON `Extensions` member. Implementations
may expose those flattened values through an internal extensions map. For
example:

```json
{
  "Name": "configure",
  "DisplayName": "Configure",
  "Description": "Lets the user choose how this activity operates.",
  "Hint": "Choose at least one item.",
  "List": []
}
```

Parsers read custom properties from trailing comment sections. A marker must be
the only content on its line and has the form `[PropertyName]`. Its value starts
on the next line and continues through the line before the next marker or the
end of that comment section. Multiple markers are allowed. Values are strings;
leading and trailing blank space is removed while internal newlines are
preserved. With an array `----` divider, the text and custom properties above
the divider document the array container, and those below it document the
element.

In TypeScript JSDoc, ordinary label/description text and custom-property blocks
must precede the first `@Libertas...` tag because document extraction stops at
that first schema tag.

```text
Items
Items shown to the user.
[Hint]
Select one or more items.
[Help]
The order is significant.
----
Item
One item.
[Hint]
Enter a single value.
```

The following case-sensitive custom-property names have standard Libertas
semantics:

| Document property | Valid owner | Meaning |
| --- | --- | --- |
| `[DefaultText]` | Field document for a schema field whose effective type is `String`, or for a built-in `Endpoint` field carrying `EndpointServer` | One-line localized default text for that field. Unlike the schema `Default` value, this document value may differ by locale. |
| `[DefaultTaskName]` | Function document only | Localizable default task name for that function. It is distinct from the function document's stable native `Name` and from its localized `DisplayName`. |

Both standard properties are flattened into their owning document objects in
the same way as every other custom property, and their values are translated.
`DefaultText` must not be placed on a package, function, type, enum member, or
any other field type. `DefaultTaskName` must not be placed on a package, type,
field, or enum member. A `DefaultText` value contains no carriage return or
newline after the parser's normal leading/trailing whitespace removal.

Client initialization order is normative. Persisted or user-assigned text has
absolute priority and cannot be changed programmatically by default
application, locale/document/schema refresh, migration, or upgrade. A saved
empty string is also persisted text and must remain empty. Only when no
persisted value exists may a client initialize a new String or Endpoint-server
presentation value. Where the [shared copy rules](#shared-copyfrom-and-copyto-rules)
leave that node to ordinary initialization, an applicable schema `Fixed`/`Default` value wins,
localized `DefaultText` is the next fallback, and the ordinary empty or
localized field-label fallback is last. For a new blank task name, localized
`DefaultTaskName` wins, followed by the localized function `DisplayName`, then
the native function name.

For example, a function and one of its `String` parameters may produce:

```json
{
  "Name": "configure",
  "DisplayName": "Configure watering",
  "Description": "Lets the user choose a watering schedule and affected area.",
  "DefaultTaskName": "Back garden watering",
  "List": [
    {
      "Type": 0,
      "Name": "Schedule name",
      "Description": "The name people use to recognize this watering schedule.",
      "DefaultText": "Weekday schedule"
    }
  ]
}
```

`DisplayName`, `Description`, `List`, `StringResources`, `Strings`, `Enum`, and
`DocumentVersion` are globally reserved custom-property names. A key that is
structural at the current level, including `Locale`, `Types`, `Name`, or `Type`,
also cannot be used as a custom property. Names are case-sensitive. Parsers
reject collisions; consumers may impose additional validation on properties
they recognize, but must preserve valid unknown string properties.

`Strings` is prohibited so it cannot become a misleading custom property. It is
not part of the `0.0.2` document shape.

Important distinctions:

- `TypeDocumentJson.Name` is the stable native function/type lookup key and is
  not translated.
- `DisplayName`, `Description`, `FieldDocumentJson.Name`, enum member `Name`, and
  package `StringResources` values are localized. All custom-property string
  values are localized too, including `DefaultText` and `DefaultTaskName`.
- `FieldDocumentJson.Type` is named historically; it stores the schema node's
  numeric `ID`, not its runtime `Type` string.
- Only enumeration field documents have `Enum`.
- Package `StringResources` stores notification, data-name, permission-reason,
  `FormattedText`, and chart templates by stable package-local key. Function
  and type documents have no resource table. A protocol peer already knows the
  task's App package and therefore needs no type-owner metadata or resource FQN.
- Type documents contain the root type entry plus flattened inline built-in
  descendants. Function documents flatten parameters similarly. Traversal stops
  at named references because local types have their own local documents and FQN
  references resolve to the owning package's document.
- TypeScript discriminator fields receive an empty display name and description
  so compiler representation details do not leak into user-facing text.
- `DisplayName` and `Description` are required string keys at package and
  function/type levels. `Name` and `Description` are required on field and enum
  documents. Empty strings are valid values.
- Every long `Description` uses HTML-style flow. One logical CR/LF break is
  canonicalized to exactly one space; a run of two or more logical breaks is
  canonicalized to one LF paragraph break. CRLF counts as one logical break.
- Every document carries `DocumentVersion: "0.0.2"`. Missing or different
  versions and every `Strings` property are rejected; there is no compatibility
  or migration interpretation.

Empty text is a structural representation allowance, not an authoring
recommendation. A codebot must give every authored schema-visible document node
a non-empty natural label and description, except a parser-suppressed compiler
discriminator field. The complete package/function/type/field/member/variant/
List, accessibility, custom-property, and StringResources prose contract is in
the [schema documentation contract](documentation.md).
The [source annotation reference](annotations.md)
lists every currently supported public TypeScript and Rust spelling.

Document normalization sorts function and type documents by stable `Name`.
Document patching treats the newly generated default document as the structural
source and a translated document as the text target:

- obsolete functions, types, fields, enum entries, and string keys are removed;
- missing structures and strings are copied from the default locale;
- existing localized text is preserved whenever its key is present, including
  an intentionally empty string;
- custom-property keys follow the generated default document: missing keys are
  copied, translated values (including empty strings) are preserved, and
  obsolete keys are removed;
- field and enum IDs are refreshed from the new schema;
- when old and new signatures are supplied, same-named types are disambiguated
  by namespace, and fields and enum entries follow their complete native schema
  paths across insertions, deletion, reordering, and ID regeneration;
- without an old signature, localized field text is retained only for an exact
  numeric shape match. A changed shape receives default-locale field text rather
  than risking a positional mistranslation.

This is why schema `NativeName` values must remain stable while natural localized
names belong in the document.

For the VS Code export/import handoff and the rules a translation bot must
follow, see the
[Libertas Schema Document Translation Guide](../guides/translation.md).

## 11. Determinism, equality, and revisions

### 11.1 Deterministic sorting

Before hashing or deployment, Studio sorts:

- `Types` by `NativeName`, then `Namespace`, then `ID`;
- `Functions` by `NativeName`;
- every attribute array by `Name`, then `Value`.

It intentionally does not sort `Children` or function parameters.

### 11.2 Schema equality

Structural equality resolves named references recursively and compares effective
runtime `Type`, attributes when requested, and ordered children. IDs,
`NativeType`, and ordinary field/type names do not define non-strict structural
equivalence. Function names must match. Strict equality additionally requires the
resolved and reference-node `NativeName` values to match.

Two unresolved external reference nodes can still be compared by canonical FQN;
their owned structures are compared when the corresponding package resources are
available in a materialized graph.

For union variants, the discriminator field named by `UnionTagName` is filtered
before language-neutral structure comparison. Recursive references use a visited
pair map to terminate cycles.

### 11.3 Revision digest

`Revision.ContentSha256` has the form `sha256:` followed by 64 lowercase
hexadecimal characters. It covers the final schema only, after presentation
attributes have moved into the Display Template, so a template change never
changes it. Studio computes it by:

1. deep-copying the complete schema;
2. removing `Revision`;
3. normalizing and deterministically sorting the schema;
4. serializing canonical JSON with object keys sorted, undefined object members
   omitted, undefined array entries represented as `null`, and array order kept;
5. hashing the canonical JSON bytes encoded as UTF-8.

Indentation, CRLF/LF choice, and original object-key order therefore do not affect
the digest. String contents, attributes, IDs, and semantically ordered arrays do.

`Revision.SourceTimestampUtcMs` is the latest relevant source or project-manifest
mtime discovered by Studio. It is diagnostic and is not used to order unequal
schemas. Direction is derived from the last synchronized digest:

| Condition | Parity |
| --- | --- |
| Local digest equals Hub digest | `equal` |
| Local equals last synchronized digest | `hub-newer` |
| Hub equals last synchronized digest | `local-newer` |
| Neither equals the synchronized digest | `diverged` |
| Digests differ and no valid synchronized digest exists | `unknown` |

Both timestamp and digest are validated before parity comparison. A missing,
malformed, or stale digest requires regeneration.

## 12. Schematic example

The following omits `Revision` only to keep the example readable; Studio adds it
to generated deployment schemas.

```json
{
  "AppDependencies": [
    {
      "package": "libertas-shared_types",
      "Version": "20260824.131415.016.3"
    }
  ],
  "Types": [
    {
      "ID": 0,
      "NativeName": "Settings",
      "Type": "Table",
      "BuiltinType": true,
      "Children": [
        {
          "ID": 1,
          "NativeName": "choices",
          "Type": "List",
          "BuiltinType": true,
          "Children": [
            {
              "ID": 2,
              "NativeName": "",
              "Type": "String",
              "BuiltinType": true,
              "Children": [],
              "Attributes": [],
              "NativeType": "",
              "Namespace": ""
            }
          ],
          "Attributes": [],
          "NativeType": "",
          "Namespace": ""
        },
        {
          "ID": 3,
          "NativeName": "selectedChoice",
          "Type": "Number",
          "BuiltinType": true,
          "Children": [],
          "Attributes": [
            { "Name": "EnumSource", "Value": "$.choices" },
            { "Name": "NumberStep", "Value": "1" },
            { "Name": "NumberType", "Value": "u16" }
          ],
          "NativeType": "",
          "Namespace": ""
        }
      ],
      "Attributes": [],
      "NativeType": "",
      "Namespace": ""
    }
  ],
  "Functions": [
    {
      "NativeName": "configure",
      "Parameters": [
        {
          "ID": 0,
          "NativeName": "settings",
          "Type": "Settings",
          "BuiltinType": false,
          "Children": [],
          "Attributes": [],
          "NativeType": "",
          "Namespace": ""
        }
      ],
      "Attributes": [
        { "Name": "Permissions", "Value": "devices.read,devices.control" },
        { "Name": "AccessHost", "Value": "api.devices.example" }
      ]
    }
  ],
  "Attributes": [
    { "Name": "DefaultLocale", "Value": "en" },
    { "Name": "StringResources", "Value": "api.devices.example,devices.control,devices.read" }
  ]
}
```

Its English document can independently use natural text:

```json
{
  "DocumentVersion": "0.0.2",
  "Locale": "en",
  "DisplayName": "Device settings",
  "Description": "Configures device behavior.",
  "StringResources": {
    "api.devices.example": "Synchronize device capabilities with the vendor service.",
    "devices.read": "Read current device settings before showing or changing them.",
    "devices.control": "Apply the device settings selected by the user."
  },
  "List": [
    {
      "Name": "configure",
      "DisplayName": "Configure",
      "Description": "Applies the selected settings.",
      "List": [
        { "Type": 0, "Name": "Settings", "Description": "Settings to apply." }
      ]
    }
  ],
  "Types": [
    {
      "Name": "Settings",
      "DisplayName": "Settings",
      "Description": "Available configuration choices.",
      "List": [
        { "Type": 0, "Name": "Settings", "Description": "" },
        { "Type": 1, "Name": "Choices", "Description": "Available choices." },
        { "Type": 2, "Name": "Choice", "Description": "One available choice." },
        { "Type": 3, "Name": "Selected choice", "Description": "Index of the selected choice." }
      ]
    }
  ]
}
```
