# Libertas source annotation reference

This is the canonical detailed reference for public source annotations accepted
by the current Libertas schema parsers. Use only the canonical spellings shown
here. Compiler acceptance alone does not establish supported schema syntax.

The codebot kit carries an
[exhaustive compact projection](../codebot/schema_annotations.md)
of this reference. This page owns the detail; the projection exists to minimize
routine App-generation context.

The language parser owns source spelling, native name and constant resolution,
and extraction. Studio's shared validator owns placement, effective host type,
value grammar, conflicts, protocol reachability, and other portable semantics.
The placement column below states the public authoring placement; it is not
permission to move an annotation merely because a parser can preserve it for a
later Studio error.

Read the [App contract](app-contract.md) first. Use the
[TypeScript](languages/typescript.md) or [Rust](languages/rust.md) guide for project structure, and
follow each row's topic link for the complete behavior and value vocabulary.
Every annotated declaration and nested schema-visible node must also follow the
[schema documentation invariants](documentation.md), including the
user-friendly label/description, member, variant, and List-item rules.

The Hub supplies [AI tool-call representation hints](presentation.md#ai-tool-call-descriptions)
from semantic attributes and [renders FormattedText results](presentation.md#ai-tool-call-output).
Source comments supply domain meaning without duplicating those generated notes.
Rows marked "compiled into the Display Template" are presentation attributes:
Studio validates them, compiles them into the package's
[Display Template](presentation.md#display-template) as its lowest-specificity
rules, and omits them from the final schema. A `display.json` rule can change
their effect per device without editing source.
This catalog exhausts schema-parser annotations, not unrelated SDK runtime
functions or serialization derives such as `LibertasAvroEncode`,
`LibertasAvroDecode`, `VariantIndex`, or `variant_index!`; those remain covered
by the public SDK and the language guides.

## Package, discovery, and parser helpers

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `/// @LibertasDefaultLocale("en-US")` | `//! #[libertas_default_locale("en-US")]` | Leading package/crate documentation in the selected entry file | Emits package `DefaultLocale`. Omission means `en`; all extracted source documentation supplies this locale's default document. | [Documentation and localization](app-contract.md#documentation-and-localization) |
| `/// @LibertasTypesOnly` | `//! #[libertas_types_only]` | Leading package/crate documentation | Emits `TypesOnly=true`. The package has no App functions and publishes every non-ignored exact-name public root type. | [Project and discovery rules](app-contract.md#project-and-discovery-rules) |
| `/// @LibertasStringResources(NAME)` | `//! #[libertas_string_resources(NAME)]` | Leading package/crate documentation | Resolves exactly one package-owned LMF1 catalog and emits its sorted keys as `StringResources`. The identifier is positional and unquoted. | [String resources and formatted output](presentation.md#string-resources-and-formatted-output) |
| `@LibertasIgnored` | Exact outer-rustdoc line `/// #[libertas_ignored]` | Public function or public named type | Omits the declaration as a schema root. A published declaration may not reach an ignored type. It takes no arguments. | [Project and discovery rules](app-contract.md#project-and-discovery-rules) |
| `@LibertasExport` | `#[libertas_export]` on a public function; `#[derive(LibertasExport)]` on a schema struct/enum | Parser/compiler helper | Enables the language's public App wrapper or helper attributes. It emits no portable `Export` attribute and does not replace the native public/export requirement. | [TypeScript guide](languages/typescript.md#supported-jsdoc-decorators), [Rust guide](languages/rust.md#schema-attributes-and-field-presentation) |
| Not applicable | Exact Union rustdoc line `/// #[libertas_export(content = "payload")]` | Rust Union declaration | Emits parser-owned `UnionContentName` representation metadata. Do not invent `tag` or `untagged` forms. | [Rust guide](languages/rust.md#schema-attributes-and-field-presentation) |
| File-leading `/** @LibertasConstEnum */` | Not applicable | Dedicated TypeScript file containing one documented exported numeric constant per statement | Treats the constants as members of the numeric Enumeration exposed by a documented exported `typeof` namespace-import alias in the App entry module. | [TypeScript enumerations](languages/typescript.md#enumerations) |

## Function-level task declarations

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasSingleton` | `#[libertas_singleton]` | Public App function | Emits `Singleton=true`; at most one task for that App function may exist on a Hub. | [Functions, permissions, and singleton tasks](app-contract.md#functions-permissions-and-singleton-tasks) |
| `@LibertasDataErrorStatus` | `#[libertas_data_error_status]` | Public App function | Enables checked Data API status returns; omission keeps Hub-owned termination. | [Data failure contract](protocols.md#explicit-error-status-opt-in) |
| `@LibertasPermissions(NAME)` | `#[libertas_permissions(NAME)]` | Public App function, or a server Endpoint field | Resolves one positional constant containing permissions and emits `Permissions`. Function permissions require nonempty same-key package string-resource reasons. Server Endpoint permissions are downstream requirements. | [Function and Endpoint permissions](protocols.md#function-and-endpoint-permissions) |
| `@LibertasAccessHost("api.example.com,{network_access_local}")` | `#[libertas_access_host("api.example.com,{network_access_local}")]` | Public App function only | Emits `AccessHost`. Every concrete DNS name or reserved local/external grant requires a nonempty same-key package string-resource reason. TCP and UDP are otherwise denied. | [App network access contract](network-access.md) |
| `@LibertasDataSchema(StoredData)` | `#[libertas_data_schema(StoredData)]` | Public App function only | Resolves the required persistent-data Union type argument by local simple name or external App FQN and emits `FunctionDataSchema=StoredData`. Studio's shared validation marks the resolved Union with `LibertasData=true`. Both languages associate persistence on the function; ordinary native Union declarations remain unchanged. A bare `@LibertasDataSchema` on a Union is unsupported; neither language uses a standalone Union marker. | [Persistent data schemas](protocols.md#persistent-data-schemas) |
| `@LibertasValidationRules('expression : "KEY";')` | `#[libertas_validation_rules(r#"expression : "KEY";"#)]` | Public App function, or any schema data node (including fields, variants, and named types) | Emits semantic `ValidationRules`; field-local, inherited-field, and referenced named-type rules are cumulative and do not replace final structural/mandatory checks. Every pure Boolean expression uses `expression : "KEY";` with a quoted package `StringResources` error-message key. A function rule sees one synthetic Table containing all arguments. The language includes typed first-non-null fallback and no general truthiness. `FinalOnly=true;` or `FinalOnly=false;` is an optional leading directive inside this string, never a separate annotation; omission means `true`. | [ValidationRules](validation-rules.md#1-purpose) |

No other portable schema attribute is public on a function. In particular,
field presentation, protocol-role, chart, and Matter annotations do not become
function annotations.

## Persistent-data Union declaration

Use the DataSchema entry in [function-level task declarations](#function-level-task-declarations).
The [persistent-data contract](protocols.md#persistent-data-schemas) gives the
complete usage rules.

## Defaults, constraints, and ordinary presentation

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasDefault(value)` | `#[libertas_default(value)]` | Schema data field/type | Emits editable initial `Default` data, including explicit `User`/`List<User>` identity selectors and, on a Union field, the native name of the variant initialization selects. On an Endpoint client, the value instead recommends a `developer-package::function` task process when no eligible server is installed. | [User and group defaults](schema.md#user-and-group-defaults), [Common field metadata](presentation.md#common-field-metadata), [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasFixed(value)` | `#[libertas_fixed(value)]` | Schema data field/type | Emits one fixed encoded value. On an Endpoint client, the value instead is a `developer-package::function` task-process identity: the Endpoint selector remains editable, compatible servers are restricted to that process, and its package becomes a dependency. Endpoint `Fixed` cannot coexist with `Default`. Native enum declarations generate member fixed values; Union declarations generate separate Discriminator values. On `LibertasUser`, Fixed is a [Hub-enforced security constraint](schema.md#fixed-current-user-security-constraint); only `current-user` is valid, at a request root or within nested request values outside Lists, optionally Hidden. | [Schema attributes](schema.md#43-value-constraint-and-presentation-attributes), [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasNumber(type=u32, min=..., max=..., step=...)` | `#[libertas_number(min=..., max=..., step=...)]` on a concrete numeric type | Number or DateTime field/type | Emits NumberType and numeric constraints. TypeScript requires `type=`: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f32`, or `f64`. DateTime requires `u64`. Rust derives type from its native declaration and does not accept a public `type=` parameter. Integer step defaults to 1 when omitted. | [Type model](app-contract.md#type-model), [TypeScript annotations](languages/typescript.md#supported-jsdoc-decorators), [Rust annotations](languages/rust.md#schema-attributes-and-field-presentation) |
| `@LibertasSize(min=..., max=...)` | `#[libertas_size(min=..., max=...)]` | String, Bytes, or List field/type | Emits inclusive nonnegative `SizeMin`/`SizeMax`: List element count, String UTF-16 code-unit length, or Bytes byte count. Empty values have size zero; nullable null is handled separately. Tables are not valid hosts. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasUnique` or `@LibertasUnique(level)` | `#[libertas_unique]` or `#[libertas_unique(level)]` | Scalar field inside one or more ancestor Lists | Emits `Unique`; omitted level means `99`, and a positive level selects the ancestor List scope. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasUnordered` | `#[libertas_unordered]` | List field/type | Emits `Unordered=true`; List order has no semantic meaning. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasSuppressLabel` | `#[libertas_suppress_label]` | Scalar, struct, or Union field/type; List field/type whose member is a Table | Compiled into the [Display Template](presentation.md#display-template) as `suppress_label`: omit the selected row’s visual label while preserving its value and controls; on a List omit the List-contributed member name and position, preserving its own heading. | [Label-free value rows](presentation.md#suppresslabel-label-free-value-rows) |
| `@LibertasFoldable(initially_open=true)` | `#[libertas_foldable(initially_open = true)]` | Table, Union, or List type or field | Compiled into the [Display Template](presentation.md#display-template) as `fold`. Allows manual folding; unmarked containers remain expanded. The optional Boolean defaults to true; false starts the value collapsed without disabling folding. | [Folding](presentation.md#folding-structs-and-unions) |
| `@LibertasInlineChoices` | `#[libertas_inline_choices]` | Enumeration or Union field or type declaration; not a Bitflags catalog | Compiled into the [Display Template](presentation.md#display-template) as `inline`. Clients show the permitted choices inline as radio controls after ConstrainedBy filtering, with a Union's selected payload beneath the row. A marker on either the field or its type enables it. | [Inline choices](presentation.md#inline-choices) |
| `@LibertasUIIncludeDescription` | `#[libertas_ui_include_description]` | Schema data field/type | Compiled into the [Display Template](presentation.md#display-template) as `include_description`. Clients show the localized long description inline together with the short display name for that UI node. It does not change data or wire encoding. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasReadOnly` | `#[libertas_read_only]` | Schema data field/type | Emits `ReadOnly=true`; clients present the value but do not offer ordinary editing. Every non-user writer (initialization, CopyFrom, EditRequest, calculation CopyTo, draft restoration) still writes it. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasHidden` | `#[libertas_hidden]` | Schema data field/type | Emits `Hidden=true`; the field remains typed transaction/context data but is hidden from ordinary visualization and editing. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasTimeInterval` | `#[libertas_time_interval]` | Numeric field/type | Emits `TimeInterval=true`; the raw number is a duration in seconds. | [Date and time](presentation.md#date-and-time) |
| `@LibertasDateOnly` | `#[libertas_date_only]` | Explicit integer Number field/type | Emits `DateOnly=true`; every value is a valid decimal Gregorian `yyyyMMdd`, never an epoch or timezone conversion. | [Date and time](presentation.md#date-and-time) |
| `@LibertasFormat("#,##0.00")` | `#[libertas_format("#,##0.00")]` | Number, DateOnly, DateTime, or TimeOnly field/type | Compiled into the [Display Template](presentation.md#display-template) as `format`: one programming-language-neutral Unicode LDML pattern; it changes display only. | [Display formats](presentation.md#display-formats) |
| `@LibertasMoney("USD", 2)` | `#[libertas_money("USD", 2)]` | Signed `i64` Number field/type | Declares exact integer minor units, explicit currency and fixed scale; enables localized currency display and inline entry. | [Money](presentation.md#money) |
| `@LibertasPhysicalUnit("millimeter-per-day")` | `#[libertas_physical_unit("millimeter-per-day")]` | Number field/type | Emits a canonical CLDR core `PhysicalUnit`; it declares the supplied unit and performs no conversion. | [Physical units](presentation.md#physical-units) |
| `@LibertasBitflags` or `@LibertasBitflags(Access)` | `#[libertas_bitflags]` or `#[libertas_bitflags(Access)]` | Bitflag catalog declaration, or integer field selecting that catalog | Emits `Bitflags=true` on the Enumeration catalog or `Bitflags=Access` on an ordinary integer. Values remain integer masks. | [Bitflags](presentation.md#bitflags), [TypeScript Bitflags](languages/typescript.md#bitflag-catalogs-and-integer-fields), [Rust Bitflags](languages/rust.md#bitflag-catalogs-and-integer-fields) |
| `@LibertasEnum(A, B, C)` | `#[libertas_enum(A, B, C)]` | Applicable String alias/field or Enumeration declaration supported by that language | Defines a catalog using positional members. The parsers currently cannot attach source documentation to those fabricated members, so codebots must use a documented native numeric enum instead and must not generate this otherwise public compatibility form. TypeScript `values=[...]` is not canonical syntax. | [TypeScript enumerations](languages/typescript.md#enumerations), [Rust attributes](languages/rust.md#schema-attributes-and-field-presentation) |
| `@LibertasEnumSource("$.choices")` | `#[libertas_enum_source("$.choices")]` | Unsigned integer field/type | Emits `EnumSource`; the integer stores the zero-based index of an item in the resolved same-tree List. | [Common field metadata](presentation.md#common-field-metadata) |
| `@LibertasFormattedText` | `#[libertas_formatted_text]` | Statically ReadOnly schema data node with effective Bytes type | Emits `FormattedText=true`. The canonical LMF1 tuple follows the owning contract's static placement, request-value sources, and runtime-use restrictions. | [String resources and formatted output](presentation.md#string-resources-and-formatted-output) |
| `@LibertasMimeTypes("image/png,image/jpeg")` | `#[libertas_mime_types("image/png,image/jpeg")]` | Bytes field or named type outside CacheIdentity | Emits a MIME filter for the [LMB1 Bytes contract](schema.md#mime-aware-bytes). Its source grammar, content-size behavior, and incompatibility with FormattedText are validated by Studio. | [MIME-aware Bytes annotation](#mime-aware-bytes-annotation) |
| `@LibertasLLMDocTrans(instruction="...", validation="@.facts", doc_type="image/*", accept_multiple_inputs=true)` | `#[libertas_llm_doc_trans(instruction = "...", validation = "@.facts", doc_type = "image/*", accept_multiple_inputs = true)]` | Named Table struct declaration used in an editable Request or SubscriptionRequest | Declares one per-occurrence document action. Instruction is required; validation path, MIME filter, and multiple-file choice are optional. Direct marked children select scalar or nested composite results, excluding Hidden and Fixed subtrees. | [Document Schema authoring](llm-document-transformation.md#app-schema-authoring) |
| `@LibertasLLMDocField` or `@LibertasLLMDocField(allowEmpty=false)` | `#[libertas_llm_doc_field]` or `#[libertas_llm_doc_field(allow_empty = false)]` | Direct result child of an LLMDocTrans Table, or a field within a selected composite | Selects any supported Libertas value, including complete nested Tables, Lists and Unions; nested scalar markers override blank conversion. Non-Nullable Number defaults `allowEmpty` to true; other values default false. True requires a typed Schema Default and a string projection, mapping exact `""` to that Default. A Union imports a variant's payload fields only when marked; with none marked in any variant it imports the variant name alone. An unchanged variant keeps its payload; a new one is initialized as an editor choice, required unmarked fields left unanswered. | [Complete conversion contract](llm-document-transformation.md#complete-json-projection-and-client-conversion) |
| `@LibertasConstrainedBy("$.allowed_choices")` | `#[libertas_constrained_by("$.allowed_choices")]` | Enumeration or Union request field; Response variant | A hidden integer list, at a root path or (field form) a parent-relative `^` path such as a row's own sibling, constrains field choices or the response NextRequest actions; the list is written by CopyFrom, EditRequest or a calculation CopyTo, never by the user. | [Backend-constrained choices](protocols.md#backend-constrained-choices) |
| `@LibertasActionControl("sections")` | `#[libertas_action_control("sections")]` | Hidden direct integer-list field of a displayed Response or SubscriptionData payload with NextRequest | Declares a named control whose current Request membership is the field's original protocol Union discriminants; `Actions` restricts the built-in button. An empty array hides its control, and malformed arrays reject the update. | [Named action controls](protocols.md#named-action-controls) |
| `@LibertasCopyFrom("$.first,$.fallback")` | `#[libertas_copy_from("$.first,$.fallback")]` | Protocol Request/SubscriptionRequest data | Emits source paths under the [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules); the [context table](schema.md#context-and-syntax) selects the reference context and grammar. | [Shared CopyFrom and CopyTo rules](schema.md#shared-copyfrom-and-copyto-rules) |
| `@LibertasRangeMin("expression")` | `#[libertas_range_min("expression")]` | Number or DateTime field/type | Emits `RangeMin`; one Data Validation bound expression compatible with the host type supplies an inclusive live lower bound for GUI and final validation. Incompatible result types are errors; null means no bound. | [Dynamic numeric ranges](validation-rules.md#31-dynamic-numeric-ranges) |
| `@LibertasRangeMax("expression")` | `#[libertas_range_max("expression")]` | Number or DateTime field/type | Emits `RangeMax`; one Data Validation bound expression compatible with the host type supplies an inclusive live upper bound for GUI and final validation. Incompatible result types are errors; null means no bound. | [Dynamic numeric ranges](validation-rules.md#31-dynamic-numeric-ranges) |

Attribute combinations and inherited effective metadata are validated by
Studio. Do not infer that two annotations can coexist merely because they have
separate rows in this catalog.
The [ValidationRules entry](#function-level-task-declarations) also covers
data fields, variants, and named types.

## Endpoint and protocol annotations

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasEndpointSchema(Protocol)` | `#[libertas_endpoint_schema(Protocol)]` | `LibertasEndpoint` field | Resolves a local or external Union and emits `ProtocolSchema`. The Endpoint's absence/presence of the server marker determines its side. | [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasEndpointServer` | `#[libertas_endpoint_server]` | `LibertasEndpoint` field | Emits `EndpointServer=true`. Without it the Endpoint is a client. | [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasEndpointBaseObjects("$.device,$.room")` | `#[libertas_endpoint_base_objects("$.device,$.room")]` | Endpoint field | Emits comma-delimited `BaseObjects` paths that bind the Endpoint instance to configuration objects. | [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasRequest` | `#[libertas_request]` | Protocol Union variant | Emits `Request=true` for one-shot client-to-server input. | [Protocol roles](protocols.md#protocol-roles) |
| `@LibertasResponse` | `#[libertas_response]` | Protocol Union variant | Emits `Response=true` for a correlated server-to-client result. | [Protocol roles](protocols.md#protocol-roles) |
| `@LibertasError` | `#[libertas_error]` | Protocol Union variant that also has an effective Response role | Emits `Error=true`; clients handle the response out-of-band of the current state transition, preserve the current interaction, and keep retry available. | [Protocol roles](protocols.md#protocol-roles) |
| `@LibertasSubscriptionRequest` | `#[libertas_subscription_request]` | Protocol Union variant | Emits `SubscriptionRequest=true` for stream establishment or renewal. | [Protocol roles](protocols.md#protocol-roles) |
| `@LibertasSubscriptionData` | `#[libertas_subscription_data]` | Protocol Union variant | Emits `SubscriptionData=true` for server-to-client stream data. | [Protocol roles](protocols.md#protocol-roles) |
| `@LibertasAccessPrivilege("Read,Write")` | `#[libertas_access_privilege("Read,Write")]` | Request or SubscriptionRequest variant | Emits the alternative-access bitmap (`Read=1`, `Write=2`, both=`3`). Omission also means both; mutating operations must explicitly require Write. | [Request access](protocols.md#request-access) |
| `@LibertasCalculationRequest("A,B")` | `#[libertas_calculation_request("A,B")]` | Protocol request data field | Emits a field's calculation variant list; Studio checks bidirectional agreement with those requests' input `CopyFrom` paths; [invariant inputs](protocols.md#invariant-calculation-inputs) need not participate. | [Field calculation requests](protocols.md#field-calculation-requests) |
| `@LibertasCopyTo("$.orderTotal")` | `#[libertas_copy_to("$.orderTotal")]` | Successful calculation or link-return Response payload field; reduced SubscriptionData variant in the optional App pattern | Emits destination paths under the [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules). [Calculation results](protocols.md#applying-calculation-results), [link returns](protocols.md#link-return), and [reduced subscriptions](protocols.md#reduced-subscriptiondata-design-pattern) define application and lifetime. | [Shared CopyFrom and CopyTo rules](schema.md#shared-copyfrom-and-copyto-rules) |
| `@LibertasAppendTo("Entries.entries")` | `#[libertas_append_to("Entries.entries")]` | Incoming SubscriptionData root array field | Appends ordered complete elements to one admitted working variant's root array. Preserves duplicates; rejects conflicting writers and invalid resulting values atomically. No name-based replacement for annotated fields, including when another variant is named. Requires an initial snapshot. | [Array append](protocols.md#subscriptiondata-array-append) |
| `@LibertasAppendIndex("$.start_index")` | `#[libertas_append_index("$.start_index")]` | Same array field as AppendTo | Excludes the referenced control field from ordinary replacement and CopyTo; optional non-null integer report field checks the exact destination length before append, including empty appends; negative/mismatched positions reject the whole report. The App owns ordering, deduplication, retention, and initial snapshots. | [Array append](protocols.md#subscriptiondata-array-append) |
| `@LibertasDefaultRequest` | `#[libertas_default_request]` | Request variant whose payload needs no user input (Nil, or only hidden/read-only Fixed or Default fields) | Optionally selects the single explicit correlated request for endpoint entry; chart-producing requests are forbidden. | [Default entry](protocols.md#default-request-entry-and-dynamic-actions) |
| `@LibertasNextRequest("A,B")` | `#[libertas_next_request("A,B")]` | Protocol Union variant | Emits the allowed next client variant native names as `NextRequest`. Outside workflows, omission of both NextRequest and WorkflowRequest is a non-transition acknowledgement; workflow completion can be terminal. | [Chaining, caching, and copied values](protocols.md#chaining-caching-and-copied-values) |
| `@LibertasWorkflowRequest("EditProfile")` | `#[libertas_workflow_request("EditProfile")]` | Response variant | Declares the `WorkflowRequest` semi-role. | [WorkflowRequest semi-role](protocols.md#workflowrequest-semi-role) |
| `@LibertasPrevRequest("ReopenProfile,ReopenAddress")` | `#[libertas_prev_request("ReopenProfile,ReopenAddress")]` | Request variant | Lists ordinary Back actions; their CopyFrom source is the current editor and each action must independently validate before sending. Their controls are suppressed in an embedded master-detail trailing pane under the pane contract. | [Workflow interactions](protocols.md#workflow-interactions) |
| `@LibertasLink("OpenTopic")` | `#[libertas_link("OpenTopic")]` | Any field; Table, Union, or Enumeration type declaration | Emits `Link` naming one Request variant of the same protocol Union. Tapping the displayed node opens that Request, pre-filled through its CopyFrom with the tapped node or host field bound; from a request editor a plain reply returns through CopyTo. The field or type declaration enables it; the field's target wins. Displayed-value targets must be in that variant's NextRequest; Hidden occurrences are rejected. | [Links](protocols.md#links) |
| `@LibertasLinkOnly` | `#[libertas_link_only]` | Request variant targeted by a declared Link in the same protocol | Emits `LinkOnly=true`. The Request is offered only through declared links: NextRequest and the filtered action set still govern its link rows, while Actions, menus, endpoint landings, and automatic singleton selection never offer it or count it. Studio rejects it on any other node, on an untargeted Request, with DefaultRequest, and with ExcludeUI. Clients predating the marker keep offering it in Actions. | [Link-only requests](protocols.md#link-only) |
| `@LibertasRequestConfirmation` | `#[libertas_request_confirmation]` | Request variant | Emits `{"Name":"RequestConfirmation","Value":"true"}` on the variant, the same marker form as `DefaultRequest` and `LinkOnly`. Marks a critical Request: whenever a person's choice or submission is about to send it, the client first shows a dialog naming the action with Send and Cancel, from every entry point and after a popup or an editor is confirmed. Cancel sends nothing. For the AI assistant, the Hub starts the generated tool's description with "Requires user confirmation." and instructs the assistant, in every turn that offers such a tool, to state the action and its values and to call the tool only after the user confirms in a later message; the Hub does not verify the confirmation. Wire form and validation are unchanged; a program calling the endpoint directly is not affected, and the service still validates every request. Studio rejects it on any other node, on a DefaultRequest, and on a request that a CalculationRequest names. | [Critical requests](protocols.md#request-confirmation) |
| `@LibertasNextResponse("A,B")` | `#[libertas_next_response("A,B")]` | Protocol Union variant | Declares allowed replies on requests, or directional Compatible SubscriptionData on SubscriptionData. | [Chaining, caching, and copied values](protocols.md#chaining-caching-and-copied-values) |
| `@LibertasCacheIdentity` | `#[libertas_cache_identity]` | Direct field of a Request root record | Enables large-page draft recovery using all marked fields as immutable identity; optional app-maintained protocol revision invalidates older drafts. Small requests omit it. | [Workflow draft identity](#cacheidentity) |
| `@LibertasExcludeUI` | `#[libertas_exclude_ui]` | Endpoint server field carrying a resolved `ProtocolSchema`, or reachable protocol `Request`/`SubscriptionRequest` variant | Emits `ExcludeUI=true`; Hub Control omits the server from its UI-device list and LLM tool collection, while generated Control GUI and Hub LLM tool-definition generation omit the operation. Admin, protocol, routing, and wire behavior remain complete. All servers sharing one resolved `ProtocolSchema` root must use the same value. | [Declaring an Endpoint](protocols.md#declaring-an-endpoint) |
| `@LibertasAINavigationPrompt("NAVIGATION_PROMPT")` | `#[libertas_ai_navigation_prompt("NAVIGATION_PROMPT")]` | Protocol Union or page-defining protocol variant | Selects a nonempty package StringResource for AI Search guidance. Union guidance precedes page guidance; both contribute. AI selects an offered, fully constructible link without supplying request fields. | [AI Search](protocols.md#ai-navigation-and-ai-search) |
| `@LibertasLLMToolCall(false)` / `@LibertasLLMToolCall(true)` | `#[libertas_llm_tool_call(false)]` / `#[libertas_llm_tool_call(true)]` | Protocol Union or descendant Request variant | Controls Hub LLM tool-definition generation. The nearest explicit Boolean value along the Union-to-variant path wins; omission defaults to true. `false` on a Union excludes its Request variants until a descendant explicitly sets `true`. It does not change Control UI, protocol messages, or authorization. | [Protocol tool projection](schema.md#45-union-endpoint-and-data-attributes) |
| `@LibertasProtocolConformance("Read & Status")` | `#[libertas_protocol_conformance("Read & Status")]` | Protocol Union, or Union-typed protocol variant | Emits `ProtocolConformance`; client and server expressions independently declare mandatory compatible variants. | [Protocol conformance](protocols.md#protocol-conformance) |

`@LibertasDefault`/`@LibertasFixed` and their Rust counterparts have their
Endpoint-client meanings described in the defaults table. `Permissions` may
also appear on an Endpoint server as described in the function table.

## External types, Matter descriptors, and Union metadata

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasForeignType("developer-package::Type")` | `#[libertas_foreign_type(developer-package::Type)]` | Published top-level compatible named type | Emits `ForeignTypeSchema` naming its direct official external App type. The compatible type keeps its own identity and explicit attributes override inherited official attributes. | [External types and App dependencies](app-contract.md#external-types-and-app-dependencies) |
| `@LibertasDeviceType("descriptor")` | `#[libertas_device_type("descriptor")]` | Effective `LibertasDevice` field | Emits the Studio-generated physical Matter `DeviceType` descriptor. Do not invent the encoded descriptor manually. | [Matter](matter/descriptors.md#canonical-generation-and-decoding) |
| `@LibertasVirtualDeviceType("descriptor")` | `#[libertas_virtual_device_type("descriptor")]` | Effective `LibertasVirtualDevice` field | Emits the Studio-generated `VirtualDevice` descriptor. Do not invent it manually. | [Matter](matter/descriptors.md#canonical-generation-and-decoding) |
| `@LibertasDiscriminator("kind")` | Not applicable | TypeScript discriminated Union when automatic detection is ambiguous | Emits parser-owned `UnionTagName`. Normal TypeScript literal/enum discriminators are detected automatically; Rust uses enum declaration order and native representation. | [Protocol roles](protocols.md#protocol-roles) |

## Two-dimensional chart annotations

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasChart(mark)` | `#[libertas_chart(mark)]` | Response/SubscriptionData chart payload or composition | Emits a primitive mark or composition as `Chart`. | [Charts](presentation.md#charts) |
| `@LibertasChartChannel(x, tooltip)` | `#[libertas_chart_channel(x, tooltip)]` | Direct chart data field | Emits its canonical `ChartChannel` roles. | [Charts](presentation.md#charts) |
| `@LibertasChartScale(id=series, kind=linear, ...)` | `#[libertas_chart_scale(id = series, kind = linear, ...)]` | Scale-bearing chart data field | Emits `ChartScale`; source `kind` is Compiled into the [Display Template](presentation.md#display-template) as the `scale` hint, and the other properties stay in the schema. | [Charts](presentation.md#charts) |
| `@LibertasChartGuide(target=x, source=scale, ...)` | `#[libertas_chart_guide(target = x, source = scale, ...)]` | Explicit guide binding or guide-label field | Emits `ChartGuide`; `target` and `source` are required. `position` is Compiled into the [Display Template](presentation.md#display-template) as the `guide` hint. | [Charts](presentation.md#charts) |

The complete mark, channel, scale, guide, composition, visible-text, and
hidden-context rules are in [Presentation and localization](presentation.md#charts)
and the normative [client-side chart specification](charts/2d.md).

## Three-dimensional chart annotations

| TypeScript canonical source | Rust canonical source | Public placement | Result and meaning | Details |
| --- | --- | --- | --- | --- |
| `@LibertasChart3D(mark)` | `#[libertas_chart3d(mark)]` | 3D Response/SubscriptionData chart payload or composition | Emits a 3D primitive mark or composition as `Chart3D`. | [3D charts](presentation.md#3d-charts) |
| `@LibertasChart3DChannel(x, z, tooltip)` | `#[libertas_chart3d_channel(x, z, tooltip)]` | Direct 3D chart data field | Emits canonical `Chart3DChannel` roles. | [3D charts](presentation.md#3d-charts) |
| `@LibertasChart3DScale(id=series, kind=linear, ...)` | `#[libertas_chart3d_scale(id = series, kind = linear, ...)]` | Scale-bearing 3D chart data field | Emits `Chart3DScale`; source `kind` is Compiled into the [Display Template](presentation.md#display-template) as the `scale` hint, and the other properties stay in the schema. | [3D charts](presentation.md#3d-charts) |
| `@LibertasChart3DGuide(target=z, source=scale, ...)` | `#[libertas_chart3d_guide(target = z, source = scale, ...)]` | Explicit 3D guide binding or guide-label field | Emits `Chart3DGuide`. | [3D charts](presentation.md#3d-charts) |
| `@LibertasChart3DView(projection=perspective, ...)` | `#[libertas_chart3d_view(projection = perspective, ...)]` | One physical 3D chart scene | Emits `Chart3DView` camera and projection settings. It does not define a gesture mode. | [3D charts](presentation.md#3d-charts) |

The normative geometry, topology, camera, interaction, fallback, and resource
rules are in [Client-side 3D charting](charts/3d.md).

## Inferred and generated metadata: never invent source annotations

The final schema contains attributes and fields that are inferred from native
source or generated from one of the public annotations above. Their presence in
schema JSON does not create another public macro or decorator.

| Generated or parser-owned result | Source of the result |
| --- | --- |
| `Nullable=true` | TypeScript optional/nullish supported type, or Rust `Option<T>`. There is no public `LibertasNullable` annotation. |
| Rust `NumberType` | Concrete Rust `u*`, `i*`, `f32`, or `f64`. TypeScript obtains it through `LibertasNumber`; never write a direct NumberType annotation. |
| `NumberMin`, `NumberMax`, `NumberStep`, `SizeMin`, `SizeMax` | Emitted components of the public Number and Size annotations. Their schema names are not source spellings. |
| `LengthMin`, `LengthMax` | Retained schema fields from the removed TypeScript String spelling. Current source uses SizeMin/SizeMax through the Size annotation. |
| `Fixed` on enum members; `Discriminator` on Union members; `UnionTagField` on a ScriptLanguage-only native tag field (such as TypeScript/Python; never Rust) | Native enum values and TypeScript discriminated-Union literals. Do not duplicate them with source annotations. |
| `UnionTagName` | Automatic TypeScript discriminator detection or `@LibertasDiscriminator`. There is no direct UnionTagName decorator. |
| `UnionContentName` | The Rust `libertas_export(content = "...")` rustdoc directive. There is no direct UnionContentName attribute. |
| `LibertasProtocol=true` | Injected for a Union reached by an Endpoint `ProtocolSchema`. There is no public LibertasProtocol source marker. |
| `LibertasData=true` | Injected by Studio for a Union reached by a function `FunctionDataSchema`. Both languages use their function-level DataSchema annotation; neither has a public standalone Union marker. |
| `EnumValues` | Parser metadata produced while constructing an Enumeration from the language's public enum syntax. Do not write it directly. |
| `FullyQualifiedName` and source locations | Transient parser resolution/diagnostic metadata. Studio removes it before final schema publication. |
| `AppDependencies` | Studio deployment planning from resolved external App types and Endpoint-client Fixed packages. Never populate it in App source. |
| Rust `no_std=true` | Native crate attribute `#![no_std]`, not a Libertas helper macro. Use it only when the App and all dependencies support that build model. |

The emitted-name column throughout this document is explanatory. Never derive a
new `@Libertas...` or `#[libertas_...]` spelling from a schema JSON property.

## Not public or not usable

Codebots must not emit any spelling in this section.

### Removed annotations

- `@LibertasString` is removed; use `@LibertasSize(min=..., max=...)` for String,
  List, or Bytes bounds. Existing serialized LengthMin/LengthMax values remain
  recognizable, but are not generated by a current public source annotation.
- `@LibertasFunctionDataSchema` is removed; use `@LibertasDataSchema(Type)` on
  the public function. The old bare Union marker is also removed. Studio owns
  the generated LibertasData marker; do not rewrite retained schema bytes.

- `@LibertasInteger` and `@LibertasFloat` are removed source spellings. Use the
  single `@LibertasNumber(type=..., min=..., max=..., step=...)` annotation.
  Existing serialized NumberType/NumberMin/NumberMax/NumberStep fields are
  unchanged; regenerate source artifacts rather than editing retained schemas.

- `@LibertasLibrary` and `#[libertas_library]` are recognized only so Studio can
  report that the `Library` schema attribute was removed.
- `@LibertasUseOriginalDocument` and `#[libertas_use_original_document]` were
  removed. Published compatible types share the official type document
  automatically through `ForeignType`.
- `@LibertasExternalNativeType` was removed. Native external ownership is
  inferred from resolved language imports and exact-name App exports.
- `@LibertasUIHeader` and `#[libertas_ui_header]` (schema attribute
  `TableHeader`) were retired, and Studio rejects them with a message that
  names the replacement: a Display Template
  [`promote`](presentation.md#tableheader-promoted-fields-and-union-selectors)
  rule selecting the field to promote, with the same `use_parent_name`
  option, or a [`flatten`](presentation.md#flatten) rule selecting the struct,
  List, or Union to flatten.
- `@LibertasUIName` and `#[libertas_ui_name]` were retired, and Studio rejects
  them with a message that names the replacement. A data-titled row is a
  Display Template [`label`](presentation.md#labels-and-titles) rule on the row's struct
  with its value from a `promote` rule on a child
  ([data-titled rows](presentation.md#data-titled-rows)).

### Unsupported authoring forms

Use the canonical forms in the catalog. In particular:

- `#[libertas_ignored]` belongs on its exact outer-rustdoc line; it is not a
  standalone SDK macro.
- TypeScript functions use `@LibertasDataSchema(Type)`; the type argument is
  required, and a bare Union marker is invalid.
- Use documented native numeric enums. Fabricated annotation members cannot
  carry required member documentation.
- Named-argument variations do not create additional supported spellings.

### Removed Rust SDK helpers

The SDK helper allowlists no longer admit `libertas_string`, `libertas_array`,
or `libertas_cacheable`, because the owning schema parser does not support
those source forms. Use `libertas_size` for size bounds and the documented
CacheIdentity behavior where needed. `libertas_discriminator` is also not a
Rust schema annotation; Rust uses native enum representation.

## MIME-aware Bytes annotation

Apply `@LibertasMimeTypes("image/png,image/jpeg")` or
`#[libertas_mime_types("image/png,image/jpeg")]` to a Bytes field or named type
to restrict its media types. The [MIME-aware Bytes contract](schema.md#mime-aware-bytes)
defines the filter, LMB1 encoding, valuation, size, copy, and submission rules.
Keep unannotated Bytes fields for raw binary protocols; `MimeTypes` is a
semantic choice for media values, not a replacement for ordinary Bytes.

## CacheIdentity

`CacheIdentity` opts large workflow requests into persistent local draft recovery.
TypeScript uses `@LibertasCacheIdentity`; Rust uses `#[libertas_cache_identity]`.
Both emit `CacheIdentity=true`. Use clients that implement this attribute;
older client binaries do not provide draft recovery.

### Eligibility and field placement

- `CacheIdentity` is a Boolean field attribute. A workflow page is cacheable
  only when at least one root-record field has effective `CacheIdentity=true`.
  Use it for large workflow requests where losing completed field edits would
  require substantial re-entry. Small requests must omit CacheIdentity; clients
  do not guess eligibility from payload size or field count.
- Multiple marked fields form one identity tuple. All must match; they are not
  alternative identities. The client does not infer identity from field names,
  Hidden, or CopyFrom.
- Mark only direct fields of the request variant's root record. A referenced
  record used as that root payload qualifies. Nested record fields, array
  elements, and fields in selectable Union branches cannot carry the attribute.
  Studio validates placement for every request use, including referenced types.
- Identity values must be defined after initialization; valid null is allowed
  and differs from undefined. They remain immutable for the editor lifetime,
  including against user edits and calculation writes. Restoring editable
  data preserves the freshly initialized identity values.

### Identity, revisions, and application responsibility

Capture the tuple after normal initialization or receipt of EditRequest data,
before user editing. Match every marked field by exact typed equality, including
null, numeric values, bytes, Union selections/payloads, and array order. The
application must supply enough distinguishing context to prevent one workflow's
draft from matching another workflow. A workflow identifier is a typical choice.

**Recommendation:** include an application-maintained data/workflow revision
when changes to saved data invalidate earlier drafts. For example, a historical
page edit may invalidate later pages; changing their identity revision prevents
restoring obsolete local drafts. The client does not infer these dependencies.

Protocol changes are allowed. Application data revisions are independent of
protocol/schema versions; tasks need not detect protocol changes. Management
tools guard schema compatibility, and clients reject incompatible cached data
at restoration. Compatible protocol changes alone do not invalidate drafts.

**Optional recommendation:** the application may maintain a protocol revision
and mark it as another CacheIdentity field. Advancing it guarantees rejection
of drafts with the old value even when a protocol change preserves structural
compatibility. Maintaining and advancing this revision is entirely the
application's choice and responsibility. Workflow revision, protocol revision,
or both may be used; the client assigns no special meaning to either field.
Without a distinguishing identity change, semantic changes that preserve the
schema's structure may remain undetectable.

### Draft scope and lifetime

The full cache key comprises Hub identity, task ID, endpoint ID, request variant
index, and the complete CacheIdentity tuple, within the authenticated account's
storage. The first four components identify the page, not the entire cache key.
Different workflow identities and revisions retain separate drafts. Initialize
the page first, then look up its complete identity directly; unrelated entries
do not need to be scanned or deleted. Garbage collection removes expired drafts.

Persist the latest committed editor state after each change, creating the entry
if absent and updating it otherwise. This includes nested fields, array
operations, Union selection, null/undefined transitions, and atomic calculation
results. Drafts survive process exit and restart. Entries expire one calendar month
after their last committed update. Clients collect expired entries at startup
and reject expired entries before restoration. Reads and restore prompts do
not extend retention. Removing the Hub, task, or
endpoint removes its drafts.

Delete a submitted editor's draft only after its successful correlated backend
reply. Transport acceptance alone is insufficient. Retain it on transport or
backend error. Calculation replies never clear the parent draft.

When offered under the [pane contract](presentation.md#master-detail-panes),
the workflow's in-app **Go Back** action selects a declared PrevRequest and
voluntarily abandons the current edit. Delete that editor's draft after the
successful correlated, non-error reply to the previous action. Cancelling its
chooser, failing validation, or receiving a transport or backend error retains
the draft. An explicit editor **Close**, including Close in the right-hand
master-detail pane, deletes that editor's draft when the close is accepted.
These actions delete only the departing editor's matching draft; they do not
delete a retained leading editor's draft or other workflow identities.

System-level Back by button or gesture, ordinary screen-navigation Back, and
process exit or interruption preserve the committed draft for recovery. They
are separate from the workflow's Go Back action. Closing a field picker or
nested editor returns to its parent and does not discard the parent's draft.

EditRequest does not itself delete a draft. If it is a successful reply to a
submission, that correlation still clears the submitted draft. Incoming page
initialization then follows the normal identity comparison and restoration
rules below.

### Restoration and calculations

Initialize the new editor first, including when entered through EditRequest.
Look up the entry for its full identity and verify the stored tuple. If it matches,
ask whether to restore. Declining removes the cache and keeps fresh initialization.
Accepting attempts restoration without submitting the workflow request. Ordinary
rerendering or returning from a field picker does not repeat the prompt.

Restoration assigns only the values the user can edit and the destinations
that a calculation or a link return writes; a link-return destination cannot
be recomputed, so it is restored as saved. Every other Hidden, ReadOnly, or Fixed value, including a
cookie and an allowed-set source that no calculation or link return writes, keeps the value
the fresh editor supplied; the draft's copy of it is not applied. A restored
selection outside the fresh allowed set becomes unanswered under the
[reset rule](protocols.md#calculated-allowed-sets) and does not discard the
draft.

Restore atomically against the current schema; a source that a calculation writes is
restored from the draft and recomputed by the seeded calculations. Corrupt
encoding or structural/type incompatibility discards the cache and rolls back
the entire attempt to the fresh editor. Compatibility includes the declared
primitive kind and representation, record fields, List element types and
available Union branches; a value of the wrong kind or an invalid encoding is
not a restorable draft.
Do not partially restore a structurally incompatible draft, coerce incompatible
data, or silently omit incompatible fields. An exact schema version match is
not required.

For a structurally compatible draft, check each restored node's own value
constraints, including numeric bounds evaluated in the current editor and
reference context under the [range contract](validation-rules.md#31-dynamic-numeric-ranges).
Replace an invalid value through ordinary initialization at that node,
including its applicable Default or an unanswered value. A descendant's
failure must not reset its containing record, List item or page; preserve every
other restored value. A List that itself violates its size constraint is
reset at that List. These local resets are part of the atomic
restoration and do not discard the compatible draft.

Undefined required fields are legitimate draft state. Preserve undefined
separately from null, false, zero, empty values, bytes, and Union selections.
A committed empty String is assigned, not undefined; if it violates its own
constraint, initialize that String normally, applying its Default when present.
Ordinary initialization may leave a value unanswered or still invalid.
Cross-field ValidationRules keep their normal evaluation, and final
ValidationRules run at Next/Submit; neither resets restored values or discards
the compatible draft. Submission stays blocked until the current editor is
complete and valid. This local recovery does not permit incomplete or invalid
Avro requests or backend persistence of unfinished requests.

After successful restoration, seed calculations once for all current instances,
including array items. Reset destinations through ordinary initialization and
reevaluate using restored inputs. Use the normal ordered scheduler and dependency
propagation; incomplete or invalid requests cannot be sent. Do not restore old
in-flight transactions or automatically submit the workflow editor.

### General field-edit commitment

Only finished editing commits a user-entered field value. This applies throughout
client editors, not just caching. Intermediate control text neither replaces
the committed value nor triggers calculations or cache updates. Typing `-` in a
number control therefore does not turn an existing committed number into
undefined. An unassigned field remains undefined until a completed edit assigns
an admissible value.

Submission must resolve the active edit through this same boundary; it cannot
silently submit an old value while replacement text remains unfinished. Native
controls may use their normal completion actions. Programmatic initialization
and atomic calculation updates retain their transaction boundaries.

### MIME-aware Bytes in workflow drafts

For a [MIME-aware Bytes](schema.md#mime-aware-bytes) field, a draft never stores
the media content. It stores a client-local file link when one exists; an
in-memory value received from the server stores only a marker to use the
freshly initialized value after recovery. An explicit clear stores an
undefined marker. The link and its temporary file stay within the client and
are never sent to the Hub. A missing or unreadable link on recovery makes
only that field undefined and reports which field needs a new file; the other
edits restore. If no corresponding fresh value exists for an in-memory marker,
that field is undefined. This is an exception to full-value draft storage,
not a relaxation of identity or schema checks. `MimeTypes` cannot occur
within a `CacheIdentity` field because that identity is persisted. A temporary
file remains while an active editor or unexpired draft refers to it and is
automatically removed after its last reference and a cleanup grace period.

`NextResponse` on SubscriptionData declares [Compatible SubscriptionData](protocols.md#compatible-subscriptiondata), rather than correlated replies.
