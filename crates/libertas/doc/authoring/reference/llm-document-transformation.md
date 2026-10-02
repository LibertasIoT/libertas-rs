# LLM document transformation

LLM document transformation is also called **AI Import**.

A Control client can ask the Hub's configured LLM connector to transform one
or more documents into a JSON value. Each request has one response sent to the
Control session that submitted it. The response repeats the caller's unsigned
64-bit transaction ID, so a client can match concurrent requests without using
the text conversation history.

Clients offer the action on every modern Hub and hide it on legacy Hubs. A Hub
must handle the document transformation messages and have an OpenAI or Gemini
connector whose selected model accepts the supplied documents and structured
output schema. A modern Hub without the handler cannot complete the request;
the client reports a timeout if no response arrives.

## Request

`MSGCtrlLLMDocumentTransformReq` has these fields in wire order:

| Field | Type | Meaning |
| --- | --- | --- |
| `_transactionId` | unsigned 64-bit integer | Caller-chosen correlation ID, returned unchanged. |
| `_documents` | array of `LibertasLLMDocument` | One to eight documents. Each item has one `_bytes` value containing the full [LMB1 envelope](schema.md#mime-aware-bytes). |
| `_instruction` | string | Required, nonempty client task instruction. At most 16 KiB encoded as UTF-8. |
| `_validationJson` | string containing a JSON object | Facts the documents must confirm. Use `"{}"` when there are no facts. At most 64 KiB. |
| `_outputSchema` | string containing a JSON Schema object | Requested result shape for the model and client. At most 64 KiB. |

The combined encoded documents must not exceed 8 MiB. This operation accepts
LMB1 values with `application/pdf`, `image/png`, `image/jpeg`, or `image/webp`
MIME types. The MIME type must agree with the file signature. The LMB1 value
contains the media bytes; the client does not send a path or file URL.
The Hub allows up to four pending transformations and limits each provider
response body to 2 MiB.

The task instruction says **what** to extract or transform. The validation
JSON supplies values that must be checked against the documents. The schema
says **which JSON shape** the result must have. These are separate fields;
clients do not interpolate placeholder tags into the instruction.

For example, a client can use this instruction:

> Read the attached employment record. Extract the employee name exactly as
> shown. Do not guess missing or illegible values.

It can send `{"year":"2026"}` as the
validation JSON, and this output schema:

```json
{
  "type": "object",
  "properties": {
    "employee_name": { "type": "string" }
  },
  "required": ["employee_name"],
  "additionalProperties": false
}
```

Validation facts need not be returned as result fields. In this example, the
Hub asks the model to verify `year` from the document and returns only the
employee name when that check succeeds.

`_validationJson` and `_outputSchema` are serialized **strings** in the
Control message. The examples above show the JSON values before string
serialization. The client must preserve the 64-bit transaction ID without
rounding it through a floating-point number.

The Hub requires `_validationJson` and `_outputSchema` to parse as JSON objects.
It supplies the schema through the configured provider's structured-output
mode: OpenAI uses `json_schema` with `strict: true`, and Gemini uses a
schema-constrained JSON response format. OpenAI requires every object property
in `required` and `additionalProperties: false`; represent a genuinely
Nullable property with a required `type: [T, "null"]`. A schema outside a
provider's supported subset can cause a provider error. The Hub does not
interpret JSON Schema keywords or enforce the schema against the returned
result. The client must check a successful result against its supplied schema
before using it.

Nullable must reflect the form's actual data model. App authors must not make
a destination Nullable solely to accommodate document recognition.

## Response and validation

`MSGCtrlLLMDocumentTransformRsp` contains `_transactionId`, `_json`, and
`_error`. On model-reported success, `_error` is empty and `_json` is its
serialized JSON result. On failure, `_json` is empty and `_error`
explains the failure. There is no partial result. A malformed request,
unsupported media type, provider failure, or malformed model response envelope
also produces an error response when the Control session remains connected.

The Hub instructs the model to use the documents as evidence, to check every
validation value, and to report a conflict or unverifiable value as an error.
The Hub forwards the model's JSON result or error without independently
checking the result fields, output schema, or validation facts. Provider
schema mode constrains the supported structure; it does not establish that the
model read the document correctly. The client checks the result shape and
applies a valid result to the editable fields; the user can inspect or edit the
filled Request before submitting it.

The request does not read or change the client's text conversation. Document
transformations can be submitted concurrently and are correlated by the
transaction ID within the originating Control session.

## App Schema authoring

An App declares one document action on a named Table struct with
`LLMDocTrans`. Mark its direct result children with `LLMDocField`.
Each occurrence of that struct in an editable Request or SubscriptionRequest
gets its own **AI Import** action. A screen can therefore contain
several independent forms, including repeated rows of one form type. Selected
files are temporary editor input; they do not require a Bytes field in the App
Request. When a form's row is promoted or suppressed, its action follows the
[row action placement rule](presentation.md#actions-on-a-promoted-or-suppressed-row).
The action is unavailable in internal calculation requests and inside Hidden,
Fixed, CacheIdentity, or ReadOnly subtrees. ReadOnly on a List restricts its
membership, not editing an existing item. Studio checks the `validation` path
for every editable occurrence of the annotated Table; a missing or non-Table
target in any such occurrence is a schema error. Calculation-only and protected
occurrences do not need an otherwise unused validation fact.

| Argument | Meaning |
| --- | --- |
| `instruction` | Required App-authored task text. Tell the model what to read, what a genuinely blank box means, and when to report an error. The Hub sends it separately from facts and result schema; UTF-8 length is at most 16 KiB. |
| `validation` | Optional [Libertas Schema Path](schema.md#libertas-schema-path) to a Table value in the current Request. `$` is the Request root, `@` is this struct occurrence, and `^` follows actual value-tree parents. The Table's **children**, without its own name, become the validation JSON object. Omission sends `{}`. |
| `doc_type` | Optional [MIME filter](schema.md#mime-aware-bytes) for selected files. The effective source choices are its intersection with PDF, PNG, JPEG, and WebP; an empty intersection is invalid. Omission accepts all four. Prepared images are sent as JPEG even if selected as PNG or WebP. |
| `accept_multiple_inputs` | Optional Boolean, default `false`. One file is the normal case. Set `true` only when a logical document may span several selected files; the Hub limit is eight. |

`LLMDocField` has optional `allowEmpty`. Its default is `true` for a
non-Nullable Number and `false` for String, Boolean, and Enumeration. A marker
on a **direct child** of the annotated Table selects that result field. Within
each selected Table, including a List item Table, marking one or more direct
children selects **only those children** for that Table's result object. If
none of that Table's direct children is marked, all its unprotected children
are selected. Mark each intermediate composite whose descendants are selected.
A nested scalar marker also sets its `allowEmpty` rule.

A selected Union imports its variant choice, and a variant's payload fields
**only when they are marked**. The all-children default above does not apply to
Union payloads: a variant's payload usually holds facts the chosen option leads
to, not values printed on the document. Marking one or more direct variants of
a selected Union limits its result schema to those variants; with no marked
variant, all unprotected variants remain available, subject to ordinary choice
constraints. Marked variants must be capable of producing a result.

A variant's payload is either a Table or a single non-Table value. A Table
payload has the same meaning however it is declared (a Rust struct variant
`V { a, b }`, a Rust `W2(W2)`, or a TypeScript interface member): its fields
import only when marked, as above. A single non-Table value, such as a Rust
tuple variant `Allocated(i64)`, has no field to mark; that value is the
variant's one payload field, named by the variant, and it is **always marked**.
A Nil (unit) variant has no payload field.

- When at least one variant has a marked payload field, the result is a
  `variant`/`value` wrapper. `value` contains that variant's marked payload
  fields, following the Table selection rule inside them, and is an empty
  object for a variant with none marked.
- When no variant has a marked payload field, the Union is **variant-only**
  and is projected and converted exactly as an **Enumeration** of its
  available variant names, so a printed code can be imported without the model
  answering the facts that code leads to.

When the chosen variant equals the current one, the client keeps the variant
and every payload value the result does not supply, in its current state:
an unanswered field stays unanswered and is validated as it was before the
import, so it does not reject the proposal. A different variant is
applied exactly as if the user had chosen it in the editor: its unmarked
payload fields receive normal construction and initialization, and required
ones without a value stay unanswered. The proposal is not rejected for them.
The result is the same state a manual choice of that variant produces:
incremental validation runs as it does after that choice, and every rule a
manual choice would defer (mandatory values, final-only ValidationRules, and
cross-field rules that read an unanswered field) is deferred to Submit's final
validation, which requires them as usual. The editor shows the unanswered
fields for the user to answer.
At least one direct child of the annotated root Table must be marked. Normal editor construction supplies
the bound struct occurrence before the action runs.

**Every supported Libertas value type is supported in result subtrees**,
including nested Tables and Lists, Enumerations, Unions, Dates, money, dynamic
EnumSource, other semantic scalars, and their declared Nullable forms. Hidden,
ReadOnly, and Fixed remain ordinary field attributes. **Hidden and Fixed fields
are excluded from LLM output**, including their complete subtrees. Placing
`LLMDocField` directly on either is invalid. A selected composite may contain
them: omit those properties from the generated output schema and preserve their
normal values when updating the existing record. New List rows or Union payloads
receive them through normal field initialization, such as Fixed, Default, or
the existing construction rules. If a required value remains unavailable,
normal validation rejects the proposal, except for the unmarked payload fields
of a newly chosen Union variant, described above. The model never supplies it. ReadOnly
results retain their normal visible presentation.
For an existing Table or the same Union variant, omitted fields keep their
current typed values, including visible fields that were outside the result
schema. A newly selected Union variant initializes omitted payload fields by
the normal construction rules. An extracted List creates new rows through normal initialization; it must not
copy Hidden identities from old rows by position. A composite containing only
protected content is omitted, including a List with a protected item type.
Marking such a field directly is invalid; no null placeholders are generated.

The client derives standard JSON Schema recursively from the selected result
fields and only the types reachable from them: object properties, arrays and
items, enum choices, variant alternatives, null, and references for recursive
types. It does not send the App's full schema. Declared constraints still apply.
This feature adds no arbitrary Table
depth, List length, or integer-width restriction; actual schema and wire limits
apply, including the 64 KiB output-schema limit. A configured provider that
rejects a schema returns a normal request error.

The following TypeScript source illustrates independent W2 and W4 actions:

```typescript
interface W2Facts { year: string; }

/**
 * @LibertasLLMDocTrans(validation="@.expected", instruction="Read the W2 exactly. Check the supplied facts against the form. Return an empty string for a clearly blank tips box; return decimal numeric text for a legible amount. Report illegible values as errors.", doc_type="application/pdf,image/*")
 */
interface W2 {
    /** @LibertasHidden */
    expected: W2Facts;
    /** @LibertasLLMDocField */
    employee_name: string;
    /** @LibertasNumber(type=f64)
     * @LibertasDefault(0)
     * @LibertasLLMDocField */
    tips: number;
    /** @LibertasSize(max=16)
     * @LibertasLLMDocField */
    boxes: W2Box[];
}

interface W2Box {
    label: string;
    /** @LibertasNumber(type=f64)
     * @LibertasDefault(0) */
    amount: number;
}

/** @LibertasLLMDocTrans(instruction="Read the W4 exactly. Return an empty string for a clearly blank dependents box. Report illegible values as errors.") */
interface W4 {
    /** @LibertasNumber(type=u16)
     * @LibertasDefault(0)
     * @LibertasLLMDocField */
    dependents: number;
}

/** @LibertasRequest */
interface TaxRequest { w2: W2; w4: W4; }
```

The Rust spellings are `#[libertas_llm_doc_trans(instruction = "...",
validation = "@.expected", doc_type = "image/*",
accept_multiple_inputs = true)]` on the struct and
`#[libertas_llm_doc_field(allow_empty = true)]` on a direct field. Omit the
field argument to use its type-dependent default. A marker on a nested scalar
inside a selected composite overrides only that scalar's blank rule. Rust and TypeScript emit the
same portable Schema attributes.

The selected validation Table may be Hidden and may receive its value through
normal Request construction, CopyFrom, or EditRequest. Its children may be
complete typed values using the same recursive JSON projection below. An absent
or incomplete required value blocks the action. Declared Nullable facts retain
their null meaning. Fact
names need no relationship to result names: the W2 example verifies `year`
without returning `year`. The model checks those facts against the document;
the Hub does not independently verify them.

### Blank values and output types

Every selected result property, including each selected property in a nested Table, is
required in the generated strict output JSON Schema; Hidden and Fixed subtrees
are omitted. A List is a JSON array
whose item schema is derived recursively. Every Table object has exactly its
selected children and `additionalProperties: false`; unselected children do
not appear in the model output. The client checks List
length against its Schema bounds, converts every item, and replaces the selected
List as one typed subtree. It merges selected Table fields
into the existing Table or same Union variant as one editor change.
A non-Nullable Number
normally uses a **JSON string** result because
the form's box might be blank even though the App's data field is required.
The model returns decimal numeric text for a legible amount and exact `""`
for a clearly blank box. The client strictly converts nonempty text to the
destination Number type. Exact `""` applies the field's effective, typed
Schema `Default`. Thus `tips: number` with `@LibertasDefault(0)` stays required
and stores zero for a blank tips box. The schema sent to the model does not
contain a JSON Schema `default` keyword or expose the App's Default value.

`allowEmpty=true` requires a real Schema `Default` and a non-Nullable scalar
whose document representation is a JSON string. An implicit empty initialization or
localized DefaultText does not count. A String with default `allowEmpty=false`
requires a nonempty result. Boolean results use JSON Boolean and cannot use
`allowEmpty=true`. Composite and Nil fields do not use `allowEmpty`. Fixed
fields are outside the model's result. An Enumeration normally uses a JSON string constrained to
its canonical member names. The client maps an exact member name to its
declared `Fixed` integer value, or to its ordinal when no `Fixed` is declared.
The model never supplies a localized display label or an arbitrary integer.
Explicit `allowEmpty=true` also permits exact `""` and applies the effective
Enumeration Default. If a Number has no meaningful Default, an App author may
set `allowEmpty=false` only when the form establishes that its box cannot be
blank; its output then uses JSON `integer` or `number` as appropriate and a
blank box is an error. Otherwise, leave that field out of automatic extraction
until its blank semantics are known. Do not invent a zero Default for this
purpose.

An Endpoint's process-provisioning `Default` does not supply an Endpoint ID, so
Endpoint results cannot enable `allowEmpty=true`.

Nullable remains a domain-model choice, never a document-recognition trick.
CodeBots must not make a field Nullable solely to support this action. A truly
Nullable result permits JSON `null` for a genuinely blank value; `null` clears
the destination. A non-null Nullable Table uses its selected child object shape;
a non-null Nullable List uses its selected item shape. A Nullable Number uses JSON string or null and converts its
non-null numeric text strictly. Nullable Enumeration uses member name or null.
`allowEmpty` is inapplicable to Nullable fields.

For ordinary optional text whose domain meaning is “blank when not supplied,”
use a non-Nullable String with an empty Schema Default and explicit
`allowEmpty=true`. In Rust, combine `#[libertas_default("")]` with
`#[libertas_llm_doc_field(allow_empty = true)]`; the bare field marker still
defaults to `false` for String. Permit length zero in its size bounds. Describe
the manual answer as “Leave it empty if absent” and tell the model to return
exact `""` for genuinely missing text. Reserve JSON `null` for truly Nullable
destinations. Unreadable text is an error, not evidence that the text is absent.
Follow the [type-selection guidance](app-contract.md#user-facing-and-non-user-facing-types)
when manual input and an internal record use different representations.

The client accepts numeric text only when the entire string is a JSON number
lexeme, with no surrounding whitespace, grouping separators, currency sign,
`NaN`, or infinity, and checks the destination type and bounds. Conversion
failure is an error; it never applies the Default. A whitespace-only String is
invalid. The client converts every supported scalar, Table, List item, and
declared null into the destination's Libertas type, checks the generated JSON
schema, then runs normal typed field and cross-field validation before offering
any change. It rejects any partial, extra, mismatched, or unconvertible result.

### Complete JSON projection and client conversion

JSON Schema describes the JSON representation. The client converts that
representation into the destination's actual Libertas type with its ordinary
value codecs and validation rules; JSON primitive types do not replace the
App's semantic types.

**Every generated schema node carries a `description`**, except a `$ref`
node. It is built from the node's Display Name and Description, the same text the
user reads, so the model maps what is printed on the document to the right
property. A `$ref` node carries no sibling keyword, because structured-output
providers reject keywords beside `$ref` (OpenAI does); the referenced
definition in `$defs` carries its type's `description` instead. A value whose choices
are listed by `enum` (an Enumeration, a variant-only Union, or dynamic
EnumSource) also describes every choice in that `description`: each canonical
name followed by its Display Name and Description, for example
`DirectRollover: G — direct rollover`.

| Libertas value | Model JSON and client conversion |
| --- | --- |
| String | JSON string; preserve text, applying the declared blank rule. |
| Boolean | JSON Boolean; no string truthiness or numeric coercion. |
| Number | Decimal numeric string when blank conversion or exact wide-integer representation requires it; otherwise JSON integer/number. Parse the complete value, preserve integer precision, and enforce native width, range, and step. |
| DateOnly | `YYYY-MM-DD` string; validate a Gregorian date and convert to the ordinary `yyyyMMdd` typed value. |
| DateTime | RFC 3339 string with an explicit offset; convert to the ordinary epoch-second instant without guessing a timezone. |
| TimeOnly | `HH:mm:ss` string; convert to seconds from local midnight under the normal TimeOnly rules. |
| Money | Decimal major-unit string in the declared currency; convert exactly to integer minor units using the declared scale. Reject excess fractional precision and overflow. |
| TimeInterval | Decimal seconds string; convert to the declared numeric duration type and bounds. |
| Enumeration | Canonical member-name string constrained by `enum`, with every member's Display Name and Description in the `description`; map to its declared Fixed value or ordinal. |
| Dynamic EnumSource | Canonical decimal index string constrained by `enum` to the currently permitted indices. Include the corresponding current choice values in schema descriptions. Resolve the returned index through that same choice snapshot and normal constraint rules. |
| Bitflags | Decimal mask string with the declared catalog described in the schema; convert exactly and reject undeclared bits. |
| User, Device, LanDevice, VirtualDevice, Endpoint | Decimal identity string; convert without precision loss and apply the ordinary identity/selector constraints. Include the current permitted candidates and their meanings when the editor provides a choice set. |
| Bytes | Base64 string; decode to Bytes and enforce ordinary byte constraints, including LMB1 or formatted-value rules when annotated. |
| Table | JSON object with every selected child required and no extra properties; recursively convert children while preserving omitted values in an existing Table or unchanged Union variant. |
| List | JSON array; recursively convert every element and enforce its declared size and element constraints. |
| Union with a marked payload field | An `anyOf` of variant objects, each with required `variant` and `value` properties and no extras. `variant` is that branch's canonical name; `value` holds that variant's marked payload fields (an empty object when it has none). Convert through the ordinary Union discriminator and payload codec. |
| Union with no marked payload field (variant-only) | As an Enumeration of its available variant names. Keep the current variant and payload when it is unchanged; otherwise select it as the editor does, with normal payload initialization. |
| Nil | JSON null. |
| Nullable | Add null to that value's JSON schema; a returned null clears the typed value. Never add Nullable merely for recognition. |

Physical units and display formats retain their ordinary semantics; they do
not change the underlying value type. Hidden and Fixed values never enter the
model's output schema or its returned result; normal initialization and
validation continue to govern them.
Number annotations such as DateOnly, Money, TimeInterval, EnumSource, and
Bitflags keep the Number default of `allowEmpty=true`; their typed Default is
applied locally for exact `""`. Other semantic scalar types default to
`allowEmpty=false` and may explicitly enable it with a meaningful Default.

Dynamic EnumSource is resolved from the bound Request when the request is
prepared. Its current choices, allowed-choice constraints, and relevant
reference values belong to the pending transaction snapshot. If they change
before application, discard the result as stale. A choice cannot
be silently reinterpreted against a newer list.

### Array import

A file often holds several forms of one type: separate forms for several
accounts, or a consolidated statement. When the annotated struct occurrence
lies anywhere inside an editable List item, directly or through nested Tables
and Union payloads, its **AI Import** action imports an array. The struct type
of the occurrence the action is on decides what is imported: only forms of that
type.

- The output schema is a JSON object with one required property, `forms`: an
  array whose every element is that struct's selected result object, as for a
  single import, with at least one element. The root stays an object because
  structured-output providers (OpenAI) require one.
  The client appends to the instruction that the model returns one element per
  distinct form in the files, that the files may contain duplicate copies of a
  form which count once, and ignores forms of other types and pages that are
  not forms.
- The first element replaces the occurrence the action is on, exactly as a
  single import does.
- Each further element is **appended** as a new item of the nearest enclosing
  List. The new item is constructed by normal initialization; every Union on
  the path from that item down to the struct selects the same variant as in the
  current item, as a manual choice does, and the element is then applied to the
  new occurrence as a single import. Other fields of the new item (for example
  its owner) keep their normal initialization. Existing items are never
  changed or removed.
- The App's `instruction` names what a duplicate looks like for its form (for
  example a tax form's Copy B, Copy C and Copy 2 printed on one page); the
  client's appended text stays generic.
- Duplicates are dropped: a file may hold several copies of one form, though
  not always. After conversion the client
  drops an element to be appended whose imported values exactly equal another
  element of the same result, or those of an existing item at the same struct
  position (the same file imported again). The review states how many
  duplicates were skipped.
- Each element is converted and validated like a single result, and the whole
  proposal is one editor change: if any element fails, or the appended items
  would exceed the List's declared size, nothing is applied.
- A struct occurrence outside any List, or inside a List whose membership is
  ReadOnly, keeps the single import.

## Client workflow

The **AI Import** action (Chinese: **AI导入**) first offers **Take a picture**
when the effective MIME filter accepts camera JPEG and a camera is available,
and **Open a file**. Each action selects one file by default, or up to eight in
user-selected order when enabled. Captured photos use the same preparation and
validation as selected image files. Clients decode and orient selected images,
flatten transparency on white, and JPEG-encode them at no more than
**1,000,000 content bytes each**. If a readable image cannot fit, the client reports an input
error. Downsampling keeps at least 2200 pixels on the longest side when the
source has that resolution, and JPEG quality stays at or above 70; a smaller
source is never upscaled. PDF bytes remain unchanged. Clients check the combined LMB1 limit of
8 MiB before sending.

The client uses a session-unique unsigned 64-bit transaction ID. A pending
action belongs to its originating Control session, editor, struct occurrence,
selected files, validation value, and result-field state. A cancel, timeout,
navigation, or disconnect, or a change to a relevant value while it is pending
(for example an arriving calculation result), makes its reply stale. Only the matching
pending reply can change the editor.

AI Import is blocking. While its transformation is pending, the client shows a
busy overlay over the whole editor with a **Cancel** action; the editor and its
actions accept no input, so no edit can make the pending reply stale and no
second transformation or Submit can start. The busy state ends, and the editor
accepts input again, when the matching reply arrives (a result is applied as
above; an error is reported), when the user cancels, when the timeout elapses,
or when the Control session disconnects. Cancel, timeout, and disconnect
discard the pending action, so a late reply is ignored; an error, a timeout, or
a disconnect is reported to the user, who can retry. Navigating away from the
editor also discards it. On success, the client checks the JSON
shape, strictly converts each value, validates the complete candidate, and
applies all selected fields in one normal editor change, including values
supplied by a Schema Default. Ordinary drafts and calculations remain active.
AI Import does not submit the Request. An error or invalid result leaves the
editor unchanged. The user can review and correct the filled fields before
submitting the Request.
