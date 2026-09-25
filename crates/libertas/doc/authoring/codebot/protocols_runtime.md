# Endpoint protocols and runtime

Use the [Libertas Schema Path specification](../reference/schema.md#libertas-schema-path)
for path syntax and context; operation contracts below define their use.

This page is the compact App-authoring projection of the full
[Endpoint protocols and runtime contract](../reference/protocols.md). Use the
installed public SDK for exact callback and function signatures.

## Declaring an Endpoint

- Apply `EndpointSchema` to a `LibertasEndpoint` field and reference a protocol
  Union. Add `EndpointServer` only when this App serves it.
- `EndpointBaseObjects` takes comma-delimited Libertas Schema Paths.
- On a client Endpoint, `Default` and `Fixed` take
  `developer-package::function`. Compatibility is always evaluated first.
  `Default` keeps every compatible server; `Fixed` restricts servers to the
  named task process and alone contributes its package to `AppDependencies`.
  Both Endpoint selectors remain editable.
- If such an Endpoint occurrence is non-nullable through every ancestor and
  every ancestor List requires at least one item, task creation waits for an
  eligible server. Either value names the peer task to provision; see the
  [canonical dependency check](../reference/schema.md#62-task-pre-creation-static-dependency-check).

## Protocol roles

Every effective protocol variant has at least one role: `Request`, `Response`,
`SubscriptionRequest`, or `SubscriptionData`. A variant may have multiple
roles. TypeScript uses a stable numeric first-field discriminator; Rust uses
enum declaration order.

`Error` accompanies `Response` and classifies the complete response as
out-of-band of the state transition without changing its payload or wire form.
Put its best user explanation in the first renderable `FormattedText`; clients
preserve the current interaction and retry state.

## Request access

`AccessPrivilege` is an alternative bitmap on Request/SubscriptionRequest:
`Read=1`, `Write=2`, and `Read,Write=3`; omission is `3`. Explicitly mark every
mutating operation `Write`. This is distinct from function `Permissions`.

## Chaining, caching, and copied values

`NextRequest` and `NextResponse` contain native variant names. Protocol
request/reply payload caching has no attribute and is not supported. For
CopyFrom, use the [shared copy rules](../reference/schema.md#shared-copyfrom-and-copyto-rules)
and [operation context table](../reference/schema.md#context-and-syntax);
do not invent per-action grammar or a stored previous request. Use the
[current interaction reference context](../reference/protocols.md#interaction-reference-context);
reads follow the [availability contract](../reference/schema.md#availability-null-and-interaction-ownership),
including explicit null handling when composing for the first time. Outside
workflows and automatic WorkflowRequest entry, an ordinary Response without
`NextRequest` is a non-transition acknowledgement (often a shared
`Success`/`Failure`), not permission to offer every Request. Clients
preserve the displayed protocol data and choices while showing its result
message transiently; reference context follows the interaction rule above. See the
[protocol-role contract](../reference/protocols.md#protocol-roles). A
SubscriptionData/DataReport operation still refreshes live state even when it
uses the same dual-role variant.

Carry session/continuation data and values supplied only to initialize later
requests in typed Hidden protocol fields (or a Hidden context container).
Copy them into receiving requests; keep coordination fields Hidden but leave
user-editable initialized fields visible. Complete EditRequest values already
include this data. Check every construction path and context replacement:
Hidden neither initializes nor exempts a field from validation, and old
responses are not searched. First entry must work without prior context.
Hidden is not authorization; validate submitted context on the server. Follow
the complete [hidden-context rule](../reference/protocols.md#hidden-protocol-context).

## Default entry and backend-selected actions

- A parameterless Request is not implicitly a default. Without a default entry,
  the landing lists requests, with executable chart requests shown as previews.
- Mark at most one input-free Request with `DefaultRequest` (Nil, or only
  hidden/read-only Fixed or Default fields such as a hidden
  `Fixed("current-user")` caller); it owns initial
  entry without needing user input or a reference context. Chart-producing
  requests cannot carry this marker. With no explicit marker, retain default
  subscription behavior; with neither, present the request-list landing.
- The entry reply determines subsequent interaction. `NextRequest` declares
  possible actions **in display order**. Never expose all protocol requests
  after an entry failure or a response with no NextRequest.
- Response-level `ConstrainedBy("$.allowed_actions")` reads a hidden integer
  array from that response. Entries are original protocol Union discriminants,
  not NextRequest positions. Union members use explicit `Discriminator`, independent
  of payload `Fixed`. Children are pre-sorted with Discriminator exactly 0 through
  N−1; reject gaps, duplicates, and mismatched ordering. Decode by checked index.
  Missing Discriminator uses declaration order; all runtimes must
  never interpret Fixed as variant identity or rewrite stored schemas. See [Unions](../reference/schema.md#54-unions).
  The array determines membership only; filtering
  preserves NextRequest order. Empty permits no actions; duplicates do not
  duplicate actions. A singleton after access/visibility filtering is selected
  automatically once per accepted response. Follow the
  [single-action presentation rule](../reference/protocols.md#single-action-confirmation-and-input-presentation):
  fieldless direct links send without confirmation; fieldless requests from
  Actions require Send/Cancel even for a singleton. Preserve origin through
  selection. One primitive or enum uses the existing one-field popup; complex
  input uses the full editor. Cancel sends
  nothing and preserves the current interaction; stale or repeated confirmations
  cannot send. Never auto-submit workflow editors.
  Invalid entries reject the update without an unrestricted
  fallback. Existing access and ExcludeUI checks still apply.
- Rows offer actions: a legal Request whose CopyFrom resolves to a displayed
  row is that row's tap action (a direct link). To make every member of a
  Response List tappable, copy through its member type
  `^<TopicRow>.section,$.suggested`; with no tapped member (Actions,
  menus, automatic selection) the next alternative applies. Such a Response
  declares no `ConstrainedBy`. See [row actions](../reference/protocols.md#row-actions).
- A response with no visible content presents only its legal actions while
  retaining hidden cookies for CopyFrom. Visible content presents a data
  screen. WorkflowRequest and EditRequest open editors without auto-submission.
- Keep the backend's authorization and current-state checks on submission;
  action visibility is not authorization.

Follow the precise [default-entry and dynamic-action contract](../reference/protocols.md#default-request-entry-and-dynamic-actions),
including source validation, precedence, pending ownership, and error handling.

## Protocol conformance

Treat a protocol as capabilities; matching selects corresponding capabilities,
then ordinary serialization handles their compatible payloads. See the
[capability and prefix distinction](../reference/protocols.md#protocols-describe-capabilities).

Keep one versionless protocol Union and append new cases only at the end.
`ProtocolConformance` uses `&`, `|`, `!`, and `^` over native variant names to
declare mandatory peer capability. Matching payloads must remain pass-through
compatible. A client checks the selected server's runtime variant indices
before sending an optional request.

## Default and explicit subscriptions

The first effective-Nil `SubscriptionRequest` in declaration order is the sole
automatic default. Every other subscription is explicit. A server starts or
renews stream state only after receiving the current subscription callback.

## Endpoint runtime rules

Follow the public SDK callback contract. Branch on action before decoding data;
signals carry no payload. Treat peer handles as opaque. Answer SubscriptionRequest
with a correlated complete snapshot (exact callback peer and transaction); use
reports for later changes. A shared snapshot variant needs both Response and
SubscriptionData roles. A report alone leaves the opening request unanswered.
Use `PeerAlive` rather than inventing a “no change” payload. See
[endpoint runtime rules](../reference/protocols.md#endpoint-runtime-rules).

## Persistent data schemas

Default Data API failure: Hub runtime stops the task, reports runtime Error with internal data-failure classification,
retains persisted enabled state, and never automatically restarts it. Only an
explicit function `DataErrorStatus` permits checked status-returning calls;
those Apps own consistency after failure. Absence is not failure. Any restart
uses a third-party `Enable` action and ordinary startup. See the
[owner](../reference/protocols.md#persistence-failure-and-the-no-status-api).

One function may reference one versionless persistent-data Union. TypeScript
uses `@LibertasDataSchema(Type)` on the function and no standalone Union marker;
Rust uses
`#[libertas_data_schema(Type)]` on the function. Append new variants only at the
end. Prefer structured record payloads; persisting `FormattedText` is permitted
but discouraged under the
[payload guidance](../reference/presentation.md#persisting-formatted-record-payloads).
That recommendation does not apply to DataName names.

### Data and database names

Every single-record or indexed-table name is the canonical byte encoding of one
package `StringResources` key plus ordered typed message arguments. Reuse that
exact tuple for the same identity; never use rendered, literal, separately
maintained, or Base64 names. See
[Libertas Message Format](../reference/message-format.md#data-and-database-names).

## Function and Endpoint permissions

Every function permission needs a same-key nonempty package string-resource
reason. A server Endpoint may declare downstream permissions without reasons;
the selected client task must already hold them. Neither form substitutes for
per-request `AccessPrivilege`.

Transient request composers preserve their originating screen: cancellation pops
only the composer, and pending submission waits for its correlated reply. See
[transaction stack](../reference/endpoint-presentation.md#protocol-transaction-stack).

Multiple endpoints use an endpoint list. Only default subscription data shown
inline there also serves as an endpoint landing; never duplicate it. Other
landings open on endpoint tap above the list. Exactly one endpoint enters its
landing directly, with no list in the stack. Retain each landing below its
working screen. Back reveals that landing, refreshing workflows in place with
no reference context but preserving live subscriptions. Back from a separate
landing returns to the list or exits a single-endpoint task. Fence abandoned
work. See [the owning contract](../reference/protocols.md#control-endpoint-landing-and-native-back).

Calculation CopyFrom/CopyTo supports relaxed `^<Type>.field` and strict
`^<Variant.field.item>.field` Libertas Schema Paths from the active variant. The variant
root itself is selectable, including primitive payloads; no lookup crosses into
the protocol Union. Studio validates every invoking occurrence; clients retain
its original value bindings, never a variant name alone. See [typed ancestor calculations](../reference/protocols.md#typed-ancestor-paths-and-per-item-calculations).

Within an Endpoint interaction, correlated request/reply transactions preserve
request send order. Deferred calculations use one ordered queue per editor: a
new change removes that instance's queued entry and appends it to the tail.
Keep the in-flight transaction's invoking node immutable; rebuild and validate
deferred requests only at dispatch. See [ordering](../reference/protocols.md#request-and-response-ordering)
and [deferred calculations](../reference/protocols.md#changes-resets-and-deferred-calculation).

A calculation request's Fixed inputs need no CopyFrom: a hidden
`Fixed("current-user")` User field is resolved from the session and verified by
the Hub like any request, and any other Fixed input takes its declared value;
every non-Fixed input declares CopyFrom.

A calculation reply may `CopyTo` a field-level `ConstrainedBy` source and so
change a page's allowed choices while it is open. Studio counts that
calculation as a writer of every field the source constrains, so it cannot
read such a field directly or through other calculations, and no other
calculation may write it. Applying the reply re-filters the chooser: a
selection outside the new set resets through ordinary initialization and
triggers the calculations reading it; one inside it is untouched; a source
reset while pending never resets the selection. See
[calculated allowed sets](../reference/protocols.md#calculated-allowed-sets).

Array removal uses dependency scope, not later row positions: cancel removed-item
work, preserve unaffected surviving item-local work, and reschedule calculations
associated with the array which cross the item boundary or consume the whole
array/projection. Never retarget a stale positional write. See
[array removal](../reference/protocols.md#array-removal-and-calculation-scope).

## Compatible subscription updates

- SubscriptionData A may declare NextResponse(B,C) to accept directional root-field
  updates from B/C while retaining A's screen, actions, and unmatched fields. All
  members are non-null Tables; matching fields must be assignable. Apply atomically.
- The App chooses whether to send compatible reports and owns consistency and
  ordering. Every SubscriptionRequest must return a complete current snapshot.
  The Hub routes without retaining payloads or merging reports. See the
  [owning contract](../reference/protocols.md#compatible-subscriptiondata).

An App may adopt the [Reduced SubscriptionData design pattern](../reference/protocols.md#reduced-subscriptiondata-design-pattern),
combining subscriptions, NextResponse, CopyTo, and typed copying; it may instead
publish complete snapshots only. The pattern uses a full snapshot for every
SubscriptionRequest. Each stable SubscriptionData
variant's NextResponse admits reduced variants; each reduced variant's CopyTo
selects its destination under the
[shared copy rules](../reference/schema.md#shared-copyfrom-and-copyto-rules).
Apply each valid recipient update atomically under the pattern's acceptance
and ownership checks. The App must design its protocol and transactions for data consistency across
every permitted interaction and state-transition path, including concurrent
changes, derived fields, and the initial snapshot boundary. Acceptance and copy
checks do not establish application-level consistency. Request acknowledgement
remains separate from subscription publication.

Array updates may use [AppendTo and optional AppendIndex](../reference/protocols.md#subscriptiondata-array-append)
on incoming root arrays. AppendTo names exactly one `Variant.field`; that
SubscriptionData must admit the report. AppendIndex references a non-null integer
report root field and must equal the destination's pre-append length, even for
empty appends. Arrays are non-null; complete elements may contain assignable
nulls. Preserve order and duplicates; never pad, overwrite, or recover implicitly.
Skip annotated fields targeting other variants without replacement fallback.
Reject conflicting writers and invalid whole results atomically. Discard before
the initial snapshot. The App owns ordering, deduplication, retention, and fresh
snapshots; matching lengths do not prove matching contents. Live charts refresh
from the resulting working array; this adds no automatic scrolling.

Append reports may combine variant-level CopyTo. Require one CopyTo destination
per recipient and exclude append fields and index controls from its typed
projection. Index controls are also excluded from ordinary replacement. Reject
any actual copied write overlapping an appended array, including its contents;
retained omitted fields do not count as writes. Commit copies and appends as one
validated candidate.
