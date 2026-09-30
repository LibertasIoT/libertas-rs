# Endpoint protocols and runtime

This page is the compact App-authoring projection of the full
[Endpoint protocols and runtime contract](../reference/protocols.md). Path
syntax and context belong to the
[Libertas Schema Path specification](../reference/schema.md#libertas-schema-path);
exact callback and function signatures belong to the installed public SDK.

## Declaring an Endpoint

- Apply `EndpointSchema` to a `LibertasEndpoint` field and reference a protocol
  Union. Add `EndpointServer` only when this App serves it.
  `EndpointBaseObjects` takes comma-delimited Libertas Schema Paths.
- On a client Endpoint, `Default` and `Fixed` take
  `developer-package::function`. Compatibility is always evaluated first.
  `Default` keeps every compatible server; `Fixed` restricts servers to the
  named task process and alone contributes its package to `AppDependencies`.
  Both selectors remain editable. If the Endpoint occurrence is non-nullable
  through every ancestor and every ancestor List requires an item, task
  creation waits for an eligible server, and either value names the peer task
  to provision ([dependency check](../reference/schema.md#62-task-pre-creation-static-dependency-check)).
- Every variant has at least one [role](../reference/protocols.md#protocol-roles)
  (`Request`, `Response`, `SubscriptionRequest`, `SubscriptionData`) and may
  have several. TypeScript uses a stable numeric first-field discriminator;
  Rust uses enum declaration order. `NextRequest`, `NextResponse`, and every
  other variant reference use native variant names.
- Keep one versionless protocol Union and append new cases only at the end.
  A protocol is a set of [capabilities](../reference/protocols.md#protocols-describe-capabilities):
  `ProtocolConformance` (`&`, `|`, `!`, `^` over native names) declares
  mandatory peer capability; matching payloads stay pass-through compatible;
  before sending an optional request, a client checks the selected server's
  runtime variant indices.
- [`AccessPrivilege`](../reference/protocols.md#request-access) on a
  Request/SubscriptionRequest is `Read=1`, `Write=2`, or `Read,Write=3`
  (the default). Explicitly mark every mutating operation `Write`.
- Every function permission needs a same-key nonempty package string-resource
  reason. A server Endpoint may declare downstream permissions without reasons;
  the selected client task must already hold them. Neither replaces
  `AccessPrivilege`.

## The four relations

Every runtime rule follows from four relations. The consequences listed under
each are the ones Apps get wrong; the linked owners hold the rest.

### 1. Reachability: a Request is reached only through a declaration that names it

| Declaration | Reached by |
| --- | --- |
| `NextRequest` on the displayed Response or SubscriptionData | the **Actions** surface, in declared order |
| `Link` on a displayed value's field or type (target also in that `NextRequest`) | a tap on that value's row |
| `Link` on a request-editor field (any Request) | the row's link button |
| `CalculationRequest` on an editor field | a change of that input; never Actions |
| `DefaultRequest` | endpoint entry, sent with no editor |
| `EditRequest` / `WorkflowRequest` reply | the server opening that editor |
| `PrevRequest` on the edited Request | the workflow Back button |
| none of the above | the request-list landing, only when the endpoint has no default entry |

Nothing is inferred: not from CopyFrom (declare the `Link` yourself on any
value a user should tap), not from an input-free payload, and not from a
missing `NextRequest` or an entry failure, which never offer every Request.
Visibility is not authorization; the backend re-checks access and state on
every submission. Modifiers:

- **Filter.** Response-level `ConstrainedBy("$.allowed_actions")` reads a
  hidden non-null integer array of original Union discriminants (not
  positions) and keeps membership only: `NextRequest` order stays, empty means
  no actions, duplicates add nothing, and missing/malformed/undeclared entries
  reject the update with no unrestricted fallback. Members carry explicit
  `Discriminator` 0..N−1 in pre-sorted order (gaps, duplicates, and misordering
  are rejected; missing values use declaration order; `Fixed` is never variant
  identity; stored schemas are never rewritten; see [Unions](../reference/schema.md#54-unions)).
  Access and `ExcludeUI` filter next; `ExcludeUI` removes a Request everywhere,
  links included. Links and menus use the same filtered set, so a Response
  wanting tappable rows declares no `ConstrainedBy`.
- **Lone survivor.** A filtered set of exactly one (not counting `LinkOnly`)
  is selected automatically once per accepted correlated Response; a
  subscription update re-filters and never selects
  ([action constraints](../reference/protocols.md#response-action-constraints)).
- **`LinkOnly`** keeps a Request only on its links: Actions, menus, landings,
  and the singleton count never offer it. It must be a link target, cannot be
  `DefaultRequest` or `ExcludeUI`, and older clients still show it in Actions
  ([link-only](../reference/protocols.md#link-only)).
- **Presentation by input.** A Request without editable input is sent at
  once when the person chooses it, by a link, from Actions, or from Quick
  Actions, with no confirmation; mark a critical Request
  `RequestConfirmation` and the person confirms in a Send/Cancel dialog before
  any send ([critical requests](../reference/protocols.md#request-confirmation)).
  Automatic singleton selection never sends. One
  primitive or enum field uses the one-field popup; complex input uses the
  full editor. Cancel sends nothing; a repeated choice cannot send twice
  ([single action](../reference/protocols.md#single-action-confirmation-and-input-presentation)).
- **`DefaultRequest`**: at most one, input-free (Nil, or only Hidden/ReadOnly
  `Fixed`/`Default` fields such as a hidden `Fixed("current-user")`), never
  chart-producing, `LinkOnly`, or `ExcludeUI`; it needs no reference context
  and owns entry. Without it, the first effective-Nil `SubscriptionRequest` is
  the sole automatic default subscription and every other is explicit; with
  neither, the landing lists Requests, previewing executable chart requests
  ([default entry](../reference/protocols.md#default-request-entry-and-dynamic-actions)).
- **Editor identity.** An editor composes exactly one variant, fixed by its
  entry point; it has no Actions surface, and a `Link` or a declared previous
  action is its only way to open another Request. A default request is sent
  without an editor ([variant identity](../reference/protocols.md#request-editor-variant-identity)).
  `PrevRequest` targets are listed in order and not filtered by
  `ConstrainedBy`: one goes directly, several open a chooser, none means no
  workflow Back.

### 2. Replies: replace the working screen or be consumed in place

| Reply | Effect |
| --- | --- |
| Response with `NextRequest` | replaces the working screen; its choices are exactly that `NextRequest` |
| `EditRequest` (a complete Request value) | installs a stable editor, replacing one even of the same variant |
| `WorkflowRequest` (Response naming one Request; no `NextRequest` or Error) | opens the named editor, never showing the Response |
| plain Response over a displayed Response or SubscriptionData | acknowledgement: the composer closes, the screen, its data and choices stay, the result message shows transiently; its CopyTo is not applied |
| plain Response to a link from an editor, or a calculation reply | [return](../reference/protocols.md#link-return): CopyTo assigns into the editor atomically, nothing navigates |
| Error (`@LibertasError`) or failure status | out-of-band: payload and wire form unchanged; the first renderable `FormattedText` shows in a dismissible dialog, so put the best explanation there; the composer or editor stays for retry |
| SubscriptionData report | updates the snapshot in place, even for a dual-role variant; never navigates |

Consequences:

- A plain Response after a workflow Submit or Prev ends the editor and is
  shown as a terminal screen, even without `NextRequest`
  ([workflows](../reference/protocols.md#workflow-interactions)). A response
  with no visible content presents only its actions, keeping hidden cookies.
- No editor is auto-submitted: EditRequest and WorkflowRequest editors wait
  for explicit submission, even when Nil or complete.
- A service that accepts a changed earlier page, before replying, recomputes
  and saves, erases, or keeps and marks unanswered each stored page the change
  affects ([saved-page rule](../reference/protocols.md#saved-page-consistency-after-historical-edits)).
- Never combine CopyTo with `NextRequest` or `WorkflowRequest` on a link
  target's reply. Every direct field of a returning or calculation reply has
  CopyTo or is Hidden. CopyTo never creates parents or extends arrays; an
  unavailable destination rejects the whole update; `Fixed` destinations are
  protected ([applying results](../reference/protocols.md#applying-calculation-results)).
- **Calculations.** A calculation reply is plain or Error: its NextResponse
  never names a variant with `NextRequest` or `WorkflowRequest`, or a Request
  ([calculations](../reference/protocols.md#field-calculation-requests)).
  Fixed inputs need no CopyFrom (a hidden
  `Fixed("current-user")` is resolved from the session and Hub-verified);
  every other input declares CopyFrom. A current-request source declares the
  calculation unless it is invariant: Hidden, ReadOnly or Fixed and no CopyTo writes
  it (a calculated allowed set is not invariant). Invariant and
  reference-context inputs need no declaration, never trigger from the editor,
  and resolve in the invoking field's context, so a page-level hidden list
  feeds a per-row calculation
  ([invariant inputs](../reference/protocols.md#invariant-calculation-inputs)).
  A change resets affected outputs
  through ordinary initialization. Deferred work is one ordered queue per
  editor with one entry per calculation type, holding the occurrences (such
  as List items) that await evaluation: a new change adds its occurrence and
  moves the type's entry to the tail, and types scheduled together enter in
  the order the changed field's `CalculationRequest` lists them; the in-flight invoking node stays immutable; an editor has at most
  one calculation request in flight, of any calculation or instance, and the
  rest wait in the queue; a change that schedules the in-flight occurrence
  again makes its reply obsolete, which is discarded and never queued again
  on its own; deferred requests are
  rebuilt and validated at dispatch. Submit waits for pending and deferred
  calculations to settle, then validates and sends; a calculation failure keeps
  the editor open
  ([deferred calculation](../reference/protocols.md#changes-resets-and-deferred-calculation)).
  Array removal follows dependency scope, not positions: cancel removed-item
  work, keep unaffected surviving item-local work, reschedule work crossing
  the item boundary or consuming the whole array or projection, and never
  retarget a stale positional write ([array removal](../reference/protocols.md#array-removal-and-calculation-scope)).
- **Calculated allowed sets.** A reply may CopyTo a field-level
  `ConstrainedBy` source; a parent-relative source (`^.allowed_x` in a List
  row) is written by the calculation bound to that row. Studio counts it as writing every constrained field,
  so it cannot read them, even indirectly; other calculations may also write
  them (the later applied assignment wins). Re-filtering resets an outside selection to
  unanswered (its Default applies only if the new set contains it; a Nullable
  field becomes unanswered, never null) and triggers its readers, then the
  singleton rule applies; an inside selection is untouched, and
  a source reset while pending never resets the selection
  ([allowed sets](../reference/protocols.md#calculated-allowed-sets)).
- **Server side.** Branch on action before decoding; signals carry no payload;
  peer handles are opaque. Start or renew a stream only on the current
  subscription callback, answer it with a correlated complete snapshot (exact
  peer and transaction), and use reports only for later changes; a report
  alone leaves the request unanswered. A complete snapshot is always a
  Response; a variant shared with later complete reports needs both Response
  and SubscriptionData roles, and a partial report variant needs
  SubscriptionData only. Use `PeerAlive`, not a "no change"
  payload ([runtime rules](../reference/protocols.md#endpoint-runtime-rules)).
- **Report shapes.** SubscriptionData A with `NextResponse(B,C)` accepts
  directional root-field updates from B/C, keeping A's screen, actions, and
  unmatched fields; all are non-null Tables with assignable matching fields,
  applied atomically. The Hub routes without retaining or merging payloads;
  the App chooses whether to send such reports and owns consistency and
  ordering ([compatible updates](../reference/protocols.md#compatible-subscriptiondata)).
  The [reduced pattern](../reference/protocols.md#reduced-subscriptiondata-design-pattern)
  is optional (complete snapshots alone suffice): stable variants' `NextResponse`
  admits reduced variants, whose CopyTo picks the destination under the
  [shared copy rules](../reference/schema.md#shared-copyfrom-and-copyto-rules);
  each valid update applies atomically; acceptance checks do not prove
  application consistency, which the App designs across concurrent changes,
  derived fields, and the initial snapshot. Request acknowledgement stays
  separate from subscription publication.
- **Append.** [`AppendTo`](../reference/protocols.md#subscriptiondata-array-append)
  names one `Variant.field` whose SubscriptionData admits the report; optional
  `AppendIndex` names a non-null integer root field equal to the pre-append
  length, even for empty appends. Arrays are non-null (elements may hold
  assignable nulls); order and duplicates are kept; nothing is padded, overwritten, or recovered; fields targeting
  other variants are skipped; conflicting writers or an invalid result reject
  atomically; reports before the initial snapshot are discarded. The App owns
  ordering, deduplication, retention, and fresh snapshots; matching lengths do not prove matching contents. Live charts
  refresh without auto-scroll. With variant CopyTo, one destination per
  recipient excludes append fields and index controls (which ordinary replacement
  also skips); an actual copied write overlapping an appended array or its
  contents is rejected (retained omitted fields are not writes); copies and appends commit as one candidate.

### 3. Copies: a target is built from the reference context

CopyFrom follows the [shared copy rules](../reference/schema.md#shared-copyfrom-and-copyto-rules)
and the [context table](../reference/schema.md#context-and-syntax); there is
no per-action grammar, stored previous request, or payload cache. The
[reference context](../reference/protocols.md#interaction-reference-context)
holds at most two separate objects of the current interaction:

- over a displayed Response: the latest acknowledgement, then that Response;
- in a subscription: the latest successful correlated Response, then the
  snapshot; an EditRequest is its own reference object.

A transitioning reply replaces both; an acknowledgement replaces only the
first; Error changes neither; older responses are never searched. First entry
and Back-triggered landing refresh have no context, so reads handle null
([availability](../reference/schema.md#availability-null-and-interaction-ownership)).

- **From an editor** (calculation, editor link): the current draft first, then
  the context. The draft is read, never sent; only the constructed target must
  be complete and valid. `PrevRequest` copies from the current editor's
  payload; an incomplete or invalid Prev is not sent.
- **Binding.** A typed selector such as `^<TopicRow>.section` binds the
  nearest self-or-ancestor occurrence on the tapped node's or host field's
  path, at any depth; with no tap (Actions, menus, lone survivor) the next
  alternative applies. Strict `^<Variant.field.item>.field` starts at the
  active variant, whose root is selectable even when primitive; no lookup
  crosses into the protocol Union. Studio validates every invoking occurrence;
  clients keep the original bindings, never a variant name alone
  ([typed ancestors](../reference/protocols.md#typed-ancestor-paths-and-per-item-calculations)).
- **Hidden context.** Carry session, continuation, and initialize-later values
  in typed Hidden fields (or a Hidden container) and copy them into receiving
  requests: coordination fields stay Hidden, user-editable ones stay visible;
  an EditRequest already carries them. Supply them on every construction path
  and across every context replacement. Hidden neither initializes nor exempts
  a field from validation, and is not authorization: validate submitted
  context on the server ([hidden context](../reference/protocols.md#hidden-protocol-context)).

### 4. Screens: one landing, one working screen, one composer

A task with several endpoints opens an endpoint list; only default
subscription data shown inline there is also a landing (never duplicate it),
and other landings stack above the list on tap. A single endpoint enters its
landing directly with no list. Above the landing sits at most one working
screen, which successive results replace or update, and above that at most one
[transient composer](../reference/endpoint-presentation.md#protocol-transaction-stack).
An editor has at most one pending Submit, Prev, or link; the pending composer
owns its reply, and within a Control session replies apply in
[send order](../reference/protocols.md#request-and-response-ordering). For a person's client ([human-task interaction](../reference/protocols.md#human-task-interaction);
software may agree its own rules with the service), interaction is strictly ordered, request by request, except
SubscriptionData. A pending Request is never abandoned on its own: dismissing
anything that awaits a reply leaves the endpoint. A person may leave an
endpoint at any time; late replies are discarded, and entering again repeats
the entry request, so admit it in every state the endpoint can be left in. A
lost connection clears every endpoint's data; reconnecting starts from entry.

- Cancel pops only the composer: nothing is sent and the screen and context
  stay.
- Native Back reveals the layer beneath: from a working screen, the landing
  (a workflow landing refreshes in place with no reference context; a live
  subscription is kept); from a separate landing, the list, or exit for a
  single endpoint. Back sends no `PrevRequest`, undoes nothing accepted, and
  fences abandoned replies, calculations, and subscriptions
  ([landing and Back](../reference/protocols.md#control-endpoint-landing-and-native-back)).
  [CacheIdentity](../reference/annotations.md#cacheidentity) governs draft
  recovery; no partial request is ever sent.

## Persistent data schemas

Default Data API failure: the Hub runtime stops the task, reports the runtime
Error with an internal data-failure classification, retains the persisted
enabled state, and never restarts it automatically. Only an explicit function
`DataErrorStatus` permits checked status-returning calls; those Apps own
consistency after failure. Absence is not failure. Any restart uses a
third-party `Enable` action and ordinary startup
([owner](../reference/protocols.md#persistence-failure-and-the-no-status-api)).

One function may reference one versionless persistent-data Union:
`@LibertasDataSchema(Type)` on the TypeScript function (no standalone Union
marker) or `#[libertas_data_schema(Type)]` in Rust. Append new variants only
at the end. Prefer structured payloads; persisting `FormattedText` is permitted
but discouraged ([payload guidance](../reference/presentation.md#persisting-formatted-record-payloads)),
which does not apply to DataName names.

Every single-record or indexed-table name is the canonical byte encoding of one
package `StringResources` key plus ordered typed message arguments. Reuse that
exact tuple for the same identity; never use rendered, literal, separately
maintained, or Base64 names
([data names](../reference/message-format.md#data-and-database-names)).
