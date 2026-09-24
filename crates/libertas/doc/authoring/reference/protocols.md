# Endpoint protocols and runtime

This page covers App-visible protocol and lifecycle rules. For exact runtime
function signatures and callback guarantees, read the installed public SDK.
Every declaration must also follow the
[schema documentation invariants](documentation.md).

Unless a block is explicitly labeled a non-copyable schematic, its declarations
are copyable into an App whose package header already has the required natural
label and description. The data-name example includes that package header
because its package-level resource declaration depends on the header.

## Declaring an Endpoint

A server Endpoint is conceptually equivalent to a conventional service endpoint: it is the interface through which an App provides a defined set of capabilities to clients. Its protocol Union declares the requests, responses, and subscription messages that clients can exchange. A client Endpoint binds to a compatible server Endpoint, and Libertas routes those interactions under the applicable access controls.

An Endpoint field references a protocol Union. `EndpointServer` marks the App
as the serving side; absence means client-side use.

```typescript
/**
 * Equipment status settings
 * Describes the equipment and service used to read its current condition.
 */
export interface Configuration {
    /**
     * Monitored equipment
     * The user chooses the equipment whose condition will be reported.
     */
    device: LibertasDevice;

    /**
     * Equipment status service
     * The App provides this service so permitted users can read the selected
     * equipment's current condition.
     * @LibertasEndpointSchema(StatusProtocol)
     * @LibertasEndpointServer
     * @LibertasEndpointBaseObjects("$.device")
     */
    endpoint: LibertasEndpoint;
}
```

```rust
/// Equipment status settings
/// Describes the equipment and service used to read its current condition.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    /// Monitored equipment
    /// The user chooses the equipment whose condition will be reported.
    pub device: LibertasDevice,

    /// Equipment status service
    /// The App provides this service so permitted users can read the selected
    /// equipment's current condition.
    #[libertas_endpoint_schema(StatusProtocol)]
    #[libertas_endpoint_server]
    #[libertas_endpoint_base_objects("$.device")]
    pub endpoint: LibertasEndpoint,
}
```

`EndpointBaseObjects` uses comma-delimited Libertas Schema Paths to bind an
endpoint instance to configuration objects.

On a client Endpoint, `Default` and `Fixed` take a
`developer-package::function` task-process identity. Every client first filters
server Endpoints by mutual protocol-schema compatibility. `Default` leaves all
compatible choices available. `Fixed` further restricts choices to servers
owned by the named task process and makes its package a mandatory logical App
dependency in finalized `AppDependencies`. Both Endpoint-ID selectors remain
editable because one process can expose multiple eligible server instances. If
the resulting set is empty, either value identifies the process to provision.
Only `Fixed` forces the package into deployed `AppDependencies`; the client
task pre-creation gate below may nevertheless require installation of the
named task process for an absolutely required Endpoint carrying either
attribute. These strings are not Endpoint IDs or schema-type references.

### Required client task-process dependencies

An App author models only the source contract: a client Endpoint carrying
`Default` or `Fixed` can become a task-creation prerequisite when that
occurrence and its complete path are non-nullable and every ancestor List
requires at least one item. Keep the real optionality and List bounds truthful;
do not add App code or generated metadata to implement provisioning.

