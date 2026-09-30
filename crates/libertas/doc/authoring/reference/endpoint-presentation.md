# Runtime endpoint presentation

This document defines the application-shell presentation shared by the Android
and iOS Libertas clients. It is not part of the chart schema grammar.

## Presentation decision

First apply [default request entry](protocols.md#default-request-entry-and-dynamic-actions).
The following chooser/workspace rules apply when explicit request selection is
needed; they do not bypass an explicit default request, default subscription, or an installed response's legal action set. Without a default entry, the request list is the landing; executable chart requests appear as previews instead of rows. The client resolves that presentation from the endpoint schema:

- an endpoint whose requests are all non-chart opens a protocol-only **Quick
  Actions** drawer;
- an endpoint with one or more chart-producing requests opens a full-height
  **Endpoint Workspace**. On a phone this is a navigation destination. A wide
  client may use a full-height pane, but content gestures never dismiss it.

Chart capability in the complete endpoint schema is the deterministic promotion
boundary. A chart-producing request promotes the endpoint even when its request
fields lack complete defaults and therefore cannot produce an automatic preview.
For this contract, chart-producing includes a response or subscription-data
path that resolves to either the `Chart` or the strictly parallel `Chart3D`
presentation namespace.
Loading state, default completeness, response success, chart row count,
orientation, and prior visits must not change the selected presentation after it
opens.

## Endpoint list with displayed subscription data

For a task with multiple endpoints, the endpoint list may display default
subscription data. That inline data also serves as its endpoint's landing.
Other landings, such as chart and workflow landings, stay unopened until the
endpoint is tapped and then stack above the list. A task with exactly one
endpoint enters its landing directly, with no endpoint-list screen in the stack. Tapping its header opens a request drawer. Use the displayed
value's legal NextRequest choices in their declared order, with existing access,
ExcludeUI, and applicable ConstrainedBy filtering. Construct the selected request
using the originating interaction's [reference context](protocols.md#interaction-reference-context).
Show the drawer even for one choice; with
no legal choices the header has no request action.

Do not reissue the default subscription or stack a duplicate data screen merely
because the endpoint header was tapped. This is action selection on an existing
destination, not initial endpoint entry. An endpoint with no displayed data and
a sole endpoint opened directly still follow the default-entry rules. Selecting
a drawer action retains the normal composer, reply, chart, and Back behavior.

## Endpoint Workspace

The workspace owns one vertical content scroll and a persistent title/back bar.
Because workspaces, previews, Chart Explorer, protocol data, and request
composers are navigation destinations, every full-height destination uses the
normal back affordance rather than a close affordance. Its section order is
invariant:

1. the top **Actions** section contains every non-chart request and every
   chart-producing request that lacks complete executable defaults;
2. the following **Charts** section contains the automatic previews whose
   chart-producing requests have complete executable defaults.

A request appears exactly once. A ready preview owns exactly one header rail.
That rail contains the chart contract title, an icon-only Configure action when
the request has editable variables, and the icon-only action that opens
Chart Explorer. There is no separate protocol-title/configure row above a ready
preview. `Nil` is never a variable. `Fixed` and `ReadOnly` values are not
variables, while `Default` only supplies an initial value and remains editable.
A request whose effective tree contains no editable variable is not configurable
and must not show Configure. Loading and unavailable previews may use the
localized request label, but their Configure action follows the same rule.
This request-payload rule does not reinterpret `Fixed` on a client Endpoint in
task configuration: there it constrains the peer task process while the
Endpoint-ID selector remains editable across all schema-compatible instances.

A chart preview is a live but gesture-neutral summary. It retains the chart
contract's deterministic initial viewport, clipping, category-density rules,
resampling disclosure, and overflow indicators. It accepts no direct pan,
zoom, selection scrub, or internal vertical/horizontal scrolling. A tap or the
localized open-chart control opens Chart Explorer. Only Chart Explorer owns
chart viewport manipulation, tooltips, guide overlays, and the complete
scrollable chart content.

For a 3D preview, the localized open-chart action is **Explore 3D chart**. It
opens a new full-screen destination without an inline camera mode. All direct 3D inspection and camera gestures
are installed only in the pushed Chart Explorer destination.

If a default chart cannot be rendered, its preview follows the shared
[chart failure policy](charts/2d.md#10-validation-and-fallback). The implementation
may provide an ordinary-data fallback or display an error message. Preserve the
endpoint workspace and its response ownership. The same choice applies to Chart
Explorer; an implementation may return a failed Explorer to its retained preview
and present the fallback or error there. Presentation failure never closes the
workspace or reissues the request.

Opening Chart Explorer or a chart preview pushes a new chart destination.
Leaving a full-screen chart pops exactly that destination and reveals the
previous chart or workspace instead of closing the complete interaction stack.
The universal Chart3D vocabulary calls these navigation operations
`fullscreen.open` and `fullscreen.close`; for a 3D chart the latter is performed
through the application shell's normal Back affordance rather than separate
chart Close chrome.

Configure pushes a separate request-composer screen above the retained chart
preview workspace. On a successful chart response, remove only the composer and
push Chart Explorer above the retained workspace. Do not replace the preview
with ProtocolData or insert a ProtocolData destination between the preview and
Explorer. Back returns to the same preview workspace. This applies to both 2D
and 3D charts on Android and iOS. Cancelling the composer leaves the workspace
intact; an error preserves the composer for retry without changing the preview.
The protocol transaction stack below governs protocol-data interactions, not
this preview-originated chart customization path.

Each automatic preview owns its [reference context](protocols.md#interaction-reference-context).
Configure uses that exact preview's context for `CopyFrom` and supported external
references under the [shared path contract](schema.md#context-and-syntax), including
its retained successful Response and current SubscriptionData when subscribed.
Do not substitute a sibling preview's data or the endpoint value that originally
opened the chooser after this preview receives its own data. Before it receives
any data, retain the chooser's originating reference context when available;
otherwise ordinary initialization with an empty context applies.

Rotation preserves navigation depth and the current interaction. Chart sizing,
clipping, overflow, and rejection follow the [2D](charts/2d.md) or
[3D](charts/3d.md) contract for the displayed data.

The workspace's vertical scroll therefore has one owner on every pointer path,
including paths that begin over a chart preview. Closing the workspace is an
explicit navigation action; downward content overscroll never closes it.

## Protocol transaction stack

A request opened from an existing screen is a **transient request composer**
stacked above that screen, including a request selected from an endpoint action
list. Cancelling before submission removes only the composer and preserves the
originating screen, its data, and its reference context. It does not run
landing navigation. While submission is pending, the composer remains the reply
owner and cannot be dismissed through native Back;
[exit while an operation is pending](protocols.md#exit-while-pending) states
the one exit. An Error keeps it available
for correction and retry; a non-transition acknowledgement removes only the
composer; a transitioning reply installs or updates the response destination
as described below. Subscription updates may continue to refresh their owning
screen underneath the composer. A [link](protocols.md#links) opens this composer;
above a stable editor its plain reply is applied to that editor as a
[return](protocols.md#link-return) rather than an acknowledgement.

A **stable endpoint destination** is an endpoint action/data screen, or the
request editor entered through EditRequest or WorkflowRequest. Initial automatic
landing entry may also create a stable request editor. Receiving a workflow
entry or installing a response ends the transient composer's role; subsequent
Back follows the stable destination's landing rules. Workflow steps replace
one active editor rather than building a history of editors.

Retain the endpoint landing beneath its working destination. Inline default
subscription data is the exception: its landing is already displayed in the
endpoint list, so retain that list without creating a duplicate landing screen.
For other multi-endpoint entries the stack is endpoint list → landing → optional
working screen. For a single-endpoint task it is landing → optional working
screen, with no endpoint list. At most one working destination exists above its
landing; successive results update or replace it. Back reveals the landing,
refreshing workflow landings but preserving live subscriptions; Back from a
separate landing returns to the list or exits a single-endpoint task.
See [landing ownership and Back](protocols.md#control-endpoint-landing-and-native-back).

On Android and iOS, ProtocolData displays the response or subscription data
owned by that screen. Its first content row represents the current Union
discriminator using its localized name, or the text of a Display Template
[`label` or `title`](presentation.md#labels-and-titles) rule on the variant. A
[promoted](presentation.md#tableheader-promoted-fields-and-union-selectors)
child appears on a separate row at the same indentation, preserving the
Union's identity.
An icon-only **Requests** button occupies the trailing end of that same row,
with a localized tooltip/accessibility label and a native-sized tap target.
Use the bulleted-list-in-a-rectangle glyph: Material `ListAlt` on Android and
SF Symbols `list.bullet.rectangle` on iOS. Do not duplicate this action in the
top bar.

Requests come from the displayed variant's effective `NextRequest`, preserving
declared order and existing visibility/access filtering. Promoting a
child changes presentation only; request resolution still belongs
to the enclosing protocol variant. With no eligible request, hide the button.
With exactly one, push its request builder directly without a chooser. With
multiple, show the next-request chooser, then open the selected request's
builder. Missing `NextRequest` never implies every request in the protocol.

The underlying ProtocolData screen remains intact while choosing or composing.
Cancelling returns to that same screen. Opening Requests does not itself change
its displayed data or subscription ownership. A request without editable input fields follows the
[single-action presentation rule](protocols.md#single-action-confirmation-and-input-presentation):
chosen from Requests or opened by a link, it is sent at once.
If the underlying data screen is the single-endpoint root, a transitioning
response for the default subscribed variant updates that root; it must not pop
the root as though another default-data screen existed underneath it.

The first transitioning transaction therefore follows this invariant sequence:
default protocol data -> Requests -> optional next-request chooser -> composer ->
send and wait -> response carrying a declared `NextRequest` -> one interactive
protocol-data destination. A sent composer remains the correlated response
owner while waiting. Receipt installs or updates the single interactive
destination with that response and exactly its declared next-request choices,
preserving the protocol interaction chain.

Outside workflows, a normal correlated Response with neither `WorkflowRequest`
nor a declared `NextRequest` is instead a
non-transition acknowledgement. It completes the in-flight attempt but does
not install a response destination, replace the displayed protocol value,
change the current choices, or interpret every
protocol Request as a choice. This is the ordinary shape for reusable
`Success`/`Failure` results. Transient request/wait presentation may finish,
but the protocol destination below it remains unchanged. Reference context
follows the [reference context rule](protocols.md#interaction-reference-context).

Apply these presentation rules under the
[shared protocol role rule](protocols.md#protocol-role-reuse). An Error-marked
correlated Response uses the separate out-of-band error procedure, preserving
retry and taking precedence over normal-response presentation.

A variant carrying both `Request` and `SubscriptionRequest` starts a
subscription. Later live data updates each eligible interaction associated with
that endpoint and compatible response type; it does not become an unrelated
interaction's reference context. `NextResponse`, when present, restricts the
eligible response variants. An acknowledgement leaves the displayed state and
subscription active; later matching live data can install or update that
interaction's SubscriptionData snapshot under the shared reference context rule.

On Hubs with explicit subscription release support (image version 20260901 or
later), leaving the last interactive screen for an explicit endpoint
subscription stops that client's explicit subscriptions for the endpoint.
Older Hubs release them when the Control connection closes. The endpoint-wide
scope can affect another explicit subscription for the same client and endpoint;
it does not stop the automatic default subscription. Reopen the interaction
when you need to subscribe again.

### Workflow editor destinations

The [workflow contract](protocols.md#workflow-interactions) defines correlated
EditRequest, automatic WorkflowRequest entry, PrevRequest construction, and
terminal response presentation. Keep one active workflow editor destination;
each successful step replaces it instead of accumulating historical editors.
Preserve the originating state-machine screen beneath the flow when one exists.
No cookie response is displayed between steps. A completed flow replaces its
editor with the final response, including responses with no NextRequest.

Display explicit Back and Next/Submit controls in a workflow. Back opens the
ordered previous-action chooser only when several eligible actions exist; Next
submits the current editor. Entry through either workflow path always waits
for user submission. Native screen Back/dismissal exits the flow; nested-editor
and picker dismissal return to the same parent editor. Those local movements
must not invoke PrevRequest or restore historical workflow drafts.

## Quick Actions

The Quick Actions drawer contains only protocol requests. Native swipe or
backdrop dismissal remains valid because the drawer contains no chart preview
or chart viewport. Selection dismisses the drawer before the selected request is
presented under the
[single-action presentation rule](protocols.md#single-action-confirmation-and-input-presentation),
and cancellation/selection completion occurs exactly once.

## Cross-platform requirement

Android and iOS use the same chart-capability promotion boundary, action-first
section order, single preview header, Configure visibility, deduplication,
localized actions, preview immutability, chart/protocol stack ownership,
Chart3D full-screen-only manipulation with no inline mode,
single-interaction-data rule, first-row Requests action and single-choice
builder routing, non-transition acknowledgement behavior, and the
[single-action presentation rule](protocols.md#single-action-confirmation-and-input-presentation). Platform
navigation components and animations may remain native; they may not change
these interaction semantics.