Control clients own the traversal, eligibility, installation, queue, cycle,
refresh, cancellation, and Hub-validation behavior. Its single canonical
definition is the
[task pre-creation dependency check](schema.md#62-task-pre-creation-static-dependency-check).

Use the no-argument `@LibertasExcludeUI` or `#[libertas_exclude_ui]` marker on
an Endpoint server carrying a resolved `ProtocolSchema` when Control UI must
act as though that Endpoint does not exist. The Hub Control service omits the
marked server Endpoint from its UI-device list before grouping devices, so it
cannot appear as a root or task child, cannot affect folder cardinality, and
cannot contribute LLM tool definitions. The Endpoint remains registered and
available to routing, subscriptions, authorization, and administration. The
same marker is valid on a `Request` or `SubscriptionRequest` variant reachable
from an Endpoint protocol Union and removes that operation from generated
Control UI and Hub LLM tool-definition generation. Admin creation, editing,
approval, persistence, Endpoint compatibility, and dependency/assignment
calculations continue to use the complete schema rather than the user-facing
projection.

Runtime bindings identify an Endpoint server by its resolved `ProtocolSchema`.
If more than one server Endpoint resolves to the same protocol root, every one
of those Endpoints must use the same `ExcludeUI` value. Studio rejects mixed
visibility because the runtime binding cannot distinguish those Endpoint
declarations.
The exact projected Control surfaces and unaffected behavior are defined by the
[schema attribute contract](schema.md#45-union-endpoint-and-data-attributes).

```typescript
/**
 * Refresh current status
 * The service refreshes equipment status in the background without offering
 * this activity as a user action.
 * @LibertasRequest
 * @LibertasExcludeUI
 */
export interface InternalRefresh { kind: 0; }

/**
 * Background refresh completed
 * Reports when the background refresh finished.
 * @LibertasResponse
 */
export interface InternalRefreshComplete {
    kind: 1;
    /**
     * Completion time
     * The instant when the background refresh completed.
     */
    completed_at: LibertasDateTime;
}

/**
 * Background status activities
 * Describes automatic status refreshes and their completion.
 */
export type StatusProtocol = InternalRefresh | InternalRefreshComplete;

/**
 * Background status settings
 * Describes the service used to keep equipment status current automatically.
 */
export interface Configuration {
    /**
     * Background status service
     * The App provides this service to refresh equipment status without
     * presenting a separate user choice.
     * @LibertasEndpointSchema(StatusProtocol)
     * @LibertasEndpointServer
     * @LibertasExcludeUI
     */
    internalEndpoint: LibertasEndpoint;
}
```

```rust
/// Background status activities
/// Describes activities that keep equipment status current automatically.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub enum StatusProtocol {
    /// Refresh current status
    /// The service refreshes equipment status in the background without
    /// offering this activity as a user action.
    #[libertas_request]
    #[libertas_exclude_ui]
    InternalRefresh,

    /// Background refresh completed
    /// Reports when the background refresh finished.
    #[libertas_response]
    InternalRefreshComplete {
        /// Completion time
        /// The instant when the background refresh completed.
        completed_at: LibertasDateTime,
    },
}

/// Background status settings
/// Describes the service used to keep equipment status current automatically.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    /// Background status service
    /// The App provides this service to refresh equipment status without
    /// presenting a separate user choice.
    #[libertas_endpoint_schema(StatusProtocol)]
    #[libertas_endpoint_server]
    #[libertas_exclude_ui]
    pub internal_endpoint: LibertasEndpoint,
}
```

## Protocol roles

Every message-bearing protocol Union variant must have at least one role:

- `Request`: one-shot request input;
- `Response`: server-to-client correlated result;
- `SubscriptionRequest`: client-to-server stream establishment/renewal; or
- `SubscriptionData`: server-to-client stream data.

<a id="protocol-role-reuse"></a>
<!-- protocol-role-reuse:start -->
A message-bearing Protocol Variant may declare more than one role. Its declared
roles identify its permitted uses; the actual operation and interaction context
determine the applicable behavior. Additional roles do not themselves initiate
operations or combine their effects. Each declaration must satisfy the
constraints applicable to its roles. Operation-selection rules still apply when
choosing which declared use to invoke.

A correlated Response operation requires a `Response` variant, and subscription
delivery requires a `SubscriptionData` variant. `EditRequest` is a correlated
server reply carrying a complete `Request` value: its variant must carry
`Request`, and a Request-only variant suffices. EditRequest is an operation,
not an additional schema role, and receiving it does not invoke the request.

<a id="interaction-reference-object"></a>
<a id="interaction-reference-context"></a>

**Reference context and request construction.** The **current request** is the
value being composed or edited. Workflow and non-workflow requests may need
related server data; a session ID is required only when the App needs one.
A **reference context** is the lookup environment for one interaction. Its
contained schema-described values are **reference objects**.

| Context | Reference objects and request construction |
| --- | --- |
| Ordinary Response | CopyFrom reads the latest accepted Response to construct the selected Request. |
| EditRequest | The received complete Request is the reference object. Adopt its data directly without CopyFrom or default initialization. |
| Subscription | CopyFrom probes the most recent eligible successful correlated Response first, then the current SubscriptionData snapshot. Either object may be absent. |

Subscription reports, including reduced-type updates, update the SubscriptionData
snapshot without replacing the retained Response. A successful correlated
Response replaces the Response object even when an acknowledgement leaves the
screen unchanged. In every interaction, a failure status or Error-marked
Response leaves the reference context unchanged. A subscription’s two objects
belong to that interaction, remain separate, and are not merged into a synthetic value. Other interactions and
historical responses are never searched. With no eligible received data, the
reference context has no objects.

`CopyFrom` names source paths on a receiving request node. It copies
already-received data from the reference context without contacting the server.
For each mapping, probe objects in context order and paths within each object
in declared order under the [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules).
Copied nodes override initialization; unaffected nodes retain ordinary
initialization, including defaults or undefined. Direct EditRequest adoption
preserves all received values and supplies a new context containing that Request.

For example, a non-workflow update can copy `item_id` and `revision` without a
session ID. In a subscription, a Response may supply the new revision while
another CopyFrom mapping reads a value available only in SubscriptionData.
An App must carry forward required continuation data; missing values are not
recovered from older responses.

<a id="hidden-protocol-context"></a>

**Hidden protocol context.** Session identifiers, continuation cookies, record
identifiers/revisions used only for application coordination, and application
data carried only to initialize a later request must be schema-defined `Hidden`
fields in the protocol payload. Apply Hidden to each such field or to its
containing cookie/context field. A visible user-facing value may also be a copy
source; do not hide that value merely because another request copies it.

- The server supplies required context in the Response, complete EditRequest,
  or SubscriptionData snapshot from which the next action is constructed.
  Declare CopyFrom on receiving request nodes that need those values. Keep
  session/coordination fields Hidden in the request too; a field initialized
  for the user to review or edit remains visible. Hidden on the source is
  presentation metadata, not a command to hide the destination of a value copy.
- Hidden data remains typed payload data: it participates in reference reads,
  copying, validation, subscription updates, and encoding. Hiding a field does
  not initialize it or exempt it from completeness checks. Follow the
  [Hidden presentation rule](presentation.md#common-field-metadata).
- Supply required data on every permitted construction path, including initial
  entry, subsequent actions, workflow navigation, and resubscription. A complete
  EditRequest already supplies its hidden values; clients must not initialize
  over them. For copied requests, copied nodes override initialization and
  unaffected nodes follow the shared initialization rules. Never rely on a
  user to fill a hidden missing value or invent a placeholder session ID.
- Design continuation data against the reference-context replacement rules
  above. Carry it forward when its supplying object is replaced; an ordinary
  success acknowledgement can replace the retained Response even while the
  screen stays unchanged. SubscriptionData remains the second reference
  object, but an older Response is not a fallback. Error does not replace either
  object. Reduced reports preserve or update hidden fields under the same
  projection rules as visible fields.
- First-time entry has no prior reference objects. Entry requests must be
  constructible from their own valid inputs and normal initialization; the
  server then supplies any continuation context needed for later actions.
  Handle absent external validation reads under the
  [validation contract](validation-rules.md). Do not copy a value into the
  request solely to validate against it when a reference-context read suffices.
- Hidden controls presentation, not confidentiality or authorization. These
  bytes reach the client. The server must validate submitted identifiers,
  revisions, session ownership, and allowed operations; client-side invisibility
  is not evidence of trust. A session identifier is needed only when the App's
  interaction requires one.

For example, a Response may carry a hidden `context` containing `session_id`
and `initial_limit`. A selected Request copies `$.context.session_id` into its
hidden `session_id` and `$.context.initial_limit` into its visible, editable
`daily_limit`. The user sees the limit, while the submitted payload carries both
values. An EditRequest instead supplies both request fields directly. These
are ordinary schema fields and shared CopyFrom paths, not a separate cookie
transport or client-maintained session history.

`ValidationRules`, `RangeMin`, and `RangeMax` use the same reference context for
external reads, retaining their own path syntax and first-non-null rules.
Current-data reads inspect the request being edited. No matching value or an
empty context yields null, which first-time composition must handle. Reading
neither copies values nor fetches server data.

Intermediate calculation CopyFrom probes the bound current editor first,
then the reference objects in the interaction context's order. Within each
source, try the declared paths in order, using the shared copy availability
and valid-null rules. A valid editor match wins; no new source-selector syntax
is needed. Parent-relative paths retain their bound editor ancestry.
These logical contexts need not use separate stored objects:
a received EditRequest may be both the current request and its reference object.
Results still apply to the bound current-request destination under the
[calculation contract](#field-calculation-requests). PrevRequest construction
retains its current-editor source under the [path context rules](schema.md#context-and-syntax).

<a id="workflowrequest-semi-role"></a>

**WorkflowRequest semi-role.** `WorkflowRequest` specializes `Response` through
a schema attribute. `NextRequest` identifies available next request actions;
`WorkflowRequest` requires the client to construct and open its named request
immediately when the variant is received as a correlated Response. Its variant
must carry `Response`, and its attribute value must name exactly one `Request`
variant in the same protocol Union. It cannot coexist with `NextRequest` or
`Error`.

The client uses the received response under the reference context and request
construction rule above. The response may contain arbitrary valid cookie data or be empty; it is not shown
as an intermediate screen. The resulting editor may be incomplete and awaits
explicit user submission, even for Nil, already complete, or entirely
Fixed/ReadOnly payloads. Normal request visibility and access rules apply; an
unavailable target must not be silently replaced with another request.

For example, a variant carrying both `Response` and `SubscriptionData` can use
`WorkflowRequest` to open its named editor on correlated Response delivery.
Subscription delivery of that same variant updates subscribed data without
opening the editor, under the shared role rule above.
<!-- protocol-role-reuse:end -->

Role-less grouping Unions and single-field Union wrappers organize capabilities
under [endpoint protocol validation](schema.md#61-endpoint-protocol-validation).
Their reachable message-bearing variants carry the roles; the grouping nodes
need not carry a role themselves.

An error result is still a `Response`. Mark it with the no-argument
`@LibertasError` or `#[libertas_error]` in addition to its ordinary response
role. The generated `Error=true` marker lets each client select its own
standard error procedure. It does not define an error payload shape or change
serialization, routing, transitions, or compatibility.

An Error-marked correlated Response is out-of-band of the current protocol
state transition.
Android and iOS consume the correlated response but do not install it as the
current response, use it as transaction context, navigate to it, or visualize
it as data or a chart. They preserve the current interaction and current
response data so the user can always retry. The clients inspect the complete
decoded response in deterministic depth-first response order, including hidden
fields, and show its first non-empty renderable `FormattedText` in a persistent
error dialog. When none is renderable, show a localized generalized error in
the same dialog. Server status errors and failures decoding a correlated reply
use this dialog too. Keep the full message readable, allowing scrolling when
needed, until the user explicitly dismisses it. Dismissal does not submit,
navigate, or discard editor values; the user can correct the request or retry. Therefore put the most
useful user-facing explanation in the first `FormattedText` field; the payload may
still contain any typed diagnostic or retry context required by App code.

Outside a workflow, a normal `Response` is not required to declare
`NextRequest` or `WorkflowRequest`. Omitting both is an intentional non-transition and is common for shared
acknowledgement variants such as `Success` or `Failure`. It does not mean that
every Request in the protocol becomes available. After correlating and
consuming such a response, a client completes the in-flight attempt but keeps
the existing protocol state, its displayed data, and its next-request choices.
Reference context follows the [current reference context rule](#interaction-reference-context);
the acknowledgement is not installed as the displayed protocol value.
A transient request/wait surface may complete,
but no protocol-data destination is added or replaced.

The absence of a next state identifies an ordinary completed result without a
separate attribute marking a result variant. Present its user-facing result
message transiently while retaining the existing interactive state; do not
replace that state with a data screen offering no further actions. This
state-preservation rule does not suppress the result message. It leaves the
separate initial-entry, workflow-completion, chart-presentation, and live
subscription behavior unchanged.

Apply these response rules under the [shared role rule](#protocol-role-reuse).
An Error-marked correlated Response uses the separate standard error procedure
above, regardless of `NextRequest`.

```typescript
/**
 * Recording could not continue
 * Explains why the recorder could not save the latest measurement so the user
 * can correct the problem and retry.
 * @LibertasResponse
 * @LibertasError
 */
export interface Failed {
    kind: 1;

    /**
     * Problem explanation
     * Tells the user why the latest measurement could not be saved.
     * @LibertasReadOnly
     * @LibertasFormattedText
     */
    reason: Uint8Array;
}

/**
 * Measurement saved
 * Confirms when the recorder saved the latest measurement.
 * @LibertasResponse
 */
export interface Saved {
    kind: 0;

    /**
     * Saved at
     * Identifies when the recorder saved the latest measurement.
     */
    savedAt: LibertasDateTime;
}

/**
 * Recording activities
 * Describes saving measurements and reporting a problem that prevents the
 * latest measurement from being saved.
 */
export type RecordingActivity = Saved | Failed;
```

```rust
/// Recording activities
/// Describes saving measurements and reporting a problem that prevents the
/// latest measurement from being saved.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub enum RecordingActivity {
    /// Recording could not continue
    /// Explains why the recorder could not save the latest measurement so the
    /// user can correct the problem and retry.
    #[libertas_response]
    #[libertas_error]
    Failed {
        /// Problem explanation
        /// Tells the user why the latest measurement could not be saved.
        #[libertas_read_only]
        #[libertas_formatted_text]
        reason: Vec<u8>,
    },
}
```

The following declarations illustrate the [shared role rule](#protocol-role-reuse)
with a Nil `ReadStatus` and a `Status` payload.

```typescript
/**
 * Check equipment status
 * The user asks whether the selected equipment is available now.
 * @LibertasRequest
 * @LibertasSubscriptionRequest
 * @LibertasAccessPrivilege("Read")
 */
export interface ReadStatus {
    kind: 0;
}

/**
 * Current equipment status
 * Reports whether the selected equipment was available when its condition was
 * checked.
 * @LibertasResponse
 * @LibertasSubscriptionData
 */
export interface Status {
    kind: 1;

    /**
     * Equipment is available
     * True means the equipment can currently be reached; false means the
     * equipment is currently unavailable.
     */
    online: boolean;
}

/**
 * Equipment status activities
 * Describes checking the selected equipment and reporting its current
 * availability.
 */
export type StatusProtocol = ReadStatus | Status;
```

```rust
/// Equipment status activities
/// Describes checking the selected equipment and reporting its current
/// availability.
#[derive(Clone, Debug, PartialEq, LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub enum StatusProtocol {
    /// Check equipment status
    /// The user asks whether the selected equipment is available now.
    #[libertas_request]
    #[libertas_subscription_request]
    #[libertas_access_privilege("Read")]
    ReadStatus,

    /// Current equipment status
    /// Reports whether the selected equipment was available when its condition
    /// was checked.
    #[libertas_response]
    #[libertas_subscription_data]
    Status {
        /// Equipment is available
        /// True means the equipment can currently be reached; false means the
        /// equipment is currently unavailable.
        online: bool,
    },
}
```

TypeScript Unions must have a stable numeric discriminator as the first
property. Use a literal or enum member. Use `@LibertasDiscriminator("field")`
only when automatic detection would be ambiguous.

## Request access

`AccessPrivilege` is an alternative-access bitmap on Request and
SubscriptionRequest variants:

| Source value | Schema value | Meaning |
| --- | ---: | --- |
| `Read` | `1` | A user with endpoint read access may invoke it. |
| `Write` | `2` | A user with endpoint write access may invoke it. |
| `Read,Write` | `3` | Either access level may invoke it. |

Omission also means `Read,Write`. Libertas cannot infer semantic mutation. The
developer must mark a mutating operation `Write`; otherwise a read-only user is
allowed to issue it. Clients may hide a disallowed action, but the Hub remains
authoritative.

This is unrelated to task-process `Permissions`, which grant access to platform
resources during task configuration.

## Request and response ordering

Within one Endpoint interaction, request/response transactions follow the exact
order in which their requests were sent. Handle requests and emit and apply
their correlated replies in that order; a later transaction must not overtake
an earlier one. This includes calculation replies, Error responses, and
EditRequest replies. Correlation and stale-operation checks still apply: an
obsolete reply is discarded in its turn, never assigned to a later request.
Timeout, cancellation, and transport failure use the existing operation-lifecycle
rules; a late reply cannot revive a completed or abandoned operation.

The ordering scope is one interaction, not all clients or Endpoints globally.
Unsolicited SubscriptionData/DataReport updates are not correlated reply slots;
they retain their own subscription ownership and sequencing rules.

## Field calculation requests

`@LibertasCalculationRequest("CalculateOrder,CalculateShipping")` and
`#[libertas_calculation_request("CalculateOrder,CalculateShipping")]` declare
which calculations use a field of a protocol Request/SubscriptionRequest as
an input. One field may participate in several calculations and several fields
may participate in one calculation. Names are comma-delimited, case-sensitive,
non-empty, distinct protocol variant native names. The annotation is field-only.

Each target is another ordinary Request with a Table payload in the same
Endpoint protocol Union. Its NextResponse names its correlated Response variants.
Every direct input field has CopyFrom under the
[shared source contexts](schema.md#context-and-syntax): calculation inputs may
read the current editor first, then reference objects in the
[interaction context order](#interaction-reference-context). Native
discriminators are transport metadata, not calculation inputs. Neither is a
Fixed field: it is filled by ordinary initialization, never by copying editor
data, so it needs no CopyFrom and is never a dependency. A
`Fixed("current-user")` User field is resolved from the authenticated session
and verified by the Hub on the calculation request under the
[security constraint](schema.md#fixed-current-user-security-constraint) as on
any other request; every other Fixed input takes its declared value.
Only the constructed calculation request must be complete and valid to send;
the parent editor may still contain undefined, unencodable fields.

Every copied input is a calculation dependency. Studio validates current-request
participation in both directions: every current-request source field used by
CopyFrom participates in the calculation, and every participating field is
used by CopyFrom. A containing Table or List may participate as a whole;
changes below it are changes to that input. Paths support named fields and
zero-based literal array indices, such as `$.items[0].quantity`. Whole-list copies
remain available. Indices denote positions, never persistent row identities.
Every input resolves in each invocation from its applicable source context,
and every declared alternative resolves somewhere. Input selection follows the [shared CopyFrom rules](schema.md#shared-copyfrom-and-copyto-rules).

### Typed ancestor paths and per-item calculations

Calculation mappings use [Libertas Schema Path](schema.md#libertas-schema-path),
including its root-relative, parent-relative, relaxed typed-ancestor and strict variant-rooted selector forms. That
section owns the syntax and ancestor-resolution rules; the following rules
own calculation-instance binding and lifecycle.

The invoking context belongs to the CalculationRequest field occurrence, not
the calculation request's input field or its response. Each transaction captures
one immutable invoking node and its ancestor bindings.
Current-request CopyFrom inputs and CopyTo destinations retain those bindings
for the entire transaction, including after a later input change; a deferred
execution is a new transaction. Reference-object inputs retain their distinct
logical source under the [interaction rule](#interaction-reference-context);
this does not require a separate stored object. Participating
fields which bind the same calculation variant to the same ancestor occurrences share one calculation
instance. Different array items have separate instances; root-relative inputs
can be shared and changes to them invalidate every dependent instance. Do not
coalesce different item instances merely because they name the same variant.

Clients retain the bound ancestor identities while waiting. Resolve output
paths against the original binding, never the currently selected row. Removing
or replacing a bound item invalidates its pending work before a reply can write.
A positional mapping which would now address another item is also invalid; never
retarget it. Surviving item-local bindings follow the
[array-removal rule](#array-removal-and-calculation-scope), rather than treating
an index shift alone as changed item data. Rediscover affected current instances
and defer/reschedule still-needed work using the ordinary calculation rules.
New items acquire their own instances. Reset only the destinations belonging
to affected instances. This adds no wire opcode or persistent row identity
requirement.

Studio validates typed paths in every reachable invoking request occurrence,
including reusable record types and nested arrays. It resolves named types and
each declared ancestor selector symbolically under the [Libertas Schema Path rules](schema.md#typed-ancestors-and-bound-occurrences), then validates suffixes, input coverage, type and
nullability compatibility, Fixed protection, overlapping writers including the
implicit writers of [calculated allowed sets](#calculated-allowed-sets),
and direct/indirect cycles. Distinct instances writing their own array items are
not overlapping writers; an enclosing whole-list write overlaps those items.
Runtime checks still enforce actual values, bounds, and stale-instance ownership.

### Array removal and calculation scope

For removal, "outside an item" means outside that item's data subtree, not
later in array order. A calculation is item-local only when all fields actually
copied as inputs and all CopyTo destinations belong to that same item. Include
nested fields and the actual projected fields of reduced copies when determining
this scope; the spelling of a path alone does not establish locality.

Removing an item has these effects:

- Cancel that item's pending and deferred calculations. A reply cannot recreate
  the item or write into the item which now occupies its former index.
- Preserve surviving item-local results and work when their bound nodes and
  copied inputs remain unchanged. Do not reset or recalculate them merely
  because their array indices decreased. A pending typed-ancestor calculation
  may retain its original surviving item binding; a literal positional mapping
  which no longer addresses the original nodes must instead be invalidated.
- Recalculate instances associated with the changed array which involve fields
  outside the individual item, including a shared parent/root rate or an output
  outside the item. This applies even if that external rate's value did not
  change. Reset affected outputs and reschedule through the ordered deferred
  queue's remove-then-append rule.
- Recalculate calculations consuming the array as a whole or a reduced array
  projection. Membership is an input change, including removal of the last
  item, even when no surviving element's individual field value changed.

For example, removing one line preserves another line's `quantity × unit_price`
subtotal when both inputs and the subtotal belong solely to that surviving
line. A line calculation which also copies the order's shared `rate` is
recalculated, as is the order total calculated from the array of subtotals.
Calculations belonging to unrelated arrays are not affected by this rule.

### Reduced record and array copies

Calculation copies use the [shared field projection](schema.md#shared-copyfrom-and-copyto-rules).
The destination type of calculation CopyFrom specifies the projection; no extra
attribute is needed. A calculation input type selects
the fields relevant to the calculation. Unrelated fields can be omitted even when
complete or immutable, including large descriptive name strings that do not
contribute to the result. Omitting these values reduces serialized payload size
and avoids repeatedly transmitting them. A required field remains included
regardless of its size or immutability. Primitive semantic types,
currency, and union branch identities/discriminators must remain compatible.
A union-valued field retains its complete union payload type; record projection
does not reduce its branches, authorize conversion, or filter branches.

Construct the destination under the shared initialization rules. A destination
field absent from the source type is unaffected by the copy and retains ordinary
initialization. By contrast, a matching source field whose value is unfinished
remains unfinished in the projection: do not replace it with a default. An
unfinished required projected field blocks the calculation.
Once the required inputs are complete and the constructed request is valid,
unrelated excluded fields do not block sending, whether complete or unfinished.
Valid null is preserved when permitted. Arrays retain
every element in its original order, including incomplete elements; never filter
or compact them. Changes to projected fields and array membership/order are
inputs; excluded fields are not direct triggers.

CopyTo may apply a reduced record to an existing record, or a reduced array to
an existing array, under that same field projection. An existing destination
field with an unavailable value is not an extra field. Reduced arrays require
the original length and order, use zero-based
positional correspondence, and cannot insert, remove, or reorder elements.
Nested projections follow the same rule. Validate all mapped fields, bounds,
protected destinations, and retained bindings before applying any assignment.
A membership change invalidates a pending whole-array result. This is a merge
into existing data, not permission to create missing parents or replace a union
with an incompatible branch. Full-type assignments retain their existing meaning.

### Applying calculation results

`@LibertasCopyTo("$.orderTotal")` and `#[libertas_copy_to("$.orderTotal")]`
on a successful calculation Response payload field assign that field to one
field of the originating editor. Every direct successful response payload field
has CopyTo; discriminators and Error responses are exempt. The source may be a
whole Table or List. In this calculation context, CopyTo has one root-relative
or typed-ancestor path, including zero-based array indices, and does not replace
the request root. Applying a calculation result does not navigate or replace
any object in the editor's reference context. The [reduced SubscriptionData design
pattern](#reduced-subscriptiondata-design-pattern) uses variant-level CopyTo
lists in a stable subscription context.

Studio checks mappings in each invoking editor, including reusable types,
field types, nullability, numeric semantics and money currency. Distinct
calculations cannot write overlapping destinations: writing a Table/List also
writes its descendants. Mutually exclusive response variants of the same
calculation may write the same destination. Assignments within one response
must not overlap. Studio rejects direct and indirect calculation cycles,
including cycles through a ConstrainedBy source, since writing a source counts
as writing every field it constrains ([calculated allowed sets](#calculated-allowed-sets)).
ReadOnly prevents user edits, not calculation assignments. Fixed destinations and Fixed descendants
cannot be overwritten (Union discriminators remain transport metadata). Applications own meaningful default values and whether
an editable destination is appropriate for an automatically assigned result.

The client checks the complete mapping before applying it atomically. A null
result assigns null to a nullable destination. An unavailable union branch,
null parent, or out-of-bounds array destination rejects the complete update;
CopyTo never creates missing parents or extends arrays. No unrelated field,
default initialization, or navigation is changed by a successful assignment.
An Error response applies no assignments and uses normal error presentation.

### Changes, resets, and deferred calculation

Editor initialization alone does not trigger calculations. EditRequest values
are supplied by the service. Changes to participating inputs, including results
assigned by another calculation or changed reference context inputs, trigger
their calculations. Changes are measured
as typed editor values, including becoming undefined; formatting alone is not
a new input. Invalid intermediate input still invalidates pending work.
Native inline editors may buffer text until Done. Buffered changes invalidate
pending replies immediately and defer sending until the local edit finishes.
Submit requires that edit to finish; an independently valid Prev request remains
available to discard it.

When inputs change, reset all affected output destinations inferred from every
successful NextResponse's CopyTo mappings through ordinary default initialization:
Fixed/Default/localized/type initialization may produce a value, null, or undefined.
Do not rerun CopyFrom for resets or introduce a separate validity rule for a
valid default. Reset and result assignments are atomic editor changes; propagate
them through the acyclic dependency graph before sending dependent requests.

Clients permit continued editing, with at most one in-flight attempt per
calculation instance. Editor changes invalidate outstanding calculation replies
and mark their work deferred, except that array removal preserves unaffected
item-local work under the [scope rule](#array-removal-and-calculation-scope).
Deferred calculations form one ordered queue for the editor, with at most one entry per instance. Whenever a change
schedules an instance, remove its existing deferred entry, if any, and append
it to the tail. Do not retain its old queue position or append a duplicate.
For example, changing B in `[A, B, C]` produces `[A, C, B]`; changing A next
produces `[C, B, A]`. Separate array-item instances remain separate entries.

An in-flight transaction keeps its original position and immutable invoking
context. A new change queues a deferred execution; it cannot rewrite the
transaction already sent. Only after that attempt completes and its obsolete
reply is discarded may its deferred execution be sent. Dispatch deferred work
from the queue head in order, never by schema declaration order or response
completion speed. A head entry still waiting for its previous attempt cannot
be bypassed. Calculation transactions also obey the
[request/response ordering guarantee](#request-and-response-ordering).

A deferred entry represents work to reevaluate, not a previously encoded
request. At dispatch, reconstruct from the current input contexts and check validity.
If invalid or its invoking context no longer exists, remove the entry without
sending; a relevant later input change can enqueue valid work again. Otherwise,
send it and remove that deferred entry. Result and reset assignments which
trigger calculations use the same remove-and-append rule after dependency
propagation. Scheduling never changes the bindings of an in-flight transaction.

Timeouts and transport failures use client retry handling; obsolete errors do
not report against a newer edit. Leaving or replacing the editor abandons its
pending/deferred calculations; late replies cannot reopen it or update array
positions that have changed ownership.

Submit and Prev send their own validated, immutable snapshots and own their
navigation. Calculation replies cannot mutate a submitted snapshot. If the
operation fails and the editor remains, reevaluate unfinished calculations.
No partially filled parent request is transmitted. Applications remain
responsible for backend business rules and meaningful defaults; Studio proves
structural declarations, while clients validate actual runtime values.

### Reusing a variant for reading and editing

A variant may carry both `Request` and `Response`. Add `NextRequest` naming that
variant to offer editing after receiving it, and `CopyFrom("$")` on the request
payload to initialize the complete editor from the
[current reference context](#interaction-reference-context). `NextResponse` may name the same variant for the server's updated
result. Root copying copies nested values into an independent editing value;
changing the editor does not mutate the received response.

This supports a receive, edit, submit, and receive cycle using the same data
shape. It does not automatically turn response display into an inline editor.
The existing [protocol navigation rules](endpoint-presentation.md#protocol-transaction-stack)
control presentation under the [shared role rule](#protocol-role-reuse). The
[shared copy rules](schema.md#shared-copyfrom-and-copyto-rules) determine
initialization when no reference context is available.

## Workflow interactions

This is the unreleased workflow interaction contract. A protocol Union may mix
state-machine interactions, whose stable screen is a response or subscription
report, and workflows, whose stable screen is a request editor. The service
selects every resulting transition. There is no separate machine-type marker.

### Valid data and editor state

Every transmitted value must be structurally valid for its Avro schema. An
editor can also contain an undefined field: it has neither a valid value nor
an encoded null. A required undefined field prevents submission. Validation
expressions may observe undefined as null, but this does not change the wire
schema or make the field nullable. No partially filled request is transmitted
for initialization or backend draft persistence. The
[CacheIdentity contract](annotations.md#cacheidentity) defines opt-in local
recovery of committed field values on large workflow pages, independently of
backend persistence. Uncommitted control text is discarded on exit. Without
CacheIdentity, all unfinished page edits are discarded and the application
resumes from its own saved, valid data.

### Entering and continuing a workflow

The server has two ways to open the next editor:

- **EditRequest:** a server-to-client opcode carrying one complete, structurally
  valid Request variant from the Endpoint protocol Union. It is a correlated
  reply to the pending Start, Submit, Prev, or Resume request and completes that
  operation under the [shared role rule](#protocol-role-reuse). The client uses
  the supplied value directly, without running CopyFrom or default
  initialization over it.
- **WorkflowRequest:** applies the [Response semi-role](#workflowrequest-semi-role).
  `@LibertasWorkflowRequest("EditProfile")`
  and `#[libertas_workflow_request("EditProfile")]` emit
  `WorkflowRequest=EditProfile`.

EditRequest also awaits explicit user submission, including Nil, already complete,
or entirely Fixed/ReadOnly values; it must not automatically resubmit them.
Its reference context follows the [current reference context rule](#interaction-reference-context).
EditRequest replaces the active editor even when the returned variant
is the same variant as the previous step; variant identity is not step identity.

On Request/SubscriptionRequest variants, NextResponse may list Response,
SubscriptionData, or Request variants. On SubscriptionData it instead declares
[directional compatible updates](#compatible-subscriptiondata). Permitted
delivery follows the [shared role rule](#protocol-role-reuse).
When NextResponse is declared,
a successful transition reply must be one of its named variants. Error responses
and standardized failure statuses use the shared error/retry path and do not
need to appear in this navigation list. These declarations constrain the
possible replies; the client never selects the service's reply for it.

### Submit, previous actions, and completion

The workflow's **Next/Submit** button validates and sends the current request.
It is not a chooser for the state machine's NextRequest actions.

`@LibertasPrevRequest("ReopenProfile,ReopenAddress")` and
`#[libertas_prev_request("ReopenProfile,ReopenAddress")]` belong on a Request
variant and emit `PrevRequest` containing one or more distinct Request variant
native names in the same protocol Union. Their declared order is presentation
order. With one eligible target, Back selects it directly; with several, Back
opens a chooser using their ordinary localized names. With none, there is no
workflow Back action.

`PrevRequest` names this action declaration, not a stored previous request or
reference context. Its declared target list is not dynamically filtered by
`ConstrainedBy`.

Each selected Back action constructs an ordinary request using the
[shared CopyFrom rules](schema.md#shared-copyfrom-and-copyto-rules). Its reference
object is the current editor's selected payload, independently of
the unfinished editor's validity. A hidden application-defined cookie can
identify any of several concurrent workflows. Cookie design and historical selection belong to the application. Accepting
a historical edit must follow the [saved-page consistency rule](#saved-page-consistency-after-historical-edits). Studio validates declarations and statically decidable mapping
rules; it does not prove that cookie fields are initialized on every path.
The client refuses to encode or send an incomplete or invalid previous action.
Only the newly constructed action request is encoded; the current draft is not.

Back expresses abandonment of the current edit. The backend decides what to
return, including a complete historical request through EditRequest. The client
does not navigate through cached historical requests. Construct and validate
the action before sending it; a rejected selection or failed attempt follows
the ordinary error/retry procedure. There is no server-side undo implied by
discarding the current edit.

For a workflow Submit or Prev operation, a non-error response without
WorkflowRequest ends the editor and is presented as the resulting stable
response. NextRequest, if present, offers its declared state-machine actions.
With no NextRequest, the response is a valid terminal screen, for example a
FormattedText congratulations message. The state-machine acknowledgement rule
for ordinary requests outside a workflow remains unchanged.

Only the explicit workflow Prev and Next/Submit buttons have workflow semantics.
Native Back, dismissal, or leaving the interaction sends no previous action.
Local draft retention follows [CacheIdentity](annotations.md#cacheidentity);
Control endpoint navigation follows
[the landing rule](#control-endpoint-landing-and-native-back). Closing a field picker or nested
editor returns to the same parent editor. Cancelling an action chooser changes
neither data nor workflow state.

### Saved-page consistency after historical edits

A workflow is a strictly ordered state machine. Later pages are assumed to
depend on earlier accepted answers, so the App must keep the saved sequence
consistent.

Compare the submitted page answers with the saved answers before invalidating
history. Ignore transport cookies, navigation metadata, and presentation-only
values in this comparison. An unchanged Next is review/navigation: preserve all
saved pages and completion state. Opening, canceling, or rejecting an edit also
preserves saved history.

When the backend accepts a changed historical page, it **must update or
invalidate every saved page after that page** in the same workflow. This is a
mandatory application rule, not an optional client navigation policy.

The simplest implementation atomically replaces the edited page and erases all
later saved pages, retaining earlier pages. Do not erase before validation or
leave a partially updated history visible.

For each later page, the backend must either recompute and revalidate its data
against the revised earlier answers and save the consistent result, or erase
that page's saved data from the database. It must not leave an old page available
for resume, editing, calculation, or completion merely because that page was
valid before the historical change. Derived results, cookies, allowed-choice
sets, and saved progress must agree with the revised history as well.

Complete this reconciliation before continuing or resuming the workflow. Resume
and EditRequest may use only the remaining consistent saved pages; retries,
concurrent editors, and late submissions must not restore invalidated later
pages from the old history. The backend owns persistence and consistency; the
client and Studio cannot infer application dependencies or repair saved pages.

Opening a historical editor does not itself invalidate anything. An abandoned
or rejected draft changes no saved history; the rule applies when the backend
accepts and saves the change. For example, changing a saved filing status from
joint to single requires updating or erasing all subsequent saved interview
pages, including their spouse-dependent data and choices, before resuming.

### Validation context and progress

Editable requests use the [shared path contract](schema.md#context-and-syntax).
Supported references use the [current reference context](#interaction-reference-context),
including on inherited types and hidden fields. CopyFrom remains separate
initialization metadata with its defined invocation source. EditRequest context
follows that same owner; it does not retain an older response in its place.

Progress is an ordinary FormattedText field following the
[static read-only placement and request-value contract](presentation.md#static-read-only-placement-and-request-values).
For example, the service can supply it in EditRequest or response data copied
into a WorkflowRequest editor. There is no progress opcode, percentage schema,
fixed step count, or client inference from previous-action lists or history.

### Operation ownership

EditRequest uses the same correlation, status envelope, transport failures,
and error/retry procedures as an ordinary reply. Allow only one pending Submit
or Prev transition per editor. A duplicate or late reply cannot replace an
editor after its operation was consumed, its editor superseded, or the
interaction exited. Subscription updates and auxiliary calculation responses
keep their own owners and cannot perform workflow navigation or overwrite a
replacement editor. Leaving the UI cannot retract a submission already received
by the backend. Persistence, retry idempotency, and resume selection belong to
the application; the client never serializes an incomplete draft to supply them.

## Chaining, caching, and copied values

Use variant native names, not type aliases or localized labels:

| TypeScript | Rust | Meaning |
| --- | --- | --- |
| `@LibertasNextRequest("A,B")` | `#[libertas_next_request("A,B")]` | Allowed next client variants. |
| `@LibertasNextResponse("A,B")` | `#[libertas_next_response("A,B")]` | Allowed replies on requests; directional compatible updates on SubscriptionData. |
| `@LibertasCopyFrom("$.value,$.legacy.value")` | `#[libertas_copy_from("$.value,$.legacy.value")]` | Populate request data using the [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules) and [reference context](schema.md#context-and-syntax). |

`NextRequest` is optional on a Response. When present, its value must be
non-empty and names the choices installed with that response. With neither
NextRequest nor WorkflowRequest outside a workflow, the response is the
non-transition acknowledgement defined under
[Protocol roles](#protocol-roles); absence is never expanded to every Request
in the protocol Union.

The following non-copyable schematic uses a compact Rust enum to illustrate
the transition annotations. Each Request names two possible Responses, and
each Response names two allowed next Requests.

[![MyProtocol Rust code beside six unique states and all twelve allowed directed transitions.](../assets/diagrams/my-protocol-state-transitions.svg)](../assets/diagrams/my-protocol-state-transitions.svg)

*Each state appears once. Double-headed links allow both directions;
crossing lines do not join.*
[Open the full-size SVG](../assets/diagrams/my-protocol-state-transitions.svg)
or [download the PNG](../assets/diagrams/my-protocol-state-transitions-six-states.png).

The [shared copy rules](schema.md#shared-copyfrom-and-copyto-rules) define path
selection, initialization, and field projection. The [context table](schema.md#context-and-syntax)
identifies the reference context for each operation, including workflow Back and
calculation construction. Copying from the current editor does not replace the
interaction's [reference context](#interaction-reference-context).

## Protocol conformance

### Protocols describe capabilities

A protocol is a declaration of capabilities: the operations and message payloads
that a participant can send, receive, or support. Client/server matching
establishes which of those capabilities correspond and whether mandatory
requirements are satisfied. Serialization then carries a value for the matched
capability. Protocol translation should be understood in that order, not as
arbitrary substitution of bytes or conversion of unrelated data structures.

A protocol path prefix identifies a payload capability through its enclosing
protocol Union variants. It is not a path to every field inside that payload.
Once the capability is selected, ordinary serialization/deserialization handles
the payload under its compatible schema. Capability correspondence and encoding
are separate concerns; finding a matching prefix does not make incompatible
payload definitions compatible. The current runtime limits are stated below.

### Required and optional capabilities

Protocol declaration and optional runtime support solve different problems.
The protocol Union itself has one versionless, lifetime-invariant name. New App
versions append new protocol payload variants to that same Union; they do not
replace it with a `V2` Union. The complete evolution contract is in the
[App data-type design guidelines](data-type-evolution.md).

`ProtocolConformance` may be declared on a protocol Union on either client or
server. During task configuration, the client expression is evaluated against
the server declaration and the server expression is evaluated against the
client declaration; both must pass. Operators are:

- `&` AND;
- `|` OR;
- `!` NOT; and
- `^` XOR.

Without an expression, every direct variant in that Union context is mandatory.

Matching variants must also have pass-through-compatible payloads. Libertas may
reorder root protocol variants by translating the first Avro discriminator in
the client task host, but positional fields and overlapping nested Union slots
must remain aligned. Nested optional variants may only extend the tail of a
smaller Union. Payload bytes are not decoded by the translator; the receiving
server or client App decoder validates them against its own schema.

The following focused snippets are non-copyable schematics: the referenced
variants and their required natural-language documentation are omitted so the
placement of `ProtocolConformance` remains visible. A generated App must use the
complete declaration style shown in the preceding role example.

```typescript
/**
 * Equipment status activities
 * Describes checking current equipment, changing its target, and reporting its
 * current condition.
 * @LibertasProtocolConformance("ReadStatus & Status")
 */
export type StatusProtocol = ReadStatus | SetTarget | Status;
```

```rust
/// Equipment status activities
/// Describes checking current equipment, changing its target, and reporting its
/// current condition.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
#[libertas_protocol_conformance("ReadStatus & Status")]
pub enum StatusProtocol {
    // Fully documented variants are omitted from this non-copyable schematic.
}
```

If a direct variant's payload is itself a Union, the conformance attribute may
be placed on that variant. That nested expression becomes the variant's
evaluation. Libertas cannot infer that an arbitrary Union is protocol-related,
so append-only protocol policy is a publishing/tooling contract rather than a
universal Union validator.

Configuration matching proves only mandatory compatibility. The server does
not know the client's complete declaration. A client that wants to use an
optional server payload must check the configured server's runtime variant
index before sending the request. In Rust use `VariantIndex` and
`variant_index!`; a nested optional payload may require a chain of variant-index
checks.

No attribute enables protocol Request/Response or subscription-payload caching.
`CacheIdentity` concerns local editor drafts only.

## Compatible SubscriptionData

A SubscriptionData variant A may declare `NextResponse(B,C)` naming other
SubscriptionData variants in the same protocol. A and these named variants form
A's **Compatible SubscriptionData** group. Compatibility is directional: B and C
may update an existing A working value; this declaration does not authorize A
updates to B, B updates to C, or transitive membership.

Every member must have a non-null Table (struct) payload. On an incoming compatible
SubscriptionData report, retain A as the working type and screen, including its
presentation attributes and NextRequest actions. A report with explicit CopyTo
uses the [reduced subscription design pattern](#reduced-subscriptiondata-design-pattern).
For the name-matched form without CopyTo, match fields by exact native
name at the struct root. Replace each matching field's complete value; do not
recursively patch nested records or merge array elements. Preserve fields present
only in A and ignore fields present only in the incoming report. Valid null is a
value and clears a matching nullable field; undefined is never transmitted.
Hidden fields participate in this merge like visible fields.

Studio validates the declaration, Table payloads, and assignability of matching
fields, including numeric/semantic types, nullability, and nested shapes. Clients
prepare and validate the complete merged value before publishing it atomically;
an invalid update leaves the working value unchanged. The App must ensure that
its updates preserve the complete working value's constraints and meaning.

Every incoming SubscriptionData report is offered to every live screen for its
source Hub, including covered screens. Each independently decides whether it
needs the report. Acceptance or rejection by one screen never stops traversal.
Compatibility changes data application, never ownership: Hub, task, endpoint,
schema context, subscription lifetime, and stale-message checks still apply.
A current A may accept its explicitly named compatible reports even when the
original SubscriptionRequest names only A. Before any working value exists,
initial data must satisfy the SubscriptionRequest's NextResponse. If it names A,
B arriving before A is discarded, never deferred or replayed after A arrives.
Process reports in arrival order; schema-loading buffers cannot reorder them. Ordinary
non-compatible reports retain existing acceptance and replacement semantics.
For actions originating from that subscription screen, the merged A supplies
the SubscriptionData object in the [reference context](#interaction-reference-context),
which also retains that interaction's latest successful Response. A report does
not replace the Response object or another interaction's context. A compatible update does not
navigate, submit a request, or execute the incoming variant's actions.
Its delivery scope follows the [shared role rule](#protocol-role-reuse).

Sending compatible reports is entirely the App's choice and responsibility.
Declaring compatibility never instructs the Hub or clients to generate, request,
or synthesize such reports. Apps may continue sending complete snapshots only.
Every SubscriptionRequest, including a repeated default subscription, must return
the current complete initial snapshot. The App owns snapshot/update ordering and
must not rely on a new subscriber having seen earlier reports.

### SubscriptionData array append

#### Relationship to compatible SubscriptionData

This extends the [compatible SubscriptionData contract](#compatible-subscriptiondata).
Existing directional acceptance, all-live-screen delivery, ownership, ordering,
initial snapshot, and atomic publication rules continue to apply. AppendTo does
not grant permission to deliver a report to a screen. A compatible append report
arriving before its required initial snapshot is discarded, never replayed later.

#### Attribute declarations

- `AppendTo("Entries.entries")` is declared on an incoming SubscriptionData's
  root array field. Its value is exactly one `Variant.field` destination, using
  native names in the same protocol. The named variant is a working
  SubscriptionData that admits this report through NextResponse. Its named
  field must be a root array. Nested paths and per-item targets are out of scope.
- Optional `AppendIndex("$.start_index")` is declared on that same array field
  and requires AppendTo. Its root-field Libertas Schema Path references one
  field in the incoming report. That field must be a non-nullable integer Number, carrying the expected
  destination length before the append. Its value must be non-negative.
  The index field should be Hidden; visibility has no effect on validation.
- Both source and destination arrays must be non-nullable. Elements may be
  nullable where assignment to the destination element type permits it.
  Element types must satisfy the existing compatible-field assignability rules.

Use `@LibertasAppendTo("Entries.entries")` and optional
`@LibertasAppendIndex("$.start_index")` on the incoming array in TypeScript.
Rust uses `#[libertas_append_to("Entries.entries")]` and optional
`#[libertas_append_index("$.start_index")]` on that field. The index is an
ordinary integer, not a date, duration, money amount, enumeration index, or
bitmask.

Schematic schema notation:

```text
Entries: SubscriptionData, NextResponse("MoreEntries")
    entries: Array<Entry>
    cursor: String

MoreEntries: SubscriptionData
    added_entries: Array<Entry>
        AppendTo("Entries.entries")
        AppendIndex("$.start_index")
    start_index: Integer, Hidden
    cursor: String
```

#### Applying a report

1. Qualify the report for the current working SubscriptionData using its existing
   directional compatibility and ownership checks.
2. For each append targeting that working variant, compare the optional index
   against the destination array's exact current length, before applying any
   changes from the report. A zero-based append position of zero therefore
   requires an empty destination. Perform this check even for an empty append.
3. Append the incoming elements in their supplied order. An empty array changes
   no elements; duplicates are retained. Without AppendIndex, append without
   the position check.
4. Exclude AppendIndex control fields from copied data. Without variant-level
   CopyTo, apply ordinary compatible root-field replacement to other unannotated
   fields; with CopyTo, apply its typed reduced projection.
   A field carrying AppendTo never also participates in name-based replacement,
   even if its name matches a destination field. If its AppendTo names another
   working variant, ignore that field for this screen; do not fall back to
   ordinary replacement.
5. Validate the complete resulting working value, then publish the report's
   changes atomically. Retain the working type, screen, and actions.

A negative or mismatched index rejects the entire report, preserves the original
working value, and reports a protocol error. Never pad, truncate, overwrite,
adjust an index, partially apply other fields, or infer automatic recovery.
Other assignment or resulting-value validation failures also reject the entire
report. A full snapshot continues to replace its arrays normally.

#### Studio validation and App responsibilities

Studio validates attribute placement, destination and index references, array
and element types, and index integer type. For each possible working variant,
reject conflicting writers: two appends to the same root array, or an append
and an ordinary replacement or CopyTo write to that array within one report.
Actual index values and current array lengths are runtime checks.

The App owns ordering, duplicate-report prevention, retention, and complete
initial snapshots for new subscriptions. If one append report serves multiple
working screens, the App must ensure each intended recipient has the required
array prefix. Under the ordered-delivery contract, an index mismatch exposes a
breach of that expected append sequence. Matching lengths do not prove equal
contents; AppendIndex is a position-consistency check, not a content checksum.

#### Combined copy and append

A report may combine variant-level CopyTo with AppendTo. Each admitted recipient
must still resolve exactly one CopyTo destination; multiple matches are an App
design flaw even when append destinations are unambiguous. Exclude AppendTo
fields and their AppendIndex control fields from that projection. Its remaining
fields follow the typed reduced-copy rules. Resolve all destinations and check
indices before applying the report to one candidate working value. Any copy,
append, index, or resulting-value failure rejects the whole recipient update.

Compute conflicting writes from the actual projection, including descendants
replaced by complete values. Selecting a Table does not itself write omitted
fields retained there. Two appends, or an append and a copied write, must not
modify the same array or its contents. Appended elements must be complete values,
not reduced projections of existing elements.

#### Live chart use

Append reports can supply live chart data. A chart
starts with its complete subscription snapshot; accepted atomic appends extend
the chart-bearing working array and refresh its presentation. Optional
AppendIndex checks the expected sample count. Rejection leaves data unchanged.
Ordering, retention, and fresh-subscriber snapshots remain App responsibilities.
Sending only new elements and update metadata reduces update-message size
compared with retransmitting the complete accumulated array; initial snapshots
remain complete. This is a use of the shared SubscriptionData mechanism, not a separate chart
patch format or an automatic scrolling requirement.

### Reduced SubscriptionData design pattern

**Reduced SubscriptionData** is an optional design pattern an App can adopt
using Libertas subscriptions, `NextResponse`, `CopyTo`, and typed copying. It
introduces no new protocol message role or schema attribute. An App may instead
publish complete snapshots for every update.

An App adopting this pattern maintains a complete stable SubscriptionData value.
Each stable Protocol Variant declares `NextResponse` naming the reduced Protocol
Variants it accepts. Each reduced Protocol Variant declares `CopyTo` with one
or more comma-delimited Libertas Schema Paths identifying the stable values or
subtrees it updates.

The pattern also reuses source-code field identifiers as a shared semantic
vocabulary. An App can give a Native Name one consistent documented meaning
across full state, partial requests, and reduced reports. Compatible types and
explicit paths establish valid field correspondence and select the intended
occurrences; the same identifier can appear in several structures. The App
authors these semantic relationships. See [static and runtime validation](schema.md#9-static-and-runtime-validation)
for the shared checks on declarations and the separate validation of values.

#### Acceptance and destination are separate

The SubscriptionRequest's `NextResponse` selects its complete initial snapshot.
The stable SubscriptionData's `NextResponse` admits later reduced reports.
The reduced SubscriptionData's `CopyTo` specifies where its payload is copied.
CopyTo does not grant delivery permission or infer an acceptance relationship.
A reduced report must satisfy both the existing subscription/ownership checks
and the stable variant's explicit acceptance declaration.

CopyTo is one positional string containing a non-empty path list. For example:

```text
ZoneState: SubscriptionData, NextResponse(NoWaterPeriodsChanged)
ZoneSettings: SubscriptionData, NextResponse(NoWaterPeriodsChanged)
NoWaterPeriodsChanged: SubscriptionData
    CopyTo("^<ZoneState>,^<ZoneSettings>")
```

The example is schematic. The named Protocol Variants belong to the same
protocol; their complete Table definitions and user documentation are required
in an App. A path may instead descend explicitly from the stable variant, such
as `^<ZoneState.schedule>`, when the reduced payload targets that subtree.
Use strict variant-rooted paths to identify the entire destination occurrence;
intermediate schema nodes must not be skipped.

#### Per-destination resolution and typed copying

Resolve CopyTo against each eligible **stable working value** under the
[shared mapping and destination-uniqueness rules](schema.md#shared-copyfrom-and-copyto-rules).
One publication may reach both a ZoneState screen and a ZoneSettings screen;
each independently resolves one destination while retaining its type and actions.
Each recipient retains its own Endpoint, schema, subscription identity/parameters,
and lifetime. There is no global atomic transaction across different recipients.

Schema compilation checks each declared report/destination pair: the stable
variant admits the report, every path names an unambiguous reachable destination,
and source-to-destination type copying is valid. No match does not authorize
name-based fallback.

A field carrying a complete array with the destination's element type replaces
that whole array, including removals and an empty array. This differs from a
reduced-element array projection, which retains the correspondence required by
its copy contract. CopyTo does not imply AppendTo, insertion, or arbitrary
recursive JSON patch semantics.

Prepare and validate the complete resulting working value before publishing
any assignments. An invalid applicable path, stale binding, incompatible value,
or violation of schema-expressible value constraints rejects that working
value's update atomically.
Unrelated fields, the stable Protocol Variant, presentation, and actions remain
unchanged. The reduced Protocol Variant carries CopyTo for its whole payload;
the selected destination type determines the checked field projection.

#### Snapshot, mutation, and publication consistency

Every SubscriptionRequest, including repeated subscription and resubscription,
receives a fresh complete current stable snapshot. A reduced report arriving
before its required baseline is discarded, not saved for later replay. Snapshot
capture and the transition to subsequent reports must not lose a committed
change or reorder publication. Renew the baseline through a subscription request
when the object, filter, or subscription context changes.

A partial-update Request changes authoritative App state after validation and
recomputes affected derived values. Its correlated Response acknowledges that
request; subscription publication is a separate operation. Use an ordinary
non-transition success acknowledgement when the working screen should remain,
and Error for a rejected request. A rejection must not publish the proposed
values as accepted state.

The App is responsible for designing its protocol and transactions to preserve
data consistency across every permitted interaction and state-transition path.
This includes partial/full requests, concurrent clients, devices, timers,
background work, restored state, subscriptions, and restart recovery. The App
coordinates authoritative state changes, affected derived or hidden values,
and publication order; fields that must change together belong in one atomic
report, or the protocol must explicitly coordinate their transition.
NextResponse and CopyTo describe acceptance and typed placement. Schema and
runtime checks enforce their declared structural constraints, but do not prove
application-level consistency across all paths or supply missing transaction
logic. For the same committed state and subscription context, the ordered
reports must produce the complete working value that a fresh snapshot would
describe. App verification must cover the snapshot boundary, concurrent changes,
rejected requests, other mutation paths, changed subscription contexts, and
restart/resubscription. The Hub routes reports without retaining their payloads
or merging them.

## Default and explicit subscriptions

If a protocol declares multiple SubscriptionRequest variants with effective
Nil payloads, only the first one in Union declaration order is the automatic
default subscription. Declaration order therefore matters.

Other subscriptions, including later Nil variants and every argument-bearing
variant, require explicit invocation. The Hub does not retain default-subscription payloads or merge compatible
reports. Each Control tracking admission or reconnect obtains a fresh initial
snapshot from the App through the default SubscriptionRequest. The default may
share one underlying stream, but each user is independently checked against that variant's
`AccessPrivilege` before receiving current or subsequent data. A privileged
user cannot authorize another user's subscription. Apps must not infer peer
identity from how that stream is shared.

For native-client screen lifetime and endpoint-wide explicit release behavior,
see [endpoint presentation](endpoint-presentation.md#protocol-transaction-stack).

The server starts/renews its application stream state only after receiving the
current SubscriptionRequest callback. Host or App restart does not authorize a
server to emit data from remembered application state without a fresh routed
subscription callback.

## Endpoint runtime rules

The public SDK provides operations to:

- register an Endpoint callback;
- send Request or SubscriptionRequest data;
- send a correlated Response;
- publish SubscriptionData; and
- publish the standardized `PeerAlive` signal.

Follow these invariants:

- Branch on the callback action before decoding `data`. `PeerUp`, `PeerDown`,
  `PeerAlive`, and `PeerTimeout` are signals and carry no application payload.
- A server-side peer value is opaque. Retain it and pass it back unchanged.
  Never interpret it as an App, task, user, endpoint, session, or object ID.
- Return the exact nonzero transaction ID and peer supplied with a Request when
  sending its Response.
- Complete an accepted SubscriptionRequest with a correlated Response carrying
  the complete current snapshot, echoing its transaction ID and peer. When the
  same snapshot variant is also used for later reports, declare both `Response`
  and `SubscriptionData` on it. Publish later changes through the report API.
  Sending only a report does not complete the opening transaction and can leave
  the client waiting indefinitely. Rejection also requires a correlated reply.
- Omit the peer for normal common reports/liveness and let the host fan out.
  Target a peer only with a currently valid callback-provided handle.
- Use `PeerAlive` when a liveness cadence is required without changed data.
  Never fabricate a schema-level “no change” value or resend unchanged data
  merely to reset a watchdog.
- `PeerAlive` does not update data, cache, UI, sequence, cursor, or recovery
  state. Protocols that need loss detection carry sequence/revision/snapshot
  information in real application messages.
- A client resubscribes after a delivered server `PeerUp`. It must not assume a
  previous server process retained stream state.

The full normative lifecycle and authorization rules are documented on
`LibertasEndpoint`/`LibertasEndpointCallback` in the public SDK. Do not model
private acknowledgements or host subscription records in App code.

## Persistent data schemas

### Persistence failure and the no-status API

By default, **a Data API operation succeeds or the Hub runtime stops the
affected task**. Reads can also report ordinary absence. Writes and removals
retain their existing no-status signatures. Failures must never appear as
success, missing records, empty collections, or default values. Termination is
owned and enforced by the Hub runtime, not by an SDK or App recovery handler.

The Hub distinguishes data failures internally and reports the existing runtime
`Error` status while preserving the persisted enabled/configured status. It must
not permanently disable the task in the database or automatically restart it.
Starting, restarting, and enabling use the same sufficient `Enable` primitive.
Only a third party may issue `Enable` after a data failure; preserving the
configured enabled state never schedules it automatically.
If restarted after storage repair, the App follows ordinary startup and loads
persisted data. No additional recovery logic is required for an ordinary App.

Most Apps depend on persistence having succeeded before acknowledging a change
or making subsequent decisions. They cannot repair the storage device, and
requiring all Apps to maintain consistency across storage failure is impractical.
Network interruption is different: a client can lose contact while the server
still maintains authoritative state. Stopping remains the default for data
failures, without erasing stored data or substituting defaults.

#### Explicit error-status opt-in

A public task function may declare the marker `DataErrorStatus=true` using
`@LibertasDataErrorStatus` in TypeScript or `#[libertas_data_error_status]` in
Rust. Omission retains the default. It is function-only, takes no arguments,
and cannot be inherited from a type, package, or dependency. Studio validates
placement, multiplicity, and marker value; the Hub runtime enforces the selected
task function's opt-in.

The opt-in permits checked counterparts of every Data API operation:
`libertasDataTry…` in TypeScript and `libertas_data_try_…` in Rust. They perform
the same operation and return its status directly and synchronously. Existing
ordinary calls retain their behavior, including Hub-owned termination on failure.
Using a checked call without the marker is a task error, not an alternative way
to bypass termination. A matching Hub and SDK with checked-operation support
are required.

| Status | Meaning |
| --- | --- |
| Success (0) | The operation completed; read results may contain ordinary absence. |
| NotFound (1) | Internal absence outcome, exposed as a successful optional read. |
| InvalidRequest (2) | Invalid operation arguments or indexed handle. |
| StorageError (3) | The backing storage operation failed. |
| DataIntegrityError (4) | Stored bytes could not be interpreted as valid data. |
| ResourceLimit (5) | The operation exceeded available resources. |

Rust checked calls return `Result<T, DataError>`; missing records are `Ok(None)`.
TypeScript checked calls return `{ status: DataStatus.Success, value: T }` or
`{ status: errorStatus }`; a missing record is successful `undefined`. Writes
and removals return a successful unit/void value. Removing an absent record is
successful. Failure never carries usable partial read results. A failure is
not a durability receipt or a guarantee that retrying an operation is safe.

An opted-in App owns deciding how to proceed after a reported failure and must
preserve its own consistency: it cannot acknowledge failed persistence as
saved data or treat failure as absence. The flag does not repair storage,
provide a cross-call transaction, or authorize automatic task restart.

### Declaring the data schema

A function may name one persistent-data Union:

```typescript
/**
 * Recording choices
 * Describes the sensor whose measurements the recorder saves.
 */
export interface Configuration {
    /**
     * Measurement sensor
     * The user chooses the sensor that supplies recorded temperatures.
     */
    sensor: LibertasDevice;
}

/**
 * Saved temperature
 * Represents one temperature retained from the selected sensor.
 */
export interface StoredSample {
    kind: 0;

    /**
     * Measured temperature
     * Records the temperature observed by the selected sensor.
     * @LibertasPhysicalUnit("celsius")
     */
    temperature: number;
}

/**
 * Saved recording position
 * Marks how many temperatures were safely retained before recording paused.
 */
export interface StoredCheckpoint {
    kind: 1;

    /**
     * Saved temperature count
     * Records how many temperatures were retained before recording paused.
     * @LibertasNumber(type=u64)
     */
    sampleCount: number;
}

/**
 * Saved temperature record
 * Describes recorded temperatures and the position from which recording can
 * continue.

 */
export type StoredData = StoredSample | StoredCheckpoint;

/**
 * Record sensor temperatures
 * The recorder saves temperatures so the user can continue the same record
 * after recording starts again. The user chooses the sensor to observe. The
 * user can stop and restart recording without discarding saved temperatures.
 * The recorder automatically saves each new temperature and the latest saved
 * position. The recorder shows the latest temperature and saved count and
 * warns the user when the selected sensor is unavailable. A compatible
 * temperature sensor must remain available while recording. Developed by
 * Example Developer (example_developer).
 * @param configuration Recording choices
 * The user chooses the sensor whose temperatures the recorder saves.
 * @LibertasDataSchema(StoredData)
 */
export function startRecording(configuration: Configuration): void {}
```

```rust
/// Recording choices
/// Describes the sensor whose measurements the recorder saves.
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    /// Measurement sensor
    /// The user chooses the sensor that supplies recorded temperatures.
    pub sensor: LibertasDevice,
}

/// Saved temperature record
/// Describes recorded temperatures and the position from which recording can
/// continue.
#[derive(LibertasExport, LibertasAvroDecode, LibertasAvroEncode)]
pub enum StoredData {
    /// Saved temperature
    /// Represents one temperature retained from the selected sensor.
    Sample {
        /// Measured temperature
        /// Records the temperature observed by the selected sensor.
        #[libertas_physical_unit("celsius")]
        temperature: f64,
    },

    /// Saved recording position
    /// Marks how many temperatures were safely retained before recording
    /// paused.
    Checkpoint {
        /// Saved temperature count
        /// Records how many temperatures were retained before recording paused.
        sample_count: u64,
    },
}

/// Record sensor temperatures
/// The recorder saves temperatures so the user can continue the same record
/// after recording starts again. The user chooses the sensor to observe. The
/// user can stop and restart recording without discarding saved temperatures.
/// The recorder automatically saves each new temperature and the latest saved
/// position. The recorder shows the latest temperature and saved count and
/// warns the user when the selected sensor is unavailable. A compatible
/// temperature sensor must remain available while recording. Developed by
/// Example Developer (example_developer).
#[libertas_export]
#[libertas_data_schema(StoredData)]
pub fn start_recording(
    /// Recording choices
    /// The user chooses the sensor whose temperatures the recorder saves.
    configuration: Configuration,
) {}
```

`@LibertasDataSchema(Type)` associates the function with its persistent Union;
Studio supplies the Union marker, matching the Rust function-associated model.
Rust uses the one public function attribute shown above and Studio marks its
resolved Union. Replace the example developer identity before publication.

The target may be local or an external resolved App type and must be a valid
data Union. For payload design, follow the
[formatted record-payload guidance](presentation.md#persisting-formatted-record-payloads).

Keep that persistent-data Union's name versionless and invariant across App
versions. Append each new persistent payload type as a new final variant of the
same Union so newer App versions continue to recognize all older stored values.
Do not replace it with a version-named Union or mutate an existing payload case.
See the [App data-type design guidelines](data-type-evolution.md).

### Data and database names

Use the public SDK Data APIs for actual records. Every single record and every
indexed database or logical table has one `DataName`:

```text
DataName tuple = package-local resource key + ordered typed message arguments
identity bytes = canonical_encode(DataName tuple)
human name     = LMF1(localized resource template, resolved arguments)
```

The canonical byte blob is the only data name. It simultaneously provides a
stable locale-independent database key and all information needed to select a
localized template and render the typed argument data for a person. The blob
does not contain the template: the task identifies the App package, the key
selects a template in that package's `StringResources`, and the arguments carry
self-describing values.

The task App package always owns the data-name resource. This remains true when
`FunctionDataSchema` refers to a type in another App: the foreign type defines
the stored value shape, not a second catalog or data-name owner. The tuple never
carries a function, type, or resource FQN. Studio can validate a declared
catalog against its package document, but it cannot prove keys constructed in
dynamic Data API calls; code review must enforce this invariant.

Declare every data-name key in the package catalog and write a complete natural
LMF1 template. For example:

```typescript
/// Irrigation history
/// Records watering activity for each configured growing area.
/// @LibertasStringResources(DATA_NAMES)

const DATA_NAMES = {
    ZONE_HISTORY: "Watering history for {0,LibertasDevice}",
} as const;

const zoneName: LibertasMessageArgument[] = [
    { type: 1, value: zoneValve },
];

const history = libertasDataOpenIndexed("ZONE_HISTORY", zoneName);
```

```rust
//! Irrigation history
//! Records watering activity for each configured growing area.
//! #[libertas_string_resources(DATA_NAMES)]

const DATA_NAMES: &[(&str, &str)] = &[
    ("ZONE_HISTORY", "Watering history for {0,LibertasDevice}"),
];

let zone_name = [LibertasMessageArgument::Object(zone_valve)];
let history = libertas_data_open_indexed("ZONE_HISTORY", &zone_name);
```

Use the installed SDK's exact argument constructors and Data API signatures.
Never pre-render the name, pass literal human text as the key, Base64-wrap the
canonical bytes, or maintain a separate machine name. Reuse the exact resource
key, argument order, argument types, and argument values for every read, write,
open, and removal of the same logical identity. A locale or template wording
change does not rename data; changing any encoded identity component does and
requires an intentional migration.

Single-record and indexed-database names occupy separate namespaces even when
their bytes match. Within an indexed database, each row's signed 64-bit index
is additional identity and is not an LMF1 argument. `DataName` is a supported
storage-identity mechanism; the recommendation about
[formatted record payloads](presentation.md#persisting-formatted-record-payloads)
does not discourage its use. See
[Libertas Message Format](message-format.md#data-and-database-names).

## Function and Endpoint permissions

A function permission constant requires a same-key nonempty reason in the
package `StringResources`. A server Endpoint may declare downstream
permissions without same-key reason resources; the selected client task
must have those permissions when the relationship is configured.

Never substitute `AccessPrivilege` for a process permission or vice versa.

## Backend-constrained choices

`ConstrainedBy` on a field constrains an Enumeration or Union in protocol
request data. On a response, it instead constrains [NextRequest actions](#response-action-constraints).

The following rules describe the field-level form. Rust uses `#[libertas_constrained_by("$.allowed_choices")]`;
TypeScript uses `@LibertasConstrainedBy("$.allowed_choices")`. The one quoted
path uses the [root-relative Libertas Schema Path form](schema.md#context-and-syntax)
and resolves against the **current request's selected payload**.
It is not an expression, an interaction reference context lookup, or a list of alternatives.

The source is a non-nullable list of non-nullable integers. It must be Hidden,
either directly or inside a hidden cookie. The service writes it and the user
never does: populate it from backend response data through CopyFrom, including
copying its containing cookie, supply it directly in EditRequest, or assign it
from a field calculation reply through CopyTo under the
[calculated allowed sets](#calculated-allowed-sets) rules. Between those
writes the set is stable; ordinary editing never replaces the source or its
containing data.

For an Enumeration, entries are its declared numeric values. For a Union,
entries are its original schema discriminants, never positions in the filtered
chooser. `[1]` permits only choice 1; `[0, 2]` permits choices 0 and 2. An empty
list permits no choice and blocks submission. Duplicate entries do not create
extra choices. Null is not a numeric choice; ordinary Nullable semantics still
apply when the allowed set is nonempty. Undefined remains unanswered and cannot
be submitted for a non-nullable field.

Clients show only permitted choices and validate the selection before sending.
A selected Union branch retains its ordinary editable payload and nested-field
validation. A singleton allowed set does not construct an unfinished payload
on the server or automatically submit the editor. Missing or malformed cookie
data, an empty allowed set, or a prefilled choice outside the set blocks
submission with a field error. Clients do not silently replace a conflicting
prefill; the application must supply consistent values and cookies.

Studio checks field placement, effective enum/union type, root-path resolution,
integer-list source shape, hidden source ancestry, and the calculation writers
of each source in every request occurrence. Runtime clients check actual cookie
values and selection membership, and reject user edits of a source. These are
ordinary client data constraints; the backend continues to own
application-level combination rules.
Historical edits must follow the [saved-page consistency rule](#saved-page-consistency-after-historical-edits)
so resumed values and allowed sets remain consistent.

### Calculated allowed sets

A successful [field calculation](#field-calculation-requests) Response field
may `CopyTo` a ConstrainedBy source, or data containing it, under the ordinary
CopyTo rules. This lets the service change which choices a page offers while
the page stays open, from what has been entered so far.

Studio treats such a calculation as a writer of every field the source
constrains, in addition to the source itself, for the overlapping-writer and
cycle checks. Consequently two calculations cannot write the same source, a
calculation cannot also write a field whose allowed set another calculation
writes, and a calculation cannot read, directly or through other calculations,
a field whose allowed set it writes. The last rule is what makes a reply unable
to trigger itself: the acyclic dependency graph is proved with the constrained
fields counted as outputs.

When a client applies a result that changes a source, it re-filters every
chooser reading that source. A current selection inside the new set, and its
payload, are untouched. A current selection outside the new set is reset
through ordinary initialization exactly as a calculation destination is reset,
discarding a removed Union branch's payload with it; that reset is an input
change for calculations reading the field and propagates through the
dependency graph like any other result. The reset and the assignment are one
atomic editor change. A source reset while its calculation is pending, because
an input changed, does not reset the selection: the chooser offers the reset
set until the reply arrives, the selection is retained, and submission
validates it against the current set as usual.

A restored [draft](annotations.md#cacheidentity) brings its calculated
sources with it and the seeded calculations recompute them; a source no
calculation writes must still match the current editor.


## Default request entry and dynamic actions

`#[libertas_default_request]` / `@LibertasDefaultRequest` marks at most one
Request variant in an Endpoint protocol Union. It must carry Request, cannot
be ExcludeUI, and its effective payload must need no user input: Nil, or a
Table whose every field is Hidden or ReadOnly and carries Fixed (including
`Fixed("current-user")`) or a Default, applied recursively to nested Tables.
The client initializes such a payload and sends it without opening an editor;
a `Fixed("current-user")` field is filled with the authenticated user under
its [security constraint](schema.md#fixed-current-user-security-constraint),
so a per-user endpoint can identify the caller from its first request. It
requires no reference context. A chart-producing request cannot be
DefaultRequest: none of its declared NextResponse payloads may contain Chart or
Chart3D, including referenced types and nested fields. Studio validates these
conditions. DefaultRequest is optional; it is never inferred from an
input-free payload.

An explicit DefaultRequest owns initial interactive entry, including when the
protocol also has a default subscription. Without the explicit marker, the
existing default subscription remains the initial data source. With neither,
the landing presents the available requests in declaration order, using chart
previews instead of rows for chart requests with executable inputs. An
input-free payload alone never makes a Request the default. Invoke an explicit default
request as a client-owned correlated
Request, never as a pooled subscription. While pending, show loading; an error
keeps retry available. A late or duplicate reply cannot reopen an interaction
that the user has left. Do not automatically replay an entry request merely
because a view recomposes or a subscription reports new data.

If no default is available, retain explicit request selection. Once a default
has been selected, never fall back to listing all protocol requests on failure.
DefaultRequest does not authorize automatic submission of an editor. Existing
WorkflowRequest and EditRequest replies open their editor without submitting.

A normal response with no visible content presents only its NextRequest
choices; its reference context follows the
[interaction reference context rule](#interaction-reference-context). Hidden
fields and hidden ancestors contribute no visible content. Nil and containers
with no visible descendants contribute none; a visible scalar or List still
counts even when its value is empty or null. Apply existing presentation rules
recursively, including union selection. A response with visible content opens
the ordinary response screen. This presentation rule does not turn a terminal
acknowledgement into a navigation transition. An action response whose dynamic
set is empty retains its action surface with no actions.

### Single-action confirmation and input presentation

Android, iOS, and Libertas Studio preserve the request's entry point when
choosing its presentation. A direct link already expresses the user's intent
to run that request; opening **Actions** is not a direct link. Do not infer
that distinction from the number of choices, the presence of reference data,
or the request's payload type.

| Entry point and constructed request input | Presentation |
| --- | --- |
| Direct link, no editable input fields | Validate and send immediately, without a confirmation dialog. |
| Actions, no editable input fields | Show a dialog naming the action with **Send** and **Cancel**. This also applies when Actions has only one eligible request and skips its chooser. |
| Either entry point, one editable primitive or enum field | Use the existing one-field popup with its normal confirmation and cancellation controls. |
| Either entry point, more complex input | Use the existing full request editor with its normal submit and cancellation controls. |

Use the existing request-composition rules after initialization, including hidden
subtrees and constrained choices; count input fields, not rendered layout rows.
Preserve the Actions origin through chooser selection and deferred presentation.
Cancel or dismiss sends nothing and preserves the current protocol data and
reference context. Opening confirmation does not start a pending request.
Send uses ordinary validation and request correlation and may submit only once.
A confirmation from an interaction that has been left or replaced cannot send.
Recomposition, repeated taps, or returning to a retained response must not
replay the action or reopen a cancelled confirmation automatically.

Static and constrained NextRequest sets follow the same entry-point rule.
Automatic singleton selection does not turn a direct link into an Actions
invocation. Existing explicit DefaultRequest entry, default subscriptions, and
chart previews retain their own rules. WorkflowRequest and EditRequest
continue to require their existing explicit editor submission.

### Control endpoint landing and native Back

A task with multiple endpoints opens an **endpoint list**. Default subscription
data may be displayed directly under an endpoint in that list; only this
inline subscription data also serves as that endpoint's landing. Do not create
a duplicate subscription landing when its endpoint header is tapped.

Other landings, including chart and workflow landings, are not displayed or
entered in the endpoint list. Tapping such an endpoint enters its landing and
stacks it above the list. With exactly one endpoint, enter its landing directly,
including a subscription-data landing: never create, display, or retain an
endpoint-list screen underneath it.

Initial endpoint entry has **no reference context** and follows default-entry
selection above. Its immediate stable destination may be SubscriptionData, a
Response, or a Request editor. An initial Response can be stable without
NextRequest; do not suppress it as an acknowledgement of a previous screen.
Transient cookie responses opening an editor are not separate destinations.
Identify the retained base by its actual navigation ownership, never by variant
name, payload equality, or all variants reachable through NextResponse.

A **working screen** may be stacked above the retained endpoint landing. For
an inline subscription landing, its retained base screen is the endpoint list.
It can display an explicitly requested subscription's data, a response, or a
stable EditRequest/WorkflowRequest editor. Successive working results replace
or update that working destination rather than accumulating historical screens.
The landing remains underneath with its own data and subscription ownership;
working data must not overwrite its context. An explicitly selected subscription
request may create a working subscription screen, even though the base already
contains a default subscription landing.

For native Back in a **stable Control endpoint destination**:

- From a working screen, abandon that working interaction and reveal the
  retained endpoint landing (or the endpoint list for an inline subscription
  landing). Refresh the current endpoint's workflow
  landing as specified below. Do not recreate the base or add a duplicate
  subscription landing.
- From a separately opened landing, return to the endpoint list for a
  multi-endpoint task; exit the task for a single-endpoint task.
- From the endpoint list itself, exit the task.

Returning preserves the base screen, with refresh determined by the current
endpoint's landing:

- **Workflow landing:** refresh it through its landing entry request so the
  backend supplies current workflow state and legal actions. Construct that
  refresh with no reference context, as for landing entry; never inherit the
  abandoned working screen's response or cookie. Install the result in the
  retained endpoint landing rather than pushing another landing screen.
- **SubscriptionData landing:** keep its live subscription and current data.
  No Back-triggered refresh or resubscription is needed because incoming
  subscription data already refreshes it. This also applies when the dismissed
  working screen was a workflow editor over a subscription-data landing.

Workflow landings refresh within their separately opened landing screen, never
by entering a workflow from the endpoint list during Back. Preserve the endpoint
list and its other endpoints' live subscriptions beneath that landing. A working
workflow editor above an inline subscription landing returns to the endpoint
list without initiating a workflow landing refresh. A genuinely new endpoint entry
still starts without a reference context.

These rules apply to the Back button and system back navigation alike. A
[transient request composer](endpoint-presentation.md#protocol-transaction-stack)
first follows its cancellation and pending-response rules. A local picker or
modal retains its ordinary dismissal behavior. Abandoned pending replies,
calculations, and explicit subscriptions cannot navigate or overwrite the
retained base. Back does not undo a request already accepted by the backend.
Workflow PrevRequest remains a separate explicit protocol action.

### Response action constraints

On a Response variant, ConstrainedBy constrains NextRequest instead of a field
choice. The response must declare NextRequest and cannot carry WorkflowRequest
or Error. Its one root-only path resolves against that response payload to a
hidden non-nullable list of non-nullable plain integers, using the same source
shape and path grammar as field-level ConstrainedBy.

NextRequest declares the complete possible action set and its display order.
The ConstrainedBy array supplies membership only: its order never reorders
actions. For example, NextRequest `A,B,C` with allowed discriminants for
`C,A` displays `A,C`. No separate order attribute is needed.
Each integer in the response's allowed array identifies an original protocol
Union discriminant, never a position in NextRequest or a filtered action list.
Union members carry explicit consecutive `Discriminator` values starting at zero;
Children are pre-sorted by that value. Runtime initialization,
legacy-schema fallback, and the separation from payload `Fixed` constraints are
defined in [Unions](schema.md#54-unions).
Only declared actions in the allowed set are displayed, after the ordinary
access and ExcludeUI restrictions. An empty array is valid and means no actions;
duplicate entries do not duplicate actions. If the constrained set, after access
and visibility filtering, contains exactly one action, select it automatically
once for that accepted response, then apply the
[single-action presentation rule](#single-action-confirmation-and-input-presentation):
fieldless requests from Actions require Send/Cancel confirmation; direct links
send without that dialog. One primitive or enum uses the existing one-field
popup, and complex input uses the full editor. Preserve the entry point through
selection. Never auto-submit a workflow editor or
serialize an unfinished request. Rendering or returning to
the same retained response must not repeat the selection. Without ConstrainedBy, NextRequest
remains static. Missing, malformed, out-of-range, or undeclared entries reject
the action update; clients never fall back to the unconstrained set.

The set belongs to the installed response and is replaced only with a newly
accepted response or subscription update. It is not evaluated against an editor
or another interaction. Every action entry point, including row actions and
menus, uses the same filtered set. Backend authorization and state checks still
apply when a selected request arrives.

```rust
#[libertas_response]
#[libertas_next_request("Start,Resume,Review")]
#[libertas_constrained_by("$.allowed_actions")]
Actions {
    #[libertas_hidden]
    allowed_actions: Vec<i32>,
    #[libertas_hidden]
    cookie: String,
}
```

Studio checks response placement, NextRequest presence, exclusions, source
resolution, and hidden integer-list shape. Clients validate entries against
declared actions before installation.
